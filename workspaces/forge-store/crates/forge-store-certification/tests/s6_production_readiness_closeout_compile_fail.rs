#[test]
fn phase_14_closeout_boundaries_reject_public_forgery() {
    let repo_root = repo_root();
    build_compile_fail_dependencies(&repo_root);
    for case in compile_fail_cases() {
        assert_compile_fails(&repo_root, case);
    }
}

struct CompileFailCase {
    name: &'static str,
    stderr_fragments: &'static [&'static str],
    allow_unresolved_import: bool,
}

fn compile_fail_cases() -> Vec<CompileFailCase> {
    vec![
        live_case(
            "closeout_fields_cannot_be_minted.rs",
            &["S6ProductionReadinessClosure", "private"],
        ),
        live_case(
            "foundational_receipt_cannot_close_s6.rs",
            &["expected `S6CertificationEvidenceAdoptionReceipt`"],
        ),
        live_case(
            "copied_counter_rows_cannot_close_s6.rs",
            &["expected `S6CertificationEvidenceAdoptionReceipt`"],
        ),
        live_case(
            "proof_summary_cannot_close_s6.rs",
            &["expected `S6CertificationEvidenceAdoptionReceipt`"],
        ),
        live_case(
            "terminal_projection_cannot_close_s6.rs",
            &["expected `S6CertificationEvidenceAdoptionReceipt`"],
        ),
        live_case(
            "public_residual_debt_rows_cannot_be_attached_to_closeout.rs",
            &["no method named `with_residual_debt`"],
        ),
        live_case(
            "public_non_claim_counts_cannot_be_attached_to_closeout.rs",
            &["no method named `with_later_milestone_non_claims`"],
        ),
        live_case(
            "public_proof_constructor_cannot_mint_closeout_proof.rs",
            &["from_phase13_adoption", "private"],
        ),
        legacy_removed_case(
            "public_closeout_trait_cannot_be_implemented.rs",
            &["S6MaterializedCertificationCloseoutEvidence"],
        ),
        legacy_removed_case(
            "public_closeout_source_cannot_be_constructed.rs",
            &["S6MaterializedCertificationCloseoutSource"],
        ),
        live_case(
            "public_adoption_receipt_cannot_be_constructed.rs",
            &["from_executed_store_law_evidence"],
        ),
        live_case(
            "certification_adoption_receipt_constructor_is_private.rs",
            &["from_materialized_bundle_evidence", "private"],
        ),
    ]
}

fn live_case(name: &'static str, stderr_fragments: &'static [&'static str]) -> CompileFailCase {
    CompileFailCase {
        name,
        stderr_fragments,
        allow_unresolved_import: false,
    }
}

fn legacy_removed_case(
    name: &'static str,
    stderr_fragments: &'static [&'static str],
) -> CompileFailCase {
    CompileFailCase {
        name,
        stderr_fragments,
        allow_unresolved_import: true,
    }
}

fn assert_compile_fails(repo_root: &std::path::Path, case: CompileFailCase) {
    let output = run_compile_fail_case(repo_root, case.name);
    assert!(
        !output.status.success(),
        "{} unexpectedly compiled",
        case.name
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !case.allow_unresolved_import {
        assert!(
            !stderr.contains("unresolved import"),
            "{} failed through stale imports instead of live API misuse:\n{stderr}",
            case.name
        );
    }
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
        .expect("crate is under workspaces/forge-store/crates")
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
        .join("ui")
        .join("s6_production_readiness_closeout")
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
        "forge_foundational",
        "forge_store_certification",
        "forge_store_readiness",
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

fn rlib_path(store_deps: &std::path::Path, crate_name: &str) -> std::path::PathBuf {
    let prefix = format!("lib{crate_name}-");
    std::fs::read_dir(store_deps)
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
        .unwrap_or_else(|| panic!("missing rlib for {crate_name} in {}", store_deps.display()))
}

fn manifest_path(path: &std::path::Path) -> String {
    path.display().to_string().replace('\\', "/")
}
