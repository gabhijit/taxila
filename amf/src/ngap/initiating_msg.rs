use sctp_rs::AssociationId;

use ngap::messages::r17::{InitiatingMessage, InitiatingMessageValue};

use super::ngap_manager::NgapManager;

impl NgapManager {
    pub(super) fn process_initiating_message(&self, id: AssociationId, init: InitiatingMessage) {
        match init.value {
            InitiatingMessageValue::Id_NGSetup(ng_setup_req) => {
                self.process_ng_setup_request(id, ng_setup_req)
            }
            _ => (),
        }
    }
}
