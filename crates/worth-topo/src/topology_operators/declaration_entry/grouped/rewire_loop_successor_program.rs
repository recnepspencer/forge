use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationCanonicalEntryKind, ForgeQueryDeclarationCanonicalValue,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationProgressionContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryGraphObligationOperatingWorldSelector,
    ForgeQueryGraphObligationRegistration, ForgeQueryGraphObligationSupportLane,
    ForgeQueryGraphTouchDescriptor, ForgeQueryGraphTouchDescriptorDenial,
    ForgeQueryNeighborhoodCapableGrouping, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalNotCompatiblePosture,
};
use forge_relational::facade::identity::{EntityId, RelationId};

use crate::query_domain::{TopologyQueryDomain, TOPOLOGY_SNAPSHOT_READ_ONLY_CONTEXT_IDENTITY};
use crate::topology_operators::{
    LoopSuccessorKind, TopologyDeclaredMutationSequence, TopologyDeclaredMutationSequenceBuilder,
};

use super::super::shared::{canonical_entity_id, canonical_relation_id};

pub(crate) const TOPOLOGY_REWIRE_LOOP_SUCCESSOR_GRAPH_OBLIGATION_COLLECTION: &str =
    "TopologyRelation";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyLoopSuccessorRewireMember {
    relation_id: RelationId,
    kind: LoopSuccessorKind,
    half_edge_id: EntityId,
    successor_half_edge_id: EntityId,
}

impl TopologyLoopSuccessorRewireMember {
    pub fn new(
        relation_id: RelationId,
        kind: LoopSuccessorKind,
        half_edge_id: EntityId,
        successor_half_edge_id: EntityId,
    ) -> Self {
        Self {
            relation_id,
            kind,
            half_edge_id,
            successor_half_edge_id,
        }
    }

    pub fn relation_id(&self) -> RelationId {
        self.relation_id
    }

    pub fn kind(&self) -> LoopSuccessorKind {
        self.kind
    }

    pub fn half_edge_id(&self) -> EntityId {
        self.half_edge_id
    }

    pub fn successor_half_edge_id(&self) -> EntityId {
        self.successor_half_edge_id
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyRewireLoopSuccessorProgramDeclaration {
    rewires: Vec<TopologyLoopSuccessorRewireMember>,
}

impl TopologyRewireLoopSuccessorProgramDeclaration {
    pub fn new(rewires: Vec<TopologyLoopSuccessorRewireMember>) -> Self {
        Self { rewires }
    }

    pub fn rewires(&self) -> &[TopologyLoopSuccessorRewireMember] {
        &self.rewires
    }

    pub(crate) fn declared_mutation_sequence(self) -> TopologyDeclaredMutationSequence {
        let mut builder = TopologyDeclaredMutationSequenceBuilder::builder();
        for rewire in self.rewires {
            builder.rewire_loop_successor(
                rewire.relation_id,
                rewire.kind,
                rewire.half_edge_id,
                rewire.successor_half_edge_id,
            );
        }
        builder.finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TopologyRewireLoopSuccessorProgramFamily;

impl ForgeQueryDeclarationFamilyMarker<TopologyQueryDomain>
    for TopologyRewireLoopSuccessorProgramFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "topology.rewire_loop_successor_program"
    }

    fn required_capability_families() -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::WorkflowOrchestration,
            ForgeQueryCapabilityFamily::IdentityEvolution,
        ]
    }

    fn required_config_sections() -> &'static [ForgeQueryConfigSectionFamily] {
        &[ForgeQueryConfigSectionFamily::Relational]
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn progression_contract(
        _handle_identity_digest: &str,
        operating_context_identity_digest: &str,
    ) -> ForgeQueryDeclarationProgressionContract {
        if operating_context_identity_digest == TOPOLOGY_SNAPSHOT_READ_ONLY_CONTEXT_IDENTITY {
            ForgeQueryDeclarationProgressionContract::rebind_required()
        } else {
            ForgeQueryDeclarationProgressionContract::admitted_current()
        }
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::relational_only()
    }

    fn orchestration_graph_touch_collection() -> Option<&'static str> {
        Some(TOPOLOGY_REWIRE_LOOP_SUCCESSOR_GRAPH_OBLIGATION_COLLECTION)
    }

    fn orchestration_graph_touch_descriptor(
    ) -> Option<Result<ForgeQueryGraphTouchDescriptor, ForgeQueryGraphTouchDescriptorDenial>> {
        Some(crate::topology_operators::adoption::topology_operator_relation_touch_descriptor())
    }

    fn orchestration_graph_obligation_registrations() -> Vec<ForgeQueryGraphObligationRegistration>
    {
        vec![
            topology_rewire_loop_successor_graph_obligation_registration(
                ForgeQueryGraphObligationSupportLane::ContributionOrchestration,
                ForgeQueryGraphObligationOperatingWorldSelector::configured_domain_handle(),
            ),
        ]
    }
}

impl ForgeQueryDeclarationInput<TopologyQueryDomain>
    for TopologyRewireLoopSuccessorProgramDeclaration
{
    type Family = TopologyRewireLoopSuccessorProgramFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        let mut entries = vec![ForgeQueryDeclarationCanonicalEntry::new(
            "family.operation",
            ForgeQueryDeclarationCanonicalEntryKind::Header,
            ForgeQueryDeclarationCanonicalValue::ExactText(
                "topology.rewire_loop_successor_program".to_string(),
            ),
        )];
        for (index, rewire) in self.rewires.iter().enumerate() {
            let prefix = format!(
                "topology.rewire_loop_successor_program.rewires.{}",
                index + 1
            );
            entries.push(ForgeQueryDeclarationCanonicalEntry::new(
                format!("{prefix}.relation_id"),
                ForgeQueryDeclarationCanonicalEntryKind::Identity,
                ForgeQueryDeclarationCanonicalValue::ExactText(canonical_relation_id(
                    rewire.relation_id,
                )),
            ));
            entries.push(ForgeQueryDeclarationCanonicalEntry::new(
                format!("{prefix}.kind"),
                ForgeQueryDeclarationCanonicalEntryKind::Identity,
                ForgeQueryDeclarationCanonicalValue::ExactText(
                    rewire.kind.relation_kind().kind_name().to_string(),
                ),
            ));
            entries.push(ForgeQueryDeclarationCanonicalEntry::new(
                format!("{prefix}.half_edge_id"),
                ForgeQueryDeclarationCanonicalEntryKind::Identity,
                ForgeQueryDeclarationCanonicalValue::ExactText(canonical_entity_id(
                    rewire.half_edge_id,
                )),
            ));
            entries.push(ForgeQueryDeclarationCanonicalEntry::new(
                format!("{prefix}.successor_half_edge_id"),
                ForgeQueryDeclarationCanonicalEntryKind::Identity,
                ForgeQueryDeclarationCanonicalValue::ExactText(canonical_entity_id(
                    rewire.successor_half_edge_id,
                )),
            ));
        }
        entries
    }
}

pub(crate) fn topology_rewire_loop_successor_graph_obligation_registration(
    support_lane: ForgeQueryGraphObligationSupportLane,
    operating_world_selector: ForgeQueryGraphObligationOperatingWorldSelector,
) -> ForgeQueryGraphObligationRegistration {
    crate::topology_operators::adoption::topology_rewire_loop_successor_registration(
        support_lane,
        operating_world_selector,
    )
}

#[cfg(test)]
mod tests {
    use forge_relational::facade::identity::{EntityId, PartitionId, RelationId};

    use super::{TopologyLoopSuccessorRewireMember, TopologyRewireLoopSuccessorProgramDeclaration};
    use crate::topology_operators::application::TopologyDeclarationMutationPayload;
    use crate::topology_operators::{LoopSuccessorKind, TopologyDeclaredMutationSequenceBuilder};

    #[test]
    fn declaration_reauthors_to_the_expected_successor_program_mutation_sequence() {
        let declaration = TopologyRewireLoopSuccessorProgramDeclaration::new(vec![
            TopologyLoopSuccessorRewireMember::new(
                RelationId::new(PartitionId::main(), 20, 1),
                LoopSuccessorKind::Next,
                EntityId::new(PartitionId::main(), 10, 1),
                EntityId::new(PartitionId::main(), 11, 1),
            ),
            TopologyLoopSuccessorRewireMember::new(
                RelationId::new(PartitionId::main(), 21, 1),
                LoopSuccessorKind::Prev,
                EntityId::new(PartitionId::main(), 10, 1),
                EntityId::new(PartitionId::main(), 9, 1),
            ),
        ]);
        let sequence = declaration.into_mutation_sequence();
        let actual_contracts = sequence
            .members()
            .map(|member| member.record().clone())
            .collect::<Vec<_>>();
        let mut expected = TopologyDeclaredMutationSequenceBuilder::builder();
        expected
            .rewire_loop_successor(
                RelationId::new(PartitionId::main(), 20, 1),
                LoopSuccessorKind::Next,
                EntityId::new(PartitionId::main(), 10, 1),
                EntityId::new(PartitionId::main(), 11, 1),
            )
            .rewire_loop_successor(
                RelationId::new(PartitionId::main(), 21, 1),
                LoopSuccessorKind::Prev,
                EntityId::new(PartitionId::main(), 10, 1),
                EntityId::new(PartitionId::main(), 9, 1),
            );

        assert_eq!(
            actual_contracts,
            expected
                .finish()
                .members()
                .map(|member| member.record().clone())
                .collect::<Vec<_>>()
        );
    }
}
