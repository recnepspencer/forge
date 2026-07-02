use crate::declaration::stable_text_digest;
use crate::graph::{
    UiGraphAxisParticipation, UiGraphCoreIndexes, UiGraphDeclarationCorrespondence,
    UiGraphGeneration, UiGraphGenerationRelation, UiGraphMountedReceiptAuthoritySeedStore,
    UiGraphMountedReceiptMutation, UiGraphMountedReceiptSlot, UiGraphMountedReceiptTransition,
    UiGraphNode, UiGraphNodeIdentity, UiGraphSnapshotComparable, UiGraphTopology,
    UiGraphWorldDifferenceKind, UiGraphWorldProfile,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphSnapshot {
    generation: UiGraphGeneration,
    world_profile: UiGraphWorldProfile,
    declaration_authority_digest: u64,
    snapshot_authority_digest: u64,
    nodes: Vec<UiGraphNode>,
    topology: UiGraphTopology,
    mounted_receipts: UiGraphMountedReceiptAuthoritySeedStore,
    core_indexes: UiGraphCoreIndexes,
}

impl UiGraphSnapshot {
    pub(crate) fn new(
        generation: UiGraphGeneration,
        world_profile: UiGraphWorldProfile,
        nodes: Vec<UiGraphNode>,
        topology: UiGraphTopology,
        mounted_receipts: UiGraphMountedReceiptAuthoritySeedStore,
        core_indexes: UiGraphCoreIndexes,
    ) -> Self {
        let declaration_correspondence = core_indexes.declaration_correspondence();
        let declaration_authority_digest =
            declaration_authority_digest(declaration_correspondence, &world_profile);
        let snapshot_authority_digest = snapshot_authority_digest(
            generation,
            &world_profile,
            &nodes,
            &topology,
            &mounted_receipts,
            declaration_authority_digest,
        );

        Self {
            generation,
            world_profile,
            declaration_authority_digest,
            snapshot_authority_digest,
            nodes,
            topology,
            mounted_receipts,
            core_indexes,
        }
    }

    pub fn generation(&self) -> UiGraphGeneration {
        self.generation
    }

    pub fn world_profile(&self) -> &UiGraphWorldProfile {
        &self.world_profile
    }

    pub(crate) fn nodes(&self) -> &[UiGraphNode] {
        &self.nodes
    }

    pub(crate) fn topology(&self) -> &UiGraphTopology {
        &self.topology
    }

    pub(crate) fn mounted_receipts(&self) -> &UiGraphMountedReceiptAuthoritySeedStore {
        &self.mounted_receipts
    }

    pub(crate) fn core_indexes(&self) -> &UiGraphCoreIndexes {
        &self.core_indexes
    }

    pub(crate) fn mounted_receipt_slot_for_node(
        &self,
        graph_node_identity: UiGraphNodeIdentity,
    ) -> Option<&UiGraphMountedReceiptSlot> {
        self.core_indexes
            .mounted_receipts()
            .slot_for_node(&self.mounted_receipts, graph_node_identity)
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn mounted_receipt_slot_count(&self) -> usize {
        self.mounted_receipts.slots().len()
    }

    pub fn mounted_receipt_mutation_for_node(
        &self,
        graph_node_identity: UiGraphNodeIdentity,
        prior_mounted_axis_participation: UiGraphAxisParticipation,
        next_mounted_axis_participation: UiGraphAxisParticipation,
    ) -> Option<UiGraphMountedReceiptMutation> {
        self.mounted_receipt_transition_for_node(
            graph_node_identity,
            prior_mounted_axis_participation,
            next_mounted_axis_participation,
        )
        .map(UiGraphMountedReceiptMutation::from_transition)
    }

    pub fn mounted_receipt_transition_for_node(
        &self,
        graph_node_identity: UiGraphNodeIdentity,
        prior_mounted_axis_participation: UiGraphAxisParticipation,
        next_mounted_axis_participation: UiGraphAxisParticipation,
    ) -> Option<UiGraphMountedReceiptTransition> {
        self.mounted_receipt_slot_for_node(graph_node_identity)
            .and_then(|slot| {
                UiGraphMountedReceiptTransition::from_slot_axis_transition(
                    *slot,
                    prior_mounted_axis_participation,
                    next_mounted_axis_participation,
                )
            })
    }

    pub fn compare_to(&self, other: &Self) -> UiGraphSnapshotComparable {
        let generation_relation = self.generation.relation_to(other.generation);
        let kind = if self.declaration_authority_digest == other.declaration_authority_digest {
            if self.world_profile.identity_digest() == other.world_profile.identity_digest() {
                same_world_difference_kind(
                    self.snapshot_authority_digest,
                    other.snapshot_authority_digest,
                    generation_relation,
                )
            } else if self.world_profile.comparison_family()
                == other.world_profile.comparison_family()
            {
                UiGraphWorldDifferenceKind::SameDeclarationDifferentWorld
            } else {
                UiGraphWorldDifferenceKind::NotComparable
            }
        } else if self.world_profile.comparison_family() == other.world_profile.comparison_family()
        {
            UiGraphWorldDifferenceKind::DifferentDeclarationAuthority
        } else {
            UiGraphWorldDifferenceKind::NotComparable
        };

        UiGraphSnapshotComparable::new(kind, generation_relation, self.generation, other.generation)
    }
}

fn declaration_authority_digest(
    declaration_correspondence: &UiGraphDeclarationCorrespondence,
    world_profile: &UiGraphWorldProfile,
) -> u64 {
    declaration_correspondence.declaration_digests().fold(
        world_profile.comparison_family().rotate_left(7),
        |digest, declaration_digest| digest.rotate_left(5) ^ declaration_digest,
    )
}

fn same_world_difference_kind(
    current_snapshot_digest: u64,
    compared_snapshot_digest: u64,
    generation_relation: UiGraphGenerationRelation,
) -> UiGraphWorldDifferenceKind {
    if current_snapshot_digest == compared_snapshot_digest {
        UiGraphWorldDifferenceKind::SameWorldEquivalent
    } else if matches!(
        generation_relation,
        UiGraphGenerationRelation::DirectSuccessor | UiGraphGenerationRelation::DirectPredecessor
    ) {
        UiGraphWorldDifferenceKind::SameWorldSuccessor
    } else {
        UiGraphWorldDifferenceKind::SameWorldUnrelatedGeneration
    }
}

fn snapshot_authority_digest(
    generation: UiGraphGeneration,
    world_profile: &UiGraphWorldProfile,
    nodes: &[UiGraphNode],
    topology: &UiGraphTopology,
    mounted_receipts: &UiGraphMountedReceiptAuthoritySeedStore,
    declaration_authority_digest: u64,
) -> u64 {
    nodes.iter().fold(
        stable_text_digest("graph-snapshot")
            ^ generation.as_u64().rotate_left(5)
            ^ declaration_authority_digest.rotate_left(17)
            ^ topology.identity_digest().rotate_left(23)
            ^ mounted_receipts.identity_digest().rotate_left(31)
            ^ world_profile.identity_digest().rotate_left(29),
        |digest, node| digest.rotate_left(7) ^ node.authority_digest(),
    )
}

#[cfg(test)]
mod tests {
    use crate::facade::WorthUi;
    use crate::graph::{UiGraphGeneration, UiGraphWorldDifferenceKind, UiGraphWorldProfile};
    use worth_ui_dsl::{
        UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey, UiDslSourceProvenance,
        UiDslStructuralToken, WorthUiDslPackage,
    };

    use super::UiGraphSnapshot;

    #[test]
    fn same_world_successor_requires_explicit_generation_lineage() {
        let initial = committed_snapshot_fixture();
        let successor = UiGraphSnapshot::new(
            UiGraphGeneration::successor_of(initial.generation()),
            UiGraphWorldProfile::authoritative(),
            initial.nodes().to_vec(),
            initial.topology().clone(),
            initial.mounted_receipts().clone(),
            initial.core_indexes().clone(),
        );

        assert_eq!(successor.core_indexes(), initial.core_indexes());

        assert_eq!(
            successor.compare_to(&initial).kind(),
            UiGraphWorldDifferenceKind::SameWorldSuccessor
        );
    }

    fn committed_snapshot_fixture() -> UiGraphSnapshot {
        WorthUi::app()
            .with_dsl_package(
                WorthUiDslPackage::named("worth-ui.runtime.graph.tests")
                    .with_semantic_artifact_spec(
                        UiDslSemanticArtifactSpec::new(
                            UiDslSemanticKey::new("ui.graph.snapshot.successor"),
                            UiDslSemanticFamily::Control,
                            UiDslSourceProvenance::file_authored("app/graph_snapshot_tests.wui", 0),
                        )
                        .with_structural_token(UiDslStructuralToken::new("control:test")),
                    ),
            )
            .freeze()
            .graph_snapshot()
            .clone()
    }
}
