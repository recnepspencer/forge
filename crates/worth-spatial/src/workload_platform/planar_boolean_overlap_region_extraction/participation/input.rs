use crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanLoopReconstructionParticipationSupport;

use super::super::readiness_boundary::PlanarBooleanOverlapRegionExtractionRequest;

pub struct PlanarBooleanOverlapParticipationRecoveryInput<'a> {
    request: &'a PlanarBooleanOverlapRegionExtractionRequest,
    loop_participation_support: &'a PlanarBooleanLoopReconstructionParticipationSupport,
}

impl<'a> PlanarBooleanOverlapParticipationRecoveryInput<'a> {
    pub fn from_request_and_loop_support(
        request: &'a PlanarBooleanOverlapRegionExtractionRequest,
        loop_participation_support: &'a PlanarBooleanLoopReconstructionParticipationSupport,
    ) -> Self {
        Self {
            request,
            loop_participation_support,
        }
    }

    pub(crate) fn request(&self) -> &'a PlanarBooleanOverlapRegionExtractionRequest {
        self.request
    }

    pub(crate) fn loop_participation_support(
        &self,
    ) -> &'a PlanarBooleanLoopReconstructionParticipationSupport {
        self.loop_participation_support
    }
}
