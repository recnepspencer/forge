use forge_query::facade::{
    admit_basis_capability, evaluate_basis_inspection_eligibility, normalize_raw_basis_intent,
    LowerRuntimeBasisEvidence, RawBasisIntent, ScopedInspectionBasis,
};
use worth_spatial::facade::planar_retained_facts::RetainedPlanarFactsContracts;

use super::contract_subject::{retained_planar_parts, retained_planar_receipt};
use super::runtime_handles::retained_planar_handle;

#[test]
fn retained_planar_facts_replay_without_live_state_repair() {
    let world = "retained-planar-replay";
    let receipt = retained_planar_receipt(world);
    let contracts = RetainedPlanarFactsContracts::new(retained_planar_handle(world));
    let replay_subject = receipt.replay_subject();

    let historical = receipt
        .historical_inspection()
        .against_replay_subject(replay_subject.clone())
        .inspect(&contracts)
        .expect("historical retained planar replay");
    let branch_basis = scoped_branch_head_inspection_basis("branch:retained-planar");
    let branch_local = receipt
        .branch_local_inspection(
            branch_basis.clone(),
            matching_branch_evidence(&branch_basis, "evidence:retained-planar"),
        )
        .against_replay_subject(replay_subject)
        .inspect(&contracts)
        .expect("branch-local retained planar replay");

    assert_replay_subject_matches_query_artifacts(&receipt.replay_subject(), &receipt);
    assert_eq!(
        historical.retained_fact_digest(),
        receipt.retained_fact_digest()
    );
    assert_eq!(
        branch_local.retained_fact_digest(),
        receipt.retained_fact_digest()
    );
    assert_eq!(
        receipt
            .basis()
            .motion_posture_receipt()
            .counters()
            .rotation_rows_inspected(),
        1
    );
    assert_eq!(
        receipt
            .basis()
            .structural_identity_receipt()
            .basis()
            .motion_posture_receipt()
            .expect("structural identity retained motion")
            .retained_motion_digest(),
        receipt
            .basis()
            .motion_posture_receipt()
            .retained_motion_digest()
    );
}

fn assert_replay_subject_matches_query_artifacts(
    subject: &worth_spatial::facade::planar_retained_facts::RetainedPlanarFactsReplaySubject,
    receipt: &worth_spatial::facade::planar_retained_facts::RetainedPlanarFactsReceipt,
) {
    assert_eq!(subject.declaration_digest(), receipt.declaration_digest());
    assert_eq!(subject.progression_digest(), receipt.progression_digest());
    assert_eq!(subject.route_plan_digest(), receipt.route_plan_digest());
    assert_eq!(
        subject.query_receipt_digest(),
        receipt.query_receipt_digest()
    );
    assert_eq!(subject.envelope_digest(), receipt.envelope_digest());
    assert_eq!(
        subject.retained_fact_digest(),
        receipt.retained_fact_digest()
    );
}

#[test]
fn retained_planar_history_cancellation_chain_replays_without_repair() {
    let parts = retained_planar_parts("retained-planar-cancellation-chain");
    assert_eq!(parts.motion.counters().rotation_rows_inspected(), 1);
    assert_eq!(parts.motion.counters().cancellation_rows_inspected(), 1);
    assert_eq!(
        parts
            .structural
            .basis()
            .motion_posture_receipt()
            .expect("motion retained by identity")
            .retained_motion_digest(),
        parts.motion.retained_motion_digest()
    );
}

pub(crate) fn scoped_branch_head_inspection_basis(branch_identity: &str) -> ScopedInspectionBasis {
    let normalized = normalize_raw_basis_intent(
        RawBasisIntent::BranchHead {
            branch_identity: branch_identity.to_string(),
            accessible: true,
        },
        "inspection",
    )
    .expect("branch-head basis");
    let eligibility = evaluate_basis_inspection_eligibility(normalized).expect("eligibility");
    forge_query::facade::scope_basis_for_inspection(admit_basis_capability(eligibility))
}

pub(crate) fn matching_branch_evidence(
    scoped_basis: &ScopedInspectionBasis,
    evidence_digest: &str,
) -> LowerRuntimeBasisEvidence {
    LowerRuntimeBasisEvidence::from_relational_facade(
        scoped_basis
            .expected_lower_runtime_binding_digest()
            .expect("branch basis digest"),
        evidence_digest,
        1,
    )
}
