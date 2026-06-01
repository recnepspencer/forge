use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationCanonicalEntryKind, ForgeQueryDeclarationCanonicalValue,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationProgressionContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryRelationalTruthAuthority, ForgeQuerySignalNotCompatiblePosture,
};
use schema::facade::platform::authority::{CreateKey, EntityReference};
use schema::facade::platform::entities::TopologyEntityKind;

use super::super::shared::canonical_entity_reference_entry;
use crate::query_domain::{TopologyQueryDomain, TOPOLOGY_SNAPSHOT_READ_ONLY_CONTEXT_IDENTITY};
use crate::topology_operators::{
    BoundaryMembershipKind, TopologyDeclaredMutationSequence,
    TopologyDeclaredMutationSequenceBuilder,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyCreateInnerLoopOnExistingFaceDeclaration {
    loop_create_key: String,
    relation_create_key: String,
    face: EntityReference,
}

impl TopologyCreateInnerLoopOnExistingFaceDeclaration {
    pub fn new(
        loop_create_key: impl Into<String>,
        relation_create_key: impl Into<String>,
        face: impl Into<EntityReference>,
    ) -> Self {
        Self {
            loop_create_key: loop_create_key.into(),
            relation_create_key: relation_create_key.into(),
            face: face.into(),
        }
    }

    pub fn loop_create_key(&self) -> &str {
        &self.loop_create_key
    }

    pub fn relation_create_key(&self) -> &str {
        &self.relation_create_key
    }

    pub fn face(&self) -> &EntityReference {
        &self.face
    }

    pub(crate) fn declared_mutation_sequence(self) -> TopologyDeclaredMutationSequence {
        let mut builder = TopologyDeclaredMutationSequenceBuilder::builder();
        builder.create_topology_entity(self.loop_create_key.clone(), TopologyEntityKind::Loop);
        builder.attach_boundary_membership(
            self.relation_create_key,
            BoundaryMembershipKind::FaceInnerLoop,
            self.face,
            EntityReference::Created(CreateKey::new(self.loop_create_key)),
        );
        builder.finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TopologyCreateInnerLoopOnExistingFaceFamily;

impl ForgeQueryDeclarationFamilyMarker<TopologyQueryDomain>
    for TopologyCreateInnerLoopOnExistingFaceFamily
{
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "topology.create_inner_loop_on_existing_face"
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
    for TopologyCreateInnerLoopOnExistingFaceDeclaration
{
    type Family = TopologyCreateInnerLoopOnExistingFaceFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            ForgeQueryDeclarationCanonicalEntry::new(
                "family.operation",
                ForgeQueryDeclarationCanonicalEntryKind::Header,
                ForgeQueryDeclarationCanonicalValue::ExactText(
                    "topology.create_inner_loop_on_existing_face".to_string(),
                ),
            ),
            ForgeQueryDeclarationCanonicalEntry::new(
                "topology.create_inner_loop_on_existing_face.loop_create_key",
                ForgeQueryDeclarationCanonicalEntryKind::Identity,
                ForgeQueryDeclarationCanonicalValue::ExactText(self.loop_create_key.clone()),
            ),
            ForgeQueryDeclarationCanonicalEntry::new(
                "topology.create_inner_loop_on_existing_face.relation_create_key",
                ForgeQueryDeclarationCanonicalEntryKind::Identity,
                ForgeQueryDeclarationCanonicalValue::ExactText(self.relation_create_key.clone()),
            ),
            canonical_entity_reference_entry(
                "topology.create_inner_loop_on_existing_face.face",
                &self.face,
            ),
        ]
    }
}

#[cfg(test)]
mod tests {
    use forge_relational::facade::identity::{EntityId, PartitionId};
    use schema::facade::platform::entities::TopologyEntityKind;

    use super::TopologyCreateInnerLoopOnExistingFaceDeclaration;
    use crate::topology_operators::application::TopologyDeclarationMutationPayload;
    use crate::topology_operators::{
        BoundaryMembershipKind, TopologyDeclaredMutationSequenceBuilder,
    };

    #[test]
    fn declaration_reauthors_to_expected_inner_loop_mutation_sequence() {
        let declaration = TopologyCreateInnerLoopOnExistingFaceDeclaration::new(
            "query-native.inner-loop.loop",
            "query-native.inner-loop.relation",
            EntityId::new(PartitionId::main(), 1, 1),
        );
        let sequence = declaration.into_mutation_sequence();
        let actual_contracts = sequence
            .members()
            .map(|member| member.record().clone())
            .collect::<Vec<_>>();
        let mut expected = TopologyDeclaredMutationSequenceBuilder::builder();
        expected
            .create_topology_entity("query-native.inner-loop.loop", TopologyEntityKind::Loop)
            .attach_boundary_membership(
                "query-native.inner-loop.relation",
                BoundaryMembershipKind::FaceInnerLoop,
                EntityId::new(PartitionId::main(), 1, 1),
                schema::facade::topology_authoring::created_ref("query-native.inner-loop.loop"),
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
