#[test]
fn phase_9_handoffs_reject_lower_authority_substitution() {
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

fn fixtures() -> [Fixture; 11] {
    [
        Fixture {
            name: "raw_declaration_cannot_satisfy_s6_handoff.rs",
            expected: &[
                "S6IoQosSecurityScopeHandoff",
                "from_s5_1_readiness",
                "S51AdmittedSecurityScopeReadiness",
                "StoreRawSecurityScopeDeclaration",
            ],
        },
        Fixture {
            name: "receipt_id_cannot_satisfy_s11_handoff.rs",
            expected: &[
                "S51SecurityFoundationHandoff",
                "from_s5_1_readiness",
                "S51AdmittedSecurityScopeReadiness",
                "StoreSecurityScopeAdmissionReceiptId",
            ],
        },
        Fixture {
            name: "copied_counters_cannot_satisfy_s7_handoff.rs",
            expected: &[
                "S7BlobChunkSecurityHandoff",
                "from_s5_1_readiness",
                "S51AdmittedSecurityScopeReadiness",
                "StoreSecurityScopeAdmissionCounterSnapshot",
            ],
        },
        Fixture {
            name: "copied_proof_progression_identity_cannot_satisfy_s11_handoff.rs",
            expected: &[
                "S51SecurityFoundationHandoff",
                "from_s5_1_readiness",
                "S51AdmittedSecurityScopeReadiness",
                "StoreSecurityScopeProofProgressionIdentity",
            ],
        },
        Fixture {
            name: "terminal_projection_cannot_satisfy_repair_handoff.rs",
            expected: &[
                "S10RepairBlastRadiusHandoff",
                "from_repair_blast_radius_readiness",
                "RepairBlastRadiusReadiness",
                "StoreTerminalProjectionText",
            ],
        },
        Fixture {
            name: "certification_row_cannot_satisfy_security_foundation_handoff.rs",
            expected: &[
                "S51SecurityFoundationHandoff",
                "from_s5_1_readiness",
                "S51AdmittedSecurityScopeReadiness",
                "RecoveryPhysicsCertificationRow",
            ],
        },
        Fixture {
            name: "repair_handoff_cannot_satisfy_backup_handoff_api.rs",
            expected: &[
                "S10BackupExportCustodyHandoff",
                "S10RepairBlastRadiusHandoff",
            ],
        },
        Fixture {
            name: "s7_handoff_cannot_satisfy_s6_handoff_api.rs",
            expected: &["S6IoQosSecurityScopeHandoff", "S7BlobChunkSecurityHandoff"],
        },
        Fixture {
            name: "s7_handoff_cannot_satisfy_backup_handoff_api.rs",
            expected: &[
                "S10BackupExportCustodyHandoff",
                "S7BlobChunkSecurityHandoff",
            ],
        },
        Fixture {
            name: "s7_handoff_cannot_satisfy_repair_handoff_api.rs",
            expected: &["S10RepairBlastRadiusHandoff", "S7BlobChunkSecurityHandoff"],
        },
        Fixture {
            name: "s7_handoff_cannot_satisfy_security_foundation_handoff_api.rs",
            expected: &["S51SecurityFoundationHandoff", "S7BlobChunkSecurityHandoff"],
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
        .arg("worth-store-aspect-native")
        .arg("-p")
        .arg("worth-store-blob-chunks")
        .arg("-p")
        .arg("worth-store-certification")
        .arg("-p")
        .arg("worth-store-io-scheduler")
        .arg("-p")
        .arg("worth-store-operations")
        .arg("-p")
        .arg("worth-store-readiness")
        .arg("-p")
        .arg("worth-store-security")
        .arg("--manifest-path")
        .arg(
            repo_root
                .join("workspaces")
                .join("worth-store")
                .join("Cargo.toml"),
        )
        .status()
        .unwrap();
    assert!(status.success(), "failed to build handoff fixture deps");
}

fn run_compile_fail_case(repo_root: &std::path::Path, fixture_name: &str) -> std::process::Output {
    let source = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("ui")
        .join("s5_1_later_milestone_handoff")
        .join(fixture_name);
    let deps = repo_root
        .join("workspaces")
        .join("worth-store")
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
        .arg(std::env::temp_dir().join("worth-store-s51-handoff-ui"))
        .output()
        .unwrap()
}

fn extern_args(deps: &std::path::Path) -> Vec<std::ffi::OsString> {
    [
        "worth_store_aspect_native",
        "worth_store_blob_chunks",
        "worth_store_certification",
        "worth_store_io_scheduler",
        "worth_store_operations",
        "worth_store_readiness",
        "worth_store_security",
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
