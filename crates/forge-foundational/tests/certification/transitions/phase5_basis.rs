use forge_foundational::{
    admit_current_basis_commit_receipt, admit_current_basis_committed_authority,
    attach_proof_bearing_profiled_commit_receipt, compare_canonical_basis,
    foundational_transition_current_basis_authority,
    foundational_transition_current_basis_readmission_authority,
    plan_foundational_profile_materialization_with_elision,
    prepare_branch_candidate_for_canonical_basis, prepare_canonical_comparison,
    prepare_commit_receipt_for_canonical_basis, prepare_committed_authority_for_canonical_basis,
    prepare_locator_for_canonical_basis, prepare_merge_verdict_for_canonical_basis,
    readmit_current_basis_commit_receipt_after_boundary,
    readmit_current_basis_committed_authority_after_boundary, request_foundational_profile_set,
    AdmissionReadinessProfile, CanonicalComparisonOutcome, CanonicalEquivalenceBasis,
    CanonicalLocatorInput, CanonicalizationRuleVersion, CertificationPostureProfile,
    CompatibilityPostureProfile, DiagnosticRichnessProfile, FoundationalBranchCandidateLocator,
    FoundationalCommitParentageLocator, FoundationalCommittedDeltaLocator,
    FoundationalMergeConflictLocator, FoundationalProfileSet, FoundationalProfileSetInput,
    FoundationalTransitionLocator, ProofBearingArtifactTarget, RetentionDeliveryProfile,
    SupportPostureProfile,
};
use forge_foundational::{
    CanonicalBasisDomain, CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus,
    CanonicalBasisValue, CanonicalIntegerWidth,
};
use forge_proof::TransitionOutcome;

use super::fixtures::branch::{authority_first_candidate, projection_shaped_candidate};
use super::fixtures::committed::{accepted_verdict, committed_authority, ordinary_commit_input};
use super::fixtures::merge::{
    authority_first_merge_candidate, conflict_locus, projection_shaped_merge_candidate,
};
use super::fixtures::receipt::{commit_id, receipt_authority, receipt_identity};

#[test]
fn transition_surfaces_canonicalize_the_same_across_independent_producers() {
    let left_candidate = ready_candidate(authority_first_candidate("mesh-update"));
    let right_candidate = ready_candidate(projection_shaped_candidate("mesh-update"));
    assert_equivalent(left_candidate, right_candidate);

    let left_verdict = ready_verdict(
        match authority_first_merge_candidate("mesh-update").admit_as_accepted() {
            TransitionOutcome::Success(verdict) => verdict,
            _ => panic!("expected accepted verdict"),
        },
    );
    let right_verdict = ready_verdict(
        match projection_shaped_merge_candidate("mesh-update").admit_as_accepted() {
            TransitionOutcome::Success(verdict) => verdict,
            _ => panic!("expected accepted verdict"),
        },
    );
    assert_equivalent(left_verdict, right_verdict);

    let left_committed = ready_committed(
        accepted_verdict("mesh-update")
            .commit_with(ordinary_commit_input(), committed_authority())
            .expect("committed authority"),
    );
    let right_committed = ready_committed(
        match projection_shaped_merge_candidate("mesh-update").admit_as_accepted() {
            TransitionOutcome::Success(verdict) => verdict,
            _ => panic!("expected accepted verdict"),
        }
        .commit_with(ordinary_commit_input(), committed_authority())
        .expect("committed authority"),
    );
    assert_equivalent(left_committed, right_committed);

    let left_receipt = ready_receipt(
        accepted_verdict("mesh-update")
            .commit_with(ordinary_commit_input(), committed_authority())
            .expect("committed authority")
            .issue_receipt(receipt_identity(90), commit_id(80), receipt_authority())
            .expect("receipt"),
    );
    let right_receipt = ready_receipt(
        match projection_shaped_merge_candidate("mesh-update").admit_as_accepted() {
            TransitionOutcome::Success(verdict) => verdict,
            _ => panic!("expected accepted verdict"),
        }
        .commit_with(ordinary_commit_input(), committed_authority())
        .expect("committed authority")
        .issue_receipt(receipt_identity(90), commit_id(80), receipt_authority())
        .expect("receipt"),
    );
    assert_equivalent(left_receipt, right_receipt);
}

#[test]
fn transition_locators_point_at_exact_branch_conflict_parentage_and_delta_loci() {
    let branch_candidate = authority_first_candidate("mesh-update");
    let branch_entries = locator_entries(FoundationalTransitionLocator::BranchCandidate(
        FoundationalBranchCandidateLocator::new(
            branch_candidate.branch_id().clone(),
            branch_candidate.candidate_id(),
        ),
    ));
    assert_eq!(
        branch_entries,
        vec![
            transition_locator_text_entry(
                "transition.branch_candidate.branch_id",
                "feature/geometry"
            ),
            transition_locator_integer_entry("transition.branch_candidate.candidate_id", 17),
            transition_locator_text_entry("transition.branch_candidate.kind", "branch-candidate"),
        ]
    );

    let merge_candidate = authority_first_merge_candidate("mesh-update");
    let conflict_entries = locator_entries(FoundationalTransitionLocator::MergeConflict(
        FoundationalMergeConflictLocator::new(
            merge_candidate.source_branch().clone(),
            merge_candidate.target_branch().clone(),
            conflict_locus(),
        ),
    ));
    assert_eq!(
        conflict_entries,
        vec![
            transition_locator_text_entry("transition.merge_conflict.category", "geometry-face"),
            transition_locator_text_entry("transition.merge_conflict.kind", "merge-conflict"),
            transition_locator_text_entry(
                "transition.merge_conflict.source_branch",
                "feature/geometry"
            ),
            transition_locator_text_entry(
                "transition.merge_conflict.source_detail",
                "source:face-7"
            ),
            transition_locator_text_entry("transition.merge_conflict.target_branch", "main"),
            transition_locator_text_entry(
                "transition.merge_conflict.target_detail",
                "target:face-7"
            ),
        ]
    );

    let committed = accepted_verdict("mesh-update")
        .commit_with(ordinary_commit_input(), committed_authority())
        .expect("committed authority");
    let parentage_entries = locator_entries(FoundationalTransitionLocator::CommitParentage(
        FoundationalCommitParentageLocator::new(commit_id(81), committed.parent_basis()),
    ));
    assert_eq!(
        parentage_entries,
        vec![
            transition_locator_integer_entry("transition.parentage.commit_id", 81),
            transition_locator_text_entry("transition.parentage.kind", "commit-parentage"),
            transition_locator_integer_entry("transition.parentage.parent_basis", 401),
        ]
    );

    let delta_entries = locator_entries(FoundationalTransitionLocator::CommittedDelta(
        FoundationalCommittedDeltaLocator::new(
            commit_id(81),
            committed.committed_delta_summary().loci()[0].clone(),
        ),
    ));
    assert_eq!(
        delta_entries,
        vec![
            transition_locator_text_entry("transition.delta.category", "geometry-face"),
            transition_locator_integer_entry("transition.delta.commit_id", 81),
            transition_locator_text_entry("transition.delta.detail", "face-7 updated"),
            transition_locator_text_entry("transition.delta.kind", "committed-delta"),
        ]
    );
}

#[test]
fn current_basis_transition_lane_reuses_real_basis_preparation_and_explicit_readmission() {
    let committed = accepted_verdict("mesh-update")
        .commit_with(ordinary_commit_input(), committed_authority())
        .expect("committed authority");
    let expected_committed_basis = ready_committed(
        accepted_verdict("mesh-update")
            .commit_with(ordinary_commit_input(), committed_authority())
            .expect("committed authority"),
    );
    let strengthened_committed = match admit_current_basis_committed_authority(
        version(),
        committed,
        foundational_transition_current_basis_authority(),
    ) {
        TransitionOutcome::Success(artifact) => artifact,
        _ => panic!("expected current-basis committed artifact"),
    };
    assert_eq!(
        strengthened_committed.strong_basis().payload().entries(),
        expected_committed_basis.payload().entries()
    );

    let readmitted_committed = readmit_current_basis_committed_authority_after_boundary(
        forge_foundational::bridge_current_basis_committed_authority_trust_boundary(
            strengthened_committed,
        ),
        expected_committed_basis,
        foundational_transition_current_basis_readmission_authority(),
    );
    assert_eq!(
        readmitted_committed.committed().transition_class(),
        forge_foundational::FoundationalAuthorityTransitionClass::Commit
    );

    let receipt = accepted_verdict("mesh-update")
        .commit_with(ordinary_commit_input(), committed_authority())
        .expect("committed authority")
        .issue_receipt(receipt_identity(55), commit_id(45), receipt_authority())
        .expect("receipt");
    let expected_receipt_basis = ready_receipt(
        accepted_verdict("mesh-update")
            .commit_with(ordinary_commit_input(), committed_authority())
            .expect("committed authority")
            .issue_receipt(receipt_identity(55), commit_id(45), receipt_authority())
            .expect("receipt"),
    );
    let strengthened_receipt = match admit_current_basis_commit_receipt(
        version(),
        receipt,
        foundational_transition_current_basis_authority(),
    ) {
        TransitionOutcome::Success(artifact) => artifact,
        _ => panic!("expected current-basis receipt artifact"),
    };
    let readmitted_receipt = readmit_current_basis_commit_receipt_after_boundary(
        forge_foundational::bridge_current_basis_commit_receipt_trust_boundary(
            strengthened_receipt,
        ),
        expected_receipt_basis,
        foundational_transition_current_basis_readmission_authority(),
    );
    assert_eq!(readmitted_receipt.receipt().commit_id(), commit_id(45));
}

#[test]
fn profile_attachment_and_reduced_richness_do_not_weaken_receipt_evidence_floor() {
    let profile = profile();
    let requested = request_foundational_profile_set(profile);
    let admitted = match forge_foundational::admit_requested_foundational_profile(
        requested,
        profile,
        None,
        forge_foundational::foundational_profile_progression_authority(),
    ) {
        TransitionOutcome::Success(admitted) => admitted,
        _ => panic!("expected admitted profile"),
    };
    let receipt = accepted_verdict("mesh-update")
        .commit_with(ordinary_commit_input(), committed_authority())
        .expect("committed authority")
        .issue_receipt(receipt_identity(77), commit_id(66), receipt_authority())
        .expect("receipt");
    let original_basis = ready_receipt(
        accepted_verdict("mesh-update")
            .commit_with(ordinary_commit_input(), committed_authority())
            .expect("committed authority")
            .issue_receipt(receipt_identity(77), commit_id(66), receipt_authority())
            .expect("receipt"),
    );

    let profiled = match attach_proof_bearing_profiled_commit_receipt(
        admitted,
        profile,
        None,
        receipt,
        forge_foundational::foundational_profile_progression_authority(),
    ) {
        TransitionOutcome::Success(profiled) => profiled,
        _ => panic!("expected profiled receipt"),
    };
    let attached_basis = ready_receipt_ref(profiled.payload().payload());
    assert_equivalent(original_basis, attached_basis);

    let plan = plan_foundational_profile_materialization_with_elision::<ProofBearingArtifactTarget>(
        profiled.payload().profile(),
        forge_foundational::FoundationalDescriptiveElisionProfile::OperationalSummary,
    );
    assert!(plan
        .decision_for(forge_foundational::FoundationalDescriptiveSurface::Provenance)
        .expect("proof-bearing provenance decision")
        .is_available());
    assert_eq!(profiled.payload().payload().commit_id(), commit_id(66));
    assert_eq!(
        profiled.payload().payload().receipt_identity(),
        receipt_identity(77)
    );
}

fn version() -> CanonicalizationRuleVersion {
    CanonicalizationRuleVersion::new("milestone-5-phase-5").expect("version")
}

fn ready_candidate(
    candidate: forge_foundational::FoundationalBranchCandidateArtifact<&'static str>,
) -> forge_foundational::CanonicalBasisReadyArtifact {
    match prepare_branch_candidate_for_canonical_basis(version(), &candidate) {
        TransitionOutcome::Success(ready) => ready,
        _ => panic!("expected ready candidate basis"),
    }
}

fn ready_verdict(
    verdict: forge_foundational::FoundationalMergeVerdict<&'static str>,
) -> forge_foundational::CanonicalBasisReadyArtifact {
    match prepare_merge_verdict_for_canonical_basis(version(), &verdict) {
        TransitionOutcome::Success(ready) => ready,
        _ => panic!("expected ready verdict basis"),
    }
}

fn ready_committed(
    committed: forge_foundational::FoundationalCommittedAuthorityArtifact<&'static str>,
) -> forge_foundational::CanonicalBasisReadyArtifact {
    match prepare_committed_authority_for_canonical_basis(version(), &committed) {
        TransitionOutcome::Success(ready) => ready,
        _ => panic!("expected ready committed basis"),
    }
}

fn ready_receipt(
    receipt: forge_foundational::FoundationalCommitReceiptArtifact,
) -> forge_foundational::CanonicalBasisReadyArtifact {
    ready_receipt_ref(&receipt)
}

fn ready_receipt_ref(
    receipt: &forge_foundational::FoundationalCommitReceiptArtifact,
) -> forge_foundational::CanonicalBasisReadyArtifact {
    match prepare_commit_receipt_for_canonical_basis(version(), receipt) {
        TransitionOutcome::Success(ready) => ready,
        _ => panic!("expected ready receipt basis"),
    }
}

fn locator_entries(
    locator: FoundationalTransitionLocator,
) -> Vec<forge_foundational::CanonicalBasisEntry> {
    match prepare_locator_for_canonical_basis(version(), CanonicalLocatorInput::Transition(locator))
    {
        TransitionOutcome::Success(ready) => ready.payload().entries().to_vec(),
        _ => panic!("expected ready locator basis"),
    }
}

fn transition_locator_text_entry(locus: &str, value: &str) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Locator,
        CanonicalBasisLocus::Named(locus.into()),
        CanonicalBasisEntryKind::TransitionLocator,
        CanonicalBasisValue::ExactText(value.into()),
    )
}

fn transition_locator_integer_entry(locus: &str, value: u64) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Locator,
        CanonicalBasisLocus::Named(locus.into()),
        CanonicalBasisEntryKind::TransitionLocator,
        CanonicalBasisValue::UnsignedInteger {
            width: CanonicalIntegerWidth::Bits64,
            value: u128::from(value),
        },
    )
}

fn exact_compare(
    left: forge_foundational::CanonicalBasisReadyArtifact,
    right: forge_foundational::CanonicalBasisReadyArtifact,
) -> CanonicalComparisonOutcome {
    let ready = match prepare_canonical_comparison(
        CanonicalEquivalenceBasis::ExactCanonicalBasis,
        left,
        right,
    ) {
        TransitionOutcome::Success(ready) => ready,
        _ => panic!("expected comparison readiness"),
    };
    compare_canonical_basis(&ready)
}

fn assert_equivalent(
    left: forge_foundational::CanonicalBasisReadyArtifact,
    right: forge_foundational::CanonicalBasisReadyArtifact,
) {
    assert!(matches!(
        exact_compare(left, right),
        CanonicalComparisonOutcome::Equivalent(_)
    ));
}

fn profile() -> FoundationalProfileSet {
    FoundationalProfileSet::new(FoundationalProfileSetInput {
        diagnostic_richness: DiagnosticRichnessProfile::Forensic,
        support_posture: SupportPostureProfile::CertificationReady,
        compatibility_posture: CompatibilityPostureProfile::CompatibilityLowered,
        admission_readiness: AdmissionReadinessProfile::ProductionGateReady,
        retention_delivery: RetentionDeliveryProfile::Retained,
        certification_posture: CertificationPostureProfile::ProductionCertified,
    })
    .expect("coherent profile")
}
