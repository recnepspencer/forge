use std::collections::BTreeMap;

use forge_query::facade::{
    ForgeQueryAspectMutationBuilder, ForgeQueryMutationBatchBuilder,
    ForgeQuerySymbolicTargetReference,
};
use schema::facade::platform::authority::EntityReference;
use schema::facade::platform::entities::TopologyEntityKind;

use crate::projection::runtime_boundary::query_runtime::TopologyQueryBindingIndex;
use crate::topology_operators::application::bindings::{query_entity_binding, QueryEntityBinding};
use crate::topology_operators::application::{
    TopologyOperatorExecutionError, TopologyOperatorRunner,
};
use crate::topology_operators::ShellOrWireMembershipKind;

enum LoweredEntityReference {
    Existing(QueryEntityBinding),
    Created { create_key: String },
}

impl<'workspace, 'assembly> TopologyOperatorRunner<'workspace, 'assembly> {
    pub(crate) fn lower_attach_shell_or_wire_membership(
        &self,
        builder: ForgeQueryMutationBatchBuilder,
        bindings: &TopologyQueryBindingIndex,
        created_entity_kinds: &BTreeMap<String, TopologyEntityKind>,
        kind: ShellOrWireMembershipKind,
        owner: &EntityReference,
        member: &EntityReference,
    ) -> Result<ForgeQueryMutationBatchBuilder, TopologyOperatorExecutionError> {
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
            bindings,
            created_entity_kinds,
            kind.relation_kind(),
            owner,
            expected_owner_kind,
            member,
            expected_member_kind,
        )
    }

    pub(crate) fn lower_relation_create(
        &self,
        builder: ForgeQueryMutationBatchBuilder,
        bindings: &TopologyQueryBindingIndex,
        created_entity_kinds: &BTreeMap<String, TopologyEntityKind>,
        relation_kind: schema::facade::platform::relations::TopologyRelationKind,
        source: &EntityReference,
        expected_source_kind: TopologyEntityKind,
        target: &EntityReference,
        expected_target_kind: TopologyEntityKind,
    ) -> Result<ForgeQueryMutationBatchBuilder, TopologyOperatorExecutionError> {
        let source =
            lower_entity_reference(bindings, created_entity_kinds, source, expected_source_kind)?;
        let target =
            lower_entity_reference(bindings, created_entity_kinds, target, expected_target_kind)?;
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
    bindings: &TopologyQueryBindingIndex,
    created_entity_kinds: &BTreeMap<String, TopologyEntityKind>,
    reference: &EntityReference,
    expected_kind: TopologyEntityKind,
) -> Result<LoweredEntityReference, TopologyOperatorExecutionError> {
    match reference {
        EntityReference::Existing(entity_id) => {
            let binding = query_entity_binding(bindings, *entity_id)?
                .ok_or(TopologyOperatorExecutionError::MissingExistingEntityBinding(*entity_id))?;
            if binding.kind != expected_kind {
                return Err(TopologyOperatorExecutionError::ExistingEntityKindMismatch {
                    entity_id: *entity_id,
                    expected: expected_kind,
                    actual: binding.kind,
                });
            }
            Ok(LoweredEntityReference::Existing(binding))
        }
        EntityReference::Created(create_key) => {
            let Some(actual_kind) = created_entity_kinds.get(create_key.as_str()).copied() else {
                return Err(
                    TopologyOperatorExecutionError::MissingCreatedEntityReference(
                        create_key.as_str().to_string(),
                    ),
                );
            };
            if actual_kind != expected_kind {
                return Err(TopologyOperatorExecutionError::CreatedEntityKindMismatch {
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
