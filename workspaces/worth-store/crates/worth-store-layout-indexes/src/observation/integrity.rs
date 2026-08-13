use super::owner_case::{sealed, ObserveOwnerCase, OwnerCaseObservation};
use crate::integrity::{
    CorruptionClassificationCaseId, ImportReadmissionCaseId, ImportReadmissionOutcome,
    LayoutCorruptionOutcome, QuarantineReadmissionCaseId, QuarantineReadmissionOutcome,
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

observe_owner_case!(LayoutCorruptionOutcome => CorruptionClassificationCaseId);
observe_owner_case!(QuarantineReadmissionOutcome => QuarantineReadmissionCaseId);
observe_owner_case!(ImportReadmissionOutcome => ImportReadmissionCaseId);
