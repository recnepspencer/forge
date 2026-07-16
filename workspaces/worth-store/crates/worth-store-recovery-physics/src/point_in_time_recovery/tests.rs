use worth_store_authority::StoreCurrentAuthorityIdentity;

use super::{
    FrontierPartialOrder, PitrCandidatePosture, PitrCandidateSelectionDenial, PitrRoundingPolicy,
    RecoveryPhysicsTimelineAuthority,
};

#[test]
fn candidate_selection_converges_across_observation_order() {
    let earlier = observation(90, 9, 9, 9, 8, 7);
    let exact = observation(100, 10, 10, 10, 9, 8);
    let left = RecoveryPhysicsTimelineAuthority::resolve_candidates(
        100,
        PitrRoundingPolicy::NearestAcknowledged,
        vec![earlier, exact],
    )
    .expect("candidate set")
    .select()
    .expect("selected candidate");
    let right = RecoveryPhysicsTimelineAuthority::resolve_candidates(
        100,
        PitrRoundingPolicy::NearestAcknowledged,
        vec![exact, earlier],
    )
    .expect("candidate set")
    .select()
    .expect("selected candidate");
    assert_eq!(left, right);
    assert_eq!(left.exact_frontier().wal_structural(), 10);
}

#[test]
fn exact_policy_refuses_to_round_across_an_evidence_gap() {
    let candidates = RecoveryPhysicsTimelineAuthority::resolve_candidates(
        100,
        PitrRoundingPolicy::ExactOnly,
        vec![observation(90, 9, 9, 9, 8, 7)],
    )
    .expect("candidate set");
    assert_eq!(
        candidates.select(),
        Err(PitrCandidateSelectionDenial::NoAdmissibleCandidate)
    );
}

#[test]
fn multiple_previous_candidates_are_canonicalized_by_distance_not_input_time() {
    let selected = RecoveryPhysicsTimelineAuthority::resolve_candidates(
        100,
        PitrRoundingPolicy::PreviousAcknowledged,
        vec![
            observation(70, 7, 7, 7, 7, 7),
            observation(90, 9, 9, 9, 9, 9),
            observation(80, 8, 8, 8, 8, 8),
        ],
    )
    .expect("valid past candidates must form a canonical set")
    .select()
    .expect("nearest previous acknowledgement");
    assert_eq!(selected.observed_time(), 90);
    assert_eq!(selected.clock_distance(), 10);
}

#[test]
fn unbounded_clock_uncertainty_saturates_instead_of_wrapping() {
    let uncertain = RecoveryPhysicsTimelineAuthority::admit_observation(
        0,
        u64::MAX,
        u64::MAX,
        1,
        1,
        1,
        1,
        1,
        StoreCurrentAuthorityIdentity::from_persisted_fingerprint([4; 32]),
        [5; 32],
        [6; 32],
        PitrCandidatePosture::Available,
    )
    .unwrap();
    let selected = RecoveryPhysicsTimelineAuthority::resolve_candidates(
        i64::MIN,
        PitrRoundingPolicy::ExactOnly,
        vec![uncertain],
    )
    .unwrap()
    .select()
    .expect("saturated uncertainty interval includes the minimum instant");
    assert_eq!(selected.clock_distance(), 0);
}

#[test]
fn frontier_comparison_preserves_partial_order_instead_of_inventing_total_order() {
    let left = observation(100, 10, 10, 10, 8, 7).frontier;
    let right = observation(100, 10, 10, 10, 7, 8).frontier;
    assert_eq!(
        left.compare(right),
        FrontierPartialOrder::IncomparableDimensions
    );
}

#[test]
fn authorization_identity_preserves_rounding_and_posture_semantics() {
    let available = observation(100, 10, 10, 10, 8, 7);
    let mut degraded = available;
    degraded.posture = PitrCandidatePosture::Degraded;
    let exact = RecoveryPhysicsTimelineAuthority::resolve_candidates(
        100,
        PitrRoundingPolicy::ExactOnly,
        vec![available],
    )
    .unwrap()
    .select()
    .unwrap();
    let nearest = RecoveryPhysicsTimelineAuthority::resolve_candidates(
        100,
        PitrRoundingPolicy::NearestAcknowledged,
        vec![available],
    )
    .unwrap()
    .select()
    .unwrap();
    let degraded = RecoveryPhysicsTimelineAuthority::resolve_candidates(
        100,
        PitrRoundingPolicy::ExactOnly,
        vec![degraded],
    )
    .unwrap()
    .select()
    .unwrap();
    assert_ne!(exact.identity(), nearest.identity());
    assert_ne!(exact.identity(), degraded.identity());
    assert_eq!(exact.exact_frontier(), nearest.exact_frontier());
}

fn observation(
    time: i64,
    checkpoint: u64,
    wal: u64,
    local: u64,
    client: u64,
    replication: u64,
) -> super::RecoveryTimelineObservation {
    RecoveryPhysicsTimelineAuthority::admit_observation(
        time,
        0,
        0,
        checkpoint,
        wal,
        local,
        client,
        replication,
        StoreCurrentAuthorityIdentity::from_persisted_fingerprint([4; 32]),
        [5; 32],
        [6; 32],
        PitrCandidatePosture::Available,
    )
    .expect("valid observation")
}
