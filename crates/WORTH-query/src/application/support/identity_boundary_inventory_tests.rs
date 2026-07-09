use super::*;

#[test]
fn inventory_lists_are_non_empty_and_unique() {
    assert!(!EVIDENCE_IDENTITY_COVERED_SURFACES.is_empty());
    assert!(!EXACT_ZERO_FORMAT_DIGEST_PATHS.is_empty());
    assert_eq!(
        EXACT_ZERO_FORMAT_DIGEST_PATHS.len(),
        EXACT_ZERO_FORMAT_DIGEST_PATHS
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len()
    );
}

#[test]
fn every_format_digest_path_has_embedded_source() {
    for path in EXACT_ZERO_FORMAT_DIGEST_PATHS {
        assert!(
            source_for_format_digest_path(path).is_some(),
            "missing embedded source for {path}"
        );
    }
}

#[test]
fn every_lower_runtime_identity_shim_path_has_embedded_source() {
    for path in LOWER_RUNTIME_IDENTITY_SHIM_PATHS {
        assert!(
            source_for_format_digest_path(path).is_some(),
            "missing embedded lower-runtime source for {path}"
        );
    }
}

#[test]
fn lower_runtime_identity_shim_scan_rejects_hash_and_bridge_harness_labels() {
    assert!(
        scan_lower_runtime_identity_shim_paths().is_empty(),
        "lower-runtime identity shim residue survived in {:?}",
        scan_lower_runtime_identity_shim_paths()
    );
}
