#[path = "cargo_artifact_message.rs"]
mod cargo_artifact_message;

pub fn assert_compile_fails(fixture_name: &str, expected_stderr: &[&str], extern_crates: &[&str]) {
    assert_compile_fails_in_ui_dir("foundations", fixture_name, expected_stderr, extern_crates);
}

pub fn assert_compile_fails_in_ui_dir(
    ui_dir: &str,
    fixture_name: &str,
    expected_stderr: &[&str],
    extern_crates: &[&str],
) {
    // Cargo-backed fixtures can update dependency artifacts while raw rustc
    // fixtures resolve their externs. Keep the proof runs atomic so a fixture
    // can fail only for the boundary it is intended to exercise.
    let _compile_guard = compile_fail_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let source_path = fixture_path(ui_dir, fixture_name);
    let output = run_compile_fail_case(&source_path, ui_dir, fixture_name, extern_crates);

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

fn compile_fail_lock() -> &'static std::sync::Mutex<()> {
    static LOCK: std::sync::OnceLock<std::sync::Mutex<()>> = std::sync::OnceLock::new();
    LOCK.get_or_init(|| std::sync::Mutex::new(()))
}

fn fixture_path(ui_dir: &str, fixture_name: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("compile_fail")
        .join("layout")
        .join(ui_dir)
        .join(fixture_name)
}

fn run_compile_fail_case(
    source_path: &std::path::Path,
    ui_dir: &str,
    fixture_name: &str,
    extern_crates: &[&str],
) -> std::process::Output {
    let output_dir = std::env::temp_dir()
        .join(format!("layout-indexes-{ui_dir}-ui"))
        .join(std::process::id().to_string())
        .join(fixture_name.trim_end_matches(".rs"));
    std::fs::create_dir_all(&output_dir).unwrap();

    let dependencies = compiled_dependency_dir();
    let rustc = std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    let mut command = std::process::Command::new(rustc);
    command
        .arg("--crate-name")
        .arg("forge_store_layout_indexes_compile_fail")
        .arg("--edition=2021")
        .arg("--emit=metadata")
        .arg("--out-dir")
        .arg(&output_dir)
        .arg("-L")
        .arg(format!("dependency={}", dependencies.display()))
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

fn compiled_dependency_dir() -> std::path::PathBuf {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = store_workspace_root(&manifest_dir);
    let target_root = std::env::var_os("CARGO_TARGET_DIR")
        .map(std::path::PathBuf::from)
        .map(|path| {
            if path.is_absolute() {
                path
            } else {
                workspace_root.join(path)
            }
        })
        .unwrap_or_else(|| workspace_root.join("target"));
    target_root.join("debug").join("deps")
}

fn compiled_extern(crate_name: &str) -> std::path::PathBuf {
    compiled_artifact_graph()
        .get(crate_name)
        .cloned()
        .unwrap_or_else(|| panic!("Cargo did not emit a compiled extern for {crate_name}"))
}

fn compiled_artifact_graph() -> &'static std::collections::HashMap<String, std::path::PathBuf> {
    static GRAPH: std::sync::OnceLock<std::collections::HashMap<String, std::path::PathBuf>> =
        std::sync::OnceLock::new();

    GRAPH.get_or_init(|| {
        let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let workspace_root = store_workspace_root(&manifest_dir);
        let output =
            std::process::Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
                .arg("check")
                .arg("--offline")
                .arg("--message-format=json")
                .arg("-p")
                .arg("forge-store-layout-indexes")
                .arg("--test")
                .arg("layout_scenarios")
                .current_dir(workspace_root)
                .output()
                .expect("Cargo must be available for compile-fail artifact discovery");
        assert!(
            output.status.success(),
            "Cargo artifact discovery failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );

        let mut graph = std::collections::HashMap::new();
        for line in String::from_utf8_lossy(&output.stdout).lines() {
            let Some(message) = cargo_artifact_message::parse(line) else {
                continue;
            };
            let artifact = message.filenames.into_iter().find(|path| {
                path.extension().is_some_and(|extension| {
                    extension == "rlib" || extension == "rmeta" || extension == "dll"
                })
            });
            if let Some(artifact) = artifact {
                graph.insert(message.target_name.replace('-', "_"), artifact);
            }
        }
        graph
    })
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
