use crate::identity::hash_parts;

use super::declaration::{ProjectionConsumptionSource, ProjectionSourceFamily};
use super::eligibility::{DeferredProjectionConsumptionReason, ProjectionConsumptionWarningKind};
use super::facts::ProjectionFactKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProjectionConsumptionSupportPosture {
    Admitted,
    AdmittedWithWarnings(ProjectionConsumptionWarningKind),
    Deferred(DeferredProjectionConsumptionReason),
    SourceMismatch,
}

impl ProjectionConsumptionSupportPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::AdmittedWithWarnings(_) => "admitted_with_warnings",
            Self::Deferred(_) => "deferred",
            Self::SourceMismatch => "source_mismatch",
        }
    }

    fn detail_key(&self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::AdmittedWithWarnings(kind) => kind.as_str(),
            Self::Deferred(reason) => match reason {
                DeferredProjectionConsumptionReason::WriteReceiptContractBindingPending => {
                    "write_receipt_contract_binding_pending"
                }
            },
            Self::SourceMismatch => "source_mismatch",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionConsumptionSupportRow {
    source_family: ProjectionSourceFamily,
    fact_kind: ProjectionFactKind,
    posture: ProjectionConsumptionSupportPosture,
    support_digest: String,
}

impl ProjectionConsumptionSupportRow {
    pub fn source_family(&self) -> ProjectionSourceFamily {
        self.source_family
    }

    pub fn fact_kind(&self) -> ProjectionFactKind {
        self.fact_kind
    }

    pub fn posture(&self) -> &ProjectionConsumptionSupportPosture {
        &self.posture
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionConsumptionSupportReport {
    rows: Vec<ProjectionConsumptionSupportRow>,
}

impl ProjectionConsumptionSupportReport {
    pub fn rows(&self) -> &[ProjectionConsumptionSupportRow] {
        &self.rows
    }
}

pub fn discover_projection_consumption_support(
    source: &ProjectionConsumptionSource,
) -> ProjectionConsumptionSupportReport {
    let evaluator = evaluator_for(source.family());
    let rows = ProjectionFactKind::all()
        .iter()
        .copied()
        .map(|fact_kind| {
            let posture = evaluator.evaluate_fact_kind(fact_kind);
            ProjectionConsumptionSupportRow {
                source_family: source.family(),
                fact_kind,
                support_digest: hash_parts(&[
                    format!("source:{}", source.family().as_str()),
                    format!("fact:{}", fact_kind.as_str()),
                    format!("posture:{}", posture.as_str()),
                    format!("detail:{}", posture.detail_key()),
                ]),
                posture,
            }
        })
        .collect();
    ProjectionConsumptionSupportReport { rows }
}

pub(crate) fn support_for_kind(
    source_family: ProjectionSourceFamily,
    fact_kind: ProjectionFactKind,
) -> ProjectionConsumptionSupportPosture {
    evaluator_for(source_family).evaluate_fact_kind(fact_kind)
}

trait ProjectionConsumptionSourceEvaluator {
    fn evaluate_fact_kind(
        &self,
        fact_kind: ProjectionFactKind,
    ) -> ProjectionConsumptionSupportPosture;
}

struct QueryReadReceiptEvaluator;
struct QueryWriteReceiptEvaluator;
struct QueryContextExecutionEvaluator;

impl ProjectionConsumptionSourceEvaluator for QueryReadReceiptEvaluator {
    fn evaluate_fact_kind(
        &self,
        fact_kind: ProjectionFactKind,
    ) -> ProjectionConsumptionSupportPosture {
        match fact_kind {
            ProjectionFactKind::EntityIdentity
            | ProjectionFactKind::ViewLocalIdentity
            | ProjectionFactKind::DisplayField
            | ProjectionFactKind::DerivedScalarField => {
                ProjectionConsumptionSupportPosture::Admitted
            }
            ProjectionFactKind::TargetIdentity
            | ProjectionFactKind::SourceReference
            | ProjectionFactKind::EffectContinuity
            | ProjectionFactKind::Membership
            | ProjectionFactKind::RelationEndpoint => {
                ProjectionConsumptionSupportPosture::SourceMismatch
            }
        }
    }
}

impl ProjectionConsumptionSourceEvaluator for QueryWriteReceiptEvaluator {
    fn evaluate_fact_kind(
        &self,
        fact_kind: ProjectionFactKind,
    ) -> ProjectionConsumptionSupportPosture {
        match fact_kind {
            ProjectionFactKind::TargetIdentity
            | ProjectionFactKind::SourceReference
            | ProjectionFactKind::EffectContinuity => {
                ProjectionConsumptionSupportPosture::Deferred(
                    DeferredProjectionConsumptionReason::WriteReceiptContractBindingPending,
                )
            }
            _ => ProjectionConsumptionSupportPosture::SourceMismatch,
        }
    }
}

impl ProjectionConsumptionSourceEvaluator for QueryContextExecutionEvaluator {
    fn evaluate_fact_kind(
        &self,
        fact_kind: ProjectionFactKind,
    ) -> ProjectionConsumptionSupportPosture {
        match fact_kind {
            ProjectionFactKind::SourceReference | ProjectionFactKind::DerivedScalarField => {
                ProjectionConsumptionSupportPosture::AdmittedWithWarnings(
                    ProjectionConsumptionWarningKind::QueryContextPayloadBound,
                )
            }
            ProjectionFactKind::DisplayField => {
                ProjectionConsumptionSupportPosture::AdmittedWithWarnings(
                    ProjectionConsumptionWarningKind::PreviewDerivedContext,
                )
            }
            ProjectionFactKind::EntityIdentity | ProjectionFactKind::ViewLocalIdentity => {
                ProjectionConsumptionSupportPosture::Admitted
            }
            ProjectionFactKind::TargetIdentity
            | ProjectionFactKind::EffectContinuity
            | ProjectionFactKind::Membership
            | ProjectionFactKind::RelationEndpoint => {
                ProjectionConsumptionSupportPosture::SourceMismatch
            }
        }
    }
}

fn evaluator_for(
    source_family: ProjectionSourceFamily,
) -> &'static dyn ProjectionConsumptionSourceEvaluator {
    match source_family {
        ProjectionSourceFamily::QueryReadReceipt => &QueryReadReceiptEvaluator,
        ProjectionSourceFamily::QueryWriteReceipt => &QueryWriteReceiptEvaluator,
        ProjectionSourceFamily::QueryContextExecution => &QueryContextExecutionEvaluator,
    }
}
