use super::{
    basis_lifecycle_migration_audit, basis_lifecycle_migration_audit_digest,
    BasisLifecycleMigrationPosture, BasisLifecycleMigrationSurface,
};

#[test]
fn migration_audit_names_required_existing_basis_surfaces() {
    let audit = basis_lifecycle_migration_audit();

    for surface in [
        BasisLifecycleMigrationSurface::BranchPreviewAdmission,
        BasisLifecycleMigrationSurface::ReadCompositionBasisContext,
        BasisLifecycleMigrationSurface::SubscriptionBasisPosture,
        BasisLifecycleMigrationSurface::CausalInspectionBasisEvidence,
        BasisLifecycleMigrationSurface::HistoricalMaterializationBasis,
        BasisLifecycleMigrationSurface::LowerRuntimeReadmissionEvidence,
        BasisLifecycleMigrationSurface::FutureNeighborStoreDurableBasis,
    ] {
        assert!(
            audit.row_for(surface).is_some(),
            "missing migration audit surface {}",
            surface.as_str()
        );
    }

    assert_eq!(audit.counters().audited_surface_count(), 7);
    assert!(!audit.audit_digest().is_empty());
}

#[test]
fn migration_audit_has_no_in_scope_compatibility_debt_remaining() {
    let audit = basis_lifecycle_migration_audit();

    assert_eq!(audit.counters().compatibility_debt_count(), 0);
    for surface in [
        BasisLifecycleMigrationSurface::BranchPreviewAdmission,
        BasisLifecycleMigrationSurface::ReadCompositionBasisContext,
        BasisLifecycleMigrationSurface::SubscriptionBasisPosture,
        BasisLifecycleMigrationSurface::CausalInspectionBasisEvidence,
        BasisLifecycleMigrationSurface::HistoricalMaterializationBasis,
    ] {
        let row = audit.row_for(surface).expect("surface must be audited");

        assert_eq!(
            row.posture(),
            BasisLifecycleMigrationPosture::LifecycleNative
        );
        assert!(
            !row.lifecycle_artifact().contains("adapt_"),
            "{} must name its native scoped artifact",
            row.surface().as_str()
        );
    }
}

#[test]
fn migration_audit_distinguishes_covered_and_deferred_surfaces() {
    let audit = basis_lifecycle_migration_audit();
    let lower_runtime = audit
        .row_for(BasisLifecycleMigrationSurface::LowerRuntimeReadmissionEvidence)
        .expect("lower-runtime evidence must be audited");
    let future = audit
        .row_for(BasisLifecycleMigrationSurface::FutureNeighborStoreDurableBasis)
        .expect("future-neighbor basis must be audited");

    assert_eq!(
        lower_runtime.posture(),
        BasisLifecycleMigrationPosture::LifecycleNative
    );
    assert_eq!(
        future.posture(),
        BasisLifecycleMigrationPosture::DeferredFutureNeighbor
    );
    assert_eq!(audit.counters().lifecycle_covered_count(), 6);
    assert_eq!(audit.counters().deferred_future_neighbor_count(), 1);
}

#[test]
fn migration_audit_does_not_hide_debt_behind_adapter_posture() {
    let audit = basis_lifecycle_migration_audit();

    for row in audit.rows() {
        match row.posture() {
            BasisLifecycleMigrationPosture::CompatibilityDebt => {
                assert!(
                    row.compatibility_debt().is_some(),
                    "{} compatibility debt must be named",
                    row.surface().as_str()
                );
            }
            BasisLifecycleMigrationPosture::LifecycleNative
            | BasisLifecycleMigrationPosture::LifecycleAdapterCovered
            | BasisLifecycleMigrationPosture::DeferredFutureNeighbor => {
                assert!(
                    row.compatibility_debt().is_none(),
                    "{} non-debt posture must not carry hidden debt",
                    row.surface().as_str()
                );
            }
        }
    }
}

#[test]
fn migration_audit_digest_is_stable_certification_evidence() {
    assert_eq!(
        basis_lifecycle_migration_audit_digest(),
        basis_lifecycle_migration_audit().audit_digest()
    );
}
