#[path = "../cargo_artifacts.rs"]
mod cargo_artifacts;

const TEST_TARGET: &str = "s6_evidence_materialization_compile_fail";

#[test]
fn materialized_authority_boundaries_reject_public_forgery() {
    let repo_root = repo_root();
    build_compile_fail_dependencies(&repo_root);
    cargo_artifacts::discover(TEST_TARGET);
    for case in compile_fail_cases() {
        assert_compile_fails(&repo_root, case);
    }
}

struct CompileFailCase {
    name: &'static str,
    stderr_fragments: &'static [&'static str],
}

fn compile_fail_cases() -> [CompileFailCase; 8] {
    [
        CompileFailCase {
            name: "bundle_fields_cannot_be_minted.rs",
            stderr_fragments: &["private"],
        },
        CompileFailCase {
            name: "source_fields_cannot_be_minted.rs",
            stderr_fragments: &["private"],
        },
        CompileFailCase {
            name: "closeout_rejects_foundational_receipt.rs",
            stderr_fragments: &["mismatched types"],
        },
        CompileFailCase {
            name: "closeout_rejects_profile_evidence.rs",
            stderr_fragments: &["mismatched types"],
        },
        CompileFailCase {
            name: "closeout_rejects_foundational_boundary.rs",
            stderr_fragments: &["mismatched types"],
        },
        CompileFailCase {
            name: "closeout_rejects_canonical_basis.rs",
            stderr_fragments: &["mismatched types"],
        },
        CompileFailCase {
            name: "closeout_rejects_proof_trace.rs",
            stderr_fragments: &["mismatched types"],
        },
        CompileFailCase {
            name: "legacy_scalar_closeout_evidence_is_unavailable.rs",
            stderr_fragments: &["S6MaterializedCertificationCloseoutEvidence"],
        },
    ]
}

fn assert_compile_fails(repo_root: &std::path::Path, case: CompileFailCase) {
    let output = run_compile_fail_case(repo_root, case.name);
    assert!(
        !output.status.success(),
        "{} unexpectedly compiled",
        case.name
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    for fragment in case.stderr_fragments {
        assert!(
            stderr.contains(fragment),
            "{} stderr missing {fragment:?}:\n{stderr}",
            case.name
        );
    }
}

fn repo_root() -> std::path::PathBuf {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .ancestors()
        .nth(4)
        .expect("crate is under workspaces/worth-store/crates")
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
    assert!(status.success(), "failed to build Store fixture deps");
}

fn run_compile_fail_case(_repo_root: &std::path::Path, fixture_name: &str) -> std::process::Output {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let store_deps = cargo_artifacts::dependency_dir();
    let fixture_path = manifest_dir
        .join("tests")
        .join("compile_fail")
        .join("scheduling")
        .join("evidence_materialization")
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
        "worth_foundational",
        "worth_store_certification",
        "worth_store_readiness",
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

fn rlib_path(_store_deps: &std::path::Path, crate_name: &str) -> std::path::PathBuf {
    cargo_artifacts::compiled_extern(TEST_TARGET, crate_name)
}

fn manifest_path(path: &std::path::Path) -> String {
    path.display().to_string().replace('\\', "/")
}
