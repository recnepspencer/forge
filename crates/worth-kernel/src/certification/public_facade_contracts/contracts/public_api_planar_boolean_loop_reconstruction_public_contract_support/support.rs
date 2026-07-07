use super::super::workload_evidence_support::{certified_real_loop_handoff, ReplayBranch};
use super::anti_theatre_fence::{
    PlanarBooleanLoopAntiTheatreFenceDenial, PlanarBooleanLoopAntiTheatreFenceProof,
    PlanarBooleanLoopAntiTheatreFenceProofInput,
};
use super::guard_coverage::{
    assert_loop_reconstruction_guard_coverage_contract, loop_reconstruction_guard_names,
};
use super::proof_rows::PlanarBooleanLoopPublicContractProofRowKind;
use super::public_contract_fence::{
    PlanarBooleanLoopPublicContractFenceDenial, PlanarBooleanLoopPublicContractFenceProof,
    PlanarBooleanLoopPublicContractFenceProofInput,
};

pub(crate) fn assert_loop_public_contract_surfaces_preserve_real_workload_backed_identities() {
    assert_loop_reconstruction_guard_coverage_contract();

    let original = certified_real_loop_handoff(
        "phase7.4 public loop contract original",
        ReplayBranch::Original,
    )
    .expect("original loop handoff should certify through the real workload path");
    let replayed = certified_real_loop_handoff(
        "phase7.4 public loop contract original",
        ReplayBranch::Replayed,
    )
    .expect("replayed loop handoff should certify through the same workload path");

    let original_public = certify_public_contract(&original);
    let replayed_public = certify_public_contract(&replayed);
    let original_anti = certify_anti_theatre(&original, &original_public);
    let replayed_anti = certify_anti_theatre(&replayed, &replayed_public);

    assert_eq!(
        original.loop_ledger_receipt().receipt_identity(),
        replayed.loop_ledger_receipt().receipt_identity()
    );
    assert_eq!(
        original.evidence_receipt().receipt_identity(),
        replayed.evidence_receipt().receipt_identity()
    );
    assert_eq!(
        original_public.proof_identity(),
        replayed_public.proof_identity()
    );
    assert_eq!(
        original_public.runtime_registration_proof_identity(),
        original.runtime_registration_proof().proof_identity()
    );
    assert_eq!(
        replayed_public.runtime_registration_proof_identity(),
        replayed.runtime_registration_proof().proof_identity()
    );
    assert_eq!(
        original_anti.proof_identity(),
        replayed_anti.proof_identity()
    );
    assert_eq!(
        original_public.workload_stage_index_identity(),
        replayed_public.workload_stage_index_identity()
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
        original_anti.guard_names(),
        &loop_reconstruction_guard_names()
    );
    assert!(original_public
        .rows()
        .iter()
        .any(|row| row.kind() == PlanarBooleanLoopPublicContractProofRowKind::WorkloadStageIndex));
    assert!(original_public
        .rows()
        .iter()
        .all(|row| !row.identity().is_empty()));
    let anti_theatre_guard_rows = original_anti
        .rows()
        .iter()
        .filter(|row| row.kind() == PlanarBooleanLoopPublicContractProofRowKind::AntiTheatreGuard)
        .count();
    assert_eq!(
        anti_theatre_guard_rows,
        loop_reconstruction_guard_names().len()
    );
    assert!(original_anti
        .rows()
        .iter()
        .any(|row| row.kind() == PlanarBooleanLoopPublicContractProofRowKind::AntiTheatreFence));
    assert!(original_anti.rows().iter().all(|row| !row.identity().is_empty()));
}

pub(crate) fn assert_loop_public_contract_fences_reject_foreign_authority() {
    let canonical = certified_real_loop_handoff(
        "phase7.4 public loop contract canonical",
        ReplayBranch::Original,
    )
    .expect("canonical loop handoff should certify");
    let foreign = certified_real_loop_handoff(
        "phase7.4 public loop contract foreign",
        ReplayBranch::Original,
    )
    .expect("foreign loop handoff should also certify");

    let mixed_public = PlanarBooleanLoopPublicContractFenceProof::certify(
        PlanarBooleanLoopPublicContractFenceProofInput::from_parts(
            canonical.loop_ledger_receipt(),
            foreign.evidence_receipt(),
            canonical.runtime_registration_proof(),
            canonical.workload_stage_index_identity(),
        ),
    )
    .expect_err("foreign evidence must deny public contract certification");
    assert_eq!(
        mixed_public,
        PlanarBooleanLoopPublicContractFenceDenial::LoopEvidenceMismatch
    );

    let canonical_public = certify_public_contract(&canonical);
    let foreign_public = certify_public_contract(&foreign);

    let mixed_anti = PlanarBooleanLoopAntiTheatreFenceProof::certify(
        PlanarBooleanLoopAntiTheatreFenceProofInput::from_parts(
            canonical.loop_ledger_receipt(),
            canonical.evidence_receipt(),
            &foreign_public,
        ),
    )
    .expect_err("foreign public fence must deny anti-theatre certification");
    assert_eq!(
        mixed_anti,
        PlanarBooleanLoopAntiTheatreFenceDenial::PublicContractFenceMismatch
    );

    assert_eq!(
        canonical_public.downstream_consumption_identity(),
        canonical
            .loop_ledger_receipt()
            .downstream_consumption_identity()
    );
}

fn certify_public_contract(
    handoff: &worth_kernel::workload_composition::CompletedBooleanLoopReconstructionHandoff,
) -> PlanarBooleanLoopPublicContractFenceProof {
    PlanarBooleanLoopPublicContractFenceProof::certify(
        PlanarBooleanLoopPublicContractFenceProofInput::from_handoff(handoff),
    )
    .expect("real loop handoff should certify the public contract fence")
}

fn certify_anti_theatre(
    handoff: &worth_kernel::workload_composition::CompletedBooleanLoopReconstructionHandoff,
    public_contract: &PlanarBooleanLoopPublicContractFenceProof,
) -> PlanarBooleanLoopAntiTheatreFenceProof {
    PlanarBooleanLoopAntiTheatreFenceProof::certify(
        PlanarBooleanLoopAntiTheatreFenceProofInput::from_parts(
            handoff.loop_ledger_receipt(),
            handoff.evidence_receipt(),
            public_contract,
        ),
    )
    .expect("real loop handoff should certify the anti-theatre fence")
}
