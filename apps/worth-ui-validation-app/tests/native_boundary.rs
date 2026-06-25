use std::fs;
use std::path::Path;

use eframe::egui::Color32;
use worth_ui_validation_app::header::applied_header_style_receipt;
use worth_ui_validation_app::ValidationWorkbenchLaunch;

#[allow(dead_code)]
mod support;

use support::{
    native_boundary_markers::{
        ACCESSIBILITY_AND_FOCUS_MEANING_MARKERS, FORBIDDEN_EGUI_MEANING_TOKENS,
        MOUNTED_CHILD_RENDER_GROUPING_MARKERS, NATIVE_EGUI_BOUNDARY_FILES,
        PRIMITIVE_EVENT_TOPOLOGY_CONSTRUCTORS, QUERY_RELOAD_PROOF_TYPES,
        THEME_RELOAD_AND_SOURCE_AUTHORITY_MARKERS,
    },
    native_boundary_scanning::{
        all_paths, file_contains,
        is_native_egui_boundary_file as path_is_native_egui_boundary_file,
        is_runtime_receipt_adapter, line_declares_forge_query_dependency, rust_files,
        strip_toml_comment,
    },
};

#[test]
fn egui_imports_stay_in_native_boundary_files() {
    let src_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let offenders: Vec<_> = rust_files(&src_root)
        .into_iter()
        .filter(|path| file_contains(path, "egui"))
        .filter(|path| {
            let relative = path
                .strip_prefix(&src_root)
                .expect("path should be under src root")
                .display()
                .to_string();
            !NATIVE_EGUI_BOUNDARY_FILES.contains(&relative.as_str())
        })
        .collect();

    assert!(
        offenders.is_empty(),
        "egui must stay in admitted native boundary files: {offenders:?}"
    );
}

#[test]
fn egui_meaning_apis_stay_in_approved_host_adapters() {
    let src_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let offenders: Vec<_> = rust_files(&src_root)
        .into_iter()
        .filter(|path| !is_native_egui_boundary_file(path, &src_root))
        .filter(|path| {
            let text = fs::read_to_string(path).expect("source should be readable");
            FORBIDDEN_EGUI_MEANING_TOKENS
                .iter()
                .any(|token| text.contains(token))
        })
        .collect();

    assert!(
        offenders.is_empty(),
        "egui composition/style APIs must stay inside approved host adapters: {offenders:?}"
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
    let offenders: Vec<_> = rust_files(&src_root)
        .into_iter()
        .filter(|path| !is_runtime_receipt_adapter(path, &src_root))
        .filter(|path| {
            let text = fs::read_to_string(path).expect("source should be readable");
            QUERY_RELOAD_PROOF_TYPES
                .iter()
                .any(|pattern| text.contains(pattern))
        })
        .collect();

    assert!(
        offenders.is_empty(),
        "validation app must not mint query/rebind/runtime proof surfaces: {offenders:?}"
    );
}

#[test]
fn validation_app_does_not_construct_primitive_event_topology() {
    let src_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let offenders: Vec<_> = rust_files(&src_root)
        .into_iter()
        .filter(|path| {
            let text = fs::read_to_string(path).expect("source should be readable");
            PRIMITIVE_EVENT_TOPOLOGY_CONSTRUCTORS
                .iter()
                .any(|pattern| text.contains(pattern))
        })
        .collect();

    assert!(
        offenders.is_empty(),
        "validation app must ask runtime graph/event planning for primitive event topology: {offenders:?}"
    );
}

#[test]
fn validation_app_does_not_select_accessibility_or_focus_meaning() {
    let src_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let offenders: Vec<_> = rust_files(&src_root)
        .into_iter()
        .filter(|path| {
            let text = fs::read_to_string(path).expect("source should be readable");
            ACCESSIBILITY_AND_FOCUS_MEANING_MARKERS
                .iter()
                .any(|pattern| text.contains(pattern))
        })
        .collect();

    assert!(
        offenders.is_empty(),
        "accessibility/focus roles, labels, descriptions, and tab order must come from Worth UI receipts: {offenders:?}"
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
fn validation_app_renders_mounted_children_without_grouped_control_or_interaction_lookup() {
    let src_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let offenders: Vec<_> = rust_files(&src_root)
        .into_iter()
        .filter(|path| {
            let text = fs::read_to_string(path).expect("source should be readable");
            MOUNTED_CHILD_RENDER_GROUPING_MARKERS
                .iter()
                .any(|pattern| text.contains(pattern))
        })
        .collect();

    assert!(
        offenders.is_empty(),
        "validation rendering must consume mounted child receipts instead of grouped control/interaction render-plan rows: {offenders:?}"
    );
}

#[test]
fn validation_app_does_not_own_theme_reload_or_source_parsing() {
    let src_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let offenders: Vec<_> = rust_files(&src_root)
        .into_iter()
        .filter(|path| {
            let text = fs::read_to_string(path).expect("source should be readable");
            THEME_RELOAD_AND_SOURCE_AUTHORITY_MARKERS
                .iter()
                .any(|pattern| text.contains(pattern))
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

fn is_native_egui_boundary_file(path: &Path, src_root: &Path) -> bool {
    path_is_native_egui_boundary_file(path, src_root, NATIVE_EGUI_BOUNDARY_FILES)
}
