use std::path::Path;

use worth_store_test_support::compiler_boundary::{
    cargo_dependency_manifest, run_cargo_ui_fixture_suite,
};

#[test]
fn security_scope_admission_rejects_forged_or_lower_authority_witnesses() {
    for fixture in compile_fail_fixtures() {
        assert_compile_fails(fixture);
    }
}

#[derive(Debug, Clone, Copy)]
struct CompileFailFixture {
    name: &'static str,
    expected_stderr: &'static [&'static str],
}

fn compile_fail_fixtures() -> Vec<CompileFailFixture> {
    vec![
        CompileFailFixture {
            name: "key_scope_witness_cannot_be_struct_literal.rs",
            expected_stderr: &["StoreCurrentKeyScopeWitness", "private"],
        },
        CompileFailFixture {
            name: "tenant_scope_witness_cannot_be_struct_literal.rs",
            expected_stderr: &["StoreCurrentTenantScopeWitness", "private"],
        },
        CompileFailFixture {
            name: "authenticity_witness_cannot_be_struct_literal.rs",
            expected_stderr: &["StoreCurrentAuthenticityScopeWitness", "private"],
        },
        CompileFailFixture {
            name: "custody_witness_cannot_be_struct_literal.rs",
            expected_stderr: &["StoreCurrentCustodyScopeWitness", "private"],
        },
        CompileFailFixture {
            name: "terminal_projection_text_cannot_satisfy_key_witness.rs",
            expected_stderr: &["StoreCurrentKeyScopeWitness", "StoreTerminalProjectionText"],
        },
        CompileFailFixture {
            name: "semantic_commit_visibility_cannot_satisfy_key_witness.rs",
            expected_stderr: &["StoreCurrentKeyScopeWitness", "StoreLowerAuthoritySource"],
        },
        CompileFailFixture {
            name: "foundational_surface_cannot_satisfy_key_witness.rs",
            expected_stderr: &[
                "StoreCurrentKeyScopeWitness",
                "FoundationalPerformancePublicSurfaceEntry",
            ],
        },
        CompileFailFixture {
            name: "admission_receipt_cannot_satisfy_key_witness.rs",
            expected_stderr: &[
                "StoreCurrentKeyScopeWitness",
                "StoreSecurityScopeAdmissionReceipt",
            ],
        },
        CompileFailFixture {
            name: "admission_receipt_id_cannot_satisfy_key_witness.rs",
            expected_stderr: &[
                "StoreCurrentKeyScopeWitness",
                "StoreSecurityScopeAdmissionReceiptId",
            ],
        },
        CompileFailFixture {
            name: "admission_counters_cannot_satisfy_key_witness.rs",
            expected_stderr: &[
                "StoreCurrentKeyScopeWitness",
                "StoreSecurityScopeAdmissionCounterSnapshot",
            ],
        },
        CompileFailFixture {
            name: "physical_security_metadata_carrier_cannot_be_struct_literal.rs",
            expected_stderr: &["StoreSecurityMetadata", "private"],
        },
        CompileFailFixture {
            name: "physical_security_metadata_envelope_cannot_be_struct_literal.rs",
            expected_stderr: &["PhysicalSecurityMetadataEnvelope", "private"],
        },
        CompileFailFixture {
            name: "terminal_physical_metadata_projection_cannot_satisfy_admitted_scope.rs",
            expected_stderr: &[
                "StoreAdmittedSecurityScope",
                "StoreRawSecurityMetadataProjection",
            ],
        },
        CompileFailFixture {
            name: "serde_loaded_physical_metadata_projection_cannot_satisfy_admitted_scope.rs",
            expected_stderr: &[
                "StoreAdmittedSecurityScope",
                "StoreRawSecurityMetadataProjection",
            ],
        },
        CompileFailFixture {
            name: "authenticity_result_cannot_satisfy_physical_security_metadata.rs",
            expected_stderr: &["StoreSecurityMetadata", "StoreAuthenticityResult"],
        },
        CompileFailFixture {
            name: "authenticity_result_cannot_be_struct_literal.rs",
            expected_stderr: &["StoreAuthenticityResult", "private"],
        },
        CompileFailFixture {
            name: "declared_checksum_cannot_satisfy_authenticity_witness.rs",
            expected_stderr: &["StoreAuthenticityWitnessInput", "DeclaredPhysicalChecksum"],
        },
        CompileFailFixture {
            name: "declared_checksum_cannot_satisfy_authenticity_result.rs",
            expected_stderr: &["StoreAuthenticityResult", "DeclaredPhysicalChecksum"],
        },
        CompileFailFixture {
            name: "integrity_evidence_bundle_cannot_satisfy_authenticity_witness.rs",
            expected_stderr: &[
                "StoreAuthenticityWitnessInput",
                "PhysicalIntegrityEvidenceBundle",
            ],
        },
        CompileFailFixture {
            name: "store_digest_evidence_cannot_satisfy_authenticity_witness.rs",
            expected_stderr: &["StoreAuthenticityWitnessInput", "StoreDigestEvidence"],
        },
    ]
}

fn ensure_compile_fail_fixture_support_crates_are_linked() {
    let _ = std::mem::size_of::<serde_json::Value>();
}

fn assert_compile_fails(fixture: CompileFailFixture) {
    let root = store_workspace_root();
    let forge_root = root.ancestors().nth(2).unwrap();
    let evidence = run_cargo_ui_fixture_suite(
        root,
        "security-scope-admission",
        cargo_dependency_manifest(
            &[
                (
                    "worth-foundational",
                    forge_root.join("crates/worth-foundational").as_path(),
                    &[],
                ),
                (
                    "worth-proof",
                    forge_root.join("crates/worth-proof").as_path(),
                    &[],
                ),
                (
                    "worth-store-aspect-native",
                    root.join("crates/worth-store-aspect-native").as_path(),
                    &[],
                ),
                (
                    "worth-store-authority",
                    root.join("crates/worth-store-authority").as_path(),
                    &[],
                ),
                (
                    "worth-store-contracts",
                    root.join("crates/worth-store-contracts").as_path(),
                    &[],
                ),
                (
                    "worth-store-physical-format",
                    root.join("crates/worth-store-physical-format").as_path(),
                    &[],
                ),
                (
                    "worth-store-physical-integrity",
                    root.join("crates/worth-store-physical-integrity").as_path(),
                    &[],
                ),
                (
                    "worth-store-readiness",
                    root.join("crates/worth-store-readiness").as_path(),
                    &[],
                ),
                (
                    "worth-store-recovery-physics",
                    root.join("crates/worth-store-recovery-physics").as_path(),
                    &[],
                ),
                (
                    "worth-store-security",
                    root.join("crates/worth-store-security").as_path(),
                    &[],
                ),
                (
                    "worth-store-wal",
                    root.join("crates/worth-store-wal").as_path(),
                    &[],
                ),
            ],
            &[("serde_json", "1")],
        ),
        "production",
        "diagnostic-test",
        &root.join(
            "crates/worth-store-certification/tests/compile_fail/security/security_scope_admission",
        ),
        &[(fixture.name, fixture.expected_stderr)],
    )
    .unwrap();
    assert_eq!(evidence.fixtures.len(), 1);
}

fn store_workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("certification crate lives under the Store workspace")
}
