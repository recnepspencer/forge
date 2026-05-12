use forge_store::{S0StableDigest, StorageFoundationS1Handoff};

fn fake<T>() -> T {
    panic!("type-check only")
}

fn main() {
    let _ = StorageFoundationS1Handoff {
        envelope: fake(),
        backend_tier_matrix_digest: S0StableDigest::new("backend").unwrap(),
        deferred_guarantee_map_digest: S0StableDigest::new("deferred").unwrap(),
        terminology_scan_digest: S0StableDigest::new("terms").unwrap(),
        audit_input_manifest_digest: S0StableDigest::new("manifest").unwrap(),
        complexity_contract_summary_digest: S0StableDigest::new("complexity").unwrap(),
        required_forbidden_shortcuts: vec![],
        required_harness_subsystems: vec![],
        allowed_backend_candidates: vec![],
        legacy_backend_fences: vec![],
        compile_time_boundary_fixtures: vec![],
        non_platform_grade_debt_rows: vec![],
        blocking_predicates: vec![],
        gate_readiness: fake(),
        accepted_evidence_provenance: fake(),
    };
}
