use super::request::{
    LsmCompactionAdmissionRequest, LsmReplayAdmissionRequest, LsmRunPublicationAdmissionRequest,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutLsmMaintenance;

pub const fn layout_lsm_maintenance() -> LayoutLsmMaintenance {
    LayoutLsmMaintenance
}

impl LayoutLsmMaintenance {
    pub fn admit_run_publication(
        self,
        request: LsmRunPublicationAdmissionRequest<'_>,
    ) -> super::LsmRunPublicationAdmissionOutcome {
        super::run_publication::admit(request)
    }

    pub fn admit_replay(
        self,
        request: LsmReplayAdmissionRequest<'_>,
    ) -> super::LsmReplayMaintenanceAdmissionOutcome {
        super::replay::admit(request)
    }

    pub fn admit_compaction(
        self,
        request: LsmCompactionAdmissionRequest<'_>,
    ) -> super::LsmCompactionMaintenanceAdmissionOutcome {
        super::compaction::admit(request)
    }
}
