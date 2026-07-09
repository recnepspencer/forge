#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthQueryGraphReadAccessRequirementKind {
    DirectionalAdjacency,
    ReverseAdjacency,
    PredicateSupport,
    OrderingSupport,
    TraversalWorkset,
    VisitedSet,
    DedupSet,
    ProofSupport,
    ResultBuffer,
    MaterializationLifecycle,
    LiveMaintenanceSupport,
    DomainOperationCapabilityRegistration,
}

impl WorthQueryGraphReadAccessRequirementKind {
    pub fn all() -> &'static [Self] {
        &[
            Self::DirectionalAdjacency,
            Self::ReverseAdjacency,
            Self::PredicateSupport,
            Self::OrderingSupport,
            Self::TraversalWorkset,
            Self::VisitedSet,
            Self::DedupSet,
            Self::ProofSupport,
            Self::ResultBuffer,
            Self::MaterializationLifecycle,
            Self::LiveMaintenanceSupport,
            Self::DomainOperationCapabilityRegistration,
        ]
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DirectionalAdjacency => "directional_adjacency",
            Self::ReverseAdjacency => "reverse_adjacency",
            Self::PredicateSupport => "predicate_support",
            Self::OrderingSupport => "ordering_support",
            Self::TraversalWorkset => "traversal_workset",
            Self::VisitedSet => "visited_set",
            Self::DedupSet => "dedup_set",
            Self::ProofSupport => "proof_support",
            Self::ResultBuffer => "result_buffer",
            Self::MaterializationLifecycle => "materialization_lifecycle",
            Self::LiveMaintenanceSupport => "live_maintenance_support",
            Self::DomainOperationCapabilityRegistration => {
                "domain_operation_capability_registration"
            }
        }
    }
}
