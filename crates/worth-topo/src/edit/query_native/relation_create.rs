use std::collections::BTreeMap;

use forge_query::facade::{
    ForgeQueryAspectMutationBuilder, ForgeQueryEntity, ForgeQueryMutationBatchBuilder,
    ForgeQuerySymbolicTargetReference,
};
use worth_schema::facade::{WorthEntityReference, WorthTopologyEntityKind};

use super::bindings::{query_entity_binding, QueryEntityBinding};
use super::{WorthTopologyQueryEditExecutionError, WorthTopologyQueryEditRunner};
use crate::edit::WorthShellOrWireMembershipKind;

enum LoweredEntityReference {
    Existing(QueryEntityBinding),
    Created { create_key: String },
}

impl<'workspace, 'assembly> WorthTopologyQueryEditRunner<'workspace, 'assembly> {
    pub(super) fn lower_attach_shell_or_wire_membership(
        &self,
        builder: ForgeQueryMutationBatchBuilder,
        entity_rows: &[ForgeQueryEntity],
        created_entity_kinds: &BTreeMap<String, WorthTopologyEntityKind>,
        kind: WorthShellOrWireMembershipKind,
        owner: &WorthEntityReference,
        member: &WorthEntityReference,
    ) -> Result<ForgeQueryMutationBatchBuilder, WorthTopologyQueryEditExecutionError> {
        let (expected_owner_kind, expected_member_kind) = match kind {
            WorthShellOrWireMembershipKind::RegionOwnsShell => (
                WorthTopologyEntityKind::Region,
                WorthTopologyEntityKind::Shell,
            ),
            WorthShellOrWireMembershipKind::ShellOwnsFace => (
                WorthTopologyEntityKind::Shell,
                WorthTopologyEntityKind::Face,
            ),
            WorthShellOrWireMembershipKind::WireOwnsHalfEdge => (
                WorthTopologyEntityKind::Wire,
                WorthTopologyEntityKind::HalfEdge,
            ),
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
        created_entity_kinds: &BTreeMap<String, WorthTopologyEntityKind>,
        relation_kind: worth_schema::facade::WorthTopologyRelationKind,
        source: &WorthEntityReference,
        expected_source_kind: WorthTopologyEntityKind,
        target: &WorthEntityReference,
        expected_target_kind: WorthTopologyEntityKind,
    ) -> Result<ForgeQueryMutationBatchBuilder, WorthTopologyQueryEditExecutionError> {
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
        Ok(builder.insert("WorthTopologyRelation", |mutation| {
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
    created_entity_kinds: &BTreeMap<String, WorthTopologyEntityKind>,
    reference: &WorthEntityReference,
    expected_kind: WorthTopologyEntityKind,
) -> Result<LoweredEntityReference, WorthTopologyQueryEditExecutionError> {
    match reference {
        WorthEntityReference::Existing(entity_id) => {
            let binding = query_entity_binding(entity_rows, *entity_id)?.ok_or(
                WorthTopologyQueryEditExecutionError::MissingExistingEntityBinding(*entity_id),
            )?;
            if binding.kind != expected_kind {
                return Err(
                    WorthTopologyQueryEditExecutionError::ExistingEntityKindMismatch {
                        entity_id: *entity_id,
                        expected: expected_kind,
                        actual: binding.kind,
                    },
                );
            }
            Ok(LoweredEntityReference::Existing(binding))
        }
        WorthEntityReference::Created(create_key) => {
            let Some(actual_kind) = created_entity_kinds.get(create_key.as_str()).copied() else {
                return Err(
                    WorthTopologyQueryEditExecutionError::MissingCreatedEntityReference(
                        create_key.as_str().to_string(),
                    ),
                );
            };
            if actual_kind != expected_kind {
                return Err(
                    WorthTopologyQueryEditExecutionError::CreatedEntityKindMismatch {
                        create_key: create_key.as_str().to_string(),
                        expected: expected_kind,
                        actual: actual_kind,
                    },
                );
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
