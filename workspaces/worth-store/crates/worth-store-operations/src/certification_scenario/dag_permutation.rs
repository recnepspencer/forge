use sha2::{Digest, Sha256};

use crate::owner_plan_dag::{
    CanonicalOwnerPlanDag, OwnerPlanEffect, OwnerPlanFootprint, OwnerPlanNode,
    OwnerPlanPrerequisite, StoreOwnerKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScenarioCanonicalOwnerDagPermutationReceipt {
    plan_fingerprint: [u8; 32],
    node_count: u64,
    edge_count: u64,
    evidence_identity: [u8; 32],
}

pub fn certify_scenario_canonical_owner_dag_permutation(
) -> ScenarioCanonicalOwnerDagPermutationReceipt {
    let footprint = OwnerPlanFootprint::bounded(0, 4096).unwrap();
    let nodes = [
        node(
            StoreOwnerKind::PhysicalIntegrity,
            OwnerPlanEffect::ClassifyQuarantine,
            footprint,
            1,
        ),
        node(
            StoreOwnerKind::PhysicalBackend,
            OwnerPlanEffect::CopyBackupComponent,
            footprint,
            2,
        ),
        node(
            StoreOwnerKind::RecoveryPhysics,
            OwnerPlanEffect::ReplayWalToExactFrontier,
            footprint,
            3,
        ),
        node(
            StoreOwnerKind::LayoutIndexes,
            OwnerPlanEffect::ReplaceQuarantinedLayout,
            footprint,
            4,
        ),
        node(
            StoreOwnerKind::BlobChunks,
            OwnerPlanEffect::ReplaceQuarantinedBlob,
            footprint,
            5,
        ),
    ];
    let edges = nodes
        .windows(2)
        .map(|pair| OwnerPlanPrerequisite::new(pair[0].identity(), pair[1].identity(), true))
        .collect::<Vec<_>>();
    let forward = CanonicalOwnerPlanDag::admit(nodes.to_vec(), edges.clone()).unwrap();
    let reverse = CanonicalOwnerPlanDag::admit(
        nodes.into_iter().rev().collect(),
        edges.into_iter().rev().collect(),
    )
    .unwrap();
    assert_eq!(forward.explanation(), reverse.explanation());
    let explanation = forward.explanation();
    let mut digest = Sha256::new();
    digest.update(b"worth-store-s10-canonical-owner-dag-permutation-v1");
    digest.update(explanation.plan_fingerprint());
    digest.update(explanation.node_count().to_be_bytes());
    digest.update(explanation.edge_count().to_be_bytes());
    ScenarioCanonicalOwnerDagPermutationReceipt {
        plan_fingerprint: explanation.plan_fingerprint(),
        node_count: explanation.node_count(),
        edge_count: explanation.edge_count(),
        evidence_identity: digest.finalize().into(),
    }
}

fn node(
    owner: StoreOwnerKind,
    effect: OwnerPlanEffect,
    footprint: OwnerPlanFootprint,
    seed: u8,
) -> OwnerPlanNode {
    OwnerPlanNode::from_owner_binding(
        owner,
        effect,
        footprint,
        4096,
        true,
        [seed; 32],
        [seed.saturating_add(16); 32],
    )
}

impl ScenarioCanonicalOwnerDagPermutationReceipt {
    pub const fn plan_fingerprint(self) -> [u8; 32] {
        self.plan_fingerprint
    }
    pub const fn node_count(self) -> u64 {
        self.node_count
    }
    pub const fn edge_count(self) -> u64 {
        self.edge_count
    }
    pub const fn evidence_identity(self) -> [u8; 32] {
        self.evidence_identity
    }
}
