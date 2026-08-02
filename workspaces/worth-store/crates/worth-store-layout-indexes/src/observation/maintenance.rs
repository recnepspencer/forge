use super::owner_case::{sealed, ObserveOwnerCase, OwnerCaseObservation};
use crate::maintenance::{
    DerivedIndexParityCaseId, DerivedIndexParityOutcome, DerivedIndexRebuildAdmissionCaseId,
    DerivedIndexRebuildAdmissionOutcome, DerivedIndexRebuildExecutionCaseId,
    DerivedIndexRebuildOutcome, LayoutMutationAdmissionCaseId, LayoutMutationAdmissionOutcome,
    LiveMaintenancePostureCaseId, LiveMaintenancePostureOutcome,
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

observe_owner_case!(LiveMaintenancePostureOutcome => LiveMaintenancePostureCaseId);
observe_owner_case!(LayoutMutationAdmissionOutcome => LayoutMutationAdmissionCaseId);
observe_owner_case!(DerivedIndexParityOutcome => DerivedIndexParityCaseId);
observe_owner_case!(DerivedIndexRebuildAdmissionOutcome => DerivedIndexRebuildAdmissionCaseId);
observe_owner_case!(DerivedIndexRebuildOutcome => DerivedIndexRebuildExecutionCaseId);
