use super::super::{
    basis_lifecycle_support_matrix, discover_basis_lifecycle_support,
    evaluate_basis_certification_eligibility, evaluate_basis_inspection_advisory_eligibility,
    evaluate_basis_inspection_eligibility, evaluate_basis_materialization_eligibility,
    evaluate_basis_observation_eligibility, normalize_raw_basis_intent, BasisFamily,
    BasisOperationLane, BasisSupportPosture, CertificationLaneWitness, DeniedBasisCapabilityKind,
    InspectionLaneWitness, MaterializationLaneWitness, ObservationLaneWitness, RawBasisIntent,
};

#[test]
fn support_matrix_is_derived_from_executable_lane_registry() {
    let matrix = basis_lifecycle_support_matrix();

    assert!(matrix.rows().iter().any(|row| {
        row.operation_lane() == "observation"
            && row.posture() == BasisSupportPosture::Admitted
            && row.family() == BasisFamily::CurrentHead
    }));
    assert!(matrix.rows().iter().any(|row| {
        row.operation_lane() == "inspection"
            && row.posture() == BasisSupportPosture::Advisory
            && row.family() == BasisFamily::PreviewDerived
    }));
    assert!(!matrix.matrix_digest().is_empty());
}

#[test]
fn support_discovery_reports_admitted_lane_before_execution() {
    let discovery = discover_basis_lifecycle_support(
        BasisFamily::CurrentHead,
        <ObservationLaneWitness as BasisOperationLane>::lane_name(),
    );
    let normalized = normalize_raw_basis_intent(
        RawBasisIntent::CurrentHead,
        <ObservationLaneWitness as BasisOperationLane>::lane_name(),
    )
    .expect("current head should normalize");

    evaluate_basis_observation_eligibility(normalized).expect("admitted support should execute");

    assert_eq!(discovery.posture(), BasisSupportPosture::Admitted);
    assert!(discovery.matched_row_digest().is_some());
    assert_eq!(discovery.counters().basis_support_lookup_count(), 1);
    assert!(!discovery.discovery_digest().is_empty());
}

#[test]
fn support_discovery_reports_advisory_without_admitted_promotion() {
    let discovery = discover_basis_lifecycle_support(
        BasisFamily::PreviewDerived,
        <InspectionLaneWitness as BasisOperationLane>::lane_name(),
    );
    let normalized = normalize_raw_basis_intent(
        RawBasisIntent::PreviewDerived {
            preview_identity: "preview-a".to_string(),
            source_basis_identity: "branch-a".to_string(),
        },
        <InspectionLaneWitness as BasisOperationLane>::lane_name(),
    )
    .expect("preview-derived basis should normalize");

    evaluate_basis_inspection_advisory_eligibility(normalized.clone())
        .expect("advisory discovery should match advisory eligibility");
    let admitted_denial = evaluate_basis_inspection_eligibility(normalized)
        .expect_err("advisory support must not become admitted");

    assert_eq!(discovery.posture(), BasisSupportPosture::Advisory);
    assert_eq!(
        admitted_denial.denial_kind(),
        DeniedBasisCapabilityKind::OperationIneligible
    );
}

#[test]
fn support_discovery_reports_deferred_and_unsupported_lanes() {
    let deferred = discover_basis_lifecycle_support(
        BasisFamily::DurableReload,
        <CertificationLaneWitness as BasisOperationLane>::lane_name(),
    );
    let durable = normalize_raw_basis_intent(
        RawBasisIntent::DurableReload {
            reload_identity: "reload-a".to_string(),
        },
        <CertificationLaneWitness as BasisOperationLane>::lane_name(),
    )
    .expect("durable reload should normalize into deferred posture");
    let durable_denial = evaluate_basis_certification_eligibility(durable)
        .expect_err("deferred support must deny execution");

    assert_eq!(deferred.posture(), BasisSupportPosture::Deferred);
    assert_eq!(
        durable_denial.denial_kind(),
        DeniedBasisCapabilityKind::DurableOverclaim
    );

    let unsupported = discover_basis_lifecycle_support(
        BasisFamily::BranchHead,
        <MaterializationLaneWitness as BasisOperationLane>::lane_name(),
    );
    let branch = normalize_raw_basis_intent(
        RawBasisIntent::BranchHead {
            branch_identity: "branch-a".to_string(),
            accessible: true,
        },
        <MaterializationLaneWitness as BasisOperationLane>::lane_name(),
    )
    .expect("branch head should normalize");
    let unsupported_denial = evaluate_basis_materialization_eligibility(branch)
        .expect_err("unsupported support must deny execution");

    assert_eq!(unsupported.posture(), BasisSupportPosture::Unsupported);
    assert!(unsupported.matched_row_digest().is_none());
    assert_eq!(
        unsupported_denial.denial_kind(),
        DeniedBasisCapabilityKind::OperationIneligible
    );
}
