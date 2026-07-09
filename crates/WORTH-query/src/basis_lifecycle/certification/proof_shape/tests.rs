use super::{
    basis_lifecycle_phase_progression_digest, basis_lifecycle_proof_shape_audit,
    basis_lifecycle_proof_shape_audit_digest, BasisLifecycleProofShapeEnforcement,
    BasisLifecycleProofShapeViolation,
};

#[test]
fn proof_shape_audit_names_every_required_rejection_lane() {
    let audit = basis_lifecycle_proof_shape_audit();

    for violation in [
        BasisLifecycleProofShapeViolation::PhaseSkipping,
        BasisLifecycleProofShapeViolation::RawIdentifierSubstitution,
        BasisLifecycleProofShapeViolation::StaleProofReuse,
        BasisLifecycleProofShapeViolation::OperationLaneWORTHry,
        BasisLifecycleProofShapeViolation::WorthdLowerRuntimeAuthority,
    ] {
        let row = audit
            .row_for(violation)
            .unwrap_or_else(|| panic!("missing proof-shape row {}", violation.as_str()));
        assert!(!row.attempted_shortcut().is_empty());
        assert!(!row.required_prior_artifact().is_empty());
        assert!(!row.rejected_artifact().is_empty());
        assert!(!row.enforcement_proof().is_empty());
        assert!(!row.row_digest().is_empty());
    }

    assert_eq!(audit.rows().len(), 5);
    assert_eq!(
        basis_lifecycle_proof_shape_audit_digest(),
        audit.proof_shape_digest()
    );
    assert_eq!(
        basis_lifecycle_phase_progression_digest(),
        audit.phase_progression_digest()
    );
    assert_ne!(audit.proof_shape_digest(), audit.phase_progression_digest());
}

#[test]
fn proof_shape_audit_rows_require_prior_artifacts_not_digest_substitutes() {
    let audit = basis_lifecycle_proof_shape_audit();

    for row in audit.rows() {
        assert!(
            proof_catalog_contains(row.enforcement(), row.enforcement_proof()),
            "{} must name a cataloged machine-checkable proof",
            row.violation().as_str()
        );
        assert!(
            !row.required_prior_artifact().starts_with("digest"),
            "{} must require a proof-bearing prior artifact",
            row.violation().as_str()
        );
        assert!(
            row.rejected_artifact().contains("Worthd")
                || row.rejected_artifact().contains("stale")
                || row.rejected_artifact().contains("identifier")
                || row.rejected_artifact().contains("draft")
                || row.rejected_artifact().contains("digest"),
            "{} must describe the hostile artifact",
            row.violation().as_str()
        );
    }
}

#[test]
fn proof_shape_audit_binds_each_violation_to_the_right_proof_family() {
    let audit = basis_lifecycle_proof_shape_audit();

    assert_row(
        &audit,
        BasisLifecycleProofShapeViolation::PhaseSkipping,
        BasisLifecycleProofShapeEnforcement::CompileFailFixture,
        "basis_lifecycle_dx_draft_is_not_scoped_proof",
    );
    assert_row(
        &audit,
        BasisLifecycleProofShapeViolation::RawIdentifierSubstitution,
        BasisLifecycleProofShapeEnforcement::BoundaryAudit,
        "basis_lifecycle_public_boundary_audit",
    );
    assert_row(
        &audit,
        BasisLifecycleProofShapeViolation::StaleProofReuse,
        BasisLifecycleProofShapeEnforcement::RuntimeDenialTest,
        "stale_runtime_snapshot_evidence_denies_at_readmission_boundary",
    );
    assert_row(
        &audit,
        BasisLifecycleProofShapeViolation::OperationLaneWORTHry,
        BasisLifecycleProofShapeEnforcement::CompileFailFixture,
        "basis_lifecycle_lane_witness_constructor_private",
    );
    assert_row(
        &audit,
        BasisLifecycleProofShapeViolation::WorthdLowerRuntimeAuthority,
        BasisLifecycleProofShapeEnforcement::CompileFailFixture,
        "basis_lifecycle_lower_runtime_evidence_constructor_private",
    );
}

fn assert_row(
    audit: &super::BasisLifecycleProofShapeAudit,
    violation: BasisLifecycleProofShapeViolation,
    enforcement: BasisLifecycleProofShapeEnforcement,
    enforcement_proof: &'static str,
) {
    let row = audit
        .row_for(violation)
        .unwrap_or_else(|| panic!("missing proof-shape row {}", violation.as_str()));

    assert_eq!(row.enforcement(), enforcement);
    assert_eq!(row.enforcement_proof(), enforcement_proof);
}

fn proof_catalog_contains(
    enforcement: BasisLifecycleProofShapeEnforcement,
    enforcement_proof: &str,
) -> bool {
    proof_catalog()
        .iter()
        .any(|(catalog_enforcement, catalog_proof)| {
            *catalog_enforcement == enforcement && *catalog_proof == enforcement_proof
        })
}

fn proof_catalog() -> &'static [(BasisLifecycleProofShapeEnforcement, &'static str)] {
    use BasisLifecycleProofShapeEnforcement::*;
    &[
        (
            CompileFailFixture,
            "basis_lifecycle_dx_draft_is_not_scoped_proof",
        ),
        (
            CompileFailFixture,
            "basis_lifecycle_lane_witness_constructor_private",
        ),
        (
            CompileFailFixture,
            "basis_lifecycle_lower_runtime_evidence_constructor_private",
        ),
        (BoundaryAudit, "basis_lifecycle_public_boundary_audit"),
        (
            RuntimeDenialTest,
            "stale_runtime_snapshot_evidence_denies_at_readmission_boundary",
        ),
    ]
}
