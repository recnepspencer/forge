use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::counters::NmtTopologyConstructionCounters;
use super::pattern_spec::NmtTopologyPattern;
use super::posture::TopologyPostureReceipt;
use super::query_receipts::NmtTopologyConstructionQueryReceipts;
use crate::workload_platform::topology_seed::TopologySeedReceipt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenPatternIdentityReceipt {
    identity_digest: String,
    pattern_name: String,
    layer_count: usize,
}

impl OpenPatternIdentityReceipt {
    pub(crate) fn new(pattern: &NmtTopologyPattern, declaration: &str) -> Self {
        let pattern_name = pattern.query_key().to_string();
        let layer_count = pattern.layer_count();
        let identity_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "nmt-topology-pattern".to_string(),
                pattern_name.clone(),
                format!("layers:{layer_count}"),
                declaration.to_string(),
            ],
        );
        Self {
            identity_digest,
            pattern_name,
            layer_count,
        }
    }

    pub fn identity_digest(&self) -> &str {
        &self.identity_digest
    }

    pub fn pattern_name(&self) -> &str {
        &self.pattern_name
    }

    pub fn layer_count(&self) -> usize {
        self.layer_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenBoundaryReceipt {
    boundary_half_edge_count: usize,
    boundary_digest: String,
}

impl OpenBoundaryReceipt {
    pub(crate) fn new(
        boundary_half_edge_count: usize,
        identity: &OpenPatternIdentityReceipt,
    ) -> Self {
        let boundary_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "nmt-open-boundary".to_string(),
                identity.identity_digest().to_string(),
                format!("half_edges:{boundary_half_edge_count}"),
            ],
        );
        Self {
            boundary_half_edge_count,
            boundary_digest,
        }
    }

    pub fn boundary_half_edge_count(&self) -> usize {
        self.boundary_half_edge_count
    }

    pub fn boundary_digest(&self) -> &str {
        &self.boundary_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RadialAdjacencyReceipt {
    non_manifold_edge_count: usize,
    radial_digest: String,
}

impl RadialAdjacencyReceipt {
    pub(crate) fn new(
        non_manifold_edge_count: usize,
        identity: &OpenPatternIdentityReceipt,
    ) -> Self {
        let radial_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "nmt-radial-adjacency".to_string(),
                identity.identity_digest().to_string(),
                format!("non_manifold_edges:{non_manifold_edge_count}"),
            ],
        );
        Self {
            non_manifold_edge_count,
            radial_digest,
        }
    }

    pub fn non_manifold_edge_count(&self) -> usize {
        self.non_manifold_edge_count
    }

    pub fn radial_digest(&self) -> &str {
        &self.radial_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NmtTopologyConstructionReceipt {
    pattern: NmtTopologyPattern,
    declaration: String,
    query_surface: String,
    query_declaration_identity: String,
    seed_receipt: TopologySeedReceipt,
    pattern_identity: OpenPatternIdentityReceipt,
    topology_posture: TopologyPostureReceipt,
    open_boundary: OpenBoundaryReceipt,
    radial_adjacency: RadialAdjacencyReceipt,
    counters: NmtTopologyConstructionCounters,
}

impl NmtTopologyConstructionReceipt {
    pub(crate) fn new(
        pattern: NmtTopologyPattern,
        declaration: String,
        query_receipts: NmtTopologyConstructionQueryReceipts,
        seed_receipt: TopologySeedReceipt,
        topology_posture: TopologyPostureReceipt,
        counters: NmtTopologyConstructionCounters,
    ) -> Self {
        let pattern_identity = OpenPatternIdentityReceipt::new(&pattern, &declaration);
        let open_boundary =
            OpenBoundaryReceipt::new(counters.boundary_half_edge_count(), &pattern_identity);
        let radial_adjacency =
            RadialAdjacencyReceipt::new(counters.non_manifold_edge_count(), &pattern_identity);
        Self {
            pattern,
            declaration,
            query_surface: query_receipts.query_surface().to_string(),
            query_declaration_identity: query_receipts
                .declaration_receipt()
                .identity()
                .name()
                .to_string(),
            seed_receipt,
            pattern_identity,
            topology_posture,
            open_boundary,
            radial_adjacency,
            counters,
        }
    }

    pub fn pattern(&self) -> &NmtTopologyPattern {
        &self.pattern
    }

    pub fn declaration(&self) -> &str {
        &self.declaration
    }

    pub fn query_surface(&self) -> &str {
        &self.query_surface
    }

    pub fn query_declaration_identity(&self) -> &str {
        &self.query_declaration_identity
    }

    pub fn topology_seed_receipt(&self) -> &TopologySeedReceipt {
        &self.seed_receipt
    }

    pub fn pattern_identity(&self) -> &OpenPatternIdentityReceipt {
        &self.pattern_identity
    }

    pub fn topology_posture(&self) -> &TopologyPostureReceipt {
        &self.topology_posture
    }

    pub fn open_boundary(&self) -> &OpenBoundaryReceipt {
        &self.open_boundary
    }

    pub fn radial_adjacency(&self) -> &RadialAdjacencyReceipt {
        &self.radial_adjacency
    }

    pub fn counters(&self) -> NmtTopologyConstructionCounters {
        self.counters
    }
}
