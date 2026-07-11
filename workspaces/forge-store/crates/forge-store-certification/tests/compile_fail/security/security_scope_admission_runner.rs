#[test]
fn security_scope_admission_rejects_forged_or_lower_authority_witnesses() {
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
        .expect("certification crate lives under workspaces/forge-store/crates")
        .to_path_buf()
}

fn build_compile_fail_dependencies(repo_root: &std::path::Path) {
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let store_status = std::process::Command::new(&cargo)
        .arg("build")
        .arg("--quiet")
        .arg("-p")
        .arg("forge-store-aspect-native")
        .arg("-p")
        .arg("forge-store-authority")
        .arg("-p")
        .arg("forge-store-contracts")
        .arg("-p")
        .arg("forge-store-physical-format")
        .arg("-p")
        .arg("forge-store-physical-integrity")
        .arg("-p")
        .arg("forge-store-readiness")
        .arg("-p")
        .arg("forge-store-recovery-physics")
        .arg("-p")
        .arg("forge-store-security")
        .arg("-p")
        .arg("forge-store-wal")
        .arg("--manifest-path")
        .arg(
            repo_root
                .join("workspaces")
                .join("forge-store")
                .join("Cargo.toml"),
        )
        .status()
        .unwrap();
    assert!(store_status.success(), "failed to build Store fixture deps");

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
            .join("security_scope_admission")
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
        .join("forge-store")
        .join("target")
        .join("debug")
        .join("deps");
    let root_deps = store_deps.clone();
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
        .join("forge-store-s51-security-scope-admission-ui")
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
        ("forge_store_aspect_native", store_deps),
        ("forge_store_authority", store_deps),
        ("forge_store_contracts", store_deps),
        ("forge_store_physical_format", store_deps),
        ("forge_store_physical_integrity", store_deps),
        ("forge_store_readiness", store_deps),
        ("forge_store_recovery_physics", store_deps),
        ("forge_store_security", store_deps),
        ("forge_store_wal", store_deps),
        ("serde_json", store_deps),
        ("forge_foundational", root_deps),
        ("forge_proof", root_deps),
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
