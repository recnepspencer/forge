use super::super::application_attempt::{authenticated_principal, idempotency};
use super::super::fixture::{
    canonical_governed_input_materialization_count, installed_capability_authorization_world,
    live_scope, CapabilityGovernedInputIdentity,
};
use super::capability_progression::{admitted_capability_program_with_governed_input, time};
use crate::domain_computation::primary_graph::WorthQueryApplicationCommitOutcome;
use worth_query_installation::facade::{
    WorthQueryCanonicalWorkEvidence, WorthQueryCanonicalWorkPhases,
};

#[test]
fn canonical_governed_input_work_is_owned_once_by_each_capability_admission() {
    let world = installed_capability_authorization_world();
    world.authorization_time.script([time(100); 8]);
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);
    let before_materialization = canonical_governed_input_materialization_count();
    let (first, first_admission) = admitted_capability_program_with_governed_input(
        &world,
        &principal,
        &request,
        "canonical-work",
        CapabilityGovernedInputIdentity::Canonical,
    );
    assert_eq!(
        canonical_governed_input_materialization_count(),
        before_materialization + 1
    );
    assert_one_canonical_derivation(first_admission.canonical_work);
    let (retry, retry_admission) = admitted_capability_program_with_governed_input(
        &world,
        &principal,
        &request,
        "canonical-work",
        CapabilityGovernedInputIdentity::Canonical,
    );
    assert_eq!(
        canonical_governed_input_materialization_count(),
        before_materialization + 2
    );
    assert_one_canonical_derivation(retry_admission.canonical_work);

    let WorthQueryApplicationCommitOutcome::Committed(committed) = world
        .application
        .compare_and_commit_application(first, idempotency(81, 81))
    else {
        panic!("the canonical governed-input attempt must commit");
    };
    assert_admission_only_work(committed.canonical_work(), first_admission.canonical_work);

    let WorthQueryApplicationCommitOutcome::AlreadyCommitted(recovered) = world
        .application
        .compare_and_commit_application(retry, idempotency(81, 81))
    else {
        panic!("the equivalent governed-input retry must recover the commit");
    };
    assert_admission_only_work(recovered.canonical_work(), retry_admission.canonical_work);
    assert_eq!(
        canonical_governed_input_materialization_count(),
        before_materialization + 2,
        "execution, provider commit, and retry resolution must not rematerialize governed input",
    );
}

#[test]
fn precomputed_governed_input_identity_adds_no_canonical_work_in_any_phase() {
    let world = installed_capability_authorization_world();
    world.authorization_time.script([time(100); 8]);
    let request = live_scope();
    let principal = authenticated_principal(&world, &request);
    let identity = CapabilityGovernedInputIdentity::FixedU64([29, 31, 37, 41]);
    let (first, first_admission) = admitted_capability_program_with_governed_input(
        &world,
        &principal,
        &request,
        "precomputed-work",
        identity,
    );
    assert_eq!(
        first_admission.canonical_work,
        WorthQueryCanonicalWorkEvidence::zero()
    );
    let (retry, retry_admission) = admitted_capability_program_with_governed_input(
        &world,
        &principal,
        &request,
        "precomputed-work",
        identity,
    );
    assert_eq!(
        retry_admission.canonical_work,
        WorthQueryCanonicalWorkEvidence::zero()
    );

    let WorthQueryApplicationCommitOutcome::Committed(committed) = world
        .application
        .compare_and_commit_application(first, idempotency(82, 82))
    else {
        panic!("the precomputed governed-input attempt must commit");
    };
    assert_admission_only_work(
        committed.canonical_work(),
        WorthQueryCanonicalWorkEvidence::zero(),
    );

    let WorthQueryApplicationCommitOutcome::AlreadyCommitted(recovered) = world
        .application
        .compare_and_commit_application(retry, idempotency(82, 82))
    else {
        panic!("the equivalent precomputed governed-input retry must recover the commit");
    };
    assert_admission_only_work(
        recovered.canonical_work(),
        WorthQueryCanonicalWorkEvidence::zero(),
    );
}

fn assert_one_canonical_derivation(work: WorthQueryCanonicalWorkEvidence) {
    assert_eq!(work.basis_preparations(), 1);
    assert_eq!(work.digest_derivations(), 1);
    assert_eq!(work.canonical_entries(), 1);
    assert!(work.canonical_encoded_bytes() > 0);
    assert!(work.canonical_material_allocation_bytes() > 0);
    assert!(work.sha256_input_bytes() > 0);
    assert!(work.sha256_compression_blocks() > 0);
    assert_eq!(work.digest_text_materializations(), 0);
}

fn assert_admission_only_work(
    phases: WorthQueryCanonicalWorkPhases,
    expected_admission: WorthQueryCanonicalWorkEvidence,
) {
    assert_eq!(phases.admission(), expected_admission);
    assert_eq!(phases.execution(), WorthQueryCanonicalWorkEvidence::zero());
    assert_eq!(
        phases.provider_commit(),
        WorthQueryCanonicalWorkEvidence::zero()
    );
    assert_eq!(phases.projection(), WorthQueryCanonicalWorkEvidence::zero());
    assert_eq!(
        phases.live_delivery(),
        WorthQueryCanonicalWorkEvidence::zero()
    );
    assert_eq!(
        phases.retry_resolution(),
        WorthQueryCanonicalWorkEvidence::zero()
    );
    assert_eq!(
        phases.recovery_inspection(),
        WorthQueryCanonicalWorkEvidence::zero()
    );
    assert_eq!(
        phases.publication(),
        WorthQueryCanonicalWorkEvidence::zero()
    );
}
