#[test]
fn bootstrap_catalog_surfaces_reject_raw_struct_shortcuts() {
    for fixture in fixtures() {
        assert_compile_fails(fixture);
    }
}

#[derive(Debug, Clone, Copy)]
struct CompileFailFixture {
    name: &'static str,
    expected_stderr: &'static [&'static str],
}

const fn fixtures() -> [CompileFailFixture; 3] {
    [
        CompileFailFixture {
            name: "bootstrap_catalog_struct_literal_is_not_public.rs",
            expected_stderr: &["private", "S8BootstrapLayoutCatalog"],
        },
        CompileFailFixture {
            name: "bootstrap_catalog_read_admission_struct_literal_is_not_public.rs",
            expected_stderr: &["private", "S8BootstrapCatalogReadAdmission"],
        },
        CompileFailFixture {
            name: "raw_persisted_layout_cannot_reopen_bootstrap_lane.rs",
            expected_stderr: &["PlatformPhysicalReplayArtifact", "mismatched types"],
        },
    ]
}

fn assert_compile_fails(fixture: CompileFailFixture) {
    let output =
        std::process::Command::new(std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into()))
            .arg("--crate-name")
            .arg("forge_store_layout_indexes_phase18_ui")
            .arg("--edition=2021")
            .arg("--emit=metadata")
            .arg("--out-dir")
            .arg(std::env::temp_dir())
            .arg("-L")
            .arg(format!(
                "dependency={}",
                compiled_dependency_dir().display()
            ))
            .arg("--extern")
            .arg(format!(
                "forge_store_layout_indexes={}",
                compiled_extern("forge_store_layout_indexes").display()
            ))
            .arg("--extern")
            .arg(format!(
                "forge_store_physical_format={}",
                compiled_extern("forge_store_physical_format").display()
            ))
            .arg(
                std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join("tests")
                    .join("ui")
                    .join("phase18")
                    .join(fixture.name),
            )
            .output()
            .unwrap();

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

fn compiled_dependency_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("layout-indexes crate lives under workspaces/forge-store/crates")
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
