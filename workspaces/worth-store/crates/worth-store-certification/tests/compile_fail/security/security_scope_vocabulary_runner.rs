use std::path::Path;

use worth_store_test_support::compiler_boundary::{
    cargo_dependency_manifest, run_cargo_ui_fixture_suite,
};

#[test]
fn security_scope_vocabulary_rejects_lower_authority_sources() {
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
            name: "raw_string_cannot_satisfy_tenant_scope.rs",
            expected_stderr: &["StoreTenantScope", "&str"],
        },
        CompileFailFixture {
            name: "semantic_id_cannot_satisfy_key_scope.rs",
            expected_stderr: &["StoreKeyScope", "StoreLowerAuthoritySource"],
        },
        CompileFailFixture {
            name: "terminal_json_label_cannot_satisfy_authenticity_class.rs",
            expected_stderr: &[
                "StoreAuthenticityRequirementClass",
                "StoreTerminalProjectionDisplayLabel",
            ],
        },
        CompileFailFixture {
            name: "serde_json_value_cannot_satisfy_tenant_scope.rs",
            expected_stderr: &["StoreTenantScope", "Value"],
        },
        CompileFailFixture {
            name: "terminal_projection_text_cannot_satisfy_authenticity_class.rs",
            expected_stderr: &[
                "StoreAuthenticityRequirementClass",
                "StoreTerminalProjectionText",
            ],
        },
        CompileFailFixture {
            name: "jwt_subject_cannot_satisfy_tenant_scope.rs",
            expected_stderr: &["StoreTenantScope", "StoreJwtSubjectClaim"],
        },
        CompileFailFixture {
            name: "app_org_id_cannot_satisfy_tenant_scope.rs",
            expected_stderr: &["StoreTenantScope", "StoreApplicationOrgIdClaim"],
        },
        CompileFailFixture {
            name: "kms_key_id_cannot_satisfy_key_scope.rs",
            expected_stderr: &["StoreKeyScope", "StoreKmsKeyIdentifier"],
        },
        CompileFailFixture {
            name: "iam_role_cannot_satisfy_custody_posture.rs",
            expected_stderr: &["StoreCustodyPosture", "StoreIamRoleClaim"],
        },
        CompileFailFixture {
            name: "operator_identity_cannot_satisfy_repair_authority.rs",
            expected_stderr: &[
                "StoreSecurityWitnessVocabulary",
                "StoreOperatorIdentityClaim",
            ],
        },
        CompileFailFixture {
            name: "operator_identity_cannot_satisfy_repair_readiness.rs",
            expected_stderr: &["RepairBlastRadiusReadiness", "StoreOperatorIdentityClaim"],
        },
        CompileFailFixture {
            name: "iam_role_cannot_satisfy_repair_readiness.rs",
            expected_stderr: &["RepairBlastRadiusReadiness", "StoreIamRoleClaim"],
        },
        CompileFailFixture {
            name: "audit_record_cannot_satisfy_repair_readiness.rs",
            expected_stderr: &["RepairBlastRadiusReadiness", "StoreRepairAuditRecordClaim"],
        },
        CompileFailFixture {
            name: "offline_repair_report_cannot_satisfy_repair_plan_declaration.rs",
            expected_stderr: &[
                "RepairBlastRadiusDeclaration",
                "OfflineRepairBlastRadiusObservation",
            ],
        },
        CompileFailFixture {
            name: "store_current_authority_cannot_satisfy_security_witness.rs",
            expected_stderr: &[
                "StoreSecurityWitnessVocabulary",
                "StoreCurrentAuthorityWitness",
            ],
        },
        CompileFailFixture {
            name: "foundational_surface_cannot_satisfy_key_scope.rs",
            expected_stderr: &["StoreKeyScope", "FoundationalPerformancePublicSurfaceEntry"],
        },
        CompileFailFixture {
            name: "proof_recipe_cannot_satisfy_security_witness.rs",
            expected_stderr: &["StoreSecurityWitnessVocabulary", "Recipe"],
        },
        CompileFailFixture {
            name: "security_witness_vocabulary_cannot_be_struct_literal.rs",
            expected_stderr: &["StoreSecurityWitnessVocabulary", "private"],
        },
        CompileFailFixture {
            name: "security_readiness_vocabulary_cannot_be_struct_literal.rs",
            expected_stderr: &["StoreSecurityReadinessVocabulary", "private"],
        },
        CompileFailFixture {
            name: "physical_metadata_cannot_declare_authenticity_result.rs",
            expected_stderr: &[
                "PhysicalSecurityMetadataDeclaration",
                "StoreAuthenticityResult",
            ],
        },
        CompileFailFixture {
            name: "key_version_posture_cannot_satisfy_key_scope.rs",
            expected_stderr: &["StoreKeyScope", "StoreKeyVersionPosture"],
        },
        CompileFailFixture {
            name: "tenant_scope_cannot_satisfy_key_scope.rs",
            expected_stderr: &["StoreKeyScope", "StoreTenantScope"],
        },
        CompileFailFixture {
            name: "authenticity_result_cannot_satisfy_authenticity_requirement.rs",
            expected_stderr: &["StoreAuthenticityRequirement", "StoreAuthenticityResult"],
        },
        CompileFailFixture {
            name: "custody_posture_cannot_satisfy_tenant_scope.rs",
            expected_stderr: &["StoreTenantScope", "StoreCustodyPosture"],
        },
        CompileFailFixture {
            name: "legacy_posture_cannot_satisfy_custody_posture.rs",
            expected_stderr: &["StoreCustodyPosture", "StoreLegacySecurityPosture"],
        },
        CompileFailFixture {
            name: "security_evidence_vocabulary_cannot_satisfy_security_witness.rs",
            expected_stderr: &[
                "StoreSecurityWitnessVocabulary",
                "StoreSecurityEvidenceVocabulary",
            ],
        },
        CompileFailFixture {
            name: "security_readiness_vocabulary_cannot_satisfy_security_witness.rs",
            expected_stderr: &[
                "StoreSecurityWitnessVocabulary",
                "StoreSecurityReadinessVocabulary",
            ],
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
        "security-scope-vocabulary",
        cargo_dependency_manifest(
            &[
                ("worth-foundational", forge_root.join("crates/worth-foundational").as_path(), &[]),
                ("worth-proof", forge_root.join("crates/worth-proof").as_path(), &[]),
                ("worth-store-aspect-native", root.join("crates/worth-store-aspect-native").as_path(), &[]),
                ("worth-store-authority", root.join("crates/worth-store-authority").as_path(), &[]),
                ("worth-store-contracts", root.join("crates/worth-store-contracts").as_path(), &[]),
                ("worth-store-offline-verifier", root.join("crates/worth-store-offline-verifier").as_path(), &[]),
                ("worth-store-operations", root.join("crates/worth-store-operations").as_path(), &[]),
                ("worth-store-physical-format", root.join("crates/worth-store-physical-format").as_path(), &[]),
                ("worth-store-security", root.join("crates/worth-store-security").as_path(), &[]),
            ],
            &[("serde_json", "1")],
        ),
        "production",
        "diagnostic-test",
        &root.join("crates/worth-store-certification/tests/compile_fail/security/security_scope_vocabulary"),
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
