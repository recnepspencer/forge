use forge_query::facade::LowerRuntimeBasisEvidence;
use worth_spatial::facade::planar_retained_facts::RetainedPlanarFacts;
use worth_spatial::facade::planar_retained_facts::{
    RetainedPlanarFactsContracts, RetainedPlanarFactsDenialKind, RetainedPlanarFactsReplaySubject,
};
use worth_spatial::facade::planar_structural_identity::{
    CanonicalPlanarTransformBasis, PlanarOrientationPolicy, PlanarStructuralIdentity,
    PlanarStructuralIdentityContracts,
};

use super::contract_subject::{retained_planar_parts, retained_planar_receipt};
use super::replay_without_repair::scoped_branch_head_inspection_basis;
use super::runtime_handles::{retained_planar_handle, structural_identity_handle};

#[test]
fn retained_planar_facts_reject_wrong_or_truncated_basis_before_partial_answer() {
    let world = "retained-planar-denials";
    let receipt = retained_planar_receipt(world);
    let contracts = RetainedPlanarFactsContracts::new(retained_planar_handle(world));
    for wrong_subject in wrong_replay_subjects(&receipt) {
        let historical = receipt
            .historical_inspection()
            .against_replay_subject(wrong_subject)
            .inspect(&contracts)
            .expect_err("historical replay must reject any wrong retained Query artifact");
        assert_eq!(
            historical.kind(),
            RetainedPlanarFactsDenialKind::TruncatedRetainedBasis
        );
    }

    let branch_basis = scoped_branch_head_inspection_basis("branch:retained-planar-denial");
    let branch_local = receipt
        .branch_local_inspection(
            branch_basis,
            LowerRuntimeBasisEvidence::from_relational_facade("wrong-branch-basis", "evidence", 1),
        )
        .inspect(&contracts)
        .expect_err("branch replay must reject wrong lower-runtime basis");
    assert!(branch_local.reason().contains("readmitted lower-runtime"));
}

#[test]
fn retained_planar_facts_deny_missing_topology_or_truncated_transform_posture() {
    let world = "retained-planar-compile-denials";
    let parts = retained_planar_parts(world);
    let contracts = RetainedPlanarFactsContracts::new(retained_planar_handle(world));

    let missing_topology =
        match RetainedPlanarFacts::from_boolean_readiness(parts.readiness.clone())
            .retain_planar_classification()
            .retain_structural_identity(parts.structural.clone())
            .retain_motion_posture(parts.motion.clone())
            .compile(&contracts)
        {
            Ok(_) => panic!("missing topology completeness must deny retained facts"),
            Err(error) => error,
        };
    assert_eq!(
        missing_topology.kind(),
        RetainedPlanarFactsDenialKind::MissingTopologyContractReceipt
    );

    let structural_without_motion =
        PlanarStructuralIdentity::from_boolean_readiness(parts.readiness.clone())
            .with_topology_identity("topology:retained-planar-no-motion")
            .with_persistent_name("name:retained-planar-no-motion")
            .with_binding_identity("binding:retained-planar-no-motion")
            .with_lineage_identity("lineage:retained-planar-no-motion")
            .with_canonical_transform_basis(canonical_transform_basis_without_motion())
            .compile(&PlanarStructuralIdentityContracts::new(
                structural_identity_handle(world),
            ))
            .expect("structural identity without motion plan")
            .certify()
            .expect("structural identity without motion receipt");

    let mismatched_motion = match RetainedPlanarFacts::from_boolean_readiness(parts.readiness)
        .retain_planar_classification()
        .retain_structural_identity(structural_without_motion)
        .retain_motion_posture(parts.motion)
        .retain_topology_contract(parts.bundle_parts.topology_contract)
        .compile(&contracts)
    {
        Ok(_) => panic!("structural identity without retained motion must deny replay"),
        Err(error) => error,
    };
    assert_eq!(
        mismatched_motion.kind(),
        RetainedPlanarFactsDenialKind::MismatchedMotionPosture
    );
}

fn canonical_transform_basis_without_motion() -> CanonicalPlanarTransformBasis {
    CanonicalPlanarTransformBasis::builder()
        .local_frame("frame:bundle")
        .movement_rotation_posture("movement:bundle-stable")
        .transform_chain_digest("transform:bundle")
        .orientation_policy(PlanarOrientationPolicy::Preserve)
        .build()
        .expect("canonical transform basis")
}

fn wrong_replay_subjects(
    receipt: &worth_spatial::facade::planar_retained_facts::RetainedPlanarFactsReceipt,
) -> Vec<RetainedPlanarFactsReplaySubject> {
    vec![
        RetainedPlanarFactsReplaySubject::new(
            "retained-planar-declaration:wrong",
            receipt.progression_digest(),
            receipt.route_plan_digest(),
            receipt.query_receipt_digest(),
            receipt.envelope_digest(),
            receipt.retained_fact_digest(),
        ),
        RetainedPlanarFactsReplaySubject::new(
            receipt.declaration_digest(),
            "retained-planar-progression:wrong",
            receipt.route_plan_digest(),
            receipt.query_receipt_digest(),
            receipt.envelope_digest(),
            receipt.retained_fact_digest(),
        ),
        RetainedPlanarFactsReplaySubject::new(
            receipt.declaration_digest(),
            receipt.progression_digest(),
            "retained-planar-route:wrong",
            receipt.query_receipt_digest(),
            receipt.envelope_digest(),
            receipt.retained_fact_digest(),
        ),
        RetainedPlanarFactsReplaySubject::new(
            receipt.declaration_digest(),
            receipt.progression_digest(),
            receipt.route_plan_digest(),
            "retained-planar-query-receipt:wrong",
            receipt.envelope_digest(),
            receipt.retained_fact_digest(),
        ),
        RetainedPlanarFactsReplaySubject::new(
            receipt.declaration_digest(),
            receipt.progression_digest(),
            receipt.route_plan_digest(),
            receipt.query_receipt_digest(),
            "retained-planar-envelope:wrong",
            receipt.retained_fact_digest(),
        ),
        RetainedPlanarFactsReplaySubject::new(
            receipt.declaration_digest(),
            receipt.progression_digest(),
            receipt.route_plan_digest(),
            receipt.query_receipt_digest(),
            receipt.envelope_digest(),
            "retained-planar-fact:wrong",
        ),
    ]
}
