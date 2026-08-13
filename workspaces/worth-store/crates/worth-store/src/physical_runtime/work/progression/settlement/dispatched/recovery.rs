use worth_store_physical_backend::{
    CompletedRecoveryStagingWrite, IndeterminateRecoveryStagingWrite,
};

use super::DispatchedPhysicalWork;
use crate::physical_runtime::work::PhysicalWorkOperationFamily;

impl DispatchedPhysicalWork {
    pub(in crate::physical_runtime) fn matches_recovery_staging(
        &self,
        physical: &CompletedRecoveryStagingWrite,
    ) -> bool {
        let coordinate = physical.coordinate();
        let common = Some(coordinate) == self.coordinate()
            && self.payload_digest == Some(physical.payload_digest())
            && physical.byte_count() == u64::from(coordinate.length())
            && matches!(
                self.intent().operation(),
                PhysicalWorkOperationFamily::ArtifactRangeWrite
                    | PhysicalWorkOperationFamily::RootPublication
            );
        if !common {
            return false;
        }
        match (physical.created(), physical.verified()) {
            (Some(created), None) => self.matches_new_artifact_binding(created, coordinate),
            (None, Some(verified)) => {
                verified.store() == self.intent().identity().store()
                    && verified.owner()
                        == self.admitted.authority().media_owner_observation().owner()
                    && verified.coordinate() == coordinate
                    && verified.completed_bytes() == u64::from(coordinate.length())
            }
            _ => false,
        }
    }

    pub(in crate::physical_runtime) fn matches_recovery_staging_indeterminate(
        &self,
        physical: &IndeterminateRecoveryStagingWrite,
    ) -> bool {
        self.coordinate().is_some_and(|coordinate| {
            physical.artifact() == coordinate.artifact()
                && self.matches_new_artifact_indeterminate_binding(physical.physical(), coordinate)
        })
    }
}
