use worth_math::sign::TriSign;
use worth_spatial::facade::planar_predicates::{
    planar_predicate_authority_entry, planar_predicate_authority_facts,
    PlanarPredicateAuthorityCase, PlanarPredicateAuthorityFactError,
    PlanarPredicateCoincidencePolicy, PlanarPredicateEvaluationFailureKind,
};

use super::proof_fixture::{
    admitted_handle, orient_basis, orient_basis_with_identities, receipt_for,
};

#[test]
fn exact_planar_predicate_authority_converges_across_equivalent_authoring_order() {
    let handle = admitted_handle("cyclic-order");
    let abc = receipt_for(
        &handle,
        orient_basis("movement:identity", [[0.0, 0.0], [3.0, 0.0], [1.0, 2.0]]),
    );
    let bca = receipt_for(
        &handle,
        orient_basis("movement:identity", [[3.0, 0.0], [1.0, 2.0], [0.0, 0.0]]),
    );
    let cab = receipt_for(
        &handle,
        orient_basis("movement:identity", [[1.0, 2.0], [0.0, 0.0], [3.0, 0.0]]),
    );

    assert_eq!(abc.certified_sign().sign(), TriSign::Pos);
    assert_eq!(abc.certified_sign(), bca.certified_sign());
    assert_eq!(abc.certified_sign(), cab.certified_sign());
    assert_eq!(abc.declaration_digest(), bca.declaration_digest());
    assert_eq!(abc.declaration_digest(), cab.declaration_digest());
    assert_eq!(abc.fact_digest(), bca.fact_digest());
    assert_eq!(abc.fact_digest(), cab.fact_digest());
    assert_eq!(abc.counters().predicate_evaluations(), 1);
    assert_eq!(abc.counters().input_point_count(), 3);
    assert_eq!(abc.counters().canonical_basis_part_count(), 12);
}

#[test]
fn exact_planar_predicate_authority_separates_odd_authoring_permutation_identity() {
    let handle = admitted_handle("odd-permutation");
    let positive = receipt_for(
        &handle,
        orient_basis("movement:identity", [[0.0, 0.0], [8.0, 0.0], [0.0, 5.0]]),
    );
    let reversed = receipt_for(
        &handle,
        orient_basis("movement:identity", [[0.0, 0.0], [0.0, 5.0], [8.0, 0.0]]),
    );

    assert_eq!(positive.certified_sign().sign(), TriSign::Pos);
    assert_eq!(reversed.certified_sign().sign(), TriSign::Neg);
    assert_ne!(positive.declaration_digest(), reversed.declaration_digest());
    assert_ne!(positive.fact_digest(), reversed.fact_digest());
}

#[test]
fn exact_planar_predicate_authority_commits_all_spatial_basis_identities() {
    let handle = admitted_handle("basis-identity");
    let baseline = receipt_for(
        &handle,
        orient_basis_with_identities(
            "frame:xy",
            "topology:face-loop",
            "movement:identity",
            "tolerance:exact",
            [[0.0, 0.0], [3.0, 0.0], [1.0, 2.0]],
        ),
    );
    let changed_topology = receipt_for(
        &handle,
        orient_basis_with_identities(
            "frame:xy",
            "topology:replacement-loop",
            "movement:identity",
            "tolerance:exact",
            [[0.0, 0.0], [3.0, 0.0], [1.0, 2.0]],
        ),
    );
    let changed_tolerance = receipt_for(
        &handle,
        orient_basis_with_identities(
            "frame:xy",
            "topology:face-loop",
            "movement:identity",
            "tolerance:micro-feature",
            [[0.0, 0.0], [3.0, 0.0], [1.0, 2.0]],
        ),
    );
    let changed_frame = receipt_for(
        &handle,
        orient_basis_with_identities(
            "frame:translated-local",
            "topology:face-loop",
            "movement:identity",
            "tolerance:exact",
            [[0.0, 0.0], [3.0, 0.0], [1.0, 2.0]],
        ),
    );

    assert_eq!(baseline.certified_sign(), changed_topology.certified_sign());
    assert_eq!(
        baseline.certified_sign(),
        changed_tolerance.certified_sign()
    );
    assert_eq!(baseline.certified_sign(), changed_frame.certified_sign());
    assert_ne!(
        baseline.declaration_digest(),
        changed_topology.declaration_digest()
    );
    assert_ne!(baseline.fact_digest(), changed_topology.fact_digest());
    assert_ne!(
        baseline.declaration_digest(),
        changed_tolerance.declaration_digest()
    );
    assert_ne!(baseline.fact_digest(), changed_tolerance.fact_digest());
    assert_ne!(
        baseline.declaration_digest(),
        changed_frame.declaration_digest()
    );
    assert_ne!(baseline.fact_digest(), changed_frame.fact_digest());
}

#[test]
fn exact_planar_predicate_authority_denies_near_graze_before_snap_or_repair() {
    let handle = admitted_handle("zero-denial");
    let basis = orient_basis("movement:identity", [[0.0, 0.0], [1.0, 1.0], [2.0, 2.0]])
        .with_coincidence_policy(PlanarPredicateCoincidencePolicy::DenyCertifiedZeroBeforeRepair);
    let entry = planar_predicate_authority_entry(PlanarPredicateAuthorityCase::orient2d(basis));

    let error = planar_predicate_authority_facts(&entry, &handle)
        .expect_err("certified zero must require snap or repair");

    assert!(matches!(
        error,
        PlanarPredicateAuthorityFactError::PredicateUncertain {
            certified_sign,
            counters,
            ..
        } if certified_sign.sign() == TriSign::Zero
            && counters.predicate_evaluations() == 1
            && counters.canonical_basis_part_count() == 12
    ));
}

#[test]
fn exact_planar_predicate_authority_rejects_non_finite_projected_points() {
    let handle = admitted_handle("non-finite-denial");
    let basis = orient_basis(
        "movement:identity",
        [[0.0, 0.0], [f64::NAN, 1.0], [2.0, 2.0]],
    );
    let entry = planar_predicate_authority_entry(PlanarPredicateAuthorityCase::orient2d(basis));

    let error = planar_predicate_authority_facts(&entry, &handle)
        .expect_err("non-finite projected points must not certify");

    assert!(matches!(
        error,
        PlanarPredicateAuthorityFactError::PredicateEvaluation { kind, reason }
            if kind == PlanarPredicateEvaluationFailureKind::NonFiniteProjectedPoint2
                && reason.contains("finite_point2")
    ));
}

#[test]
fn exact_planar_predicate_authority_normalizes_signed_zero_authoring_drift() {
    let handle = admitted_handle("signed-zero");
    let positive_zero = receipt_for(
        &handle,
        orient_basis("movement:identity", [[0.0, 0.0], [3.0, 0.0], [1.0, 2.0]]),
    );
    let negative_zero = receipt_for(
        &handle,
        orient_basis("movement:identity", [[-0.0, -0.0], [3.0, -0.0], [1.0, 2.0]]),
    );

    assert_eq!(
        positive_zero.certified_sign(),
        negative_zero.certified_sign()
    );
    assert_eq!(
        positive_zero.declaration_digest(),
        negative_zero.declaration_digest()
    );
    assert_eq!(positive_zero.fact_digest(), negative_zero.fact_digest());
}

#[test]
fn mb_m6_1_coplanar_overlap_contract_storm_predicate_rows() {
    let handle = admitted_handle("contract-storm");
    let identity = receipt_for(
        &handle,
        orient_basis("movement:identity", [[0.0, 0.0], [8.0, 0.0], [0.0, 5.0]]),
    );
    let rotated = receipt_for(
        &handle,
        orient_basis("movement:rot90", [[0.0, 0.0], [8.0, 0.0], [0.0, 5.0]]),
    );
    let translated = receipt_for(
        &handle,
        orient_basis(
            "movement:translation",
            [[4.0, 9.0], [12.0, 9.0], [4.0, 14.0]],
        ),
    );

    assert_eq!(identity.certified_sign().sign(), TriSign::Pos);
    assert_eq!(translated.certified_sign(), identity.certified_sign());
    assert_ne!(
        identity.fact_digest(),
        rotated.fact_digest(),
        "movement/rotation posture must participate in the authority fact"
    );
    assert_ne!(
        identity.declaration_digest(),
        rotated.declaration_digest(),
        "movement/rotation posture must participate in Query declaration identity"
    );
    assert_ne!(
        identity.fact_digest(),
        translated.fact_digest(),
        "translation-equivalent coordinates are not the same retained basis"
    );
}
