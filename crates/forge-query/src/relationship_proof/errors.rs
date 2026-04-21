use super::RelationshipProofCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RelationshipProofFailureClass {
    HostCallbackForbidden,
    UnboundedRecursiveWalk,
    MissingProofBasis,
    QueryShapeMismatch,
    PolicyMismatch,
    TenantSchemaMismatch,
    RelationshipProofBudgetExceeded,
    UnboundedProofTopology,
}

impl RelationshipProofFailureClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::HostCallbackForbidden => "host_callback_forbidden",
            Self::UnboundedRecursiveWalk => "unbounded_recursive_walk",
            Self::MissingProofBasis => "missing_proof_basis",
            Self::QueryShapeMismatch => "query_shape_mismatch",
            Self::PolicyMismatch => "policy_mismatch",
            Self::TenantSchemaMismatch => "tenant_schema_mismatch",
            Self::RelationshipProofBudgetExceeded => "relationship_proof_budget_exceeded",
            Self::UnboundedProofTopology => "unbounded_proof_topology",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationshipProofError {
    failure_class: RelationshipProofFailureClass,
    message: &'static str,
    counters: RelationshipProofCounters,
}

impl RelationshipProofError {
    pub(crate) fn new(
        failure_class: RelationshipProofFailureClass,
        message: &'static str,
        counters: RelationshipProofCounters,
    ) -> Self {
        Self {
            failure_class,
            message,
            counters,
        }
    }

    pub fn failure_class(&self) -> RelationshipProofFailureClass {
        self.failure_class
    }

    pub fn message(&self) -> &'static str {
        self.message
    }

    pub fn counters(&self) -> &RelationshipProofCounters {
        &self.counters
    }
}
