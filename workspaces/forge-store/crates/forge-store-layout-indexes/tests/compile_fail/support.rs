#[derive(Debug)]
pub struct CompileFailCasePaths {
    pub manifest_path: std::path::PathBuf,
    pub source_path: std::path::PathBuf,
}

pub fn assert_compile_fails(fixture_name: &str, expected_stderr: &[&str], extern_crates: &[&str]) {
    assert_compile_fails_in_ui_dir("foundations", fixture_name, expected_stderr, extern_crates);
}

pub fn assert_compile_fails_in_ui_dir(
    ui_dir: &str,
    fixture_name: &str,
    expected_stderr: &[&str],
    extern_crates: &[&str],
) {
    let case_paths = prepare_compile_fail_case_in_ui_dir(ui_dir, fixture_name, extern_crates);
    let output = run_compile_fail_case(
        &case_paths.source_path,
        extern_crates,
        &case_paths.manifest_path,
    );

    assert!(
        !output.status.success(),
        "{ui_dir}/{fixture_name} unexpectedly compiled"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_intended_compile_failure(ui_dir, fixture_name, &stderr);
    for expected in expected_stderr {
        assert!(
            stderr.contains(expected),
            "{ui_dir}/{fixture_name} failed for the wrong reason; missing stderr fragment {expected:?}\nstderr:\n{stderr}",
        );
    }
}

pub fn compiled_dependency_dir() -> std::path::PathBuf {
    store_workspace_root(&std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")))
        .join("target")
        .join("layout-indexes-ui-dependencies")
        .join("debug")
        .join("deps")
}

pub fn compiled_extern(crate_name: &str) -> std::path::PathBuf {
    ensure_compiled_extern(crate_name);
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

fn ensure_compiled_extern(crate_name: &str) {
    static BUILD_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let _guard = BUILD_LOCK.lock().unwrap();
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = store_workspace_root(&manifest_dir);
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let mut command = std::process::Command::new(cargo);
    command
        .arg("build")
        .arg("--quiet")
        .arg("--manifest-path")
        .arg(workspace_root.join("Cargo.toml"))
        .arg("--target-dir")
        .arg(
            workspace_root
                .join("target")
                .join("layout-indexes-ui-dependencies"),
        )
        .arg("-p")
        .arg("forge-store-layout-indexes");
    if crate_name != "forge_store_layout_indexes" {
        command.arg("-p").arg(crate_name.replace('_', "-"));
    }
    let status = command.status().unwrap();
    assert!(
        status.success(),
        "failed to build UI dependency {crate_name}"
    );
}

fn prepare_compile_fail_case(fixture_name: &str, extern_crates: &[&str]) -> CompileFailCasePaths {
    prepare_compile_fail_case_in_ui_dir("foundations", fixture_name, extern_crates)
}

fn prepare_compile_fail_case_in_ui_dir(
    ui_dir: &str,
    fixture_name: &str,
    extern_crates: &[&str],
) -> CompileFailCasePaths {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let case_dir = store_workspace_root(&manifest_dir)
        .join("target")
        .join(format!("layout-indexes-{ui_dir}-ui"))
        .join(std::process::id().to_string())
        .join("cases")
        .join(fixture_name.trim_end_matches(".rs"));
    std::fs::create_dir_all(&case_dir).unwrap();
    let source_dir = case_dir.join("src");
    std::fs::create_dir_all(&source_dir).unwrap();
    let source_path = source_dir.join("main.rs");
    std::fs::copy(
        manifest_dir
            .join("tests")
            .join("compile_fail")
            .join("layout")
            .join(ui_dir)
            .join(fixture_name),
        &source_path,
    )
    .unwrap();

    let manifest_path = case_dir.join("Cargo.toml");
    std::fs::write(
        &manifest_path,
        compile_fail_manifest_contents(extern_crates, &manifest_dir),
    )
    .unwrap();
    CompileFailCasePaths {
        manifest_path,
        source_path,
    }
}

fn compile_fail_manifest_contents(
    extern_crates: &[&str],
    manifest_dir: &std::path::Path,
) -> String {
    let mut dependencies = vec![manifest_dependency_entry(
        "forge_store_layout_indexes",
        &manifest_dir.to_path_buf(),
    )];
    for crate_name in extern_crates {
        dependencies.push(manifest_dependency_entry(
            crate_name,
            &dependency_path(crate_name, manifest_dir),
        ));
    }

    format!(
        "[package]\nname = \"layout-compile-fail-case\"\nversion = \"0.1.0\"\nedition = \"2021\"\nworkspace = \"{}\"\n\n[dependencies]\n{}\n",
        store_workspace_root(manifest_dir)
            .display()
            .to_string()
            .replace('\\', "/"),
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
    source_path: &std::path::Path,
    extern_crates: &[&str],
    manifest_path: &std::path::Path,
) -> std::process::Output {
    if fixture_uses_direct_rustc() {
        return run_compile_fail_case_with_rustc(source_path, extern_crates);
    }

    run_compile_fail_case_with_cargo(manifest_path)
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
                .join("layout-indexes-compile-fail")
                .join("target"),
        );
    command.output().unwrap()
}

fn run_compile_fail_case_with_rustc(
    source_path: &std::path::Path,
    extern_crates: &[&str],
) -> std::process::Output {
    let rustc = std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    let deps_dir = compiled_dependency_dir();
    let mut command = std::process::Command::new(rustc);
    command
        .arg("--crate-name")
        .arg("forge_store_layout_indexes_compile_fail")
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

    for crate_name in extern_crates {
        command.arg("--extern").arg(format!(
            "{crate_name}={}",
            compiled_extern(crate_name).display()
        ));
    }

    command.arg(source_path).output().unwrap()
}

fn assert_intended_compile_failure(ui_dir: &str, fixture_name: &str, stderr: &str) {
    for environmental_failure in [
        "E0463",
        "E0560",
        "E0583",
        "E0609",
        "can't find crate",
        "couldn't read",
        "No such file or directory",
        "failed to get `",
        "failed to load source for dependency",
    ] {
        assert!(
            !stderr.contains(environmental_failure),
            "{ui_dir}/{fixture_name} failed for an environmental reason {environmental_failure:?}\nstderr:\n{stderr}",
        );
    }
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
