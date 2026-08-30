#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiServiceProposalCertificationSnapshot {
    proposals: u16,
    occupancy_leases: u16,
    cancellation_records: u16,
    stage_receipts: u16,
    live_occupancies: usize,
    live_cancellations: usize,
}

pub trait WorthUiServiceProposalCertificationExt {
    fn inspect_service_proposals_for_certification(&self)
        -> UiServiceProposalCertificationSnapshot;
}

impl WorthUiServiceProposalCertificationExt for crate::facade::WorthUiActiveApplicationSession {
    fn inspect_service_proposals_for_certification(
        &self,
    ) -> UiServiceProposalCertificationSnapshot {
        crate::facade::WorthUiActiveApplicationSession::inspect_service_proposals_for_certification(
            self,
        )
    }
}

impl UiServiceProposalCertificationSnapshot {
    pub(crate) const fn new(
        proposals: u16,
        occupancy_leases: u16,
        cancellation_records: u16,
        stage_receipts: u16,
        live_occupancies: usize,
        live_cancellations: usize,
    ) -> Self {
        Self {
            proposals,
            occupancy_leases,
            cancellation_records,
            stage_receipts,
            live_occupancies,
            live_cancellations,
        }
    }

    pub const fn is_zero(self) -> bool {
        self.proposals == 0
            && self.occupancy_leases == 0
            && self.cancellation_records == 0
            && self.stage_receipts == 0
            && self.live_occupancies == 0
            && self.live_cancellations == 0
    }

    pub const fn entries(self) -> [(&'static str, u64); 6] {
        [
            ("proposals", self.proposals as u64),
            ("occupancy_leases", self.occupancy_leases as u64),
            ("cancellation_records", self.cancellation_records as u64),
            ("stage_receipts", self.stage_receipts as u64),
            ("live_occupancies", self.live_occupancies as u64),
            ("live_cancellations", self.live_cancellations as u64),
        ]
    }
}
