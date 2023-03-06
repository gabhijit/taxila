use std::collections::HashMap;

use asn1_codecs::{aper::AperCodec, PerCodecData};

use tokio::sync::mpsc::{self, Receiver, Sender};

use sctp_rs::{
    AssociationId, BindxFlags, Listener, SendData, SendInfo, Socket, SocketToAssociation,
};

use ngap::messages::r17::NGAP_PDU;
use ngap::messages::r17::{GlobalRANNodeID, SupportedTAList};
use ngap::messages::r17::{RRCEstablishmentCause, UEContextRequest, UserLocationInformation};

use crate::amf::config::AmfConfig;
use crate::amf::messages::{
    AmfToNgapMessage, NgapMgrToRanConnMessage, NgapToAmfMessage, RanConnToNgapMgrMessage,
    ReceivedDataMessage, SendDataMessage,
};

use super::ran_connection::{RanConnection, NGAP_SCTP_PPID};

const NGAP_SCTP_PORT: u16 = 38412;
const NGAP_INPUT_STREAMS: u16 = 100;
const NGAP_OUTPUT_STREAMS: u16 = 100;

//`NgapRanUe`: Structure representing the Ngap specific information about the UE.
pub(crate) struct NgapRanUe {
    ran_ngap_ue_id: u32,                    // (Received during `InitialUEMessage`)
    amf_ngap_ue_id: u64,                    // (Generated by us)
    input_stream: u16,                      // (Input stream for UE associated signaling).
    output_stream: u16,                     // (Output stream for UE associated signaling)
    user_location: UserLocationInformation, // User Location, received in the Initial UE Message
    rrc_establishment_cause: Option<RRCEstablishmentCause>, // RRC Establishment Cause
    ue_context_requested: Option<UEContextRequest>, // RRC Establishment Cause
}

// RanNode: A structure of this type is maintained for each of the RAN Node that is connected to
// this AMF.
//
// The RAN Node can be of type `GNB` or `N3IWF`. (Currently only GNB Types supported.)
#[allow(dead_code)]
pub(crate) struct RanNode {
    pub(crate) ran_node_id: Box<GlobalRANNodeID>, // Boxed: To keep the size in check
    pub(crate) supported_ta_list: Box<SupportedTAList>, // Boxed: To keep the size in check
    pub(crate) name: String,
    pub(crate) sctp_id: AssociationId,
    pub(crate) ngsetup_success: bool,
    pub(crate) ran_ues: HashMap<u32, NgapRanUe>, // Associating RanUe with ran_ue_ngap_id
}

impl std::fmt::Display for RanNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let gnb_name = if self.name.is_empty() {
            "EmptyGNBName".to_string()
        } else {
            self.name.clone()
        };

        write!(f, "{}({})", gnb_name, self.sctp_id)
    }
}

// NgapManager: Structure representing the NGAP Handling for the AMF.
//
// Whenever a new connection arrives on the listening socket, `NgapManager` spawns a task for
// processing the connection. A Map of 'AssociationId' -> 'Sender' (channel Sender) is maintained
// by the NgapManager. Whenever a message is received from the 'Amf', it will have a header
// containing the `AssociationID`, which determines the channel to be used  for sending the message
// to the 'GNB'. A message with `AssociationID` of '0' is a special control message. 'AMF' will use
// this ID for sending Control messages to 'NgapManager'. Such control messages can be used for
// performing graceful shutdown etc.
pub(crate) struct NgapManager {
    pub(crate) config: AmfConfig,
    socket: Listener,
    pub(crate) ran_connections: HashMap<AssociationId, Sender<NgapMgrToRanConnMessage>>, // associating SCTP Association ID to RanConnection via channel
    pub(crate) ran_nodes: HashMap<AssociationId, RanNode>, // Associating RanNode with SCTP Association ID
    pub(crate) next_ue_stream: u16,                        // Used for generating next stream ID
    pub(crate) next_amf_ngap_ue_id: u64,                   // Used for generating next stream ID
    pub(crate) amf_ues: HashMap<u64, (AssociationId, u32)>, // Associating an amf_ue_ngap_id with ran_ue_ngap_id
    pub(crate) ngap_to_amf_tx: Option<Sender<NgapToAmfMessage>>,
}

impl NgapManager {
    pub(crate) fn from_config(config: AmfConfig) -> std::io::Result<Self> {
        let socket = Socket::new_v6(SocketToAssociation::OneToOne)?;

        let port = if config.ngap.port.is_some() {
            config.ngap.port.unwrap()
        } else {
            NGAP_SCTP_PORT
        };

        let mut bind_addrs = vec![];
        for addr in &config.ngap.addrs {
            let bind_addr = if addr.is_ipv6() {
                format!("[{}]:{}", addr, port).parse().unwrap()
            } else {
                format!("{}:{}", addr, port).parse().unwrap()
            };
            bind_addrs.push(bind_addr);
        }

        socket.sctp_bindx(&bind_addrs, BindxFlags::Add)?;
        let ostreams = if config.ngap.output_streams.is_some() {
            config.ngap.output_streams.unwrap()
        } else {
            NGAP_OUTPUT_STREAMS
        };
        let istreams = if config.ngap.input_streams.is_some() {
            config.ngap.input_streams.unwrap()
        } else {
            NGAP_INPUT_STREAMS
        };
        socket.sctp_setup_init_params(ostreams, istreams, 0, 0)?;

        // TODO: Make it configurable
        let socket = socket.listen(100)?;

        Ok(Self {
            config,
            socket,
            ran_connections: HashMap::new(),
            ran_nodes: HashMap::new(),
            next_ue_stream: 1,
            next_amf_ngap_ue_id: 1,
            amf_ues: HashMap::new(),
            ngap_to_amf_tx: None,
        })
    }

    // Main `NgapManager` functions. uses `tokio::select!` for waiting for data on one of the
    // channels, and then handles the data.
    //
    // Data received on the rx channel connecting to `RanConnection` tasks, contains NGAP Messages.
    // The messages are decoded and handled.
    //
    // Data received oon the rx channel connecting to `Amf` will receive the signals received by
    // the main `Amf::run` task and will be handled for graceful shutdown etc.
    pub(in crate::amf) async fn run(
        mut self,
        mut amf_to_ngap_rx: Receiver<AmfToNgapMessage>,
        ngap_to_amf_tx: Sender<NgapToAmfMessage>,
    ) -> std::io::Result<()> {
        let _ = self.ngap_to_amf_tx.replace(ngap_to_amf_tx);

        let (tx, mut rx) = mpsc::channel::<RanConnToNgapMgrMessage>(10);
        let mut tasks = vec![];
        loop {
            tokio::select! {
                accepted = self.socket.accept() => {
                    let (accepted, client_addr) = accepted?;
                    let conn_status = accepted.sctp_get_status(0)?;

                    let (ngap_to_ranconn_tx, ngap_to_ranconn_rx) = mpsc::channel(10);

                    let gnb_connection = RanConnection::new(
                        conn_status.assoc_id,
                        accepted,
                        client_addr,
                        tx.clone(),
                        ngap_to_ranconn_rx
                    );

                    self.ran_connections.insert(conn_status.assoc_id, ngap_to_ranconn_tx);

                    log::info!(
                        "Spawning New Task for GNB: (Association:{}, ClientAddress: {}).",
                        conn_status.assoc_id,
                        client_addr
                    );
                    tasks.push(tokio::spawn(
                        RanConnection::handle_new_connection(gnb_connection))
                    );
                    log::debug!("spawned task!");
                }
                Some(RanConnToNgapMgrMessage::ReceivedData(
                        ReceivedDataMessage { id, rxdata}
                    )) = rx.recv() => {
                    let mut codec_data =
                    PerCodecData::from_slice_aper(&rxdata.payload);
                    let pdu = NGAP_PDU::aper_decode(&mut codec_data).unwrap();
                    let sid = if rxdata.rcv_info.is_some() {
                        rxdata.rcv_info.as_ref().unwrap().sid
                    } else {
                        log::warn!("RecvInfo not received assuming default stream ID 0.");
                        0
                    };
                    let result = match pdu {
                        NGAP_PDU::InitiatingMessage(init) => self.process_initiating_message(id, sid, init).await,
                        NGAP_PDU::SuccessfulOutcome(success) => {
                            self.process_successful_outcome(id, sid, success)
                        }
                        NGAP_PDU::UnsuccessfulOutcome(failure) => {
                            self.process_unsuccessful_outcome(id, sid, failure)
                        }
                    };
                    if result.is_err() {
                        log::error!("Error Processing NGAP Message: {:#?}" , result.err().unwrap());
                    }
                }
                Some(_amf_data) = amf_to_ngap_rx.recv() => {
                    log::warn!("Signal Received from AMF.");
                    log::debug!("Sending close to all RAN Connections.");
                    for (_k, v) in self.ran_connections {
                        let _ = v.send(NgapMgrToRanConnMessage::Signal(15)).await;
                    }
                    break ;
                }
            }
            log::debug!("select loop completed..");
        }

        log::warn!("Waiting for All the Ran Connection Tasks to finish.");
        futures::future::join_all(tasks).await;
        log::warn!("Closing NgapManager task!");

        Ok(())
    }

    pub(in crate::amf::ngap) async fn ngap_send_pdu(
        &self,
        id: AssociationId,
        pdu: NGAP_PDU,
        ran_ue_id: Option<u32>,
    ) -> std::io::Result<()> {
        log::debug!("Sending PDU to `RanConnection` task to send to RAN Node.");

        log::trace!("PDU: {:#?}", pdu);

        let mut codec_data = PerCodecData::new_aper();
        let result = pdu.aper_encode(&mut codec_data); // TODO: Handle error
        log::debug!("Result: encode: {:#?}", result);
        let data = codec_data.get_inner().unwrap();

        let snd_info = if ran_ue_id.is_some() {
            let ran_node = self.ran_nodes.get(&id).unwrap();
            let ran_ue = ran_node.ran_ues.get(&ran_ue_id.unwrap());
            let sid = if ran_ue.is_none() {
                log::warn!("RAN UE Node not found, using NON-UE-SIGNALING Stream ID");
                0
            } else {
                let ran_ue = ran_ue.unwrap();
                ran_ue.output_stream
            };

            Some(SendInfo {
                sid,
                ppid: NGAP_SCTP_PPID,
                flags: 0,
                assoc_id: id,
                context: 0, // TODO: Use context later
            })
        } else {
            None
        };

        let senddata = NgapMgrToRanConnMessage::SendData(SendDataMessage {
            txdata: SendData {
                payload: data,
                snd_info,
            },
            _id: id,
        });

        let tx = self.ran_connections.get(&id).unwrap();
        // TODO : Handle Error.
        if let Err(e) = tx.send(senddata).await {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Error Tx: `NgapMgr` -> `RanConnection`:{}", e),
            ))
        } else {
            Ok(())
        }
    }

    // Returns the `amf_ue_ngap_id` created to the caller - so caller can use it.
    pub(in crate::amf::ngap) fn add_ran_ue(
        &mut self,
        id: AssociationId,
        input_stream: u16,
        ran_ngap_ue_id: u32,
        user_location: UserLocationInformation,
        ue_context_requested: Option<UEContextRequest>,
        rrc_establishment_cause: Option<RRCEstablishmentCause>,
    ) -> u64 {
        let amf_ngap_ue_id = self.next_amf_ngap_ue_id;
        let ran_ue = NgapRanUe {
            ran_ngap_ue_id,
            input_stream,
            output_stream: self.next_ue_stream,
            amf_ngap_ue_id,
            user_location,
            ue_context_requested,
            rrc_establishment_cause,
        };
        let mut ran_node = self.ran_nodes.get_mut(&id).unwrap();
        ran_node.ran_ues.insert(ran_ngap_ue_id, ran_ue);
        self.amf_ues.insert(amf_ngap_ue_id, (id, ran_ngap_ue_id));

        log::info!(
            "Added New Ran UE: ran_ngap_ue_id:{}, ran_amf_ue_id:{}, output_stream: {}",
            ran_ngap_ue_id,
            self.next_amf_ngap_ue_id,
            self.next_ue_stream
        );

        // Send the NAS
        // TODO: Get a proper pool allocator
        self.next_ue_stream += 1;
        self.next_amf_ngap_ue_id += 1;

        amf_ngap_ue_id
    }
}
