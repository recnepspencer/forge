#[test]
fn authority_projection_readmission_denies_lower_authority_inputs() {
    for fixture in authority_projection_readmission_fixtures() {
        assert_compile_fails(fixture);
    }
}

#[derive(Debug, Clone, Copy)]
struct AuthorityProjectionReadmissionFixture {
    name: &'static str,
    expected_stderr: &'static [&'static str],
}

fn authority_projection_readmission_fixtures() -> Vec<AuthorityProjectionReadmissionFixture> {
    vec![
        AuthorityProjectionReadmissionFixture {
            name: "digest_string_cannot_satisfy_current_authority.rs",
            expected_stderr: &["StoreCurrentAuthorityWitness", "String"],
        },
        AuthorityProjectionReadmissionFixture {
            name: "derived_evidence_cannot_satisfy_current_authority.rs",
            expected_stderr: &[
                "StoreCurrentAuthorityWitness",
                "StoreDerivedAuthorityEvidence",
            ],
        },
        AuthorityProjectionReadmissionFixture {
            name: "external_token_cannot_satisfy_current_authority.rs",
            expected_stderr: &[
                "StoreCurrentAuthorityWitness",
                "StoreExternalAuthorityToken",
            ],
        },
        AuthorityProjectionReadmissionFixture {
            name: "filename_cannot_satisfy_current_authority.rs",
            expected_stderr: &["StoreCurrentAuthorityWitness", "StoreAuthorityFilename"],
        },
        AuthorityProjectionReadmissionFixture {
            name: "retained_evidence_cannot_satisfy_current_physical_authority.rs",
            expected_stderr: &[
                "StoreCurrentPhysicalAuthorityWitness",
                "StoreRetainedAuthorityEvidence",
            ],
        },
        AuthorityProjectionReadmissionFixture {
            name: "stable_id_cannot_construct_canonical_authority_record.rs",
            expected_stderr: &["CanonicalAuthorityRecord", "new"],
        },
        AuthorityProjectionReadmissionFixture {
            name: "stable_id_cannot_satisfy_canonical_authority_witness.rs",
            expected_stderr: &["StoreCurrentAuthorityWitness", "StableArtifactId"],
        },
        AuthorityProjectionReadmissionFixture {
            name: "terminal_projection_text_cannot_satisfy_current_authority.rs",
            expected_stderr: &[
                "StoreCurrentAuthorityWitness",
                "StoreTerminalProjectionText",
            ],
        },
    ]
}

fn assert_compile_fails(fixture: AuthorityProjectionReadmissionFixture) {
    let case_dir = prepare_compile_fail_case(fixture.name);
    let output = run_compile_fail_case(&case_dir);

    assert!(
        !output.status.success(),
        "{} unexpectedly compiled successfully",
        fixture.name
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    for expected in fixture.expected_stderr {
        assert!(
            stderr.contains(expected),
            "{} failed for the wrong reason; missing stderr fragment {expected:?}\nstderr:\n{stderr}",
            fixture.name
        );
    }
}

fn prepare_compile_fail_case(fixture_name: &str) -> std::path::PathBuf {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir
        .ancestors()
        .nth(4)
        .expect("certification crate lives under workspaces/forge-store/crates");
    let fixture_path = authority_projection_readmission_fixture_path(&manifest_dir, fixture_name);
    let case_dir =
        authority_projection_readmission_compile_fail_case_dir(&manifest_dir, fixture_name);
    let source_dir = case_dir.join("src");
    std::fs::create_dir_all(&source_dir).unwrap();
    std::fs::copy(&fixture_path, source_dir.join("main.rs")).unwrap();
    std::fs::write(case_dir.join("Cargo.toml"), fixture_manifest(repo_root)).unwrap();

    case_dir
}

fn run_compile_fail_case(case_dir: &std::path::Path) -> std::process::Output {
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    std::process::Command::new(cargo)
        .arg("check")
        .arg("--quiet")
        .arg("--manifest-path")
        .arg(case_dir.join("Cargo.toml"))
        .env("CARGO_TARGET_DIR", compile_fail_case_target_dir(case_dir))
        .output()
        .unwrap()
}

fn authority_projection_readmission_fixture_path(
    manifest_dir: &std::path::Path,
    fixture_name: &str,
) -> std::path::PathBuf {
    manifest_dir
        .join("tests")
        .join("ui")
        .join("authority_projection_readmission")
        .join(fixture_name)
}

fn authority_projection_readmission_compile_fail_case_dir(
    manifest_dir: &std::path::Path,
    fixture_name: &str,
) -> std::path::PathBuf {
    let _ = manifest_dir;
    std::env::temp_dir()
        .join("forge-store-authority-ui")
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
        .join("authority_projection_readmission_ui")
        .join("target")
}

fn fixture_manifest(repo_root: &std::path::Path) -> String {
    format!(
        "[package]\nname = \"authority_projection_readmission_ui\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[workspace]\n\n[dependencies]\nforge-store-aspect-native = {{ path = \"{}\" }}\nforge-store-authority = {{ path = \"{}\" }}\nforge-store-contracts = {{ path = \"{}\" }}\n",
        manifest_path(&repo_root.join("workspaces").join("forge-store").join("crates").join(
            "forge-store-aspect-native"
        )),
        manifest_path(
            &repo_root
                .join("workspaces")
                .join("forge-store")
                .join("crates")
                .join("forge-store-authority")
        ),
        manifest_path(
            &repo_root
                .join("workspaces")
                .join("forge-store")
                .join("crates")
                .join("forge-store-contracts")
        ),
    )
}

fn manifest_path(path: &std::path::Path) -> String {
    path.display().to_string().replace('\\', "/")
}
