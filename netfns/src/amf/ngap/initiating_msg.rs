use sctp_rs::AssociationId;

use ngap::messages::r17::{InitiatingMessage, InitiatingMessageValue};

use super::ngap_manager::NgapManager;

impl NgapManager {
    pub(super) async fn process_initiating_message(
        &mut self,
        id: AssociationId,
        sid: u16,
        init: InitiatingMessage,
    ) -> std::io::Result<()> {
        match init.value {
            InitiatingMessageValue::Id_NGSetup(ng_setup_req) => {
                self.process_ng_setup_request(id, ng_setup_req).await
            }
            InitiatingMessageValue::Id_InitialUEMessage(initial_ue_message) => {
                self.process_initial_ue_message(id, sid, initial_ue_message)
                    .await
            }
            _ => {
                log::error!("Unsupported Message received: {:?}", init.procedure_code);
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Unsupported Initiating Message".to_string(),
                ))
            }
        }
    }
}
