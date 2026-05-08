use std::collections::BTreeMap;

use forge_query::facade::{
    ForgeQueryAspectMutationBuilder, ForgeQueryEntity, ForgeQueryMutationBatchBuilder,
    ForgeQuerySymbolicTargetReference,
};
use schema::facade::{EntityReference, TopologyEntityKind};

use super::bindings::{query_entity_binding, QueryEntityBinding};
use super::{TopologyQueryEditExecutionError, TopologyQueryEditRunner};
use crate::edit::ShellOrWireMembershipKind;

enum LoweredEntityReference {
    Existing(QueryEntityBinding),
    Created { create_key: String },
}

impl<'workspace, 'assembly> TopologyQueryEditRunner<'workspace, 'assembly> {
    pub(super) fn lower_attach_shell_or_wire_membership(
        &self,
        builder: ForgeQueryMutationBatchBuilder,
        entity_rows: &[ForgeQueryEntity],
        created_entity_kinds: &BTreeMap<String, TopologyEntityKind>,
        kind: ShellOrWireMembershipKind,
        owner: &EntityReference,
        member: &EntityReference,
    ) -> Result<ForgeQueryMutationBatchBuilder, TopologyQueryEditExecutionError> {
        let (expected_owner_kind, expected_member_kind) = match kind {
            ShellOrWireMembershipKind::RegionOwnsShell => {
                (TopologyEntityKind::Region, TopologyEntityKind::Shell)
            }
            ShellOrWireMembershipKind::ShellOwnsFace => {
                (TopologyEntityKind::Shell, TopologyEntityKind::Face)
            }
            ShellOrWireMembershipKind::WireOwnsHalfEdge => {
                (TopologyEntityKind::Wire, TopologyEntityKind::HalfEdge)
            }
        };
        self.lower_relation_create(
            builder,
            entity_rows,
            created_entity_kinds,
            kind.relation_kind(),
            owner,
            expected_owner_kind,
            member,
            expected_member_kind,
        )
    }

    pub(super) fn lower_relation_create(
        &self,
        builder: ForgeQueryMutationBatchBuilder,
        entity_rows: &[ForgeQueryEntity],
        created_entity_kinds: &BTreeMap<String, TopologyEntityKind>,
        relation_kind: schema::facade::TopologyRelationKind,
        source: &EntityReference,
        expected_source_kind: TopologyEntityKind,
        target: &EntityReference,
        expected_target_kind: TopologyEntityKind,
    ) -> Result<ForgeQueryMutationBatchBuilder, TopologyQueryEditExecutionError> {
        let source = lower_entity_reference(
            entity_rows,
            created_entity_kinds,
            source,
            expected_source_kind,
        )?;
        let target = lower_entity_reference(
            entity_rows,
            created_entity_kinds,
            target,
            expected_target_kind,
        )?;
        Ok(builder.insert("TopologyRelation", |mutation| {
            let mutation = authored_relation_endpoint(
                mutation.aspect("topology.kind", relation_kind.kind_name()),
                "topology.source_identity",
                &source,
            );
            authored_relation_endpoint(mutation, "topology.target_identity", &target)
        }))
    }
}

fn lower_entity_reference(
    entity_rows: &[ForgeQueryEntity],
    created_entity_kinds: &BTreeMap<String, TopologyEntityKind>,
    reference: &EntityReference,
    expected_kind: TopologyEntityKind,
) -> Result<LoweredEntityReference, TopologyQueryEditExecutionError> {
    match reference {
        EntityReference::Existing(entity_id) => {
            let binding = query_entity_binding(entity_rows, *entity_id)?
                .ok_or(TopologyQueryEditExecutionError::MissingExistingEntityBinding(*entity_id))?;
            if binding.kind != expected_kind {
                return Err(
                    TopologyQueryEditExecutionError::ExistingEntityKindMismatch {
                        entity_id: *entity_id,
                        expected: expected_kind,
                        actual: binding.kind,
                    },
                );
            }
            Ok(LoweredEntityReference::Existing(binding))
        }
        EntityReference::Created(create_key) => {
            let Some(actual_kind) = created_entity_kinds.get(create_key.as_str()).copied() else {
                return Err(
                    TopologyQueryEditExecutionError::MissingCreatedEntityReference(
                        create_key.as_str().to_string(),
                    ),
                );
            };
            if actual_kind != expected_kind {
                return Err(TopologyQueryEditExecutionError::CreatedEntityKindMismatch {
                    create_key: create_key.as_str().to_string(),
                    expected: expected_kind,
                    actual: actual_kind,
                });
            }
            Ok(LoweredEntityReference::Created {
                create_key: create_key.as_str().to_string(),
            })
        }
    }
}

fn authored_relation_endpoint(
    mutation: ForgeQueryAspectMutationBuilder,
    aspect_path: &'static str,
    reference: &LoweredEntityReference,
) -> ForgeQueryAspectMutationBuilder {
    match reference {
        LoweredEntityReference::Existing(binding) => {
            mutation.aspect(aspect_path, binding.query_identity.clone())
        }
        LoweredEntityReference::Created { create_key } => mutation.symbolic_entity_identity(
            aspect_path,
            ForgeQuerySymbolicTargetReference::new(create_key.clone())
                .expect("created entity keys are non-empty"),
        ),
    }
}
