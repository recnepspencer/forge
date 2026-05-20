use std::collections::BTreeMap;

use forge_query::facade::ForgeQueryMutationBatchBuilder;
use schema::facade::{EntityReference, TopologyEntityKind};

use crate::projection::runtime_boundary::query_runtime::TopologyQueryBindingIndex;
use crate::topology_operators::application::{
    TopologyOperatorExecutionError, TopologyOperatorRunner,
};
use crate::topology_operators::{BoundaryMembershipKind, TopologyEditAction, TopologyEditContract};

pub(crate) fn supports_admitted_relation_create_program(
    contracts: &[TopologyEditContract],
) -> bool {
    let [create, attach] = contracts else {
        return false;
    };
    let (
        TopologyEditAction::CreateTopologyEntity {
            create_key,
            kind: TopologyEntityKind::Loop,
            ..
        },
        TopologyEditAction::AttachBoundaryMembership {
            kind: BoundaryMembershipKind::FaceInnerLoop,
            owner: EntityReference::Existing(_),
            member: EntityReference::Created(member_key),
            ..
        },
    ) = (&create.action, &attach.action)
    else {
        return false;
    };
    create_key.as_str() == member_key.as_str()
}

impl<'workspace, 'assembly> TopologyOperatorRunner<'workspace, 'assembly> {
    pub(crate) fn lower_attach_boundary_membership(
        &self,
        builder: ForgeQueryMutationBatchBuilder,
        bindings: &TopologyQueryBindingIndex,
        created_entity_kinds: &BTreeMap<String, TopologyEntityKind>,
        kind: BoundaryMembershipKind,
        owner: &EntityReference,
        member: &EntityReference,
    ) -> Result<ForgeQueryMutationBatchBuilder, TopologyOperatorExecutionError> {
        let (expected_owner_kind, expected_member_kind) = match kind {
            BoundaryMembershipKind::FaceOuterLoop | BoundaryMembershipKind::FaceInnerLoop => {
                (TopologyEntityKind::Face, TopologyEntityKind::Loop)
            }
            BoundaryMembershipKind::LoopOwnsHalfEdge => {
                (TopologyEntityKind::Loop, TopologyEntityKind::HalfEdge)
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
}
