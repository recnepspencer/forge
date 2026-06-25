//! Phase 8 downstream folklore inventory for worth-topo query-runtime paths.
//!
//! Mirrors the forge-query inventory shape: curated scan paths plus explicit
//! exclusions for harness-only residue deferred to later milestones.

pub const PHASE_EIGHT_QUERY_RUNTIME_SCAN_PATHS: &[&str] = &[
    "projection/runtime_boundary/query_runtime/adapters.rs",
    "projection/runtime_boundary/query_runtime/adapters/bridge_source.rs",
    "projection/runtime_boundary/query_runtime/adapters/bridge_source_support.rs",
    "projection/runtime_boundary/query_runtime/adapters/write_authority.rs",
    "projection/runtime_boundary/query_runtime/adapters/query_rows.rs",
    "projection/runtime_boundary/query_runtime/operator_bindings.rs",
    "query_native_runtime_boundary/identity_reporting.rs",
    "projection/runtime_boundary/query_support/admitted_commit_identity.rs",
    "projection/runtime_boundary/read_execution/",
    "topology_operators/authority_identity.rs",
    "certification/bridge.rs",
];

/// Harness and certification-only paths that may retain projection labels until Phase 9.
pub const PHASE_EIGHT_EXCLUDED_FOLKLORE_PATHS: &[&str] = &[
    "certification/topology_operator_closeout/tests/",
    "certification/projection_closeout/tests/",
    "projection/runtime_boundary/query_runtime/tests/",
    "test_support/",
];

pub const PHASE_EIGHT_FORBIDDEN_SUBSTITUTION_PATTERNS: &[&str] = &[
    "snapshot_token(",
    "from_external_authority_label(",
    "continuity_rebind_existing_target(format!(",
    "ForgeQueryExistingEntityTarget::new(format!",
    "ForgeQueryExistingRelationTarget::new(format!",
    ".evidence_identity().as_str()",
    "BridgeIdentityEvidence::as_str()",
    "format!(\"{entity_id:?}\")",
    "format!(\"{relation_id:?}\")",
    "TruthCommitIdentity::new(",
    "TruthSnapshotIdentity::new(",
    "TruthBranchIdentity::new(",
];
