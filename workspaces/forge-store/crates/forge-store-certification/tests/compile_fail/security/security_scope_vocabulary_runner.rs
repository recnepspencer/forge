#[path = "../cargo_artifacts.rs"]
mod cargo_artifacts;

const TEST_TARGET: &str = "s5_1_security_scope_vocabulary_compile_fail";

#[test]
fn security_scope_vocabulary_rejects_lower_authority_sources() {
    ensure_compile_fail_fixture_support_crates_are_linked();
    cargo_artifacts::discover(TEST_TARGET);
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
    let case_dir = prepare_compile_fail_case(fixture.name);
    let output = run_compile_fail_case(&case_dir, fixture.name);
    assert!(
        !output.status.success(),
        "{} unexpectedly compiled",
        fixture.name
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    for expected in fixture.expected_stderr {
        assert!(
            stderr.contains(expected),
            "{} failed for the wrong reason; missing {expected:?}\nstderr:\n{stderr}",
            fixture.name
        );
    }
}

fn prepare_compile_fail_case(fixture_name: &str) -> std::path::PathBuf {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let case_dir = compile_fail_case_dir(fixture_name);
    if case_dir.exists() {
        std::fs::remove_dir_all(&case_dir).unwrap();
    }
    let source_dir = case_dir.join("src");
    std::fs::create_dir_all(&source_dir).unwrap();
    std::fs::copy(
        manifest_dir
            .join("tests")
            .join("compile_fail")
            .join("security")
            .join("security_scope_vocabulary")
            .join(fixture_name),
        source_dir.join("main.rs"),
    )
    .unwrap();
    case_dir
}

fn run_compile_fail_case(case_dir: &std::path::Path, fixture_name: &str) -> std::process::Output {
    let dependencies = cargo_artifacts::dependency_dir();
    std::process::Command::new("rustc")
        .arg("--edition=2021")
        .arg(case_dir.join("src").join("main.rs"))
        .arg("--crate-name")
        .arg(fixture_name.trim_end_matches(".rs"))
        .arg("--crate-type")
        .arg("bin")
        .arg("-L")
        .arg(format!("dependency={}", dependencies.display()))
        .args(extern_args())
        .arg("--out-dir")
        .arg(compile_fail_case_target_dir(case_dir))
        .output()
        .unwrap()
}

fn compile_fail_case_dir(fixture_name: &str) -> std::path::PathBuf {
    std::env::temp_dir()
        .join("forge-store-s51-security-scope-vocabulary-ui")
        .join(std::process::id().to_string())
        .join("cases")
        .join(fixture_name.trim_end_matches(".rs"))
}

fn compile_fail_case_target_dir(case_dir: &std::path::Path) -> std::path::PathBuf {
    case_dir
        .parent()
        .expect("compile-fail case lives under a cases directory")
        .parent()
        .expect("compile-fail cases directory lives under a process directory")
        .join("target")
}

fn extern_args() -> Vec<std::ffi::OsString> {
    let crates = [
        "forge_store_aspect_native",
        "forge_store_authority",
        "forge_store_contracts",
        "forge_store_offline_verifier",
        "forge_store_operations",
        "forge_store_physical_format",
        "forge_store_security",
        "serde_json",
        "forge_foundational",
        "forge_proof",
    ];
    let mut args = Vec::new();
    for crate_name in crates {
        args.push("--extern".into());
        args.push(
            format!(
                "{crate_name}={}",
                cargo_artifacts::compiled_extern(TEST_TARGET, crate_name).display()
            )
            .into(),
        );
    }
    args
}
