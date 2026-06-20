#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryGraphReadAccessInvalidationBasis {
    AuthoritativeRelationDelta,
    AuthoritativeFieldDelta,
    ReadGraphProofDelta,
    RuntimeLifecycleDelta,
}

impl ForgeQueryGraphReadAccessInvalidationBasis {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AuthoritativeRelationDelta => "authoritative_relation_delta",
            Self::AuthoritativeFieldDelta => "authoritative_field_delta",
            Self::ReadGraphProofDelta => "read_graph_proof_delta",
            Self::RuntimeLifecycleDelta => "runtime_lifecycle_delta",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryGraphReadAccessComplexityContract {
    DirectionalRelationLookup,
    ReverseRelationLookup,
    BoundedTraversalWorkset,
    CandidatePredicateSupport,
    CandidateOrderingSupport,
    ProofEvidenceSupport,
    ResultPressureBuffer,
    LifecycleSupportAdmission,
}

impl ForgeQueryGraphReadAccessComplexityContract {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DirectionalRelationLookup => "directional_relation_lookup",
            Self::ReverseRelationLookup => "reverse_relation_lookup",
            Self::BoundedTraversalWorkset => "bounded_traversal_workset",
            Self::CandidatePredicateSupport => "candidate_predicate_support",
            Self::CandidateOrderingSupport => "candidate_ordering_support",
            Self::ProofEvidenceSupport => "proof_evidence_support",
            Self::ResultPressureBuffer => "result_pressure_buffer",
            Self::LifecycleSupportAdmission => "lifecycle_support_admission",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryGraphReadAccessMemoryEstimateBasis {
    RelationDegreeBound,
    FrontierDepthBound,
    PredicateCandidateSet,
    OrderedCandidateSet,
    ProofEvidenceSet,
    ResultPressureBound,
    LifecycleManagedSupport,
}

impl ForgeQueryGraphReadAccessMemoryEstimateBasis {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::RelationDegreeBound => "relation_degree_bound",
            Self::FrontierDepthBound => "frontier_depth_bound",
            Self::PredicateCandidateSet => "predicate_candidate_set",
            Self::OrderedCandidateSet => "ordered_candidate_set",
            Self::ProofEvidenceSet => "proof_evidence_set",
            Self::ResultPressureBound => "result_pressure_bound",
            Self::LifecycleManagedSupport => "lifecycle_managed_support",
        }
    }
}
