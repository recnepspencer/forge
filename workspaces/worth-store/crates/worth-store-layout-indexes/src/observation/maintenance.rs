use super::owner_case::{sealed, ObserveOwnerCase, OwnerCaseObservation};
use crate::maintenance::{
    CopyOnWriteLayoutMutationExecutionCaseId, CopyOnWriteLayoutMutationExecutionOutcome,
    DerivedIndexParityCaseId, DerivedIndexParityOutcome, DerivedIndexRebuildAdmissionCaseId,
    DerivedIndexRebuildAdmissionOutcome, DerivedIndexRebuildExecutionCaseId,
    DerivedIndexRebuildOutcome, ExactBTreePublicationCaseId, ExactBTreePublicationOutcome,
    LayoutMutationAdmissionCaseId, LayoutMutationAdmissionOutcome, LiveExactMaintenanceCaseId,
    LiveExactMaintenanceOutcome, LiveMaintenancePostureCaseId, LiveMaintenancePostureOutcome,
};

macro_rules! observe_owner_case {
    ($outcome:ty => $case_id:ty) => {
        impl sealed::Sealed for $outcome {}

        impl ObserveOwnerCase for $outcome {
            type CaseId = $case_id;

            fn owner_case_observation(&self) -> OwnerCaseObservation<Self::CaseId> {
                OwnerCaseObservation::issued(self.case_id())
            }
        }
    };
}

observe_owner_case!(ExactBTreePublicationOutcome => ExactBTreePublicationCaseId);
observe_owner_case!(LiveMaintenancePostureOutcome => LiveMaintenancePostureCaseId);
observe_owner_case!(LayoutMutationAdmissionOutcome => LayoutMutationAdmissionCaseId);
observe_owner_case!(LiveExactMaintenanceOutcome => LiveExactMaintenanceCaseId);
observe_owner_case!(DerivedIndexParityOutcome => DerivedIndexParityCaseId);
observe_owner_case!(DerivedIndexRebuildAdmissionOutcome => DerivedIndexRebuildAdmissionCaseId);
observe_owner_case!(DerivedIndexRebuildOutcome => DerivedIndexRebuildExecutionCaseId);
observe_owner_case!(CopyOnWriteLayoutMutationExecutionOutcome => CopyOnWriteLayoutMutationExecutionCaseId);
