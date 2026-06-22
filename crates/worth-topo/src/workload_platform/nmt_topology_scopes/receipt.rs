use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::{NmtTopologyScopeCounters, NmtTopologyScopeKind};
use crate::workload_platform::nmt_topology_construction::NmtTopologyPosture;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NmtTopologyScopeReceipt {
    parent_construction_identity: String,
    pattern_identity: String,
    scope_identity: String,
    kind: NmtTopologyScopeKind,
    layer_index: Option<usize>,
    face_identities: Vec<String>,
    edge_identities: Vec<String>,
    loop_identities: Vec<String>,
    topology_posture: NmtTopologyPosture,
    open_boundary_identity: String,
    radial_adjacency_identity: String,
    counters: NmtTopologyScopeCounters,
}

impl NmtTopologyScopeReceipt {
    pub(crate) fn new(input: NmtTopologyScopeReceiptInput) -> Self {
        let scope_identity = scope_identity(&input);
        Self {
            parent_construction_identity: input.parent_construction_identity,
            pattern_identity: input.pattern_identity,
            scope_identity,
            kind: input.kind,
            layer_index: input.layer_index,
            face_identities: input.face_identities,
            edge_identities: input.edge_identities,
            loop_identities: input.loop_identities,
            topology_posture: input.topology_posture,
            open_boundary_identity: input.open_boundary_identity,
            radial_adjacency_identity: input.radial_adjacency_identity,
            counters: input.counters,
        }
    }

    pub fn parent_construction_identity(&self) -> &str {
        &self.parent_construction_identity
    }

    pub fn pattern_identity(&self) -> &str {
        &self.pattern_identity
    }

    pub fn scope_identity(&self) -> &str {
        &self.scope_identity
    }

    pub fn kind(&self) -> NmtTopologyScopeKind {
        self.kind
    }

    pub fn layer_index(&self) -> Option<usize> {
        self.layer_index
    }

    pub fn face_identities(&self) -> &[String] {
        &self.face_identities
    }

    pub fn edge_identities(&self) -> &[String] {
        &self.edge_identities
    }

    pub fn loop_identities(&self) -> &[String] {
        &self.loop_identities
    }

    pub fn topology_posture(&self) -> NmtTopologyPosture {
        self.topology_posture
    }

    pub fn open_boundary_identity(&self) -> &str {
        &self.open_boundary_identity
    }

    pub fn radial_adjacency_identity(&self) -> &str {
        &self.radial_adjacency_identity
    }

    pub fn counters(&self) -> NmtTopologyScopeCounters {
        self.counters
    }

    pub fn contains_topology_entity(&self, identity: &str) -> bool {
        self.face_identities
            .iter()
            .any(|candidate| candidate == identity)
            || self
                .edge_identities
                .iter()
                .any(|candidate| candidate == identity)
            || self
                .loop_identities
                .iter()
                .any(|candidate| candidate == identity)
    }
}

pub(crate) struct NmtTopologyScopeReceiptInput {
    pub parent_construction_identity: String,
    pub pattern_identity: String,
    pub kind: NmtTopologyScopeKind,
    pub layer_index: Option<usize>,
    pub face_identities: Vec<String>,
    pub edge_identities: Vec<String>,
    pub loop_identities: Vec<String>,
    pub topology_posture: NmtTopologyPosture,
    pub open_boundary_identity: String,
    pub radial_adjacency_identity: String,
    pub counters: NmtTopologyScopeCounters,
}

fn scope_identity(input: &NmtTopologyScopeReceiptInput) -> String {
    let mut parts = vec![
        "nmt-topology-scope".to_string(),
        input.parent_construction_identity.clone(),
        input.pattern_identity.clone(),
        format!("kind:{:?}", input.kind),
        format!("layer:{:?}", input.layer_index),
        format!("posture:{:?}", input.topology_posture),
    ];
    parts.extend(input.face_identities.iter().cloned());
    parts.extend(input.edge_identities.iter().cloned());
    parts.extend(input.loop_identities.iter().cloned());
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}
