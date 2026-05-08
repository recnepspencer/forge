use std::collections::BTreeMap;

use forge_query::facade::{ForgeQueryEntity, ForgeQueryMutationBatchBuilder};
use schema::facade::{EntityReference, TopologyEntityKind};

use super::{TopologyQueryEditExecutionError, TopologyQueryEditRunner};
use crate::edit::{BoundaryMembershipKind, TopologyEditAction, TopologyEditContract};

pub(super) fn supports_admitted_relation_create_workflow(
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

impl<'workspace, 'assembly> TopologyQueryEditRunner<'workspace, 'assembly> {
    pub(super) fn lower_attach_boundary_membership(
        &self,
        builder: ForgeQueryMutationBatchBuilder,
        entity_rows: &[ForgeQueryEntity],
        created_entity_kinds: &BTreeMap<String, TopologyEntityKind>,
        kind: BoundaryMembershipKind,
        owner: &EntityReference,
        member: &EntityReference,
    ) -> Result<ForgeQueryMutationBatchBuilder, TopologyQueryEditExecutionError> {
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
            entity_rows,
            created_entity_kinds,
            kind.relation_kind(),
            owner,
            expected_owner_kind,
            member,
            expected_member_kind,
        )
    }
}
