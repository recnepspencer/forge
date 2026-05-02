use std::collections::BTreeMap;

use forge_query::facade::{ForgeQueryEntity, ForgeQueryMutationBatchBuilder};
use worth_schema::facade::{WorthEntityReference, WorthTopologyEntityKind};

use super::{WorthTopologyQueryEditExecutionError, WorthTopologyQueryEditRunner};
use crate::edit::{
    WorthBoundaryMembershipKind, WorthTopologyEditAction, WorthTopologyEditContract,
};

pub(super) fn supports_admitted_relation_create_workflow(
    contracts: &[WorthTopologyEditContract],
) -> bool {
    let [create, attach] = contracts else {
        return false;
    };
    let (
        WorthTopologyEditAction::CreateTopologyEntity {
            create_key,
            kind: WorthTopologyEntityKind::Loop,
            ..
        },
        WorthTopologyEditAction::AttachBoundaryMembership {
            kind: WorthBoundaryMembershipKind::FaceInnerLoop,
            owner: WorthEntityReference::Existing(_),
            member: WorthEntityReference::Created(member_key),
            ..
        },
    ) = (&create.action, &attach.action)
    else {
        return false;
    };
    create_key.as_str() == member_key.as_str()
}

impl<'workspace, 'assembly> WorthTopologyQueryEditRunner<'workspace, 'assembly> {
    pub(super) fn lower_attach_boundary_membership(
        &self,
        builder: ForgeQueryMutationBatchBuilder,
        entity_rows: &[ForgeQueryEntity],
        created_entity_kinds: &BTreeMap<String, WorthTopologyEntityKind>,
        kind: WorthBoundaryMembershipKind,
        owner: &WorthEntityReference,
        member: &WorthEntityReference,
    ) -> Result<ForgeQueryMutationBatchBuilder, WorthTopologyQueryEditExecutionError> {
        let (expected_owner_kind, expected_member_kind) = match kind {
            WorthBoundaryMembershipKind::FaceOuterLoop
            | WorthBoundaryMembershipKind::FaceInnerLoop => {
                (WorthTopologyEntityKind::Face, WorthTopologyEntityKind::Loop)
            }
            WorthBoundaryMembershipKind::LoopOwnsHalfEdge => (
                WorthTopologyEntityKind::Loop,
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
}
