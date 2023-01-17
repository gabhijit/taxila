use tokio::signal::unix::{signal, SignalKind};
use tokio::sync::mpsc::{self, Sender};

use crate::config::AmfConfig;
use crate::messages::{AmfToNgapMessage, NgapToAmfMessage};
use crate::ngap::ngap_manager::NgapManager;

pub struct Amf {
    config: AmfConfig,
    amf_to_ngap_tx: Option<Sender<AmfToNgapMessage>>,
}

impl Amf {
    pub fn from_config(config: AmfConfig) -> std::io::Result<Self> {
        Ok(Self {
            config,
            amf_to_ngap_tx: None,
        })
    }

    pub async fn run(mut self) -> std::io::Result<()> {
        log::info!(
            "Started AMF: PLMN: [MCC:({}), MNC:({})], TACs:{:?}",
            self.config.plmn.mcc,
            self.config.plmn.mnc,
            self.config.tacs
        );

        let mut sigterm = signal(SignalKind::terminate())?;
        let mut sigint = signal(SignalKind::interrupt())?;

        let (amf_to_ngap_tx, amf_to_ngap_rx) = mpsc::channel(10);
        self.amf_to_ngap_tx = Some(amf_to_ngap_tx);

        let (ngap_to_amf_tx, mut ngap_to_amf_rx) = mpsc::channel::<NgapToAmfMessage>(10);

        let ngap = NgapManager::from_config(self.config.clone())?;
        let ngap_task = tokio::spawn(NgapManager::run(ngap, amf_to_ngap_rx, ngap_to_amf_tx));

        loop {
            tokio::select! {
                Some(_) = ngap_to_amf_rx.recv() => {
                }
                _ = sigterm.recv() => {
                    log::warn!("Received SIGTERM Sending to all threads.");
                    _ = self.amf_to_ngap_tx.as_ref().unwrap().send(AmfToNgapMessage::Signal(15)).await;
                    break;
                }
                _ = sigint.recv() => {
                    log::warn!("Received INT Sending to all threads.");
                    _ = self.amf_to_ngap_tx.as_ref().unwrap().send(AmfToNgapMessage::Signal(15)).await;
                    break;
                }
            }
        }

        log::info!("Program Closing waiting for the tasks to finish!");
        let _ = ngap_task.await?;

        log::info!("Closing the main application task for AMF.");
        Ok(())
    }
}
