use super::anti_theatre_closeout::certify_anti_theatre_closeout;
use super::chain_rows::{
    PlanarBooleanLoopSummumBonumProofBranch as Branch,
    PlanarBooleanLoopSummumBonumProofRowKind as Kind,
};
use super::guard_coverage::{
    assert_loop_reconstruction_guard_coverage_contract, loop_reconstruction_guard_names,
};
use super::proof_bundle::certify_planar_boolean_loop_reconstruction_summum_bonum_closeout;
use super::proof_bundle::PlanarBooleanLoopSummumBonumCloseoutProofBundle;
use super::public_contract_closeout::certify_public_contract_closeout;
use super::public_contract_support::proof_rows::PlanarBooleanLoopPublicContractProofRowKind;
use super::workload_evidence_support::certified_real_loop_replay_closeout_chain;
use worth_kernel::workload_composition::{
    CompletedBooleanLoopReconstructionHandoff, CompletedBooleanLoopReconstructionProducts,
};

pub(crate) fn assert_loop_reconstruction_summum_bonum_closeout_certifies_real_production_chain() {
    assert_loop_reconstruction_guard_coverage_contract();

    let proof = certify_planar_boolean_loop_reconstruction_summum_bonum_closeout();
    let chain =
        certified_real_loop_replay_closeout_chain("phase7.4 loop reconstruction metaboss closeout");
    let original_products = chain
        .original
        .products()
        .expect("original loop handoff should retain canonical loop products");
    let replayed_products = chain
        .replayed
        .products()
        .expect("replayed loop handoff should retain canonical loop products");

    assert!(!proof.replay_parity_identity.is_empty());
    assert!(!proof.public_contract_proof_identity.is_empty());
    assert!(!proof.anti_theatre_proof_identity.is_empty());
    assert_branch_rows_recover_canonical_artifacts(
        &proof,
        Branch::Original,
        &chain.original,
        original_products,
    );
    assert_branch_rows_recover_canonical_artifacts(
        &proof,
        Branch::Replayed,
        &chain.replayed,
        replayed_products,
    );
    assert_branch_row_counts(&proof, Branch::Original, &chain.original, original_products);
    assert_branch_row_counts(&proof, Branch::Replayed, &chain.replayed, replayed_products);
    assert_shared_row_counts(&proof, &chain.original, chain.replay_parity.rows().len());
    assert!(has_row(&proof, Branch::Shared, Kind::ReplayParityReceipt));
    assert!(has_row(
        &proof,
        Branch::Shared,
        Kind::PublicContractFenceRow
    ));
    assert!(has_row(&proof, Branch::Shared, Kind::AntiTheatreFence));
    assert!(
        proof
            .proof_rows
            .iter()
            .all(|row| !row.identity().is_empty()),
        "every closeout proof row must retain a concrete identity"
    );
    assert_eq!(
        proof.proof_rows.len(),
        expected_total_row_count(
            &chain.original,
            original_products,
            replayed_products,
            chain.replay_parity.rows().len(),
        ),
        "summum bonum proof must fully account for every branch and shared typed artifact"
    );
}

pub(crate) fn assert_loop_reconstruction_summum_bonum_replay_closeout_holds() {
    let proof = certify_planar_boolean_loop_reconstruction_summum_bonum_closeout();
    let chain =
        certified_real_loop_replay_closeout_chain("phase7.4 loop reconstruction metaboss closeout");
    let replayed_products = chain
        .replayed
        .products()
        .expect("replayed loop handoff should retain canonical loop products");

    assert!(proof
        .row(
            Branch::Replayed,
            Kind::LoopLedgerReceipt,
            chain.replayed.loop_ledger_receipt().receipt_identity(),
        )
        .is_some());
    assert!(proof
        .row(
            Branch::Replayed,
            Kind::LoopEvidenceReceipt,
            chain.replayed.evidence_receipt().receipt_identity(),
        )
        .is_some());
    assert!(proof
        .row(
            Branch::Replayed,
            Kind::RuntimeRegistrationProof,
            chain.replayed.runtime_registration_proof().proof_identity(),
        )
        .is_some());
    assert!(proof
        .row(
            Branch::Replayed,
            Kind::WalkOutcomeSet,
            replayed_products
                .walk_outcomes()
                .walk_outcome_set_identity(),
        )
        .is_some());
    assert!(proof
        .row(
            Branch::Replayed,
            Kind::IslandPartition,
            replayed_products.island_partition().partition_identity(),
        )
        .is_some());
    assert!(proof
        .row(
            Branch::Replayed,
            Kind::SplitAttribution,
            replayed_products.split_attribution().attribution_identity(),
        )
        .is_some());
    assert!(
        proof
            .rows_for(Branch::Shared, Kind::ReplayParityRow)
            .count()
            == 11
    );
}

pub(crate) fn assert_loop_reconstruction_summum_bonum_public_contract_fences_hold() {
    assert_loop_reconstruction_guard_coverage_contract();

    let proof = certify_planar_boolean_loop_reconstruction_summum_bonum_closeout();

    let guards = proof
        .rows_for(Branch::Shared, Kind::AntiTheatreGuard)
        .map(|row| row.identity().to_string())
        .collect::<Vec<_>>();
    assert_eq!(guards, loop_reconstruction_guard_names());
}

fn has_row(
    proof: &PlanarBooleanLoopSummumBonumCloseoutProofBundle,
    branch: Branch,
    kind: Kind,
) -> bool {
    proof.rows_for(branch, kind).next().is_some()
}

fn count_rows(
    proof: &PlanarBooleanLoopSummumBonumCloseoutProofBundle,
    branch: Branch,
    kind: Kind,
) -> usize {
    proof.rows_for(branch, kind).count()
}

fn assert_kind_row_count(
    proof: &PlanarBooleanLoopSummumBonumCloseoutProofBundle,
    branch: Branch,
    kind: Kind,
    expected: usize,
) {
    assert_eq!(
        count_rows(proof, branch, kind),
        expected,
        "unexpected {:?} row count for {:?} branch",
        kind,
        branch
    );
}

fn assert_branch_row_counts(
    proof: &PlanarBooleanLoopSummumBonumCloseoutProofBundle,
    branch: Branch,
    handoff: &CompletedBooleanLoopReconstructionHandoff,
    products: &CompletedBooleanLoopReconstructionProducts,
) {
    let scalar_kinds = [
        Kind::LoopLedgerReceipt,
        Kind::LoopEvidenceReceipt,
        Kind::RuntimeRegistrationProof,
        Kind::WorkloadStageIndex,
        Kind::DownstreamLoopConsumption,
        Kind::WalkOutcomeSet,
        Kind::AdmittedLoopCandidateSet,
        Kind::DeniedLoopCandidateSet,
        Kind::ReconstructedLoopSet,
        Kind::BornLoopSet,
        Kind::IslandPartition,
        Kind::SplitAttribution,
        Kind::RoleOutcomeSet,
        Kind::ContainmentPostureSet,
        Kind::DegenerateOutcomeSet,
        Kind::DecisionLog,
        Kind::LoopLedger,
    ];
    for kind in scalar_kinds {
        assert_kind_row_count(proof, branch, kind, 1);
    }
    assert_kind_row_count(
        proof,
        branch,
        Kind::WalkOutcomeRow,
        products.walk_outcomes().rows().len(),
    );
    assert_kind_row_count(
        proof,
        branch,
        Kind::AdmittedLoopCandidateRow,
        products.candidate_boundary().loop_candidates().rows().len(),
    );
    assert_kind_row_count(
        proof,
        branch,
        Kind::DeniedLoopCandidateRow,
        products
            .candidate_boundary()
            .denied_loop_candidates()
            .rows()
            .len(),
    );
    assert_kind_row_count(
        proof,
        branch,
        Kind::ReconstructedLoopRow,
        products
            .reconstructed_boundary()
            .reconstructed_loops()
            .rows()
            .len(),
    );
    assert_kind_row_count(
        proof,
        branch,
        Kind::BornLoopRow,
        products.reconstructed_boundary().born_loops().rows().len(),
    );
    assert_kind_row_count(
        proof,
        branch,
        Kind::IslandPartitionRow,
        products.island_partition().rows().len(),
    );
    assert_kind_row_count(
        proof,
        branch,
        Kind::SplitAttributionRow,
        products.split_attribution().rows().len(),
    );
    assert_kind_row_count(
        proof,
        branch,
        Kind::RoleOutcomeRow,
        products.role_outcomes().rows().len(),
    );
    assert_kind_row_count(
        proof,
        branch,
        Kind::ContainmentPostureRow,
        products.containment_postures().rows().len(),
    );
    assert_kind_row_count(
        proof,
        branch,
        Kind::DegenerateOutcomeRow,
        products.degenerate_outcomes().rows().len(),
    );
    assert_kind_row_count(
        proof,
        branch,
        Kind::LedgerRow,
        products.loop_ledger().rows().len(),
    );
    assert!(
        proof
            .row(
                branch,
                Kind::WorkloadStageIndex,
                handoff.workload_stage_index_identity(),
            )
            .is_some(),
        "each branch must retain the exact workload stage index identity"
    );
    assert!(
        proof
            .row(
                branch,
                Kind::DownstreamLoopConsumption,
                handoff
                    .loop_ledger_receipt()
                    .downstream_consumption_identity(),
            )
            .is_some(),
        "each branch must retain the exact downstream loop consumption identity"
    );
}

fn assert_shared_row_counts(
    proof: &PlanarBooleanLoopSummumBonumCloseoutProofBundle,
    original_handoff: &CompletedBooleanLoopReconstructionHandoff,
    replay_parity_row_count: usize,
) {
    let public_contract = certify_public_contract_closeout(original_handoff);
    let anti_theatre = certify_anti_theatre_closeout(original_handoff, &public_contract);
    let anti_theatre_guard_count = anti_theatre
        .rows()
        .iter()
        .filter(|row| row.kind() == PlanarBooleanLoopPublicContractProofRowKind::AntiTheatreGuard)
        .count();
    let anti_theatre_fence_count = anti_theatre
        .rows()
        .iter()
        .filter(|row| row.kind() == PlanarBooleanLoopPublicContractProofRowKind::AntiTheatreFence)
        .count();

    assert_kind_row_count(proof, Branch::Shared, Kind::ReplayParityReceipt, 1);
    assert_kind_row_count(
        proof,
        Branch::Shared,
        Kind::ReplayParityRow,
        replay_parity_row_count,
    );
    assert_kind_row_count(
        proof,
        Branch::Shared,
        Kind::PublicContractFenceRow,
        public_contract.rows().len(),
    );
    assert_kind_row_count(
        proof,
        Branch::Shared,
        Kind::AntiTheatreGuard,
        anti_theatre_guard_count,
    );
    assert_kind_row_count(
        proof,
        Branch::Shared,
        Kind::AntiTheatreFence,
        anti_theatre_fence_count,
    );
}

fn expected_total_row_count(
    original_handoff: &CompletedBooleanLoopReconstructionHandoff,
    original_products: &CompletedBooleanLoopReconstructionProducts,
    replayed_products: &CompletedBooleanLoopReconstructionProducts,
    replay_parity_row_count: usize,
) -> usize {
    let original_branch_count = expected_branch_row_count(original_products);
    let replayed_branch_count = expected_branch_row_count(replayed_products);
    let public_contract = certify_public_contract_closeout(original_handoff);
    let anti_theatre = certify_anti_theatre_closeout(original_handoff, &public_contract);
    original_branch_count
        + replayed_branch_count
        + 1
        + replay_parity_row_count
        + public_contract.rows().len()
        + anti_theatre
            .rows()
            .iter()
            .filter(|row| {
                matches!(
                    row.kind(),
                    PlanarBooleanLoopPublicContractProofRowKind::AntiTheatreGuard
                        | PlanarBooleanLoopPublicContractProofRowKind::AntiTheatreFence
                )
            })
            .count()
}

fn expected_branch_row_count(products: &CompletedBooleanLoopReconstructionProducts) -> usize {
    let scalar_row_count = 17;
    scalar_row_count
        + products.walk_outcomes().rows().len()
        + products.candidate_boundary().loop_candidates().rows().len()
        + products
            .candidate_boundary()
            .denied_loop_candidates()
            .rows()
            .len()
        + products
            .reconstructed_boundary()
            .reconstructed_loops()
            .rows()
            .len()
        + products.reconstructed_boundary().born_loops().rows().len()
        + products.island_partition().rows().len()
        + products.split_attribution().rows().len()
        + products.role_outcomes().rows().len()
        + products.containment_postures().rows().len()
        + products.degenerate_outcomes().rows().len()
        + products.loop_ledger().rows().len()
}

fn assert_branch_rows_recover_canonical_artifacts(
    proof: &PlanarBooleanLoopSummumBonumCloseoutProofBundle,
    branch: Branch,
    handoff: &CompletedBooleanLoopReconstructionHandoff,
    products: &CompletedBooleanLoopReconstructionProducts,
) {
    assert!(proof
        .row(
            branch,
            Kind::LoopLedgerReceipt,
            handoff.loop_ledger_receipt().receipt_identity()
        )
        .is_some());
    assert!(proof
        .row(
            branch,
            Kind::LoopEvidenceReceipt,
            handoff.evidence_receipt().receipt_identity()
        )
        .is_some());
    assert!(proof
        .row(
            branch,
            Kind::RuntimeRegistrationProof,
            handoff.runtime_registration_proof().proof_identity(),
        )
        .is_some());
    assert!(proof
        .row(
            branch,
            Kind::DecisionLog,
            products.decision_log().decision_log_identity()
        )
        .is_some());
    assert!(proof
        .row(
            branch,
            Kind::LoopLedger,
            products.loop_ledger().ledger_identity()
        )
        .is_some());
    assert!(proof
        .row(
            branch,
            Kind::WalkOutcomeSet,
            products.walk_outcomes().walk_outcome_set_identity()
        )
        .is_some());
    assert!(proof
        .row(
            branch,
            Kind::AdmittedLoopCandidateSet,
            products
                .candidate_boundary()
                .loop_candidates()
                .loop_candidate_set_identity(),
        )
        .is_some());
    assert!(proof
        .row(
            branch,
            Kind::DeniedLoopCandidateSet,
            products
                .candidate_boundary()
                .denied_loop_candidates()
                .denied_loop_candidate_set_identity(),
        )
        .is_some());
    assert!(proof
        .row(
            branch,
            Kind::ReconstructedLoopSet,
            products
                .reconstructed_boundary()
                .reconstructed_loops()
                .reconstructed_loop_set_identity(),
        )
        .is_some());
    assert!(proof
        .row(
            branch,
            Kind::BornLoopSet,
            products
                .reconstructed_boundary()
                .born_loops()
                .born_loop_set_identity(),
        )
        .is_some());

    for row in products.walk_outcomes().rows() {
        assert!(proof
            .row_with_trace(
                branch,
                Kind::WalkOutcomeRow,
                row.walk_outcome_identity(),
                row.source_loop_identity(),
            )
            .is_some());
    }
    for row in products.candidate_boundary().loop_candidates().rows() {
        assert!(proof
            .row_with_trace(
                branch,
                Kind::AdmittedLoopCandidateRow,
                row.loop_candidate_identity(),
                row.walk_outcome_identity(),
            )
            .is_some());
    }
    for row in products
        .candidate_boundary()
        .denied_loop_candidates()
        .rows()
    {
        assert!(proof
            .row_with_trace(
                branch,
                Kind::DeniedLoopCandidateRow,
                row.denied_loop_candidate_identity(),
                row.walk_outcome_identity(),
            )
            .is_some());
    }
    for row in products
        .reconstructed_boundary()
        .reconstructed_loops()
        .rows()
    {
        assert!(proof
            .row_with_trace(
                branch,
                Kind::ReconstructedLoopRow,
                row.reconstructed_loop_identity(),
                row.loop_candidate_identity(),
            )
            .is_some());
    }
    for row in products.reconstructed_boundary().born_loops().rows() {
        assert!(proof
            .row_with_trace(
                branch,
                Kind::BornLoopRow,
                row.born_loop_identity(),
                row.loop_candidate_identity(),
            )
            .is_some());
    }
    for row in products.island_partition().rows() {
        assert!(proof
            .row_with_trace(
                branch,
                Kind::IslandPartitionRow,
                row.island_identity(),
                row.source_loop_identity(),
            )
            .is_some());
    }
    for row in products.split_attribution().rows() {
        assert!(proof
            .row_with_trace(
                branch,
                Kind::SplitAttributionRow,
                row.attribution_identity(),
                row.source_loop_identity(),
            )
            .is_some());
    }
    for row in products.role_outcomes().rows() {
        assert!(proof
            .row_with_trace(
                branch,
                Kind::RoleOutcomeRow,
                row.role_outcome_identity(),
                row.loop_identity(),
            )
            .is_some());
    }
    for row in products.containment_postures().rows() {
        assert!(proof
            .row_with_trace(
                branch,
                Kind::ContainmentPostureRow,
                row.containment_posture_identity(),
                row.loop_identity(),
            )
            .is_some());
    }
    for row in products.degenerate_outcomes().rows() {
        assert!(proof
            .row_with_trace(
                branch,
                Kind::DegenerateOutcomeRow,
                row.degenerate_loop_outcome_identity(),
                row.loop_identity(),
            )
            .is_some());
    }
    for row in products.loop_ledger().rows() {
        assert!(proof
            .row_with_trace(
                branch,
                Kind::LedgerRow,
                row.ledger_row_identity(),
                row.canonical_loop_identity(),
            )
            .is_some());
    }
}
