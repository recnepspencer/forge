#[path = "../cargo_artifacts.rs"]
mod cargo_artifacts;

const TEST_TARGET: &str = "s5_1_certification_closeout_compile_fail";

#[test]
fn closeout_rejects_certification_owned_authority_shortcuts() {
    let repo_root = repo_root();
    build_compile_fail_dependencies(&repo_root);
    cargo_artifacts::discover(TEST_TARGET);
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
                "SecurityScopeHarnessEvidence",
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
        .expect("certification crate lives under workspaces/worth-store/crates")
        .to_path_buf()
}

fn build_compile_fail_dependencies(repo_root: &std::path::Path) {
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let status = std::process::Command::new(cargo)
        .arg("build")
        .arg("--quiet")
        .arg("-p")
        .arg("worth-store-certification")
        .arg("-p")
        .arg("worth-store-physical-certification")
        .arg("-p")
        .arg("worth-store-readiness")
        .arg("--manifest-path")
        .arg(
            repo_root
                .join("workspaces")
                .join("worth-store")
                .join("Cargo.toml"),
        )
        .status()
        .unwrap();
    assert!(status.success(), "failed to build closeout fixture deps");
}

fn run_compile_fail_case(_repo_root: &std::path::Path, fixture_name: &str) -> std::process::Output {
    let source = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("compile_fail")
        .join("security")
        .join("certification_closeout")
        .join(fixture_name);
    let deps = cargo_artifacts::dependency_dir();
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
        .arg(std::env::temp_dir().join("worth-store-s51-closeout-ui"))
        .output()
        .unwrap()
}

fn extern_args(deps: &std::path::Path) -> Vec<std::ffi::OsString> {
    [
        "worth_store_certification",
        "worth_store_physical_certification",
        "worth_store_readiness",
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

fn rlib_path(_deps: &std::path::Path, crate_name: &str) -> std::path::PathBuf {
    cargo_artifacts::compiled_extern(TEST_TARGET, crate_name)
}

fn manifest_path(path: &std::path::Path) -> String {
    path.display().to_string().replace('\\', "/")
}
