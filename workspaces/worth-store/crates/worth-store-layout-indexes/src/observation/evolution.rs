use super::owner_case::{sealed, ObserveOwnerCase, OwnerCaseObservation};
use crate::evolution::migration::{
    LayoutBackwardReadCompatibilityCaseId, LayoutBackwardReadOutcome, LayoutBindingAdmissionCaseId,
    LayoutBindingAdmissionOutcome, MigrationPlanningCaseId, MigrationPlanningOutcome,
    RollbackPlanningCaseId, RollbackPlanningOutcome,
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

observe_owner_case!(MigrationPlanningOutcome => MigrationPlanningCaseId);
observe_owner_case!(RollbackPlanningOutcome => RollbackPlanningCaseId);
observe_owner_case!(LayoutBackwardReadOutcome => LayoutBackwardReadCompatibilityCaseId);
observe_owner_case!(LayoutBindingAdmissionOutcome => LayoutBindingAdmissionCaseId);
