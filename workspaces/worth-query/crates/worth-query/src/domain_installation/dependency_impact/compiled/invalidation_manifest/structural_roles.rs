use super::WorthQuerySemanticDependencyRole;
use worth_foundational::facade::AuthoritativeAspectChangeKind;
use worth_runtime_bridge::facade::{
    BridgeCommittedRecordChangeKind, BridgeDeliveredCorrespondenceChange,
    BridgeSemanticDependencyCandidate,
};

pub(super) fn append(
    structural_membership: bool,
    dependency: &BridgeSemanticDependencyCandidate,
    change: &BridgeDeliveredCorrespondenceChange,
    roles: &mut Vec<WorthQuerySemanticDependencyRole>,
) {
    let Some(record) = change.structural_change() else {
        return;
    };
    roles.extend(roles_for(
        structural_membership,
        record.kind(),
        change.effective_change_kind_for(dependency),
    ));
}

fn roles_for(
    structural_membership: bool,
    record_kind: BridgeCommittedRecordChangeKind,
    effective_kind: Option<AuthoritativeAspectChangeKind>,
) -> Vec<WorthQuerySemanticDependencyRole> {
    let mut roles = Vec::new();
    if structural_membership
        && matches!(
            record_kind,
            BridgeCommittedRecordChangeKind::Created | BridgeCommittedRecordChangeKind::Deleted
        )
    {
        roles.push(WorthQuerySemanticDependencyRole::SelectionOrMembership);
    }
    if matches!(
        effective_kind,
        Some(
            AuthoritativeAspectChangeKind::LifecycleCreate
                | AuthoritativeAspectChangeKind::LifecycleDelete
                | AuthoritativeAspectChangeKind::LifecycleRetainForAudit
        )
    ) {
        roles.push(WorthQuerySemanticDependencyRole::SupportAndLifecycle);
    }
    roles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_delete_mint_direct_membership_roles_without_signal_consequences() {
        for (record_kind, effective_kind) in [
            (
                BridgeCommittedRecordChangeKind::Created,
                AuthoritativeAspectChangeKind::StructuralCreate,
            ),
            (
                BridgeCommittedRecordChangeKind::Deleted,
                AuthoritativeAspectChangeKind::StructuralDelete,
            ),
        ] {
            assert_eq!(
                roles_for(true, record_kind, Some(effective_kind)),
                vec![WorthQuerySemanticDependencyRole::SelectionOrMembership]
            );
        }
    }

    #[test]
    fn lifecycle_record_change_adds_its_direct_support_role() {
        assert_eq!(
            roles_for(
                false,
                BridgeCommittedRecordChangeKind::Deleted,
                Some(AuthoritativeAspectChangeKind::LifecycleDelete),
            ),
            vec![WorthQuerySemanticDependencyRole::SupportAndLifecycle]
        );
    }
}
