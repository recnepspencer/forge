#[test]
fn phase_zero_public_boundary_denies_forged_or_weaker_authority() {
    for fixture in compile_fail_fixtures() {
        assert_compile_fails(fixture);
    }
}

#[derive(Debug, Clone, Copy)]
struct CompileFailFixture {
    name: &'static str,
    expected_stderr: &'static [&'static str],
    extern_crates: &'static [&'static str],
}

fn compile_fail_fixtures() -> [CompileFailFixture; 13] {
    [
        fixture(
            "raw_struct_cannot_construct_admitted_layout_strategy.rs",
            &["private associated function", "new"],
            &[],
        ),
        fixture(
            "raw_struct_cannot_construct_phase_obligation.rs",
            &["private field", "S8PhaseSkeletonObligationRow"],
            &[],
        ),
        fixture(
            "deep_import_internal_skeleton_is_unavailable.rs",
            &["private module", "skeleton"],
            &[],
        ),
        fixture(
            "certification_closeout_cannot_satisfy_executed_evidence.rs",
            &["S8ExecutedAccessEvidence"],
            &["forge_store_certification"],
        ),
        fixture(
            "physical_certification_harness_cannot_satisfy_executed_evidence.rs",
            &["S8ExecutedAccessEvidence"],
            &["forge_store_physical_certification"],
        ),
        fixture(
            "test_support_fixture_cannot_satisfy_executed_evidence.rs",
            &["S8ExecutedAccessEvidence"],
            &["forge_store_test_support"],
        ),
        fixture(
            "offline_report_cannot_satisfy_readmission_witness.rs",
            &["S8LayoutReadmissionWitness", "OfflineLayoutReport"],
            &["forge_store_offline_verifier"],
        ),
        fixture(
            "foundational_materialized_report_cannot_satisfy_readmission_witness.rs",
            &[
                "S8LayoutReadmissionWitness",
                "FoundationalMaterializedPerformanceReport",
            ],
            &["forge_foundational"],
        ),
        fixture(
            "copied_counter_rows_cannot_satisfy_planned_vs_observed.rs",
            &[
                "S8PlannedVsObservedCounterReceipt",
                "FoundationalPerformanceCounterRow",
            ],
            &["forge_foundational"],
        ),
        fixture(
            "terminal_projection_fixture_cannot_satisfy_readmission_witness.rs",
            &[
                "S8LayoutReadmissionWitness",
                "StoreTerminalProjectionJsonFixture",
            ],
            &["forge_store_test_support"],
        ),
        fixture(
            "certification_helper_surface_is_not_public.rs",
            &["certification_test_authority"],
            &[],
        ),
        fixture(
            "strategy_declaration_surface_is_not_public.rs",
            &["S8StrategyDeclaration", "S8StrategyCapability"],
            &[],
        ),
        fixture(
            "generic_execution_surface_is_not_public.rs",
            &[
                "access_execution",
                "S8ExecutedAccessEvidence",
                "S8ExecutionReadyAccessPlan",
                "S8LoweredAccessPlan",
            ],
            &[],
        ),
    ]
}

const fn fixture(
    name: &'static str,
    expected_stderr: &'static [&'static str],
    extern_crates: &'static [&'static str],
) -> CompileFailFixture {
    CompileFailFixture {
        name,
        expected_stderr,
        extern_crates,
    }
}

fn assert_compile_fails(fixture: CompileFailFixture) {
    let case_paths = prepare_compile_fail_case(fixture);
    let output = run_compile_fail_case(fixture, &case_paths);

    assert!(
        !output.status.success(),
        "{} unexpectedly compiled",
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

#[derive(Debug)]
struct CompileFailCasePaths {
    manifest_path: std::path::PathBuf,
    source_path: std::path::PathBuf,
}

fn prepare_compile_fail_case(fixture: CompileFailFixture) -> CompileFailCasePaths {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let case_dir = std::env::temp_dir()
        .join("layout-indexes-phase0-ui")
        .join(std::process::id().to_string())
        .join("cases")
        .join(fixture.name.trim_end_matches(".rs"));
    std::fs::create_dir_all(&case_dir).unwrap();
    let source_dir = case_dir.join("src");
    std::fs::create_dir_all(&source_dir).unwrap();
    let source_path = source_dir.join("main.rs");
    std::fs::copy(
        manifest_dir
            .join("tests")
            .join("ui")
            .join("phase0")
            .join(fixture.name),
        &source_path,
    )
    .unwrap();

    let manifest_path = case_dir.join("Cargo.toml");
    std::fs::write(
        &manifest_path,
        compile_fail_manifest_contents(fixture, &manifest_dir),
    )
    .unwrap();
    CompileFailCasePaths {
        manifest_path,
        source_path,
    }
}

fn compile_fail_manifest_contents(
    fixture: CompileFailFixture,
    manifest_dir: &std::path::Path,
) -> String {
    let mut dependencies = vec![manifest_dependency_entry(
        "forge_store_layout_indexes",
        &manifest_dir.to_path_buf(),
    )];
    for crate_name in fixture.extern_crates {
        dependencies.push(manifest_dependency_entry(
            crate_name,
            &dependency_path(crate_name, manifest_dir),
        ));
    }

    format!(
        "[package]\nname = \"phase0-ui-case\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\n{}\n",
        dependencies.join("\n")
    )
}

fn manifest_dependency_entry(crate_name: &str, path: &std::path::Path) -> String {
    format!(
        "{} = {{ path = \"{}\" }}",
        crate_name.replace('_', "-"),
        path.display().to_string().replace('\\', "/")
    )
}

fn dependency_path(crate_name: &str, manifest_dir: &std::path::Path) -> std::path::PathBuf {
    match crate_name {
        "forge_foundational" => repository_root(manifest_dir)
            .join("crates")
            .join("forge-foundational"),
        _ => store_workspace_root(manifest_dir)
            .join("crates")
            .join(crate_name.replace('_', "-")),
    }
}

fn run_compile_fail_case(
    fixture: CompileFailFixture,
    case_paths: &CompileFailCasePaths,
) -> std::process::Output {
    if fixture_uses_direct_rustc() {
        return run_compile_fail_case_with_rustc(fixture, &case_paths.source_path);
    }

    run_compile_fail_case_with_cargo(&case_paths.manifest_path)
}

const fn fixture_uses_direct_rustc() -> bool {
    true
}

fn run_compile_fail_case_with_cargo(manifest_path: &std::path::Path) -> std::process::Output {
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let mut command = std::process::Command::new(cargo);
    command
        .arg("check")
        .arg("--quiet")
        .arg("--manifest-path")
        .arg(manifest_path)
        .arg("--target-dir")
        .arg(
            std::env::temp_dir()
                .join("layout-indexes-phase0-ui")
                .join("target"),
        );
    command.output().unwrap()
}

fn run_compile_fail_case_with_rustc(
    fixture: CompileFailFixture,
    source_path: &std::path::Path,
) -> std::process::Output {
    let rustc = std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    let deps_dir = compiled_dependency_dir();
    let mut command = std::process::Command::new(rustc);
    command
        .arg("--crate-name")
        .arg("forge_store_layout_indexes_phase0_ui")
        .arg("--edition=2021")
        .arg("--emit=metadata")
        .arg("--out-dir")
        .arg(source_path.parent().unwrap())
        .arg("-L")
        .arg(format!("dependency={}", deps_dir.display()))
        .arg("--extern")
        .arg(format!(
            "forge_store_layout_indexes={}",
            compiled_extern("forge_store_layout_indexes").display()
        ));

    for crate_name in fixture.extern_crates {
        command.arg("--extern").arg(format!(
            "{crate_name}={}",
            compiled_extern(crate_name).display()
        ));
    }

    command.arg(source_path).output().unwrap()
}

fn compiled_dependency_dir() -> std::path::PathBuf {
    store_workspace_root(&std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")))
        .join("target")
        .join("debug")
        .join("deps")
}

fn compiled_extern(crate_name: &str) -> std::path::PathBuf {
    let crate_prefix = format!("{crate_name}-");
    let lib_prefix = format!("lib{crate_name}-");
    let mut matches = std::fs::read_dir(compiled_dependency_dir())
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            path.extension()
                .is_some_and(|ext| ext == "rmeta" || ext == "rlib")
                && path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| {
                        name.starts_with(&crate_prefix) || name.starts_with(&lib_prefix)
                    })
        })
        .collect::<Vec<_>>();

    matches.sort_by_key(|path| {
        std::fs::metadata(path)
            .and_then(|metadata| metadata.modified())
            .ok()
    });

    matches
        .iter()
        .rev()
        .find(|path| path.extension().is_some_and(|ext| ext == "rlib"))
        .cloned()
        .or_else(|| {
            matches
                .iter()
                .rev()
                .find(|path| path.extension().is_some_and(|ext| ext == "rmeta"))
                .cloned()
        })
        .unwrap_or_else(|| panic!("missing compiled extern for {crate_name}"))
}

fn store_workspace_root(manifest_dir: &std::path::Path) -> &std::path::Path {
    manifest_dir
        .ancestors()
        .nth(2)
        .expect("layout-indexes crate lives under workspaces/forge-store/crates")
}

fn repository_root(manifest_dir: &std::path::Path) -> &std::path::Path {
    manifest_dir
        .ancestors()
        .nth(4)
        .expect("forge repository root sits above workspaces/forge-store/crates")
}
