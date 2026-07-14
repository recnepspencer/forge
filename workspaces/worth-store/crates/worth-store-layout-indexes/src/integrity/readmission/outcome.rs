use super::LayoutReadmissionWitness;
use crate::integrity::{CorruptionDenial, LayoutReadmissionCounterSnapshot};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadmissionDenied(CorruptionDenial);

impl ReadmissionDenied {
    pub const fn denial(&self) -> &CorruptionDenial {
        &self.0
    }
}

macro_rules! readmission_outcome {
    ($outcome:ident, $view:ident, $case:ident, $case_id:ident) => {
        #[derive(Debug, PartialEq, Eq)]
        enum $case {
            Readmitted(LayoutReadmissionWitness),
            Denied(ReadmissionDenied, $case_id),
        }

        #[derive(Debug, PartialEq, Eq)]
        pub struct $outcome {
            case: $case,
            counters: LayoutReadmissionCounterSnapshot,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $view<'a> {
            Readmitted(&'a LayoutReadmissionWitness),
            Denied(&'a ReadmissionDenied),
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $case_id(&'static str);

        impl $case_id {
            pub const fn as_str(self) -> &'static str {
                self.0
            }
        }

        impl $outcome {
            pub(super) fn readmitted(
                witness: LayoutReadmissionWitness,
                counters: LayoutReadmissionCounterSnapshot,
            ) -> Self {
                Self {
                    case: $case::Readmitted(witness),
                    counters,
                }
            }

            pub(super) fn denied(
                denial: CorruptionDenial,
                case_id: $case_id,
                counters: LayoutReadmissionCounterSnapshot,
            ) -> Self {
                Self {
                    case: $case::Denied(ReadmissionDenied(denial), case_id),
                    counters,
                }
            }

            pub fn view(&self) -> $view<'_> {
                match &self.case {
                    $case::Readmitted(witness) => $view::Readmitted(witness),
                    $case::Denied(denial, _) => $view::Denied(denial),
                }
            }

            pub fn case_id(&self) -> $case_id {
                match &self.case {
                    $case::Readmitted(_) => $case_id::READMITTED,
                    $case::Denied(_, case_id) => *case_id,
                }
            }

            pub const fn counters(&self) -> LayoutReadmissionCounterSnapshot {
                self.counters
            }
        }
    };
}

readmission_outcome!(
    QuarantineReadmissionOutcome,
    QuarantineReadmissionView,
    QuarantineReadmissionCase,
    QuarantineReadmissionCaseId
);
readmission_outcome!(
    OfflineReadmissionOutcome,
    OfflineReadmissionView,
    OfflineReadmissionCase,
    OfflineReadmissionCaseId
);
readmission_outcome!(
    ImportReadmissionOutcome,
    ImportReadmissionView,
    ImportReadmissionCase,
    ImportReadmissionCaseId
);

impl QuarantineReadmissionCaseId {
    pub(super) const READMITTED: Self = Self("layout.integrity.quarantine_readmission.readmitted");
    pub(super) const FAMILY_IDENTITY: Self =
        Self("layout.integrity.quarantine_readmission.denied.identity");
    pub(super) const IMPORT_REQUIRED: Self =
        Self("layout.integrity.quarantine_readmission.denied.import");
}

pub fn quarantine_readmission_cases() -> impl Iterator<Item = QuarantineReadmissionCaseId> {
    [
        QuarantineReadmissionCaseId::READMITTED,
        QuarantineReadmissionCaseId::FAMILY_IDENTITY,
        QuarantineReadmissionCaseId::IMPORT_REQUIRED,
    ]
    .into_iter()
}

impl OfflineReadmissionCaseId {
    pub(super) const READMITTED: Self = Self("layout.integrity.offline_readmission.readmitted");
    pub(super) const FAMILY_IDENTITY: Self =
        Self("layout.integrity.offline_readmission.denied.identity");
    pub(super) const WRONG_CLASS: Self = Self("layout.integrity.offline_readmission.denied.class");
}

pub fn offline_readmission_cases() -> impl Iterator<Item = OfflineReadmissionCaseId> {
    [
        OfflineReadmissionCaseId::READMITTED,
        OfflineReadmissionCaseId::FAMILY_IDENTITY,
        OfflineReadmissionCaseId::WRONG_CLASS,
    ]
    .into_iter()
}

impl ImportReadmissionCaseId {
    pub(super) const READMITTED: Self = Self("layout.integrity.import_readmission.readmitted");
    pub(super) const FAMILY_IDENTITY: Self =
        Self("layout.integrity.import_readmission.denied.identity");
    pub(super) const QUARANTINE_REQUIRED: Self =
        Self("layout.integrity.import_readmission.denied.quarantine");
}

pub fn import_readmission_cases() -> impl Iterator<Item = ImportReadmissionCaseId> {
    [
        ImportReadmissionCaseId::READMITTED,
        ImportReadmissionCaseId::FAMILY_IDENTITY,
        ImportReadmissionCaseId::QUARANTINE_REQUIRED,
    ]
    .into_iter()
}
