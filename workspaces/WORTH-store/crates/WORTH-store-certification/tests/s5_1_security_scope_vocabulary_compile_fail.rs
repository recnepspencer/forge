#[test]
fn s5_1_security_scope_vocabulary_rejects_lower_authority_sources() {
    let repo_root = repo_root();
    ensure_compile_fail_fixture_support_crates_are_linked();
    build_compile_fail_dependencies(&repo_root);
    for fixture in compile_fail_fixtures() {
        assert_compile_fails(&repo_root, fixture);
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

fn assert_compile_fails(repo_root: &std::path::Path, fixture: CompileFailFixture) {
    let case_dir = prepare_compile_fail_case(fixture.name);
    let output = run_compile_fail_case(repo_root, &case_dir, fixture.name);
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

fn repo_root() -> std::path::PathBuf {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .ancestors()
        .nth(4)
        .expect("certification crate lives under workspaces/worth-store/crates")
        .to_path_buf()
}

fn build_compile_fail_dependencies(repo_root: &std::path::Path) {
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let store_status = std::process::Command::new(&cargo)
        .arg("build")
        .arg("--quiet")
        .arg("-p")
        .arg("worth-store-aspect-native")
        .arg("-p")
        .arg("worth-store-security")
        .arg("-p")
        .arg("worth-store-authority")
        .arg("-p")
        .arg("worth-store-offline-verifier")
        .arg("-p")
        .arg("worth-store-operations")
        .arg("-p")
        .arg("worth-store-physical-format")
        .arg("--manifest-path")
        .arg(
            repo_root
                .join("workspaces")
                .join("worth-store")
                .join("Cargo.toml"),
        )
        .status()
        .unwrap();
    assert!(store_status.success(), "failed to build Store fixture deps");

    let root_status = std::process::Command::new(cargo)
        .arg("build")
        .arg("--quiet")
        .arg("-p")
        .arg("worth-foundational")
        .arg("-p")
        .arg("worth-proof")
        .arg("--manifest-path")
        .arg(repo_root.join("Cargo.toml"))
        .status()
        .unwrap();
    assert!(
        root_status.success(),
        "failed to build Foundational/Proof fixture deps"
    );
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
            .join("ui")
            .join("s5_1_security_scope_vocabulary")
            .join(fixture_name),
        source_dir.join("main.rs"),
    )
    .unwrap();
    case_dir
}

fn run_compile_fail_case(
    repo_root: &std::path::Path,
    case_dir: &std::path::Path,
    fixture_name: &str,
) -> std::process::Output {
    let store_deps = repo_root
        .join("workspaces")
        .join("worth-store")
        .join("target")
        .join("debug")
        .join("deps");
    let root_deps = repo_root.join("target").join("debug").join("deps");
    std::process::Command::new("rustc")
        .arg("--edition=2021")
        .arg(case_dir.join("src").join("main.rs"))
        .arg("--crate-name")
        .arg(fixture_name.trim_end_matches(".rs"))
        .arg("--crate-type")
        .arg("bin")
        .arg("-L")
        .arg(format!("dependency={}", manifest_path(&store_deps)))
        .arg("-L")
        .arg(format!("dependency={}", manifest_path(&root_deps)))
        .args(extern_args(&store_deps, &root_deps))
        .arg("--out-dir")
        .arg(compile_fail_case_target_dir(case_dir))
        .output()
        .unwrap()
}

fn compile_fail_case_dir(fixture_name: &str) -> std::path::PathBuf {
    std::env::temp_dir()
        .join("worth-store-s51-security-scope-vocabulary-ui")
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

fn extern_args(
    store_deps: &std::path::Path,
    root_deps: &std::path::Path,
) -> Vec<std::ffi::OsString> {
    let crates = [
        ("worth_store_aspect_native", store_deps),
        ("worth_store_authority", store_deps),
        ("worth_store_contracts", store_deps),
        ("worth_store_offline_verifier", store_deps),
        ("worth_store_operations", store_deps),
        ("worth_store_physical_format", store_deps),
        ("worth_store_security", store_deps),
        ("serde_json", store_deps),
        ("worth_foundational", root_deps),
        ("worth_proof", root_deps),
    ];
    let mut args = Vec::new();
    for (crate_name, deps_dir) in crates {
        args.push("--extern".into());
        args.push(
            format!(
                "{crate_name}={}",
                manifest_path(&rlib_path(deps_dir, crate_name))
            )
            .into(),
        );
    }
    args
}

fn rlib_path(deps_dir: &std::path::Path, crate_name: &str) -> std::path::PathBuf {
    let prefix = format!("lib{crate_name}-");
    std::fs::read_dir(deps_dir)
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension().and_then(|extension| extension.to_str()) == Some("rlib")
                && path
                    .file_name()
                    .and_then(|file_name| file_name.to_str())
                    .is_some_and(|file_name| file_name.starts_with(&prefix))
        })
        .max_by_key(|path| {
            path.metadata()
                .and_then(|metadata| metadata.modified())
                .unwrap()
        })
        .unwrap_or_else(|| panic!("missing compiled rlib for {crate_name}"))
}

fn manifest_path(path: &std::path::Path) -> String {
    path.display().to_string().replace('\\', "/")
}
