use forge_query::facade::{
    admit_basis_capability, evaluate_basis_inspection_eligibility, normalize_raw_basis_intent,
    LowerRuntimeBasisEvidence, RawBasisIntent, ScopedInspectionBasis,
};
use worth_spatial::facade::planar_motion_posture::{
    PlanarMotionCancellation, PlanarMotionPosture, PlanarMotionPostureContracts,
    PlanarReorientation,
};
use worth_spatial::facade::planar_retained_facts::{
    RetainedPlanarFacts, RetainedPlanarFactsContracts, RetainedPlanarFactsDenialKind,
    RetainedPlanarFactsReplaySubject,
};
use worth_spatial::facade::planar_structural_identity::{
    PlanarStructuralIdentity, PlanarStructuralIdentityContracts,
};

use super::super::bundle_closeout::contract_bundle::readiness_receipt;
use super::super::bundle_closeout::runtime_handles::{
    motion_posture_handle, retained_planar_handle, structural_identity_handle,
};

#[test]
fn kernel_consumes_retained_planar_facts_without_live_state_repair() {
    let readiness = readiness_receipt();
    let topology_contract = readiness.basis().topology_contract_receipt().clone();
    let motion = PlanarMotionPosture::from_boolean_readiness(readiness.clone())
        .after_exact_translation("motion:kernel-retained-translate")
        .after_exact_rotation("motion:kernel-retained-rotation")
        .after_reorientation(PlanarReorientation::PreservesHandedness)
        .with_cancellation_policy(PlanarMotionCancellation::ExactBasisReplay)
        .compile(&PlanarMotionPostureContracts::new(motion_posture_handle()))
        .expect("motion posture plan")
        .certify()
        .expect("motion posture receipt");
    let structural = PlanarStructuralIdentity::from_boolean_readiness(readiness.clone())
        .with_motion_posture(motion.clone())
        .with_topology_identity("topology:kernel-retained")
        .with_persistent_name("name:kernel-retained")
        .with_binding_identity("binding:kernel-retained")
        .with_lineage_identity("lineage:kernel-retained")
        .compile(&PlanarStructuralIdentityContracts::new(
            structural_identity_handle(),
        ))
        .expect("structural identity plan")
        .certify()
        .expect("structural identity receipt");
    let contracts = RetainedPlanarFactsContracts::new(retained_planar_handle());
    let retained = RetainedPlanarFacts::from_boolean_readiness(readiness)
        .retain_planar_classification()
        .retain_structural_identity(structural)
        .retain_motion_posture(motion)
        .retain_topology_contract(topology_contract)
        .compile(&contracts)
        .expect("retained planar facts plan")
        .retain()
        .expect("retained planar facts receipt");

    let historical = retained
        .historical_inspection()
        .against_replay_subject(retained.replay_subject())
        .inspect(&contracts)
        .expect("historical replay");
    let branch_basis = scoped_branch_head_inspection_basis("branch:kernel-retained");
    let branch_local = retained
        .branch_local_inspection(
            branch_basis.clone(),
            matching_branch_evidence(&branch_basis, "evidence:kernel-retained"),
        )
        .inspect(&contracts)
        .expect("branch-local replay");

    assert_eq!(
        historical.retained_fact_digest(),
        retained.retained_fact_digest()
    );
    assert_eq!(
        branch_local.retained_fact_digest(),
        retained.retained_fact_digest()
    );
    assert_eq!(retained.counters().retained_family_rows_inspected(), 11);
    assert_eq!(
        retained
            .basis()
            .motion_posture_receipt()
            .counters()
            .rotation_rows_inspected(),
        1
    );
}

#[test]
fn kernel_rejects_wrong_retained_planar_basis_before_partial_answer() {
    let readiness = readiness_receipt();
    let topology_contract = readiness.basis().topology_contract_receipt().clone();
    let motion = PlanarMotionPosture::from_boolean_readiness(readiness.clone())
        .after_exact_rotation("motion:kernel-retained-wrong-basis")
        .with_cancellation_policy(PlanarMotionCancellation::ExactBasisReplay)
        .compile(&PlanarMotionPostureContracts::new(motion_posture_handle()))
        .expect("motion posture plan")
        .certify()
        .expect("motion posture receipt");
    let structural = PlanarStructuralIdentity::from_boolean_readiness(readiness.clone())
        .with_motion_posture(motion.clone())
        .with_topology_identity("topology:kernel-retained-denial")
        .with_persistent_name("name:kernel-retained-denial")
        .with_binding_identity("binding:kernel-retained-denial")
        .with_lineage_identity("lineage:kernel-retained-denial")
        .compile(&PlanarStructuralIdentityContracts::new(
            structural_identity_handle(),
        ))
        .expect("structural identity plan")
        .certify()
        .expect("structural identity receipt");
    let contracts = RetainedPlanarFactsContracts::new(retained_planar_handle());
    let retained = RetainedPlanarFacts::from_boolean_readiness(readiness)
        .retain_planar_classification()
        .retain_structural_identity(structural)
        .retain_motion_posture(motion)
        .retain_topology_contract(topology_contract)
        .compile(&contracts)
        .expect("retained planar facts plan")
        .retain()
        .expect("retained planar facts receipt");

    let wrong_subject = RetainedPlanarFactsReplaySubject::new(
        retained.declaration_digest(),
        retained.progression_digest(),
        retained.route_plan_digest(),
        retained.query_receipt_digest(),
        retained.envelope_digest(),
        "retained:wrong",
    );
    let denied = retained
        .historical_inspection()
        .against_replay_subject(wrong_subject)
        .inspect(&contracts)
        .expect_err("wrong retained subject must deny replay");
    assert_eq!(
        denied.kind(),
        RetainedPlanarFactsDenialKind::TruncatedRetainedBasis
    );

    let branch_basis = scoped_branch_head_inspection_basis("branch:kernel-retained-denial");
    let branch_denial = retained
        .branch_local_inspection(
            branch_basis,
            LowerRuntimeBasisEvidence::from_relational_facade("wrong-basis", "evidence", 1),
        )
        .inspect(&contracts)
        .expect_err("wrong branch evidence must deny replay");
    assert!(branch_denial.reason().contains("readmitted lower-runtime"));
}

fn scoped_branch_head_inspection_basis(branch_identity: &str) -> ScopedInspectionBasis {
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

fn matching_branch_evidence(
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
