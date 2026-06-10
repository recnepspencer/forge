use worth_spatial::facade::planar_overlap::{
    CoplanarOverlapContractExtractor, CoplanarOverlapDenialKind, CoplanarOverlapPolicy,
};

use super::proof_fixture::{
    overlap_contracts, overlap_face, overlap_face_with_containment_candidate, NEIGHBORHOOD,
};

#[test]
fn coplanar_overlap_contract_extractor_emits_stable_islands_intervals_and_containment() {
    let world = "overlap-stable";
    let first = overlap_face_with_containment_candidate(
        world,
        "face:left",
        "movement:stable",
        &[[0.0, 0.0], [2.0e-9, 0.0], [2.0e-9, 2.0e-9], [0.0, 2.0e-9]],
        &[
            [0.5e-9, 0.5e-9],
            [0.75e-9, 0.5e-9],
            [0.75e-9, 0.75e-9],
            [0.5e-9, 0.75e-9],
        ],
    );
    let second = overlap_face(
        world,
        "face:right",
        "movement:stable",
        &[
            [2.0e-9, 0.0],
            [4.0e-9, 0.0],
            [4.0e-9, 2.0e-9],
            [2.0e-9, 2.0e-9],
        ],
    );
    let contracts = overlap_contracts(world);
    let plan = CoplanarOverlapContractExtractor::between(first.clone(), second.clone())
        .within_planar_neighborhood(NEIGHBORHOOD)
        .with_policy(CoplanarOverlapPolicy::ExtractContractsOnly)
        .compile(&contracts)
        .expect("overlap plan");

    assert_eq!(plan.candidate_pair_breadth(), 32);
    assert_eq!(plan.topology_mutations(), 0);
    assert_eq!(plan.boolean_classifications(), 0);

    let receipt = plan.extract().expect("overlap receipt");
    assert_eq!(receipt.boolean_result(), None);
    assert_eq!(receipt.imprint_action(), None);
    assert_eq!(receipt.shared_intervals().len(), 1);
    assert_eq!(receipt.overlap_islands().len(), 1);
    assert_eq!(receipt.containment_relations().len(), 1);
    assert_eq!(
        receipt.containment_relations()[0].containment(),
        "contained-hole"
    );
    assert_eq!(receipt.counters().candidate_pair_breadth(), 32);

    let reversed = CoplanarOverlapContractExtractor::between(second, first)
        .within_planar_neighborhood(NEIGHBORHOOD)
        .compile(&contracts)
        .expect("reversed overlap plan")
        .extract()
        .expect("reversed overlap receipt");
    assert_eq!(receipt.fact_digest(), reversed.fact_digest());
}

#[test]
fn coplanar_overlap_contract_extractor_denies_ambiguous_or_policy_required_cases_before_imprint() {
    let first = overlap_face(
        "overlap-deny-a",
        "face:left",
        "movement:stable",
        &[[0.0, 0.0], [2.0e-9, 0.0], [2.0e-9, 2.0e-9], [0.0, 2.0e-9]],
    );
    let moved = overlap_face(
        "overlap-deny-b",
        "face:right",
        "movement:tiny-rotation-exits-coplanar-class",
        &[
            [2.0e-9, 0.0],
            [4.0e-9, 0.0],
            [4.0e-9, 2.0e-9],
            [2.0e-9, 2.0e-9],
        ],
    );

    let denial = match CoplanarOverlapContractExtractor::between(first, moved)
        .within_planar_neighborhood(NEIGHBORHOOD)
        .compile(&overlap_contracts("overlap-deny-a"))
    {
        Ok(_) => panic!("movement mismatch should deny before extraction"),
        Err(denial) => denial,
    };

    assert_eq!(
        denial.kind(),
        CoplanarOverlapDenialKind::MismatchedMovementRotationPosture
    );
}

#[test]
fn coplanar_overlap_contract_extractor_retains_policy_required_exits_without_boolean_success() {
    let world = "overlap-policy-required";
    let policy_face = overlap_face_with_containment_candidate(
        world,
        "face:policy",
        "movement:stable",
        &[[0.0, 0.0], [2.0e-9, 0.0], [2.0e-9, 2.0e-9], [0.0, 2.0e-9]],
        &[
            [20.0e-9, 0.0],
            [22.0e-9, 0.0],
            [22.0e-9, 2.0e-9],
            [20.0e-9, 2.0e-9],
        ],
    );
    let ordinary_face = overlap_face(
        world,
        "face:ordinary",
        "movement:stable",
        &[
            [2.0e-9, 0.0],
            [4.0e-9, 0.0],
            [4.0e-9, 2.0e-9],
            [2.0e-9, 2.0e-9],
        ],
    );

    let contracts = overlap_contracts(world);
    let receipt =
        CoplanarOverlapContractExtractor::between(policy_face.clone(), ordinary_face.clone())
            .within_planar_neighborhood(NEIGHBORHOOD)
            .compile(&contracts)
            .expect("policy-required overlap plan")
            .extract()
            .expect("policy-required exits are retained contract facts");

    assert_eq!(receipt.boolean_result(), None);
    assert_eq!(receipt.imprint_action(), None);
    assert_eq!(receipt.policy_required_exits().len(), 1);
    assert_eq!(receipt.shared_intervals().len(), 0);
    assert_eq!(receipt.counters().segment_contacts_certified(), 0);
    assert_eq!(receipt.counters().policy_required_exits(), 1);
    assert!(receipt.policy_required_exits()[0]
        .reason()
        .contains("signed-area-policy-required-before-overlap-imprint"));

    let reversed = CoplanarOverlapContractExtractor::between(ordinary_face, policy_face)
        .within_planar_neighborhood(NEIGHBORHOOD)
        .compile(&contracts)
        .expect("reversed policy-required overlap plan")
        .extract()
        .expect("reversed policy-required overlap receipt");
    assert_eq!(receipt.fact_digest(), reversed.fact_digest());
    assert_eq!(
        receipt.policy_required_exits()[0].consumed_fact_digest(),
        reversed.policy_required_exits()[0].consumed_fact_digest()
    );
}

#[test]
fn coplanar_overlap_contract_extractor_canonicalizes_containment_rows_across_reversed_faces() {
    let world = "overlap-dual-containment";
    let first = overlap_face_with_containment_candidate(
        world,
        "face:with-hole-a",
        "movement:stable",
        &[[0.0, 0.0], [2.0e-9, 0.0], [2.0e-9, 2.0e-9], [0.0, 2.0e-9]],
        &[
            [0.5e-9, 0.5e-9],
            [0.75e-9, 0.5e-9],
            [0.75e-9, 0.75e-9],
            [0.5e-9, 0.75e-9],
        ],
    );
    let second = overlap_face_with_containment_candidate(
        world,
        "face:with-hole-b",
        "movement:stable",
        &[
            [2.0e-9, 0.0],
            [4.0e-9, 0.0],
            [4.0e-9, 2.0e-9],
            [2.0e-9, 2.0e-9],
        ],
        &[
            [2.5e-9, 0.5e-9],
            [2.75e-9, 0.5e-9],
            [2.75e-9, 0.75e-9],
            [2.5e-9, 0.75e-9],
        ],
    );
    let contracts = overlap_contracts(world);

    let receipt = CoplanarOverlapContractExtractor::between(first.clone(), second.clone())
        .within_planar_neighborhood(NEIGHBORHOOD)
        .compile(&contracts)
        .expect("dual-containment overlap plan")
        .extract()
        .expect("dual-containment overlap receipt");
    let reversed = CoplanarOverlapContractExtractor::between(second, first)
        .within_planar_neighborhood(NEIGHBORHOOD)
        .compile(&contracts)
        .expect("reversed dual-containment overlap plan")
        .extract()
        .expect("reversed dual-containment overlap receipt");

    assert_eq!(receipt.containment_relations().len(), 2);
    assert_eq!(reversed.containment_relations().len(), 2);
    assert_eq!(
        receipt.containment_relations(),
        reversed.containment_relations()
    );
    assert_eq!(receipt.fact_digest(), reversed.fact_digest());
}

#[test]
fn coplanar_overlap_contract_extractor_retains_ambiguous_contacts_without_boolean_success() {
    let world = "overlap-ambiguous-contact";
    let first = overlap_face(
        world,
        "face:ambiguous-a",
        "movement:stable",
        &[[0.0, 0.0], [3.0e-9, 0.0], [3.0e-9, 3.0e-9], [0.0, 3.0e-9]],
    );
    let second = overlap_face(
        world,
        "face:ambiguous-b",
        "movement:stable",
        &[
            [1.0e-9, 1.0e-9],
            [4.0e-9, 1.0e-9],
            [4.0e-9, 4.0e-9],
            [1.0e-9, 4.0e-9],
        ],
    );

    let receipt = CoplanarOverlapContractExtractor::between(first, second)
        .within_planar_neighborhood(NEIGHBORHOOD)
        .compile(&overlap_contracts(world))
        .expect("ambiguous contact plan")
        .extract()
        .expect("ambiguous contacts are retained contract facts");

    assert_eq!(receipt.boolean_result(), None);
    assert_eq!(receipt.imprint_action(), None);
    assert_eq!(receipt.shared_intervals().len(), 0);
    assert!(!receipt.ambiguous_contacts().is_empty());
    assert!(receipt
        .ambiguous_contacts()
        .iter()
        .all(|row| !row.segment_fact_digest().is_empty()));
}

#[test]
fn mb_m6_1_coplanar_overlap_contract_storm_complete_contract_bundle() {
    let world = "overlap-storm";
    let base = overlap_face(
        world,
        "face:storm-a",
        "movement:move-then-rotate-equivalent",
        &[[0.0, 0.0], [3.0e-9, 0.0], [3.0e-9, 3.0e-9], [0.0, 3.0e-9]],
    );
    let flush = overlap_face(
        world,
        "face:storm-b",
        "movement:move-then-rotate-equivalent",
        &[
            [3.0e-9, 0.0],
            [6.0e-9, 0.0],
            [6.0e-9, 3.0e-9],
            [3.0e-9, 3.0e-9],
        ],
    );
    let receipt = CoplanarOverlapContractExtractor::between(base, flush)
        .within_planar_neighborhood(NEIGHBORHOOD)
        .compile(&overlap_contracts(world))
        .expect("storm plan")
        .extract()
        .expect("storm receipt");

    assert_eq!(receipt.shared_intervals().len(), 1);
    assert_eq!(receipt.policy_required_exits().len(), 0);
    assert_eq!(receipt.counters().segment_contacts_certified(), 16);
    assert!(receipt
        .shared_intervals()
        .iter()
        .all(|row| row.segment_fact_digest().len() > 20));
    assert_eq!(receipt.boolean_result(), None);
}
