use std::fs;
use std::path::{Path, PathBuf};

#[test]
fn egui_imports_stay_in_native_boundary_files() {
    let src_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let allowed = ["app.rs", "header\\header_renderer.rs"];
    let offenders: Vec<_> = rust_files(&src_root)
        .into_iter()
        .filter(|path| file_contains(path, "egui"))
        .filter(|path| {
            let relative = path
                .strip_prefix(&src_root)
                .expect("path should be under src root")
                .display()
                .to_string();
            !allowed.contains(&relative.as_str())
        })
        .collect();

    assert!(
        offenders.is_empty(),
        "egui must stay in admitted native boundary files: {offenders:?}"
    );
}

#[test]
fn erased_app_has_no_workspace_or_page_mosaic_modules() {
    let src_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let forbidden_names = [
        "workspace",
        "validation_page_layout_renderer.rs",
        "validation_workspace_shell_renderer.rs",
        "validation_page_content_renderer.rs",
    ];
    let offenders: Vec<_> = all_paths(&src_root)
        .into_iter()
        .filter(|path| {
            let relative = path
                .strip_prefix(&src_root)
                .expect("path should be under src root")
                .display()
                .to_string();
            forbidden_names.iter().any(|name| relative.contains(name))
        })
        .collect();

    assert!(
        offenders.is_empty(),
        "the reset app should not retain old shell/page mosaic modules: {offenders:?}"
    );
}

#[test]
fn validation_app_does_not_define_header_menu_authority() {
    let src_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let forbidden = [
        "struct HeaderMenu",
        "struct HeaderCommandItem",
        "HeaderMenuBar",
    ];
    let offenders: Vec<_> = rust_files(&src_root)
        .into_iter()
        .filter(|path| !path.ends_with("header_renderer.rs"))
        .filter(|path| {
            let text = fs::read_to_string(path).expect("source should be readable");
            forbidden.iter().any(|pattern| text.contains(pattern))
        })
        .collect();

    assert!(
        offenders.is_empty(),
        "header menu authority must live in Worth UI runtime surfaces: {offenders:?}"
    );
}

#[test]
fn validation_app_does_not_own_theme_reload_or_source_parsing() {
    let src_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let forbidden = [
        "WorthUiHeaderThemeRuntime",
        "WorthUiHeaderThemeRuntimeFrame",
        "WorthUiHeaderThemeRuntimeDenial",
        "WorthUiSourceParser",
        "WorthUiParsedSourceToArtifactInputLowerer",
        "WorthUiArtifactInputResolver",
        "WorthUiCanonicalArtifactAssembler",
        "HeaderThemeHotReload",
        "WorthUiSourceWatcher",
        "WorthUiWatchedCandidateSubmission",
        "WorthUiCandidateAdmission",
        "WorthUiReplacementCandidate",
        "from_snapshot_and_source_file",
        "WorthUiHeaderThemePlan::from_snapshot_and_source",
        "update_token_color",
        "query_delivery_count",
    ];
    let offenders: Vec<_> = rust_files(&src_root)
        .into_iter()
        .filter(|path| {
            let text = fs::read_to_string(path).expect("source should be readable");
            forbidden.iter().any(|pattern| text.contains(pattern))
        })
        .collect();

    assert!(
        offenders.is_empty(),
        "theme reload and source interpretation must live in Worth UI runtime/query surfaces: {offenders:?}"
    );
}

#[test]
fn worth_ui_facade_does_not_export_header_reload_runtime() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("app crate should live under workspace apps directory");
    let worth_ui_src = workspace.join("crates").join("worth-ui").join("src");
    let inspected_files = [
        worth_ui_src.join("facade").join("mod.rs"),
        worth_ui_src.join("runtime").join("mod.rs"),
        worth_ui_src
            .join("runtime")
            .join("header_surface")
            .join("mod.rs"),
    ];
    let forbidden = [
        "WorthUiHeaderThemeRuntime",
        "WorthUiHeaderThemeRuntimeFrame",
        "WorthUiHeaderThemeRuntimeDenial",
    ];
    let offenders: Vec<_> = inspected_files
        .into_iter()
        .filter(|path| {
            let text = fs::read_to_string(path).expect("Worth UI facade file should be readable");
            forbidden.iter().any(|pattern| text.contains(pattern))
        })
        .collect();

    assert!(
        offenders.is_empty(),
        "header-specific reload runtime must not be exported as a platform API: {offenders:?}"
    );
}

#[test]
fn renderer_does_not_define_theme_palette() {
    let src_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let renderer = src_root.join("header").join("header_renderer.rs");
    let text = fs::read_to_string(renderer).expect("renderer should be readable");

    for forbidden in ["from_rgb(30", "from_rgb(37", "#1E1E1E", "#007ACC"] {
        assert!(
            !text.contains(forbidden),
            "renderer must consume Worth UI theme receipts, not define palette value `{forbidden}`"
        );
    }
}

#[test]
fn validation_app_does_not_depend_on_forge_query_directly() {
    let manifest = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"))
        .expect("manifest should be readable");

    assert!(
        !manifest
            .lines()
            .map(strip_toml_comment)
            .any(line_declares_forge_query_dependency),
        "validation app must receive query/runtime truth through Worth UI"
    );
}

#[test]
fn forge_query_dependency_guard_rejects_direct_renamed_and_target_specific_forms() {
    let hostile_lines = [
        "forge-query = { workspace = true }",
        "[dependencies.forge-query]",
        "[dev-dependencies.forge-query]",
        "[target.'cfg(windows)'.dependencies.forge-query]",
        "query = { package = \"forge-query\", workspace = true }",
        "query = { package=\"forge-query\", workspace = true }",
        "forge-query = { workspace = true } # even when followed by a comment",
    ];

    for line in hostile_lines {
        assert!(
            line_declares_forge_query_dependency(strip_toml_comment(line)),
            "guard missed direct Forge Query dependency form: {line}"
        );
    }

    let safe_lines = [
        "worth-ui = { workspace = true }",
        "egui = { workspace = true }",
        "# forge-query = { workspace = true }",
        "description = \"mentions forge-query in prose only\"",
    ];

    for line in safe_lines {
        assert!(
            !line_declares_forge_query_dependency(strip_toml_comment(line)),
            "guard rejected non-dependency manifest line: {line}"
        );
    }
}

fn rust_files(root: &Path) -> Vec<PathBuf> {
    all_paths(root)
        .into_iter()
        .filter(|path| path.extension().is_some_and(|extension| extension == "rs"))
        .collect()
}

fn all_paths(root: &Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    collect_paths(root, &mut paths);
    paths
}

fn collect_paths(root: &Path, paths: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(root).expect("source directory should be readable") {
        let path = entry.expect("directory entry should be readable").path();
        paths.push(path.clone());
        if path.is_dir() {
            collect_paths(&path, paths);
        }
    }
}

fn file_contains(path: &Path, needle: &str) -> bool {
    fs::read_to_string(path)
        .expect("source file should be readable")
        .contains(needle)
}

fn strip_toml_comment(line: &str) -> &str {
    line.split_once('#')
        .map_or(line, |(before_comment, _)| before_comment)
        .trim()
}

fn line_declares_forge_query_dependency(line: &str) -> bool {
    line.starts_with("[dependencies.forge-query]")
        || line.starts_with("[dev-dependencies.forge-query]")
        || line.contains(".dependencies.forge-query]")
        || cargo_key_matches_forge_query(line)
        || line.contains("package = \"forge-query\"")
        || line.contains("package=\"forge-query\"")
}

fn cargo_key_matches_forge_query(line: &str) -> bool {
    let Some((key, _)) = line.split_once('=') else {
        return false;
    };
    key.trim() == "forge-query"
}
