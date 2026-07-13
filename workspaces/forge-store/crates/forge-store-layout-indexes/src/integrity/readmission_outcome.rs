use super::denial::CorruptionDenial;
use super::readmission::LayoutReadmissionWitness;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadmissionDenied(CorruptionDenial);

impl ReadmissionDenied {
    pub const fn denial(&self) -> &CorruptionDenial {
        &self.0
    }
}

macro_rules! readmission_outcome {
    ($outcome:ident, $view:ident, $case:ident, $case_id:ident, $cases_fn:ident, $prefix:literal) => {
        #[derive(Debug, PartialEq, Eq)]
        enum $case {
            Readmitted(LayoutReadmissionWitness),
            Denied(ReadmissionDenied),
        }

        #[derive(Debug, PartialEq, Eq)]
        pub struct $outcome {
            case: $case,
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

        pub fn $cases_fn() -> impl Iterator<Item = $case_id> {
            [
                $case_id(concat!($prefix, ".readmitted")),
                $case_id(concat!($prefix, ".denied")),
            ]
            .into_iter()
        }

        impl $outcome {
            pub(super) fn readmitted(witness: LayoutReadmissionWitness) -> Self {
                Self {
                    case: $case::Readmitted(witness),
                }
            }

            pub(super) fn denied(denial: CorruptionDenial) -> Self {
                Self {
                    case: $case::Denied(ReadmissionDenied(denial)),
                }
            }

            pub fn view(&self) -> $view<'_> {
                match &self.case {
                    $case::Readmitted(witness) => $view::Readmitted(witness),
                    $case::Denied(denial) => $view::Denied(denial),
                }
            }

            pub fn case_id(&self) -> $case_id {
                match &self.case {
                    $case::Readmitted(_) => $case_id(concat!($prefix, ".readmitted")),
                    $case::Denied(_) => $case_id(concat!($prefix, ".denied")),
                }
            }
        }
    };
}

readmission_outcome!(
    QuarantineReadmissionOutcome,
    QuarantineReadmissionView,
    QuarantineReadmissionCase,
    QuarantineReadmissionCaseId,
    quarantine_readmission_cases,
    "layout.integrity.quarantine_readmission"
);
readmission_outcome!(
    OfflineReadmissionOutcome,
    OfflineReadmissionView,
    OfflineReadmissionCase,
    OfflineReadmissionCaseId,
    offline_readmission_cases,
    "layout.integrity.offline_readmission"
);
readmission_outcome!(
    ImportReadmissionOutcome,
    ImportReadmissionView,
    ImportReadmissionCase,
    ImportReadmissionCaseId,
    import_readmission_cases,
    "layout.integrity.import_readmission"
);
