use super::{LayoutOwnerCaseDeclarations, LayoutOwnerFamily};

pub(super) fn register(declarations: &mut LayoutOwnerCaseDeclarations) {
    use forge_store_lsm_authority::LsmMembershipOperation;
    for (family, operation) in [
        (
            LayoutOwnerFamily::LsmMembershipOpen,
            LsmMembershipOperation::Open,
        ),
        (
            LayoutOwnerFamily::LsmMembershipPersistRecord,
            LsmMembershipOperation::PersistRecord,
        ),
        (
            LayoutOwnerFamily::LsmMembershipSelectCompaction,
            LsmMembershipOperation::SelectCompaction,
        ),
        (
            LayoutOwnerFamily::LsmMembershipReplace,
            LsmMembershipOperation::ReplaceMembership,
        ),
        (
            LayoutOwnerFamily::LsmMembershipPublishedLookup,
            LsmMembershipOperation::LookupPublishedReplacement,
        ),
    ] {
        declarations.insert(
            family,
            forge_store_lsm_authority::lsm_membership_owner_case_inventory()
                .filter(move |case| case.id().operation() == operation)
                .map(|case| case.id().disposition().as_str()),
        );
    }
    use forge_store_layout_indexes::LsmExecutionOperation;
    for (family, operation) in [
        (
            LayoutOwnerFamily::LsmCompactionPreparation,
            LsmExecutionOperation::PrepareCompaction,
        ),
        (
            LayoutOwnerFamily::LsmPhysicalCompactionBinding,
            LsmExecutionOperation::BindPhysicalCompaction,
        ),
        (
            LayoutOwnerFamily::LsmMembershipActivation,
            LsmExecutionOperation::PrepareMembershipActivation,
        ),
        (
            LayoutOwnerFamily::LsmCompactionPublication,
            LsmExecutionOperation::PublishCompaction,
        ),
        (
            LayoutOwnerFamily::LsmReplayExecution,
            LsmExecutionOperation::ExecuteReplay,
        ),
    ] {
        declarations.insert(
            family,
            forge_store_layout_indexes::lsm_execution_owner_case_inventory()
                .filter(move |case| case.id().operation() == operation)
                .map(|case| case.id().disposition().as_str()),
        );
    }
    declarations.insert(
        LayoutOwnerFamily::PhysicalCompaction,
        forge_store_physical_isolation::compaction_owner_case_inventory()
            .map(|case| case.id().name()),
    );
}
