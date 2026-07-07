#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PlannerOwnedRoutingLifecycleRole {
    PriorProofInputConsumer,
    FamilyRouteProduct,
    SelectedRouteConsumer,
    PublicProofProjection,
    DerivedDiagnosticProjection,
    ForbiddenLegacyExplainer,
}

impl PlannerOwnedRoutingLifecycleRole {
    pub const ALL: [Self; 6] = [
        Self::PriorProofInputConsumer,
        Self::FamilyRouteProduct,
        Self::SelectedRouteConsumer,
        Self::PublicProofProjection,
        Self::DerivedDiagnosticProjection,
        Self::ForbiddenLegacyExplainer,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PriorProofInputConsumer => "prior-proof-input-consumer",
            Self::FamilyRouteProduct => "family-route-product",
            Self::SelectedRouteConsumer => "selected-route-consumer",
            Self::PublicProofProjection => "public-proof-projection",
            Self::DerivedDiagnosticProjection => "derived-diagnostic-projection",
            Self::ForbiddenLegacyExplainer => "forbidden-legacy-explainer",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlannerOwnedRoutingDisposition {
    Migrate,
    Delete,
    Cap,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlannerOwnedRoutingOwner {
    WorthKernel,
    WorthTopo,
    WorthSpatial,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PlannerOwnedRoutingDisplacedLane {
    KernelPublicCloseout,
    KernelSourceFirewall,
    TopoDiagnosticProjectionInputResidue,
    TopoQueryBackedConsumerCutover,
    SpatialEvidenceLookupPublicCloseout,
}

impl PlannerOwnedRoutingDisplacedLane {
    pub const fn path(self) -> &'static str {
        match self {
            Self::KernelPublicCloseout => {
                "crates/worth-kernel/src/workload_composition/public_closeout/"
            }
            Self::KernelSourceFirewall => {
                "crates/worth-kernel/src/workload_composition/source_firewall/"
            }
            Self::TopoDiagnosticProjectionInputResidue => {
                "crates/worth-topo/src/projection/planner_owned_routing/diagnostic_projection_input/report_types.rs"
            }
            Self::TopoQueryBackedConsumerCutover => {
                "crates/worth-topo/src/projection/query_backed_consumer_cutover/"
            }
            Self::SpatialEvidenceLookupPublicCloseout => {
                "crates/worth-spatial/src/workload_platform/planner_owned_routing/public_closeout_route/"
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PlannerOwnedRoutingReplacementLane {
    KernelAdmittedPublicProofInput,
    KernelSelectedRoute,
    KernelPublicProof,
    KernelPublicFacade,
    KernelSourceFirewall,
    TopoQueryBackedReadFamily,
    TopoInvalidationRoute,
    TopoDiagnosticProjectionInput,
    SpatialEvidenceLookupRoute,
    SpatialPublicCloseoutRoute,
    SpatialDiagnosticProjectionInput,
}

impl PlannerOwnedRoutingReplacementLane {
    pub const fn path(self) -> &'static str {
        match self {
            Self::KernelAdmittedPublicProofInput => {
                "crates/worth-kernel/src/workload_composition/planner_owned_routing/admitted_public_proof_input/"
            }
            Self::KernelSelectedRoute => {
                "crates/worth-kernel/src/workload_composition/planner_owned_routing/selected_route/"
            }
            Self::KernelPublicProof => {
                "crates/worth-kernel/src/workload_composition/planner_owned_routing/public_proof/"
            }
            Self::KernelPublicFacade => {
                "crates/worth-kernel/src/workload_composition/planner_owned_routing/public_facade/"
            }
            Self::KernelSourceFirewall => {
                "crates/worth-kernel/src/workload_composition/planner_owned_routing/source_firewall/"
            }
            Self::TopoQueryBackedReadFamily => {
                "crates/worth-topo/src/projection/touched_graph_parity_closeout/read_family/"
            }
            Self::TopoInvalidationRoute => {
                "crates/worth-topo/src/projection/touched_graph_parity_closeout/invalidation_family/"
            }
            Self::TopoDiagnosticProjectionInput => {
                "crates/worth-topo/src/projection/runtime_boundary/diagnostic_projection/"
            }
            Self::SpatialEvidenceLookupRoute => {
                "crates/worth-spatial/src/workload_platform/touched_graph_parity_closeout/evidence_lookup_family/"
            }
            Self::SpatialPublicCloseoutRoute => {
                "crates/worth-spatial/src/workload_platform/touched_graph_parity_closeout/retained_surface_family/"
            }
            Self::SpatialDiagnosticProjectionInput => {
                "crates/worth-spatial/src/workload_platform/planner_owned_routing/diagnostic_projection_input/"
            }
        }
    }
}
