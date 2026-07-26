use super::*;
use crate::domain_computation::{basis_lifecycle, BasisFamily, DeniedBasisCapabilityKind};

#[test]
fn fluent_and_explicit_current_head_observation_share_one_scoped_identity() {
    let fluent = basis_lifecycle().current_head().observe().unwrap();
    let explicit = basis_lifecycle()
        .current_head()
        .for_observation()
        .unwrap()
        .admit()
        .unwrap()
        .scope();

    assert_eq!(fluent, explicit);
    assert_eq!(fluent.family(), BasisFamily::CurrentHead);
}

#[test]
fn fluent_and_explicit_branch_head_mutation_share_one_scoped_identity() {
    let fluent = basis_lifecycle()
        .branch_head("branch:phase-3", true)
        .prepare_mutation()
        .unwrap();
    let explicit = scope_basis_for_mutation_preparation(
        basis_lifecycle()
            .branch_head("branch:phase-3", true)
            .for_mutation_preparation()
            .unwrap()
            .admit()
            .unwrap(),
    );

    assert_eq!(fluent, explicit);
    assert_eq!(fluent.family(), BasisFamily::BranchHead);
}

#[test]
fn fluent_and_explicit_branch_snapshot_inspection_share_one_scoped_identity() {
    let fluent = basis_lifecycle()
        .branch_snapshot("branch:phase-3", "snapshot:phase-3")
        .inspect()
        .unwrap();
    let explicit = basis_lifecycle()
        .branch_snapshot("branch:phase-3", "snapshot:phase-3")
        .for_inspection()
        .unwrap()
        .admit()
        .unwrap()
        .scope();

    assert_eq!(fluent, explicit);
    assert_eq!(fluent.family(), BasisFamily::BranchSnapshot);
}

#[test]
fn fluent_and_explicit_preview_closeout_share_one_scoped_identity() {
    let fluent = basis_lifecycle()
        .preview("preview:phase-3")
        .close_preview()
        .unwrap();
    let explicit = basis_lifecycle()
        .preview("preview:phase-3")
        .for_preview_closeout()
        .unwrap()
        .admit()
        .unwrap()
        .scope();

    assert_eq!(fluent, explicit);
    assert_eq!(fluent.family(), BasisFamily::Preview);
}

#[test]
fn fluent_and_explicit_runtime_snapshot_observation_share_one_scoped_identity() {
    let fluent = basis_lifecycle()
        .runtime_snapshot("snapshot:phase-3", "binding:phase-3")
        .observe()
        .unwrap();
    let explicit = basis_lifecycle()
        .runtime_snapshot("snapshot:phase-3", "binding:phase-3")
        .for_observation()
        .unwrap()
        .admit()
        .unwrap()
        .scope();

    assert_eq!(fluent, explicit);
    assert_eq!(fluent.family(), BasisFamily::RuntimeSnapshot);
}

#[test]
fn fluent_and_explicit_historical_snapshot_replay_share_one_scoped_identity() {
    let fluent = basis_lifecycle()
        .historical_snapshot("snapshot:phase-3", true)
        .replay()
        .unwrap();
    let explicit = basis_lifecycle()
        .historical_snapshot("snapshot:phase-3", true)
        .for_replay()
        .unwrap()
        .admit()
        .unwrap()
        .scope();

    assert_eq!(fluent, explicit);
    assert_eq!(fluent.family(), BasisFamily::HistoricalSnapshot);
}

#[test]
fn historical_commit_is_a_real_materialization_family() {
    let fluent = basis_lifecycle()
        .historical_commit("commit:phase-3", true)
        .materialize()
        .unwrap();
    let explicit = basis_lifecycle()
        .historical_commit("commit:phase-3", true)
        .for_materialization()
        .unwrap()
        .admit()
        .unwrap()
        .scope();

    assert_eq!(fluent, explicit);
    assert_eq!(fluent.family(), BasisFamily::HistoricalCommit);
}

#[test]
fn tenant_and_policy_declarations_converge_with_explicit_paths() {
    let tenant_fluent = basis_lifecycle()
        .tenant_scoped("tenant:a", "branch:a", "schema:a", true)
        .observe()
        .unwrap();
    let tenant_explicit = basis_lifecycle()
        .tenant_scoped("tenant:a", "branch:a", "schema:a", true)
        .for_observation()
        .unwrap()
        .admit()
        .unwrap()
        .scope();
    let policy_fluent = basis_lifecycle()
        .policy_scoped("policy:a", "tenant:a", "branch:a", "schema:a")
        .inspect()
        .unwrap();

    assert_eq!(tenant_fluent, tenant_explicit);
    assert_eq!(tenant_fluent.family(), BasisFamily::TenantScoped);
    assert_eq!(policy_fluent.family(), BasisFamily::PolicyScoped);
}

#[test]
fn tenant_and_policy_subscription_declarations_are_scoped_and_fail_closed() {
    let tenant = basis_lifecycle()
        .tenant_scoped("tenant:a", "branch:a", "schema:a", true)
        .declare_subscription()
        .unwrap();
    let policy = basis_lifecycle()
        .policy_scoped("policy:a", "tenant:a", "branch:a", "schema:a")
        .declare_subscription()
        .unwrap();
    let tenant_mismatch = basis_lifecycle()
        .tenant_scoped("tenant:a", "branch:a", "schema:b", false)
        .declare_subscription()
        .unwrap_err();
    let policy_mask = basis_lifecycle()
        .policy_scoped("policy:a", "tenant:a", "branch:a", "schema:a")
        .policy_masks_operation()
        .declare_subscription()
        .unwrap_err();

    assert_eq!(tenant.family(), BasisFamily::TenantScoped);
    assert_eq!(policy.family(), BasisFamily::PolicyScoped);
    assert_denial(
        tenant_mismatch,
        DeniedBasisCapabilityKind::SchemaIncompatible,
    );
    assert_denial(policy_mask, DeniedBasisCapabilityKind::PolicyMasked);
}

#[test]
fn hostile_declarations_deny_without_a_partial_scoped_successor() {
    let stale = basis_lifecycle()
        .stale_preview("preview:stale")
        .close_preview()
        .unwrap_err();
    let inaccessible = basis_lifecycle()
        .branch_head("branch:hidden", false)
        .observe()
        .unwrap_err();
    let tenant_mismatch = basis_lifecycle()
        .tenant_scoped("tenant:a", "branch:a", "schema:b", false)
        .observe()
        .unwrap_err();
    let policy_mask = basis_lifecycle()
        .policy_scoped("policy:a", "tenant:a", "branch:a", "schema:a")
        .policy_masks_operation()
        .observe()
        .unwrap_err();
    let replay_gap = basis_lifecycle()
        .historical_commit("commit:no-replay", false)
        .replay()
        .unwrap_err();

    assert_denial(stale, DeniedBasisCapabilityKind::PreviewDrifted);
    assert_denial(inaccessible, DeniedBasisCapabilityKind::Inaccessible);
    assert_denial(
        tenant_mismatch,
        DeniedBasisCapabilityKind::SchemaIncompatible,
    );
    assert_denial(policy_mask, DeniedBasisCapabilityKind::PolicyMasked);
    assert_denial(
        replay_gap,
        DeniedBasisCapabilityKind::HistoricalReplayUnsupported,
    );
}

#[test]
fn distinct_snapshot_generations_cannot_collapse_to_one_scoped_capability() {
    let generation_one = basis_lifecycle()
        .branch_snapshot("branch:stable-label", "snapshot:generation-1")
        .observe()
        .unwrap();
    let generation_two = basis_lifecycle()
        .branch_snapshot("branch:stable-label", "snapshot:generation-2")
        .observe()
        .unwrap();

    assert_ne!(generation_one, generation_two);
    assert_ne!(
        generation_one.scoped_basis_digest(),
        generation_two.scoped_basis_digest()
    );
}

fn assert_denial(error: BasisLifecycleDeclarationError, expected: DeniedBasisCapabilityKind) {
    let denial = error
        .eligibility_denial()
        .expect("hostile declaration should reach a typed eligibility denial");
    assert_eq!(denial.denial_kind(), expected);
    assert_eq!(denial.counters().denied_residue_count(), 0);
}
