use schema::facade::platform::authority::EntityReference;
use schema::facade::platform::entities::TopologyEntityKind;
use schema::facade::platform::relations::TopologyRelationKind;

use crate::projection::runtime_boundary::query_runtime::TopologyQueryBindingIndex;
use crate::query_native_runtime_boundary::TopologyNativeQueryRowField;
use crate::topology_operators::application::{
    ensure_declared_touched_basis_covers_sequence_before_write, TopologyDeclaredMutationArtifact,
    TopologyMutationApplicationError, TopologyMutationApplicationRunner,
    TopologyRetainedApplicationHandoff,
};
use crate::topology_operators::topology_relation_dependency_path;
use crate::topology_operators::{
    BoundaryMembershipKind, TopologyDeclaredMutationActionRef, TopologyDeclaredMutationSequence,
    TopologyMutationApplicationMode, TopologyMutationFamily,
};

impl<'workspace, 'surfaces> TopologyMutationApplicationRunner<'workspace, 'surfaces> {
    pub(crate) fn compose_face_inner_loop_program<I>(
        &mut self,
        retained_handoff: TopologyRetainedApplicationHandoff<I>,
        mode: TopologyMutationApplicationMode,
        semantic_family_key: &'static str,
        sequence: &TopologyDeclaredMutationSequence,
        bindings: &TopologyQueryBindingIndex,
    ) -> Result<TopologyDeclaredMutationArtifact, TopologyMutationApplicationError>
    where
        I: forge_query::facade::ForgeQueryDeclarationInput<
            crate::query_domain::TopologyQueryDomain,
        >,
    {
        ensure_declared_touched_basis_covers_sequence_before_write(
            &retained_handoff,
            sequence,
            mode.clone(),
        )?;
        let members = sequence.members().collect::<Vec<_>>();
        let [create, attach] = members.as_slice() else {
            return Err(TopologyMutationApplicationError::UnsupportedFamilies(vec![
                TopologyMutationFamily::AttachBoundaryMembership,
            ]));
        };
        let (
            TopologyDeclaredMutationActionRef::CreateTopologyEntity {
                create_key,
                kind: TopologyEntityKind::Loop,
            },
            TopologyDeclaredMutationActionRef::AttachBoundaryMembership {
                kind: BoundaryMembershipKind::FaceInnerLoop,
                owner: EntityReference::Existing(face_id),
                member: EntityReference::Created(member_key),
            },
        ) = (create.action_ref(), attach.action_ref())
        else {
            return Err(TopologyMutationApplicationError::UnsupportedFamilies(vec![
                TopologyMutationFamily::AttachBoundaryMembership,
            ]));
        };
        if create_key != member_key.as_str() {
            return Err(TopologyMutationApplicationError::UnsupportedFamilies(vec![
                TopologyMutationFamily::AttachBoundaryMembership,
            ]));
        }
        let face_binding = crate::topology_operators::application::bindings::query_entity_binding(
            bindings, *face_id,
        )?
        .ok_or(TopologyMutationApplicationError::MissingExistingEntityBinding(*face_id))?;
        if face_binding.kind != TopologyEntityKind::Face {
            return Err(
                TopologyMutationApplicationError::ExistingEntityKindMismatch {
                    entity_id: *face_id,
                    expected: TopologyEntityKind::Face,
                    actual: face_binding.kind,
                },
            );
        }
        let create_key = create_key.to_string();
        let receipt = self
            .workspace
            .compose_graph(|graph| {
                let loop_symbol =
                    graph.insert_entity(create_key.clone(), "TopologyEntity", |mutation| {
                        TopologyNativeQueryRowField::NamingPersistentName.set_on(
                            TopologyNativeQueryRowField::TopologyStructure.set_on(
                                TopologyNativeQueryRowField::TopologyKind
                                    .set_on(mutation, TopologyEntityKind::Loop.kind_name()),
                                create_key.clone(),
                            ),
                            create_key.clone(),
                        )
                    })?;
                graph.insert_relation("TopologyRelation", |relation| {
                    let relation = TopologyNativeQueryRowField::TopologyKind
                        .set_on_relation(relation, TopologyRelationKind::FaceInnerLoop.kind_name());
                    let relation = relation
                        .existing_entity_identity(
                            TopologyNativeQueryRowField::TopologySourceIdentity.touch(),
                            face_binding.query_identity.clone(),
                        )
                        .symbolic_entity_identity(
                            TopologyNativeQueryRowField::TopologyTargetIdentity.touch(),
                            &loop_symbol,
                        );
                    if let Some(field) = topology_relation_dependency_path(
                        schema::facade::platform::relations::RelationKind::Topology(
                            TopologyRelationKind::FaceInnerLoop,
                        ),
                    )
                    .and_then(TopologyNativeQueryRowField::from_query_aspect_path)
                    {
                        field.set_on_relation(
                            relation,
                            TopologyRelationKind::FaceInnerLoop.kind_name(),
                        )
                    } else {
                        relation
                    }
                })?;
                Ok(())
            })
            .map_err(TopologyMutationApplicationError::from)?;
        self.finish_composed_membership_execution(
            mode,
            retained_handoff,
            semantic_family_key,
            sequence,
            receipt,
        )
    }
}
