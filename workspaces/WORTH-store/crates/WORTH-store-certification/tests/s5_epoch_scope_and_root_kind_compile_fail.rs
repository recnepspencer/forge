use worth_store_physical_isolation as _;

#[test]
fn epoch_scope_and_root_kind_misuse_does_not_compile() {
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
            name: "root_epoch_cannot_be_compared_to_page_epoch.rs",
            expected_stderr: &["RootEpoch", "PageEpoch"],
        },
        CompileFailFixture {
            name: "page_epoch_cannot_be_compared_directly.rs",
            expected_stderr: &["PageEpoch", "PartialEq"],
        },
        CompileFailFixture {
            name: "physical_epoch_vector_cannot_be_compared_directly.rs",
            expected_stderr: &["PhysicalEpochVector", "PartialEq"],
        },
        CompileFailFixture {
            name: "checkpoint_root_cannot_admit_stable_read_plan.rs",
            expected_stderr: &["CurrentPhysicalRoot", "CheckpointPublicationRoot"],
        },
        CompileFailFixture {
            name: "raw_page_id_cannot_be_generation_counted_reference.rs",
            expected_stderr: &["GenerationCountedPhysicalReference", "PhysicalPageId"],
        },
        CompileFailFixture {
            name: "root_epoch_cannot_be_publicly_constructed.rs",
            expected_stderr: &["RootEpoch", "private"],
        },
        CompileFailFixture {
            name: "generation_reference_cannot_mint_page_epoch.rs",
            expected_stderr: &["page_epoch", "GenerationCountedPhysicalReference"],
        },
        CompileFailFixture {
            name: "checkpoint_root_requires_checkpoint_basis.rs",
            expected_stderr: &["CheckpointPublicationRootBasis", "RootEpoch"],
        },
        CompileFailFixture {
            name: "manifest_locator_requires_locator_basis.rs",
            expected_stderr: &["ManifestLocatorRootBasis", "RootEpoch"],
        },
        CompileFailFixture {
            name: "raw_publication_ordinal_cannot_mint_page_epoch.rs",
            expected_stderr: &["page_epoch_for_publication", "CurrentPhysicalRoot"],
        },
        CompileFailFixture {
            name: "raw_generation_reference_cannot_admit_publication_epoch.rs",
            expected_stderr: &[
                "CurrentGenerationPhysicalReference",
                "GenerationCountedPhysicalReference",
            ],
        },
    ]
}

fn assert_compile_fails(fixture: CompileFailFixture) {
    let case_dir = prepare_compile_fail_case(fixture.name);
    let output = run_compile_fail_case(&case_dir);
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
    let source_dir = case_dir.join("src");
    if case_dir.exists() {
        std::fs::remove_dir_all(&case_dir).unwrap();
    }
    std::fs::create_dir_all(&source_dir).unwrap();
    write_compile_fail_manifest(&case_dir);
    std::fs::copy(
        manifest_dir
            .join("tests")
            .join("ui")
            .join("s5_epoch_scope_and_root_kind")
            .join(fixture_name),
        source_dir.join("main.rs"),
    )
    .unwrap();
    case_dir
}

fn run_compile_fail_case(case_dir: &std::path::Path) -> std::process::Output {
    std::process::Command::new("cargo")
        .arg("check")
        .arg("--quiet")
        .arg("--manifest-path")
        .arg(case_dir.join("Cargo.toml"))
        .env("CARGO_TARGET_DIR", compile_fail_case_target_dir(case_dir))
        .output()
        .unwrap()
}

fn compile_fail_case_dir(fixture_name: &str) -> std::path::PathBuf {
    std::env::temp_dir()
        .join("worth-store-s5-epoch-scope-ui")
        .join(std::process::id().to_string())
        .join("cases")
        .join(fixture_name.trim_end_matches(".rs"))
}

fn write_compile_fail_manifest(case_dir: &std::path::Path) {
    std::fs::write(
        case_dir.join("Cargo.toml"),
        format!(
            "[package]\nname = \"s5-epoch-scope-ui\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\n[dependencies]\nworth-store-physical-isolation = {{ path = {:?} }}\nworth-store-physical-format = {{ path = {:?} }}\n",
            physical_isolation_crate_dir(),
            physical_format_crate_dir(),
        ),
    )
    .unwrap();
}

fn compile_fail_case_target_dir(case_dir: &std::path::Path) -> std::path::PathBuf {
    case_dir.join("target")
}

fn physical_isolation_crate_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("worth-store-physical-isolation")
}

fn physical_format_crate_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("worth-store-physical-format")
}
