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
use crate::facade::{TopologyQueryDomain, TOPOLOGY_SNAPSHOT_READ_ONLY_CONTEXT_IDENTITY};
#[cfg(test)]
use crate::topology_operators::TopologyEditAction;
#[cfg(test)]
use crate::topology_operators::TopologyEditBatch;
use crate::topology_operators::{BoundaryMembershipKind, TopologyEditContract};

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

    pub(crate) fn into_contracts(self) -> Vec<TopologyEditContract> {
        vec![
            TopologyEditContract::create_topology_entity(
                self.loop_create_key.clone(),
                TopologyEntityKind::Loop,
            ),
            TopologyEditContract::attach_boundary_membership(
                self.relation_create_key,
                BoundaryMembershipKind::FaceInnerLoop,
                self.face,
                EntityReference::Created(CreateKey::new(self.loop_create_key)),
            ),
        ]
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
pub(crate) fn declaration_for_canonical_create_inner_loop_on_existing_face_batch(
    batch: &TopologyEditBatch,
) -> Option<TopologyCreateInnerLoopOnExistingFaceDeclaration> {
    let [create, attach] = batch.contracts() else {
        return None;
    };
    let (
        TopologyEditAction::CreateTopologyEntity {
            create_key,
            kind: TopologyEntityKind::Loop,
            ..
        },
        TopologyEditAction::AttachBoundaryMembership {
            create_key: relation_create_key,
            kind: BoundaryMembershipKind::FaceInnerLoop,
            owner,
            member: EntityReference::Created(member_key),
        },
    ) = (&create.action, &attach.action)
    else {
        return None;
    };
    if create_key.as_str() != member_key.as_str() {
        return None;
    }
    let declaration = TopologyCreateInnerLoopOnExistingFaceDeclaration::new(
        create_key.as_str().to_string(),
        relation_create_key.as_str().to_string(),
        owner.clone(),
    );
    let canonical_batch = TopologyEditBatch::new(declaration.clone().into_contracts())
        .expect("grouped inner-loop declaration should always form a non-empty batch");
    (batch == &canonical_batch).then_some(declaration)
}

#[cfg(test)]
mod tests {
    use forge_relational::facade::identity::{EntityId, PartitionId};
    use schema::facade::platform::authority::{CreateKey, EntityReference};
    use schema::facade::platform::entities::TopologyEntityKind;

    use super::{
        declaration_for_canonical_create_inner_loop_on_existing_face_batch,
        TopologyCreateInnerLoopOnExistingFaceDeclaration,
    };
    use crate::topology_operators::{
        BoundaryMembershipKind, TopologyEditBatch, TopologyEditContract,
        TopologyEditDerivedFallbackPolicy,
    };

    #[test]
    fn canonical_inner_loop_batch_promotes_to_grouped_query_declaration() {
        let batch = TopologyEditBatch::new(vec![
            TopologyEditContract::create_topology_entity(
                "query-native.inner-loop.loop",
                TopologyEntityKind::Loop,
            ),
            TopologyEditContract::attach_boundary_membership(
                "query-native.inner-loop.relation",
                BoundaryMembershipKind::FaceInnerLoop,
                EntityId::new(PartitionId::main(), 1, 1),
                EntityReference::Created(CreateKey::new("query-native.inner-loop.loop")),
            ),
        ])
        .expect("grouped inner-loop batch should be non-empty");

        let declaration =
            declaration_for_canonical_create_inner_loop_on_existing_face_batch(&batch)
                .expect("canonical inner-loop batch should promote");

        assert_eq!(
            declaration,
            TopologyCreateInnerLoopOnExistingFaceDeclaration::new(
                "query-native.inner-loop.loop",
                "query-native.inner-loop.relation",
                EntityId::new(PartitionId::main(), 1, 1),
            )
        );
    }

    #[test]
    fn non_canonical_inner_loop_batch_stays_off_query_declaration_promotion() {
        let batch = TopologyEditBatch::new(vec![
            TopologyEditContract::create_topology_entity(
                "query-native.inner-loop.loop",
                TopologyEntityKind::Loop,
            ),
            TopologyEditContract::attach_boundary_membership(
                "query-native.inner-loop.relation",
                BoundaryMembershipKind::FaceInnerLoop,
                EntityId::new(PartitionId::main(), 1, 1),
                EntityReference::Created(CreateKey::new("query-native.inner-loop.loop")),
            )
            .with_derived_fallback_policy(TopologyEditDerivedFallbackPolicy::RejectAnyFallback),
        ])
        .expect("grouped inner-loop batch should be non-empty");

        assert!(
            declaration_for_canonical_create_inner_loop_on_existing_face_batch(&batch).is_none(),
            "non-canonical grouped inner-loop batch should not be silently re-authored as a query declaration"
        );
    }
}
