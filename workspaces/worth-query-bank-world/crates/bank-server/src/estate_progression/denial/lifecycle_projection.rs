use worth_query_host::facade::primary_graph::{
    WorthQueryEntityResolutionDenial, WorthQueryInvariantDecisionPlanDenial,
    WorthQueryInvariantProjectionTraversalDenial,
};

use crate::{
    BankEntityResolutionDenial, BankInvariantDecisionPlanDenial,
    BankInvariantProjectionTraversalDenial,
};

#[derive(Debug)]
pub enum BankEstateLifecycleProjectionDenial {
    ReceiptIdentity(&'static str),
    RelationCardinality {
        relation: &'static str,
        expected: usize,
        observed: usize,
    },
    RelationTargetMismatch {
        relation: &'static str,
    },
    EntityResolution(BankEntityResolutionDenial),
    DecisionPlan(BankInvariantDecisionPlanDenial),
    Traversal(BankInvariantProjectionTraversalDenial),
}

impl std::fmt::Display for BankEstateLifecycleProjectionDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReceiptIdentity(subject) => {
                write!(formatter, "invalid elevation receipt identity: {subject}")
            }
            Self::RelationCardinality {
                relation,
                expected,
                observed,
            } => write!(
                formatter,
                "lifecycle relation {relation} expected {expected} target, observed {observed}"
            ),
            Self::RelationTargetMismatch { relation } => {
                write!(
                    formatter,
                    "lifecycle relation {relation} targets the wrong estate"
                )
            }
            Self::EntityResolution(denial) => denial.fmt(formatter),
            Self::DecisionPlan(denial) => denial.fmt(formatter),
            Self::Traversal(denial) => denial.fmt(formatter),
        }
    }
}

impl std::error::Error for BankEstateLifecycleProjectionDenial {}

impl From<WorthQueryEntityResolutionDenial> for BankEstateLifecycleProjectionDenial {
    fn from(denial: WorthQueryEntityResolutionDenial) -> Self {
        Self::EntityResolution(BankEntityResolutionDenial::from_query(denial.kind()))
    }
}

impl From<WorthQueryInvariantDecisionPlanDenial> for BankEstateLifecycleProjectionDenial {
    fn from(denial: WorthQueryInvariantDecisionPlanDenial) -> Self {
        Self::DecisionPlan(BankInvariantDecisionPlanDenial::from_query(denial.kind()))
    }
}

impl From<WorthQueryInvariantProjectionTraversalDenial> for BankEstateLifecycleProjectionDenial {
    fn from(denial: WorthQueryInvariantProjectionTraversalDenial) -> Self {
        Self::Traversal(BankInvariantProjectionTraversalDenial::from_query(
            denial.kind(),
        ))
    }
}
