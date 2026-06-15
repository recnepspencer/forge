use topology::facade::{
    TopologySeedKind, TopologySeedReceipt, TopologySeedTopologyPosture, TopologyWorkloadReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyBindingTarget {
    seed_kind: TopologySeedKind,
    topology_posture: TopologySeedTopologyPosture,
    topology_stage_receipt: TopologyWorkloadReceipt,
    topology_receipt_identity: String,
    topology_query_surface: String,
    face_targets: Vec<String>,
    edge_targets: Vec<String>,
    loop_targets: Vec<String>,
}

impl TopologyBindingTarget {
    pub(crate) fn from_seed(seed: &TopologySeedReceipt) -> Self {
        let entities = seed.entity_identities();
        Self {
            seed_kind: seed.kind(),
            topology_posture: seed.topology_posture(),
            topology_stage_receipt: seed.query_receipts().declaration_receipt().clone(),
            topology_receipt_identity: seed
                .query_receipts()
                .declaration_receipt()
                .identity()
                .name()
                .to_string(),
            topology_query_surface: seed.query_receipts().query_surface().to_string(),
            face_targets: entities.face_identity_tokens(),
            edge_targets: entities.edge_identity_tokens(),
            loop_targets: entities.loop_identity_tokens(),
        }
    }

    pub fn seed_kind(&self) -> TopologySeedKind {
        self.seed_kind
    }

    pub fn topology_posture(&self) -> TopologySeedTopologyPosture {
        self.topology_posture
    }

    pub fn topology_receipt_identity(&self) -> &str {
        &self.topology_receipt_identity
    }

    pub(crate) fn topology_stage_receipt(&self) -> &TopologyWorkloadReceipt {
        &self.topology_stage_receipt
    }

    pub fn topology_query_surface(&self) -> &str {
        &self.topology_query_surface
    }

    pub fn face_targets(&self) -> &[String] {
        &self.face_targets
    }

    pub fn edge_targets(&self) -> &[String] {
        &self.edge_targets
    }

    pub fn loop_targets(&self) -> &[String] {
        &self.loop_targets
    }
}
