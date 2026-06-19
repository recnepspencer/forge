#[path = "../public_api_planar_boolean_loop_reconstruction_workload_evidence_fixtures.rs"]
mod fixtures;
#[path = "../public_api_planar_boolean_common_plane_reduced_operand_pair_support.rs"]
mod reduced_pair_support;

use topology::facade::{
    PlanarBooleanLoopBlueprintRegistry, PlanarBooleanLoopValidatorRuntimeLane as Lane,
};
use worth_kernel::workload_composition::WorkloadCompositionError;
use worth_spatial::facade::planar_boolean_loop_reconstruction::{
    ComparePlanarBooleanLoopReplayParity, PlanarBooleanLoopReconstructionEvidenceInput,
    PlanarBooleanLoopReconstructionEvidenceReceipt, PlanarBooleanLoopReplayParityDenialKind,
    PlanarBooleanLoopReplayParityInput,
};
use worth_spatial::facade::workload_vocabulary::{WorkloadEvidenceRow, WorkloadEvidenceStage};

use super::edge_splitting_replay_parity_support::build_edge_split_replay_parity_subject;
use super::metaboss_support::MetabossEventExtractionSubject;
use super::real_handoff_support::{real_loop_handoff_for_branch, ReplayBranch};
use fixtures::{assert_runtime_registration_proof, CounterlessLoopLedgerEvidence};

pub(crate) fn assert_loop_ledger_satisfies_workload_requirement_and_runtime_registration() {
    let registry = PlanarBooleanLoopBlueprintRegistry::phase_2();
    let matrix = registry.operator_classification_matrix();
    let validators = registry.validator_registration_plan();
    let subject =
        MetabossEventExtractionSubject::certify("phase7.4 completed loop ledger workload evidence");
    let handoff =
        real_loop_handoff_for_branch(&subject, ReplayBranch::Original, &matrix, &validators)
            .expect(
                "completed loop ledger handoff must compose through the production closeout seam",
            );

    handoff.require_boolean_loop_reconstruction().expect(
        "completed loop reconstruction ledger receipt must satisfy BooleanLoopReconstruction closeout",
    );
    assert_eq!(
        handoff
            .completed_workload()
            .evidence_ledger()
            .matched_boolean_row_for_receipt(handoff.loop_ledger_receipt())
            .expect("loop evidence row must match the concrete loop ledger receipt")
            .evidence_identity(),
        handoff.loop_ledger_receipt().receipt_identity()
    );
    assert_runtime_registration_proof(
        handoff.runtime_registration_proof(),
        handoff.loop_ledger_receipt(),
        handoff.workload_stage_index_identity(),
        matrix.registry_identity().digest(),
    );
}

pub(crate) fn assert_loop_ledger_rejects_manual_or_counterless_evidence() {
    let subject =
        MetabossEventExtractionSubject::certify("phase7.4 hostile loop ledger workload evidence");
    let registry = PlanarBooleanLoopBlueprintRegistry::phase_2();
    let matrix = registry.operator_classification_matrix();
    let validators = registry.validator_registration_plan();
    let handoff =
        real_loop_handoff_for_branch(&subject, ReplayBranch::Original, &matrix, &validators)
            .expect("real loop closeout should certify before hostile row substitution");
    let receipt = handoff.loop_ledger_receipt();

    let manual_workload = reduced_pair_support::rebuild_left_workload(
        subject.pair(),
        vec![WorkloadEvidenceRow::new(
            WorkloadEvidenceStage::BooleanLoopReconstruction,
            receipt.receipt_identity(),
        )],
    );
    let manual_denial = manual_workload
        .require_boolean_loop_reconstruction(receipt)
        .expect_err("manual loop evidence must not satisfy completed loop closeout");
    assert_eq!(
        manual_denial,
        WorkloadCompositionError::ManualEvidenceStage(
            WorkloadEvidenceStage::BooleanLoopReconstruction,
        )
    );

    let counterless_workload = reduced_pair_support::rebuild_left_workload(
        subject.pair(),
        vec![WorkloadEvidenceRow::from_boolean_evidence_receipt(
            &CounterlessLoopLedgerEvidence::new(receipt.receipt_identity()),
        )],
    );
    let counterless_denial = counterless_workload
        .require_boolean_loop_reconstruction(receipt)
        .expect_err("counterless loop evidence must not satisfy completed loop closeout");
    assert_eq!(
        counterless_denial,
        WorkloadCompositionError::CounterlessEvidenceStage(
            WorkloadEvidenceStage::BooleanLoopReconstruction,
        )
    );
}

pub(crate) fn assert_loop_ledger_replay_branch_preserves_workload_requirement() {
    let registry = PlanarBooleanLoopBlueprintRegistry::phase_2();
    let matrix = registry.operator_classification_matrix();
    let validators = registry.validator_registration_plan();
    let subject =
        MetabossEventExtractionSubject::certify("phase7.4 replay branch workload evidence");
    let original =
        real_loop_handoff_for_branch(&subject, ReplayBranch::Original, &matrix, &validators)
            .expect("original loop ledger must satisfy workload closeout");
    let replayed =
        real_loop_handoff_for_branch(&subject, ReplayBranch::Replayed, &matrix, &validators)
            .expect("replayed loop ledger must satisfy workload closeout");

    original
        .require_boolean_loop_reconstruction()
        .expect("original loop ledger must satisfy the loop reconstruction workload requirement");
    replayed.require_boolean_loop_reconstruction().expect(
        "replayed loop ledger must satisfy the same loop reconstruction workload requirement",
    );
    assert_eq!(
        original.loop_ledger_receipt().receipt_identity(),
        replayed.loop_ledger_receipt().receipt_identity()
    );
    assert_eq!(
        original.loop_ledger_receipt().request_identity(),
        replayed.loop_ledger_receipt().request_identity()
    );
    assert_eq!(
        original
            .loop_ledger_receipt()
            .downstream_consumption_identity(),
        replayed
            .loop_ledger_receipt()
            .downstream_consumption_identity()
    );
    assert_eq!(
        original.workload_stage_index_identity(),
        replayed.workload_stage_index_identity()
    );
    assert_eq!(
        original.runtime_registration_proof().proof_identity(),
        replayed.runtime_registration_proof().proof_identity()
    );
}

pub(crate) fn assert_loop_stage_requirement_maps_only_to_loop_ledger_receipts() {
    let subject =
        MetabossEventExtractionSubject::certify("phase7.4 loop ledger only workload evidence");
    let registry = PlanarBooleanLoopBlueprintRegistry::phase_2();
    let matrix = registry.operator_classification_matrix();
    let validators = registry.validator_registration_plan();
    let handoff =
        real_loop_handoff_for_branch(&subject, ReplayBranch::Original, &matrix, &validators)
            .expect("real loop closeout should certify before exact-receipt checks");
    let receipt = handoff.loop_ledger_receipt().clone();
    let base_workload = reduced_pair_support::rebuild_left_workload(subject.pair(), vec![]);

    let missing_denial = base_workload
        .require_boolean_loop_reconstruction(&receipt)
        .expect_err("split-only workload must not satisfy loop reconstruction stage");
    assert_eq!(
        missing_denial,
        WorkloadCompositionError::MissingEvidenceStage(
            WorkloadEvidenceStage::BooleanLoopReconstruction,
        )
    );

    let foreign_subject =
        MetabossEventExtractionSubject::certify("phase7.4 foreign loop ledger workload evidence");
    let foreign_handoff = real_loop_handoff_for_branch(
        &foreign_subject,
        ReplayBranch::Original,
        &matrix,
        &validators,
    )
    .expect("foreign loop closeout should also certify through the production seam");
    let foreign_workload = reduced_pair_support::rebuild_left_workload(subject.pair(), vec![])
        .with_completed_boolean_loop_reconstruction(
            foreign_handoff.loop_ledger_receipt(),
            foreign_handoff.evidence_receipt(),
            &matrix,
            &validators,
        )
        .expect("a real foreign loop receipt still composes into a completed workload row");
    let mismatch_denial = foreign_workload
        .completed_workload()
        .require_boolean_loop_reconstruction(&receipt)
        .expect_err("only the exact completed loop receipt may satisfy the workload stage");
    assert_eq!(
        mismatch_denial,
        WorkloadCompositionError::MismatchedEvidenceStage(
            WorkloadEvidenceStage::BooleanLoopReconstruction,
        )
    );
}

pub(crate) fn assert_loop_closeout_rejects_malformed_runtime_registration_artifacts() {
    let subject = MetabossEventExtractionSubject::certify(
        "phase7.4 malformed runtime registration workload evidence",
    );
    let registry = PlanarBooleanLoopBlueprintRegistry::phase_2();
    let matrix = registry.operator_classification_matrix();
    let validators = registry.validator_registration_plan();
    let handoff =
        real_loop_handoff_for_branch(&subject, ReplayBranch::Original, &matrix, &validators)
            .expect(
                "real loop closeout should certify before runtime registration mutation checks",
            );
    let receipt = handoff.loop_ledger_receipt().clone();
    let base_workload = reduced_pair_support::rebuild_left_workload(subject.pair(), vec![]);

    let missing_operator_denial = base_workload
        .with_completed_boolean_loop_reconstruction(
            &receipt,
            handoff.evidence_receipt(),
            &matrix.without_operator_named("CompareLoopReconstructionCheckpointParity"),
            &validators,
        )
        .expect_err(
            "kernel closeout must reject runtime registration matrices missing required operators",
        );
    assert_eq!(
        missing_operator_denial,
        WorkloadCompositionError::LoopRuntimeRegistration(
            "loop runtime registration is missing required phase 15 operator `CompareLoopReconstructionCheckpointParity`".to_string(),
        )
    );

    let wrong_lane_denial = base_workload
        .with_completed_boolean_loop_reconstruction(
            &receipt,
            handoff.evidence_receipt(),
            &matrix,
            &validators.with_validator_runtime_lane(
                "ValidateLoopValidatorRuntimeRegistration",
                Lane::SpatialPreparedProductValidation,
            ),
        )
        .expect_err("kernel closeout must reject validators that leave the declared runtime lane");
    assert_eq!(
        wrong_lane_denial,
        WorkloadCompositionError::LoopRuntimeRegistration(
            "loop runtime registration requires `ValidateLoopValidatorRuntimeRegistration` to stay on the Query graph-invariant runtime lane".to_string(),
        )
    );
}

pub(crate) fn assert_loop_replay_closeout_rejects_foreign_loop_authority() {
    let label = "phase7.4 loop reconstruction replay authority closeout";
    let registry = PlanarBooleanLoopBlueprintRegistry::phase_2();
    let matrix = registry.operator_classification_matrix();
    let validators = registry.validator_registration_plan();
    let subject = MetabossEventExtractionSubject::certify(label);
    let replay_subject = build_edge_split_replay_parity_subject(&subject);
    let original =
        real_loop_handoff_for_branch(&subject, ReplayBranch::Original, &matrix, &validators)
            .expect("original loop closeout should certify before hostile authority checks");
    let foreign_subject =
        MetabossEventExtractionSubject::certify("phase7.4 foreign loop authority closeout");
    let foreign = real_loop_handoff_for_branch(
        &foreign_subject,
        ReplayBranch::Original,
        &matrix,
        &validators,
    )
    .expect("foreign loop closeout should also certify through the real production seam");
    let foreign_products = foreign
        .products()
        .expect("foreign loop closeout should retain canonical products for hostile replay checks");
    let foreign_evidence = PlanarBooleanLoopReconstructionEvidenceReceipt::admit(
        PlanarBooleanLoopReconstructionEvidenceInput::from_phase_sixteen_products(
            foreign_products.reconstructed_boundary(),
            foreign_products.island_partition(),
            foreign_products.split_attribution(),
            foreign_products.role_outcomes(),
            foreign_products.degenerate_outcomes(),
            foreign_products.decision_log(),
            foreign.loop_ledger_receipt(),
            &replay_subject.replay_receipts,
        ),
    );
    let denial = ComparePlanarBooleanLoopReplayParity::compare(
        PlanarBooleanLoopReplayParityInput::admit_from_ledger_and_evidence(
            original.loop_ledger_receipt(),
            foreign.loop_ledger_receipt(),
            original.evidence_receipt(),
            &foreign_evidence,
            &replay_subject.replay_receipts,
        )
        .expect("foreign loop evidence should still admit a typed replay parity input"),
    )
    .expect_err("foreign loop authority must deny replay closeout through the production seam");

    assert!(
        matches!(
            denial.kind(),
            PlanarBooleanLoopReplayParityDenialKind::LoopEvidenceMismatch
                | PlanarBooleanLoopReplayParityDenialKind::DecisionLogMismatch
                | PlanarBooleanLoopReplayParityDenialKind::LoopLedgerMismatch
        ),
        "foreign authority should deny on a typed loop replay-closeout surface: {denial:?}"
    );
}

pub(crate) fn assert_loop_replay_closeout_rejects_foreign_retained_replay_authority() {
    let label = "phase7.4 loop reconstruction retained replay authority closeout";
    let registry = PlanarBooleanLoopBlueprintRegistry::phase_2();
    let matrix = registry.operator_classification_matrix();
    let validators = registry.validator_registration_plan();
    let subject = MetabossEventExtractionSubject::certify(label);
    let original =
        real_loop_handoff_for_branch(&subject, ReplayBranch::Original, &matrix, &validators)
            .expect(
                "original loop closeout should certify before retained replay authority checks",
            );
    let replayed =
        real_loop_handoff_for_branch(&subject, ReplayBranch::Replayed, &matrix, &validators)
            .expect(
                "replayed loop closeout should certify before retained replay authority checks",
            );
    let foreign_replay_receipts =
        build_edge_split_replay_parity_subject(&MetabossEventExtractionSubject::certify(
            "phase7.4 foreign retained replay authority closeout",
        ))
        .replay_receipts;
    let denial = PlanarBooleanLoopReplayParityInput::admit_from_ledger_and_evidence(
        original.loop_ledger_receipt(),
        replayed.loop_ledger_receipt(),
        original.evidence_receipt(),
        replayed.evidence_receipt(),
        &foreign_replay_receipts,
    )
    .expect_err("foreign retained replay authority must fail replay-closeout admission");

    assert_eq!(
        denial.kind(),
        PlanarBooleanLoopReplayParityDenialKind::CheckpointAuthorityMismatch
    );
}
