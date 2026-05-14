use super::{
    basis_lifecycle_public_boundary_audit, basis_lifecycle_public_boundary_audit_digest,
    BasisLifecyclePublicBoundarySurface,
};

#[test]
fn public_boundary_audit_names_every_raw_basis_escape_surface() {
    let audit = basis_lifecycle_public_boundary_audit();

    for surface in [
        BasisLifecyclePublicBoundarySurface::BranchIdentifier,
        BasisLifecyclePublicBoundarySurface::SnapshotIdentifier,
        BasisLifecyclePublicBoundarySurface::PreviewIdentifier,
        BasisLifecyclePublicBoundarySurface::TenantIdentifier,
        BasisLifecyclePublicBoundarySurface::PolicyIdentifier,
        BasisLifecyclePublicBoundarySurface::RuntimeSnapshotIdentifier,
        BasisLifecyclePublicBoundarySurface::RawBasisIntentSubstitution,
        BasisLifecyclePublicBoundarySurface::NormalizedIntentSubstitution,
    ] {
        let row = audit
            .row_for(surface)
            .unwrap_or_else(|| panic!("missing boundary surface {}", surface.as_str()));
        assert!(!row.forbidden_token().is_empty());
        assert!(!row.blocked_entrypoint().is_empty());
        assert!(row.required_capability().contains("Basis"));
        assert!(!row.enforcement_proof().is_empty());
        assert!(!row.row_digest().is_empty());
    }

    assert_eq!(audit.rows().len(), 8);
    assert_eq!(
        basis_lifecycle_public_boundary_audit_digest(),
        audit.audit_digest()
    );
}

#[test]
fn public_boundary_audit_has_no_documentation_only_rows() {
    let audit = basis_lifecycle_public_boundary_audit();

    for row in audit.rows() {
        assert!(
            row.enforcement_proof().contains("_"),
            "{} must point at a concrete compile-fail or certification proof",
            row.surface().as_str()
        );
        assert!(
            !row.required_capability().contains("raw"),
            "{} must require proof-bearing capability, not another raw token",
            row.surface().as_str()
        );
    }
}
