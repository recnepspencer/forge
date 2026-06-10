#![allow(dead_code)]

use forge_server::{
    ForgeServerCompatibilityFileEnvelope, ForgeServerQueryHandoffDenial,
    ForgeServerQueryHandoffDenialCode,
};

pub(crate) fn assert_same_canonical_file_identity(
    left: &ForgeServerCompatibilityFileEnvelope,
    right: &ForgeServerCompatibilityFileEnvelope,
) {
    assert_eq!(
        left.canonical_file_identity(),
        right.canonical_file_identity()
    );
    assert_eq!(
        left.metadata_receipt().metadata_identity(),
        right.metadata_receipt().metadata_identity()
    );
    assert_eq!(
        left.canonical_filename().canonical(),
        right.canonical_filename().canonical()
    );
}

pub(crate) fn assert_private_cacheability(
    envelope: &ForgeServerCompatibilityFileEnvelope,
    expected_surface_kind: &str,
) {
    let policy = envelope.cacheability_policy();
    assert_eq!(policy.surface_kind(), expected_surface_kind);
    assert_eq!(policy.cache_control(), "private, no-store");
    assert_eq!(
        policy.vary(),
        &[
            "authorization".to_string(),
            "x-forge-branch".to_string(),
            "x-forge-diagnostics".to_string(),
        ]
    );
    assert!(!policy.publicly_reusable());
    assert!(!policy.intermediary_reuse_safe());
    assert!(policy.branch_scoped());
    assert!(policy.auth_scoped());
    assert!(!policy.remask_safe_for_shared_caches());
}

pub(crate) fn assert_cacheability_matches_read_policy(
    read: &forge_server::ForgeServerCompatibilityRead,
) {
    assert_eq!(
        read.file_envelope().cacheability_policy().cache_control(),
        read.cache_policy().cache_control()
    );
    assert_eq!(
        read.file_envelope().cacheability_policy().vary(),
        read.cache_policy().vary()
    );
    assert_eq!(
        read.file_envelope()
            .cacheability_policy()
            .publicly_reusable(),
        read.cache_policy().publicly_reusable()
    );
}

pub(crate) fn assert_denial(
    denial: &ForgeServerQueryHandoffDenial,
    expected_code: ForgeServerQueryHandoffDenialCode,
    detail_fragment: &str,
) {
    assert_eq!(denial.code(), expected_code);
    assert!(
        denial.detail().contains(detail_fragment),
        "expected denial detail `{}` to contain `{detail_fragment}`",
        denial.detail()
    );
}
