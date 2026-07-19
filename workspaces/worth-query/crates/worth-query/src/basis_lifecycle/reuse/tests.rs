use super::{
    basis_lifecycle_adapter_shape_contract_digest, basis_lifecycle_reuse_matrix,
    basis_lifecycle_reuse_matrix_digest, basis_lifecycle_signal_authority_digest,
    BasisLifecycleReuseSurface,
};

#[test]
fn reuse_matrix_names_every_required_lower_runtime_authority_surface() {
    let matrix = basis_lifecycle_reuse_matrix();

    for surface in [
        BasisLifecycleReuseSurface::BridgeSubscriptionBasis,
        BasisLifecycleReuseSurface::BridgeTruthViewBasis,
        BasisLifecycleReuseSurface::BridgeContinuityBasis,
        BasisLifecycleReuseSurface::BridgePreviewBasis,
        BasisLifecycleReuseSurface::BridgeWritebackBasis,
        BasisLifecycleReuseSurface::BridgeCausalEnvelopeBasis,
        BasisLifecycleReuseSurface::RelationalTruthHistorySnapshotBasis,
        BasisLifecycleReuseSurface::RelationalBridgeAdapterBasis,
        BasisLifecycleReuseSurface::SignalSnapshotReplayLineageBasis,
    ] {
        assert!(
            matrix.row_for(surface).is_some(),
            "missing reuse matrix row {}",
            surface.as_str()
        );
    }

    assert_eq!(matrix.rows().len(), 9);
    assert_eq!(
        basis_lifecycle_reuse_matrix_digest(),
        matrix.matrix_digest()
    );
}

#[test]
fn reuse_matrix_rows_are_not_vague_reuse_claims() {
    let matrix = basis_lifecycle_reuse_matrix();

    for row in matrix.rows() {
        assert!(
            row.owning_facade().contains("facade"),
            "{} must name a facade boundary",
            row.surface().as_str()
        );
        assert!(
            !row.authority_artifact().is_empty(),
            "{} must name the authority-owned artifact",
            row.surface().as_str()
        );
        assert!(
            row.query_wrapper().contains("LowerRuntimeBasisEvidence"),
            "{} must reuse the lifecycle readmission wrapper",
            row.surface().as_str()
        );
        assert!(
            !row.allowed_carried_fields().contains("fields"),
            "{} allowed fields must be summarized by meaning, not raw ownership",
            row.surface().as_str()
        );
        assert!(
            row.forbidden_duplicate_fields().contains("fields")
                || row.forbidden_duplicate_fields().contains("internals"),
            "{} must name forbidden duplicate authority state",
            row.surface().as_str()
        );
        assert!(
            !row.enforcement_proof().is_empty(),
            "{} must point at an enforcing test or compile-fail proof",
            row.surface().as_str()
        );
        assert_eq!(
            row.posture(),
            "reused",
            "{} must not carry compatibility-debt or ambiguous reuse posture at 9.3.2 closeout",
            row.surface().as_str()
        );
    }
}

#[test]
fn adapter_shape_contract_digest_is_distinct_from_matrix_enumeration() {
    assert_ne!(
        basis_lifecycle_adapter_shape_contract_digest(),
        basis_lifecycle_reuse_matrix_digest()
    );
}

#[test]
fn signal_authority_digest_is_bound_to_the_signal_reuse_row() {
    let matrix = basis_lifecycle_reuse_matrix();
    let signal = matrix
        .row_for(BasisLifecycleReuseSurface::SignalSnapshotReplayLineageBasis)
        .expect("signal reuse row must exist");

    assert_eq!(
        basis_lifecycle_signal_authority_digest(),
        signal.row_digest()
    );
    assert_eq!(signal.posture(), "reused");
    assert_eq!(signal.owning_crate(), "worth-signal");
}
