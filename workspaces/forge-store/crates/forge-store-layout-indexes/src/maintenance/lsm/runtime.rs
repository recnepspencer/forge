use super::{
    denial::LsmMaintenanceAdmissionDenied,
    request::{
        LsmCompactionAdmissionRequest, LsmReplayAdmissionRequest, LsmRunPublicationAdmissionRequest,
    },
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
    ) -> Result<crate::BaselineLsmRunPublicationAdmission, LsmMaintenanceAdmissionDenied> {
        super::run_publication::admit(request)
    }

    pub fn admit_replay(
        self,
        request: LsmReplayAdmissionRequest<'_>,
    ) -> Result<crate::BaselineLsmReplayAdmission, LsmMaintenanceAdmissionDenied> {
        super::replay::admit(request)
    }

    pub fn admit_compaction(
        self,
        request: LsmCompactionAdmissionRequest<'_>,
    ) -> Result<crate::BaselineLsmCompactionAdmission, LsmMaintenanceAdmissionDenied> {
        super::compaction::admit(request)
    }
}
