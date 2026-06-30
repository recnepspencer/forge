use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationCanonicalEntryKind, ForgeQueryDeclarationCanonicalValue,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationProgressionContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryRelationalTruthAuthority, ForgeQuerySignalNotCompatiblePosture,
};
use forge_relational::facade::identity::{EntityId, RelationId};

use crate::query_domain::{TopologyQueryDomain, TOPOLOGY_SNAPSHOT_READ_ONLY_CONTEXT_IDENTITY};
use crate::topology_operators::{
    TopologyDeclaredMutationSequence, TopologyDeclaredMutationSequenceBuilder,
    TopologyDeclaredTouchedGraphBasisProof, TopologyTouchedOperatingWorld,
};

use super::super::shared::{canonical_entity_id, canonical_relation_id};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyRadialSpliceMember {
    relation_id: RelationId,
    half_edge_id: EntityId,
    radial_next_half_edge_id: EntityId,
}

impl TopologyRadialSpliceMember {
    pub fn new(
        relation_id: RelationId,
        half_edge_id: EntityId,
        radial_next_half_edge_id: EntityId,
    ) -> Self {
        Self {
            relation_id,
            half_edge_id,
            radial_next_half_edge_id,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologySpliceRadialAdjacencyProgramDeclaration {
    splices: Vec<TopologyRadialSpliceMember>,
}

impl TopologySpliceRadialAdjacencyProgramDeclaration {
    pub fn new(splices: Vec<TopologyRadialSpliceMember>) -> Self {
        Self { splices }
    }

    pub fn splices(&self) -> &[TopologyRadialSpliceMember] {
        &self.splices
    }

    pub fn declared_touched_basis_proof(
        &self,
        semantic_family_key: &'static str,
        operating_world: TopologyTouchedOperatingWorld,
    ) -> Result<
        TopologyDeclaredTouchedGraphBasisProof,
        forge_query::facade::ForgeQueryGraphTouchDescriptorDenial,
    > {
        TopologyDeclaredTouchedGraphBasisProof::from_mutation_sequence(
            semantic_family_key,
            &self.clone().declared_mutation_sequence(),
            operating_world,
        )
    }

    pub(crate) fn declared_mutation_sequence(self) -> TopologyDeclaredMutationSequence {
        let mut builder = TopologyDeclaredMutationSequenceBuilder::builder();
        for splice in self.splices {
            builder.splice_radial_adjacency(
                splice.relation_id,
                splice.half_edge_id,
                splice.radial_next_half_edge_id,
            );
        }
        builder.finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TopologySpliceRadialAdjacencyProgramFamily;

impl ForgeQueryDeclarationFamilyMarker<TopologyQueryDomain>
    for TopologySpliceRadialAdjacencyProgramFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "topology.splice_radial_adjacency_program"
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
}

impl ForgeQueryDeclarationInput<TopologyQueryDomain>
    for TopologySpliceRadialAdjacencyProgramDeclaration
{
    type Family = TopologySpliceRadialAdjacencyProgramFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        let mut entries = vec![ForgeQueryDeclarationCanonicalEntry::new(
            "family.operation",
            ForgeQueryDeclarationCanonicalEntryKind::Header,
            ForgeQueryDeclarationCanonicalValue::ExactText(
                "topology.splice_radial_adjacency_program".to_string(),
            ),
        )];
        for (index, splice) in self.splices.iter().enumerate() {
            let prefix = format!(
                "topology.splice_radial_adjacency_program.splices.{}",
                index + 1
            );
            entries.push(ForgeQueryDeclarationCanonicalEntry::new(
                format!("{prefix}.relation_id"),
                ForgeQueryDeclarationCanonicalEntryKind::Identity,
                ForgeQueryDeclarationCanonicalValue::ExactText(canonical_relation_id(
                    splice.relation_id,
                )),
            ));
            entries.push(ForgeQueryDeclarationCanonicalEntry::new(
                format!("{prefix}.half_edge_id"),
                ForgeQueryDeclarationCanonicalEntryKind::Identity,
                ForgeQueryDeclarationCanonicalValue::ExactText(canonical_entity_id(
                    splice.half_edge_id,
                )),
            ));
            entries.push(ForgeQueryDeclarationCanonicalEntry::new(
                format!("{prefix}.radial_next_half_edge_id"),
                ForgeQueryDeclarationCanonicalEntryKind::Identity,
                ForgeQueryDeclarationCanonicalValue::ExactText(canonical_entity_id(
                    splice.radial_next_half_edge_id,
                )),
            ));
        }
        entries
    }
}

#[cfg(test)]
mod tests {
    use forge_relational::facade::identity::{EntityId, PartitionId, RelationId};

    use super::{TopologyRadialSpliceMember, TopologySpliceRadialAdjacencyProgramDeclaration};
    use crate::topology_operators::application::TopologyDeclarationMutationPayload;
    use crate::topology_operators::{
        TopologyDeclaredMutationSequenceBuilder, TopologyTouchedOperatingWorld,
    };

    #[test]
    fn declaration_reauthors_to_the_expected_radial_splice_program_mutation_sequence() {
        let declaration = TopologySpliceRadialAdjacencyProgramDeclaration::new(vec![
            TopologyRadialSpliceMember::new(
                RelationId::new(PartitionId::main(), 20, 1),
                EntityId::new(PartitionId::main(), 10, 1),
                EntityId::new(PartitionId::main(), 11, 1),
            ),
            TopologyRadialSpliceMember::new(
                RelationId::new(PartitionId::main(), 21, 1),
                EntityId::new(PartitionId::main(), 11, 1),
                EntityId::new(PartitionId::main(), 12, 1),
            ),
        ]);
        let sequence = declaration.into_mutation_sequence();
        let actual_contracts = sequence
            .members()
            .map(|member| member.record().clone())
            .collect::<Vec<_>>();
        let mut expected = TopologyDeclaredMutationSequenceBuilder::builder();
        expected
            .splice_radial_adjacency(
                RelationId::new(PartitionId::main(), 20, 1),
                EntityId::new(PartitionId::main(), 10, 1),
                EntityId::new(PartitionId::main(), 11, 1),
            )
            .splice_radial_adjacency(
                RelationId::new(PartitionId::main(), 21, 1),
                EntityId::new(PartitionId::main(), 11, 1),
                EntityId::new(PartitionId::main(), 12, 1),
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

    #[test]
    fn declaration_can_lower_declared_touched_basis_proof() {
        let declaration = TopologySpliceRadialAdjacencyProgramDeclaration::new(vec![
            TopologyRadialSpliceMember::new(
                RelationId::new(PartitionId::main(), 20, 1),
                EntityId::new(PartitionId::main(), 10, 1),
                EntityId::new(PartitionId::main(), 11, 1),
            ),
        ]);

        let proof = declaration
            .declared_touched_basis_proof(
                "topology.splice_radial_adjacency_program",
                TopologyTouchedOperatingWorld::mainline(),
            )
            .expect("declared touch proof should lower");

        assert_eq!(
            proof.semantic_family_key(),
            "topology.splice_radial_adjacency_program"
        );
        assert!(!proof.basis_digest().is_empty());
    }
}
