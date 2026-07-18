use std::path::Path;

use worth_store_test_support::compiler_boundary::{
    cargo_dependency_manifest, run_cargo_ui_fixture_suite,
};

#[test]
fn semantic_visibility_cannot_satisfy_physical_read_stability_authority() {
    let root = store_workspace_root();
    let forge_root = root.ancestors().nth(2).unwrap();
    let evidence = run_cargo_ui_fixture_suite(
        root,
        "physical-semantic-isolation",
        cargo_dependency_manifest(
            &[
                ("worth-foundational", forge_root.join("crates/worth-foundational").as_path(), &[]),
                ("worth-relational", forge_root.join("crates/worth-relational").as_path(), &[]),
                ("worth-store-physical-isolation", root.join("crates/worth-store-physical-isolation").as_path(), &[]),
            ],
            &[],
        ),
        "production",
        "diagnostic-test",
        &root.join("crates/worth-store-certification/tests/compile_fail/physical_isolation/physical_semantic_isolation"),
        FIXTURES,
    ).unwrap();
    assert_eq!(evidence.fixtures.len(), FIXTURES.len());
}

const FIXTURES: &[(&str, &[&str])] = &[
    (
        "transaction_id_cannot_satisfy_physical_authority.rs",
        &["PhysicalReadStabilityAuthority", "TransactionId"],
    ),
    (
        "branch_id_cannot_satisfy_physical_authority.rs",
        &["PhysicalReadStabilityAuthority", "BranchId"],
    ),
    (
        "snapshot_handle_cannot_satisfy_physical_authority.rs",
        &["PhysicalReadStabilityAuthority", "SnapshotHandle"],
    ),
    (
        "semantic_snapshot_scalar_cannot_admit_stable_read_plan.rs",
        &["PhysicalEpoch", "unresolved import"],
    ),
    (
        "semantic_reference_cannot_satisfy_physical_authority.rs",
        &["PhysicalReadStabilityAuthority", "SemanticNodeReference"],
    ),
    (
        "correlation_cannot_satisfy_physical_authority.rs",
        &[
            "PhysicalReadStabilityAuthority",
            "SemanticPhysicalCorrelation",
        ],
    ),
    (
        "derived_role_claim_cannot_satisfy_physical_authority.rs",
        &[
            "PhysicalReadStabilityAuthority",
            "FoundationalBoundaryRoleClaim",
        ],
    ),
];

fn store_workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap()
}
