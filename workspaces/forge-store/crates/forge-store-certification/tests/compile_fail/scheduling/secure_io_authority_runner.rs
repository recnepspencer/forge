#[test]
fn io_qos_secure_io_authority_rejects_lower_authority_sources() {
    let repo_root = repo_root();
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

fn compile_fail_fixtures() -> [CompileFailFixture; 6] {
    [
        CompileFailFixture {
            name: "identity_provider_claim_cannot_satisfy_secure_io_scope.rs",
            expected_stderr: &[
                "SecureIoPreservationRequest::new",
                "IoSchedulerSecurityScopeAdmission",
                "StoreJwtSubjectClaim",
            ],
        },
        CompileFailFixture {
            name: "kms_key_id_cannot_satisfy_secure_io_scope.rs",
            expected_stderr: &[
                "SecureIoPreservationRequest::new",
                "IoSchedulerSecurityScopeAdmission",
                "StoreKmsKeyIdentifier",
            ],
        },
        CompileFailFixture {
            name: "iam_role_cannot_satisfy_secure_io_scope.rs",
            expected_stderr: &[
                "SecureIoPreservationRequest::new",
                "IoSchedulerSecurityScopeAdmission",
                "StoreIamRoleClaim",
            ],
        },
        CompileFailFixture {
            name: "operator_identity_cannot_satisfy_secure_io_scope.rs",
            expected_stderr: &[
                "SecureIoPreservationRequest::new",
                "IoSchedulerSecurityScopeAdmission",
                "StoreOperatorIdentityClaim",
            ],
        },
        CompileFailFixture {
            name: "terminal_projection_cannot_satisfy_secure_io_scope.rs",
            expected_stderr: &[
                "SecureIoPreservationRequest::new",
                "IoSchedulerSecurityScopeAdmission",
                "StoreTerminalProjectionText",
            ],
        },
        CompileFailFixture {
            name: "security_scope_identity_cannot_satisfy_secure_io_scope.rs",
            expected_stderr: &[
                "SecureIoPreservationRequest::new",
                "IoSchedulerSecurityScopeAdmission",
                "StoreSecurityScopeIdentity",
            ],
        },
    ]
}

fn assert_compile_fails(repo_root: &std::path::Path, fixture: CompileFailFixture) {
    let output = run_compile_fail_case(repo_root, fixture.name);
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
    let status = std::process::Command::new(cargo)
        .arg("build")
        .arg("--quiet")
        .arg("-p")
        .arg("forge-store-aspect-native")
        .arg("-p")
        .arg("forge-store-contracts")
        .arg("-p")
        .arg("forge-store-io-scheduler")
        .arg("-p")
        .arg("forge-store-security")
        .arg("--manifest-path")
        .arg(
            repo_root
                .join("workspaces")
                .join("forge-store")
                .join("Cargo.toml"),
        )
        .status()
        .unwrap();
    assert!(status.success(), "failed to build Store fixture deps");
}

fn run_compile_fail_case(repo_root: &std::path::Path, fixture_name: &str) -> std::process::Output {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let store_deps = repo_root
        .join("workspaces")
        .join("forge-store")
        .join("target")
        .join("debug")
        .join("deps");
    let fixture_path = manifest_dir
        .join("tests")
        .join("compile_fail")
            .join("scheduling")
        .join("secure_io_authority")
        .join(fixture_name);
    std::process::Command::new("rustc")
        .arg("--edition=2021")
        .arg(fixture_path)
        .arg("--crate-name")
        .arg(fixture_name.trim_end_matches(".rs"))
        .arg("--crate-type")
        .arg("bin")
        .arg("-L")
        .arg(format!("dependency={}", manifest_path(&store_deps)))
        .args(extern_args(&store_deps))
        .output()
        .unwrap()
}

fn extern_args(store_deps: &std::path::Path) -> Vec<std::ffi::OsString> {
    let crates = [
        "forge_store_aspect_native",
        "forge_store_contracts",
        "forge_store_io_scheduler",
        "forge_store_security",
    ];
    let mut args = Vec::new();
    for crate_name in crates {
        args.push("--extern".into());
        args.push(
            format!(
                "{crate_name}={}",
                manifest_path(&rlib_path(store_deps, crate_name))
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
