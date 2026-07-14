use super::denial::DerivedIndexRebuildDenied;
use super::{DerivedIndexRebuildPlan, DerivedIndexRebuildReceipt, RebuildOutcomeIssuer};

#[derive(Debug, PartialEq, Eq)]
enum RebuildAdmissionCase {
    Admitted(Box<DerivedIndexRebuildPlan>),
    Denied {
        denial: Box<DerivedIndexRebuildDenied>,
        case_id: DerivedIndexRebuildAdmissionCaseId,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct RebuildAdmissionDenial {
    denial: DerivedIndexRebuildDenied,
    case_id: DerivedIndexRebuildAdmissionCaseId,
}

impl RebuildAdmissionDenial {
    pub(super) fn strategy(denial: DerivedIndexRebuildDenied) -> Self {
        Self::new(denial, "denied.strategy")
    }

    pub(super) fn source_not_authority(denial: DerivedIndexRebuildDenied) -> Self {
        Self::new(denial, "denied.source_not_authority")
    }

    pub(super) fn shape(denial: DerivedIndexRebuildDenied) -> Self {
        Self::new(denial, "denied.shape")
    }

    pub(super) fn source_strategy(denial: DerivedIndexRebuildDenied) -> Self {
        Self::new(denial, "denied.source_strategy")
    }

    pub(super) fn source_identity(denial: DerivedIndexRebuildDenied) -> Self {
        Self::new(denial, "denied.source_identity")
    }

    pub(super) fn source_security(denial: DerivedIndexRebuildDenied) -> Self {
        Self::new(denial, "denied.source_security")
    }

    pub(super) fn source_authority(denial: DerivedIndexRebuildDenied) -> Self {
        Self::new(denial, "denied.source_authority")
    }

    fn new(denial: DerivedIndexRebuildDenied, case_id: &'static str) -> Self {
        Self {
            denial,
            case_id: DerivedIndexRebuildAdmissionCaseId(case_id),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct DerivedIndexRebuildAdmissionOutcome {
    case: RebuildAdmissionCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DerivedIndexRebuildAdmissionView<'a> {
    Admitted(&'a DerivedIndexRebuildPlan),
    Denied(&'a DerivedIndexRebuildDenied),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DerivedIndexRebuildAdmissionCaseId(&'static str);

impl DerivedIndexRebuildAdmissionCaseId {
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

pub fn derived_index_rebuild_admission_cases(
) -> impl Iterator<Item = DerivedIndexRebuildAdmissionCaseId> {
    [
        "admitted",
        "denied.strategy",
        "denied.source_not_authority",
        "denied.shape",
        "denied.source_strategy",
        "denied.source_identity",
        "denied.source_security",
        "denied.source_authority",
    ]
    .into_iter()
    .map(DerivedIndexRebuildAdmissionCaseId)
}

impl DerivedIndexRebuildAdmissionOutcome {
    pub(super) fn from_result(
        _issuer: RebuildOutcomeIssuer,
        result: Result<DerivedIndexRebuildPlan, RebuildAdmissionDenial>,
    ) -> Self {
        match result {
            Ok(plan) => Self {
                case: RebuildAdmissionCase::Admitted(Box::new(plan)),
            },
            Err(failure) => Self {
                case: RebuildAdmissionCase::Denied {
                    denial: Box::new(failure.denial),
                    case_id: failure.case_id,
                },
            },
        }
    }

    pub const fn view(&self) -> DerivedIndexRebuildAdmissionView<'_> {
        match &self.case {
            RebuildAdmissionCase::Admitted(plan) => {
                DerivedIndexRebuildAdmissionView::Admitted(plan)
            }
            RebuildAdmissionCase::Denied { denial, .. } => {
                DerivedIndexRebuildAdmissionView::Denied(denial)
            }
        }
    }

    pub const fn case_id(&self) -> DerivedIndexRebuildAdmissionCaseId {
        match &self.case {
            RebuildAdmissionCase::Admitted(_) => DerivedIndexRebuildAdmissionCaseId("admitted"),
            RebuildAdmissionCase::Denied { case_id, .. } => *case_id,
        }
    }

    pub fn into_admitted(self) -> Result<DerivedIndexRebuildPlan, Self> {
        match self.case {
            RebuildAdmissionCase::Admitted(plan) => Ok(*plan),
            case => Err(Self { case }),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct DerivedIndexRebuildOutcome {
    receipt: Box<DerivedIndexRebuildReceipt>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DerivedIndexRebuildExecutionCaseId(&'static str);

impl DerivedIndexRebuildExecutionCaseId {
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

pub fn derived_index_rebuild_execution_cases(
) -> impl Iterator<Item = DerivedIndexRebuildExecutionCaseId> {
    ["rebuilt"]
        .into_iter()
        .map(DerivedIndexRebuildExecutionCaseId)
}

impl DerivedIndexRebuildOutcome {
    pub(super) fn rebuilt(
        _issuer: RebuildOutcomeIssuer,
        value: DerivedIndexRebuildReceipt,
    ) -> Self {
        Self {
            receipt: Box::new(value),
        }
    }

    pub const fn receipt(&self) -> &DerivedIndexRebuildReceipt {
        &self.receipt
    }

    pub fn case_id(&self) -> DerivedIndexRebuildExecutionCaseId {
        DerivedIndexRebuildExecutionCaseId("rebuilt")
    }

    pub fn into_rebuilt(self) -> DerivedIndexRebuildReceipt {
        *self.receipt
    }
}
