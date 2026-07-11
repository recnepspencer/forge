#[test]
fn phase_11_closeout_rejects_certification_owned_authority_shortcuts() {
    let repo_root = repo_root();
    build_compile_fail_dependencies(&repo_root);
    for fixture in fixtures() {
        assert_compile_fails(&repo_root, fixture);
    }
}

#[derive(Debug, Clone, Copy)]
struct Fixture {
    name: &'static str,
    expected: &'static [&'static str],
}

fn fixtures() -> [Fixture; 3] {
    [
        Fixture {
            name: "closeout_input_cannot_be_struct_literal.rs",
            expected: &["S51CertificationCloseoutInput", "private"],
        },
        Fixture {
            name: "performance_rows_cannot_satisfy_closeout_input.rs",
            expected: &[
                "from_replay_and_security_scope",
                "S51SecurityScopeHarnessEvidence",
                "S51CloseoutPerformanceRows",
            ],
        },
        Fixture {
            name: "closeout_evidence_cannot_satisfy_readiness.rs",
            expected: &[
                "S51AdmittedSecurityScopeReadiness",
                "S51CertificationCloseoutEvidence",
            ],
        },
    ]
}

fn assert_compile_fails(repo_root: &std::path::Path, fixture: Fixture) {
    let output = run_compile_fail_case(repo_root, fixture.name);
    assert!(
        !output.status.success(),
        "{} unexpectedly compiled",
        fixture.name
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    for expected in fixture.expected {
        assert!(
            stderr.contains(expected),
            "{} failed for the wrong reason; missing {expected:?}\nstderr:\n{stderr}",
            fixture.name
        );
    }
}

fn repo_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
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
        .arg("forge-store-certification")
        .arg("-p")
        .arg("forge-store-physical-certification")
        .arg("-p")
        .arg("forge-store-readiness")
        .arg("--manifest-path")
        .arg(
            repo_root
                .join("workspaces")
                .join("forge-store")
                .join("Cargo.toml"),
        )
        .status()
        .unwrap();
    assert!(status.success(), "failed to build closeout fixture deps");
}

fn run_compile_fail_case(repo_root: &std::path::Path, fixture_name: &str) -> std::process::Output {
    let source = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("ui")
        .join("s5_1_certification_closeout")
        .join(fixture_name);
    let deps = repo_root
        .join("workspaces")
        .join("forge-store")
        .join("target")
        .join("debug")
        .join("deps");
    std::process::Command::new("rustc")
        .arg("--edition=2021")
        .arg(source)
        .arg("--crate-name")
        .arg(fixture_name.trim_end_matches(".rs"))
        .arg("--crate-type")
        .arg("bin")
        .arg("-L")
        .arg(format!("dependency={}", manifest_path(&deps)))
        .args(extern_args(&deps))
        .arg("--out-dir")
        .arg(std::env::temp_dir().join("forge-store-s51-closeout-ui"))
        .output()
        .unwrap()
}

fn extern_args(deps: &std::path::Path) -> Vec<std::ffi::OsString> {
    [
        "forge_store_certification",
        "forge_store_physical_certification",
        "forge_store_readiness",
    ]
    .into_iter()
    .flat_map(|crate_name| {
        [
            "--extern".into(),
            format!(
                "{crate_name}={}",
                manifest_path(&rlib_path(deps, crate_name))
            )
            .into(),
        ]
    })
    .collect()
}

fn rlib_path(deps: &std::path::Path, crate_name: &str) -> std::path::PathBuf {
    let prefix = format!("lib{crate_name}-");
    std::fs::read_dir(deps)
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
        .max_by_key(|path| path.metadata().and_then(|m| m.modified()).unwrap())
        .unwrap_or_else(|| panic!("missing compiled rlib for {crate_name}"))
}

fn manifest_path(path: &std::path::Path) -> String {
    path.display().to_string().replace('\\', "/")
}
