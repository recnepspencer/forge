use super::*;

#[test]
fn resource_diagnostics_summary_digest_tracks_retained_denial_drift() {
    let left = resource_diagnostics_summary_for_unknown_completion(ResourceRequestId::new(9_999));
    let right = resource_diagnostics_summary_for_unknown_completion(ResourceRequestId::new(9_998));

    assert_ne!(left.provenance_digest(), right.provenance_digest());
    assert_ne!(
        left.replay_reconstruction().denied_completion_digest(),
        right.replay_reconstruction().denied_completion_digest()
    );
    assert_eq!(left.runtime_summary(), right.runtime_summary());
}

#[test]
fn resource_diagnostics_summary_digest_tracks_expansion_budget() {
    let strict = resource_diagnostics_summary_for_budget(
        ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(2),
    );
    let loose = resource_diagnostics_summary_for_budget(
        ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(8),
    );

    assert_ne!(strict.provenance_digest(), loose.provenance_digest());
    assert_eq!(
        strict.replay_reconstruction().replay_digest(),
        loose.replay_reconstruction().replay_digest()
    );
    assert_eq!(strict.runtime_summary(), loose.runtime_summary());
}
