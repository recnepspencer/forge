use super::WorthUiApplicationSessionState;

#[path = "service_proposal/portal.rs"]
mod portal;
#[path = "service_proposal/settlement.rs"]
mod settlement;
#[path = "service_proposal/terminal.rs"]
mod terminal;

use portal::UiPortalProposalSettlement;
pub(crate) use portal::{
    UiIndeterminatePortalProposalTransaction, UiPortalProposalPreparation,
    UiPortalProposalPreparationDenial, UiStagedPortalProposalTransaction,
};

impl WorthUiApplicationSessionState {
    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn inspect_service_proposal_resources_for_certification(
        &self,
    ) -> ([u16; 4], usize, usize) {
        let entries = self.runtime.service_proposals.census().entries();
        (
            [entries[0].1, entries[1].1, entries[2].1, entries[3].1],
            self.runtime.service_proposals.live_occupancy_count(),
            self.runtime.service_proposals.live_cancellation_count(),
        )
    }
}
