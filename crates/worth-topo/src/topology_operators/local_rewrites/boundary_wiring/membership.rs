use std::collections::BTreeMap;

use forge_query::facade::ForgeQueryMutationBatchBuilder;
use schema::facade::platform::authority::EntityReference;
use schema::facade::platform::entities::TopologyEntityKind;

use crate::projection::runtime_boundary::query_runtime::TopologyQueryBindingIndex;
use crate::topology_operators::application::{
    TopologyMutationApplicationError, TopologyMutationApplicationRunner,
};
use crate::topology_operators::BoundaryMembershipKind;
#[cfg(test)]
use crate::topology_operators::TopologyDeclaredMutationActionRef;
#[cfg(test)]
use crate::topology_operators::TopologyDeclaredMutationSequence;

#[cfg(test)]
pub(crate) fn supports_admitted_relation_create_program(
    sequence: &TopologyDeclaredMutationSequence,
) -> bool {
    let mut members = sequence.members();
    let (Some(create), Some(attach), None) = (members.next(), members.next(), members.next())
    else {
        return false;
    };
    let (
        TopologyDeclaredMutationActionRef::CreateTopologyEntity {
            create_key,
            kind: TopologyEntityKind::Loop,
        },
        TopologyDeclaredMutationActionRef::AttachBoundaryMembership {
            kind: BoundaryMembershipKind::FaceInnerLoop,
            owner: EntityReference::Existing(_),
            member: EntityReference::Created(member_key),
        },
    ) = (create.action_ref(), attach.action_ref())
    else {
        return false;
    };
    create_key == member_key.as_str()
}

impl<'workspace, 'surfaces> TopologyMutationApplicationRunner<'workspace, 'surfaces> {
    pub(crate) fn lower_attach_boundary_membership(
        &self,
        builder: ForgeQueryMutationBatchBuilder,
        bindings: &TopologyQueryBindingIndex,
        created_entity_kinds: &BTreeMap<String, TopologyEntityKind>,
        kind: BoundaryMembershipKind,
        owner: &EntityReference,
        member: &EntityReference,
    ) -> Result<ForgeQueryMutationBatchBuilder, TopologyMutationApplicationError> {
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
