#[path = "s4_closeout/fixture.rs"]
mod closeout_fixture;
#[path = "s6_reclaim_policy/support.rs"]
mod reclaim_support;
#[allow(dead_code)]
#[path = "s5_epoch_scope_and_root_kind/support.rs"]
mod support;

use forge_store_physical_format::{PhysicalReclaimRegion, ReclaimedByteInterpretation};
use forge_store_physical_isolation::{
    HazardLeaseTable, HazardLeaseTableCapacity, ReclaimDenial, ReclaimEligibilityProof,
};
use forge_store_reclaim_policy::{
    ReclaimLaterHandoffPolicy, ReclaimPermit, ReclaimPolicyAdmission, ReclaimPolicyDenialKind,
    ReclaimPolicyExecutionObservation, ReclaimPolicyProofAuthority,
    ReclaimPolicyReachabilityDenial, ReclaimPolicyRequest, ReclaimPolicySecurityScope,
    ReclaimPolicyViolationKind,
};
use forge_store_tiering::ColdTierIoPosture;

use reclaim_support::{
    admitted_backend, admitted_policy_for_region, backend_without_reclaim_posture,
    base_real_chain_request, execute_policy_with_observation, internal_security_scope,
    reachability_from_s5_removal, real_reachability_for_region, region_for_generation,
    S6ReclaimFixture,
};

#[test]
fn reclaim_policy_consumes_real_s5_reachability_removal() {
    let generation = 333;
    let region = region_for_generation(generation);
    let reachability = real_reachability_for_region(generation, region);
    let backend = admitted_backend();
    let authority = ReclaimPolicyProofAuthority::for_admitted_backend(&backend);
    let admitted = ReclaimPolicyAdmission::admit(
        authority,
        ReclaimPolicyRequest::new()
            .for_region(region)
            .with_posture(
                authority
                    .cold_tier_io_posture(
                        ReclaimedByteInterpretation::NonObservableReclaimedStorage,
                    )
                    .unwrap(),
            )
            .with_reachability(reachability)
            .with_security_scope(internal_security_scope())
            .with_reclaim_permit(ReclaimPermit::new(1).unwrap())
            .with_later_handoff_policy(authority.non_claim_later_handoff()),
    )
    .unwrap();
    let receipt = execute_policy_with_observation(
        admitted,
        ReclaimPolicyExecutionObservation::new(
            region,
            ReclaimedByteInterpretation::NonObservableReclaimedStorage,
            internal_security_scope(),
            true,
        ),
    )
    .unwrap()
    .clone();

    assert_eq!(
        receipt.observed_interpretation(),
        ReclaimedByteInterpretation::NonObservableReclaimedStorage
    );
    assert_eq!(
        ColdTierIoPosture::from_reclaim_receipt(receipt)
            .unwrap()
            .reclaim_receipt()
            .policy()
            .posture()
            .operation(),
        forge_store_reclaim_policy::ReclaimPolicyOperation::ColdTierMovementPosture,
    );
}

#[test]
fn reclaim_policy_executes_distinct_byte_interpretations_through_real_s5_removal() {
    let cases = [
        ReclaimedByteInterpretation::PhysicalZeros,
        ReclaimedByteInterpretation::LogicalHole,
        ReclaimedByteInterpretation::UnavailableBytes,
        ReclaimedByteInterpretation::NonObservableReclaimedStorage,
    ];

    for (index, interpretation) in cases.into_iter().enumerate() {
        let generation = 430 + index as u64;
        let region = region_for_generation(generation);
        let policy = admitted_policy_for_region(region, interpretation);
        let receipt = execute_policy_with_observation(
            policy,
            ReclaimPolicyExecutionObservation::new(
                region,
                interpretation,
                internal_security_scope(),
                true,
            ),
        )
        .unwrap();

        assert_eq!(receipt.observed_interpretation(), interpretation);
        assert_eq!(receipt.policy().posture().interpretation(), interpretation);
        assert_eq!(receipt.counters().executed(), 1);
    }
}

#[test]
fn reclaim_policy_denies_region_not_covered_by_s5_reachability_removal() {
    let world = S6ReclaimFixture::new(337);
    let proof = ReclaimEligibilityProof::admit(
        world.executed_reachability(),
        HazardLeaseTable::with_capacity(HazardLeaseTableCapacity::bounded_slots(1).unwrap())
            .live_index_snapshot(),
    )
    .unwrap();
    let removal = proof.admit_reachability_removal().unwrap();
    let wrong_region = region_for_generation(338);

    assert_eq!(
        reachability_from_s5_removal(
            removal
                .lower_for_s6_reclaim_policy(region_for_generation(337))
                .unwrap(),
            wrong_region,
        )
        .unwrap_err(),
        ReclaimPolicyReachabilityDenial::RegionNotCoveredByReachabilityRemoval
    );
}

#[test]
fn reclaim_policy_denies_same_owner_region_reuse() {
    let region = region_for_generation(339);
    let different_region = PhysicalReclaimRegion::new(region.reference(), 8192).unwrap();
    let world = S6ReclaimFixture::new(339);
    let proof = ReclaimEligibilityProof::admit(
        world.executed_reachability(),
        HazardLeaseTable::with_capacity(HazardLeaseTableCapacity::bounded_slots(1).unwrap())
            .live_index_snapshot(),
    )
    .unwrap();
    let removal = proof.admit_reachability_removal().unwrap();

    assert_eq!(
        reachability_from_s5_removal(
            removal.lower_for_s6_reclaim_policy(region).unwrap(),
            different_region,
        )
        .unwrap_err(),
        ReclaimPolicyReachabilityDenial::RegionNotCoveredByReachabilityRemoval
    );
}

#[test]
fn reclaim_policy_denies_live_hazard_before_s6_admission_exists() {
    let world = S6ReclaimFixture::new(441);
    let proof =
        ReclaimEligibilityProof::admit(world.executed_reachability(), world.live_hazard_snapshot())
            .unwrap();

    assert!(matches!(
        proof.admit_reachability_removal().unwrap_err(),
        ReclaimDenial::BlockedByLiveHazardLease { .. }
    ));
}

#[test]
fn reclaim_policy_denies_real_chain_unsupported_backend_and_missing_security_scope() {
    let region = region_for_generation(442);
    let reachability = real_reachability_for_region(442, region);
    let supported = admitted_backend();
    let unsupported = backend_without_reclaim_posture();
    let supported_authority = ReclaimPolicyProofAuthority::for_admitted_backend(&supported);
    let unsupported_authority = ReclaimPolicyProofAuthority::for_admitted_backend(&unsupported);
    let unsupported_request = ReclaimPolicyRequest::new()
        .for_region(region)
        .with_posture(
            supported_authority
                .punch_hole_posture(ReclaimedByteInterpretation::LogicalHole)
                .unwrap(),
        )
        .with_reachability(reachability.clone())
        .with_security_scope(internal_security_scope())
        .with_reclaim_permit(ReclaimPermit::new(1).unwrap())
        .with_later_handoff_policy(unsupported_authority.non_claim_later_handoff());

    assert_eq!(
        ReclaimPolicyAdmission::admit(unsupported_authority, unsupported_request)
            .unwrap_err()
            .kind(),
        &ReclaimPolicyDenialKind::UnsupportedBackendPosture
    );

    let missing_security = base_real_chain_request(region, reachability)
        .with_posture(
            supported_authority
                .punch_hole_posture(ReclaimedByteInterpretation::LogicalHole)
                .unwrap(),
        )
        .with_reclaim_permit(ReclaimPermit::new(1).unwrap())
        .with_later_handoff_policy(supported_authority.non_claim_later_handoff());

    assert_eq!(
        ReclaimPolicyAdmission::admit(supported_authority, missing_security)
            .unwrap_err()
            .kind(),
        &ReclaimPolicyDenialKind::MissingSecurityScope
    );
}

#[test]
fn reclaim_policy_reports_real_chain_execution_scope_and_handoff_violations() {
    let region = region_for_generation(443);
    let wrong_region = region_for_generation(444);
    let wrong_scope = ReclaimPolicySecurityScope::from_admitted_scope(
        &forge_store_security::admitted_wrong_s6_io_qos_security_scope_for_test(),
    );

    let wrong_region_violation = execute_policy_with_observation(
        admitted_policy_for_region(region, ReclaimedByteInterpretation::LogicalHole),
        ReclaimPolicyExecutionObservation::new(
            wrong_region,
            ReclaimedByteInterpretation::LogicalHole,
            internal_security_scope(),
            true,
        ),
    )
    .unwrap_err();
    assert_eq!(
        wrong_region_violation.kind(),
        ReclaimPolicyViolationKind::ProtectedReachabilityLost
    );

    let wrong_scope_violation = execute_policy_with_observation(
        admitted_policy_for_region(region, ReclaimedByteInterpretation::LogicalHole),
        ReclaimPolicyExecutionObservation::new(
            region,
            ReclaimedByteInterpretation::LogicalHole,
            wrong_scope,
            true,
        ),
    )
    .unwrap_err();
    assert_eq!(
        wrong_scope_violation.kind(),
        ReclaimPolicyViolationKind::SecurityScopeLost
    );

    let strengthened_handoff = execute_policy_with_observation(
        admitted_policy_for_region(region, ReclaimedByteInterpretation::LogicalHole),
        ReclaimPolicyExecutionObservation::new(
            region,
            ReclaimedByteInterpretation::LogicalHole,
            internal_security_scope(),
            false,
        ),
    )
    .unwrap_err();
    assert_eq!(
        strengthened_handoff.kind(),
        ReclaimPolicyViolationKind::LaterHandoffStrengthened
    );
}

#[test]
fn reclaim_policy_denies_real_chain_later_lifecycle_claims() {
    let region = region_for_generation(445);
    let backend = admitted_backend();
    let authority = ReclaimPolicyProofAuthority::for_admitted_backend(&backend);
    let request = base_real_chain_request(region, real_reachability_for_region(445, region))
        .with_posture(
            authority
                .cold_tier_io_posture(ReclaimedByteInterpretation::NonObservableReclaimedStorage)
                .unwrap(),
        )
        .with_security_scope(internal_security_scope())
        .with_reclaim_permit(ReclaimPermit::new(1).unwrap())
        .with_later_handoff_policy(ReclaimLaterHandoffPolicy::claims_later_lifecycle_for_denial());

    assert_eq!(
        ReclaimPolicyAdmission::admit(authority, request)
            .unwrap_err()
            .kind(),
        &ReclaimPolicyDenialKind::LaterLifecycleClaimAttempted
    );
}
