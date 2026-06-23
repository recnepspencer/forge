use std::fs;
use std::path::{Path, PathBuf};

use eframe::egui::Color32;
use worth_ui_validation_app::header::applied_header_style_receipt;
use worth_ui_validation_app::ValidationWorkbenchLaunch;
#[test]
fn egui_imports_stay_in_native_boundary_files() {
    let src_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let allowed = [
        "app.rs",
        "app\\primitive_denial_rendering.rs",
        "app\\primitive_content_rendering.rs",
        "app\\primitive_paint_colors.rs",
        "app\\primitive_proof.rs",
        "header\\header_renderer.rs",
        "main.rs",
        "native_window.rs",
        "pages\\manual_flow_matrix\\renderer.rs",
        "pages\\product_summary\\renderer.rs",
        "pages\\page_slot_interaction\\renderer.rs",
    ];
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
fn validation_app_does_not_import_runtime_internals() {
    let src_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let offenders: Vec<_> = rust_files(&src_root)
        .into_iter()
        .filter(|path| file_contains(path, "worth_ui::runtime"))
        .collect();

    assert!(
        offenders.is_empty(),
        "validation app must consume Worth UI facade receipts, not runtime internals: {offenders:?}"
    );
}

#[test]
fn validation_app_does_not_construct_query_reload_proof() {
    let src_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let forbidden = [
        "WorthUiQueryRuntimeFactLoweringInput",
        "WorthUiQueryProjectionFactReceipt",
        "WorthUiQueryStateSnapshotReceipt",
        "WorthUiQueryEffectPostureReceipt",
        "WorthUiQueryLiveRebindPlan",
        "WorthUiQueryLiveRebindCounters",
        "WorthUiQueryLiveRebindEntry",
        "WorthUiQueryLiveRebindOutcome",
        "WorthUiQueryBindingPreservation",
        "WorthUiQueryBindingRebind",
        "WorthUiQueryBindingRetirement",
        "WorthUiQueryBindingDriftDenial",
        "WorthUiAdmittedRuntimeChangeEvidence",
        "WorthUiAdmittedProjectionPlan",
        "WorthUiProjectionRebindPlan",
    ];
    let offenders: Vec<_> = rust_files(&src_root)
        .into_iter()
        .filter(|path| !is_runtime_receipt_adapter(path, &src_root))
        .filter(|path| {
            let text = fs::read_to_string(path).expect("source should be readable");
            forbidden.iter().any(|pattern| text.contains(pattern))
        })
        .collect();

    assert!(
        offenders.is_empty(),
        "validation app must not mint query/rebind/runtime proof surfaces: {offenders:?}"
    );
}

#[test]
fn product_summary_does_not_define_local_product_state() {
    let src_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("pages")
        .join("product_summary");
    let forbidden = [
        "LocalProductSummaryState",
        "ProductRow",
        "Vec<Product",
        "HashMap<",
        "BTreeMap<",
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
        "product summary must project runtime receipts, not local product state: {offenders:?}"
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
fn validation_app_has_no_local_style_component_page_shell_or_theme_authority_modules() {
    let src_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let forbidden_path_fragments = [
        "style_map",
        "component_registry",
        "page_map",
        "shell_map",
        "theme_state",
    ];
    let forbidden_type_markers = [
        "LocalStyleMap",
        "ValidationStyleMap",
        "LocalComponentRegistry",
        "ValidationComponentRegistry",
        "LocalPageMap",
        "ValidationPageMap",
        "LocalShellMap",
        "ValidationShellMap",
        "LocalThemeState",
        "ValidationThemeState",
    ];
    let offenders: Vec<_> = all_paths(&src_root)
        .into_iter()
        .filter(|path| {
            let relative = path
                .strip_prefix(&src_root)
                .expect("path should be under src root")
                .display()
                .to_string();
            forbidden_path_fragments
                .iter()
                .any(|fragment| relative.contains(fragment))
                || (path.extension().is_some_and(|extension| extension == "rs")
                    && forbidden_type_markers
                        .iter()
                        .any(|pattern| file_contains(path, pattern)))
        })
        .collect();

    assert!(
        offenders.is_empty(),
        "validation app must not grow local style/component/page/shell/theme authority: {offenders:?}"
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
fn renderer_projects_applied_header_style_from_runtime_receipts() {
    let launch = ValidationWorkbenchLaunch::new()
        .prepare()
        .expect("validation app should launch through Worth UI");
    let applied = applied_header_style_receipt(
        launch.header_theme_plan().execute_frame(),
        launch.header_appearance_plan().execute_frame(),
    );

    assert_eq!(applied.menu_min_width_points(), 220.0);
    assert_eq!(applied.font_size_points(), 13.0);
    assert_eq!(applied.control_spacing_points(), 8.0);
    assert_eq!(applied.border_width_points(), 1.0);
    assert_eq!(applied.row_padding_horizontal_points(), 6.0);
    assert_eq!(applied.row_padding_vertical_points(), 1.0);
    assert_eq!(applied.container_margin().left, 8);
    assert_eq!(applied.container_margin().top, 4);
    assert_eq!(applied.menu_margin().left, 6);
    assert_eq!(applied.menu_margin().top, 1);
    assert_eq!(applied.shadow().offset, [0, 1]);
    assert_eq!(applied.shadow().blur, 3);
    assert_eq!(applied.panel_fill(), Color32::from_rgb(31, 41, 51));
    assert_eq!(applied.menu_fill(), Color32::from_rgb(37, 37, 38));
    assert_eq!(applied.border_color(), Color32::from_rgb(63, 63, 70));
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

fn is_runtime_receipt_adapter(path: &Path, src_root: &Path) -> bool {
    let relative = path
        .strip_prefix(src_root)
        .expect("path should be under src root")
        .display()
        .to_string();
    matches!(
        relative.as_str(),
        "reload\\validation_runtime_change_evidence.rs" | "runtime_workbench\\rebind_execution.rs"
    )
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
    line.split_once('=')
        .is_some_and(|(key, _)| key.trim() == "forge-query")
}
