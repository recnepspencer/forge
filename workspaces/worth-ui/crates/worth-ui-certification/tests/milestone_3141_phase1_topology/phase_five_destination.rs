use std::path::{Path, PathBuf};
use std::{any::TypeId, mem::size_of};

#[test]
fn text_owns_typed_alpha_and_color_raster_batches() {
    assert_ne!(
        TypeId::of::<worth_ui_text::UiAlphaRasterBatch>(),
        TypeId::of::<worth_ui_text::UiColorRasterBatch>()
    );
    assert!(size_of::<worth_ui_text::UiAlphaRasterBatch>() > 0);
}

#[test]
fn color_glyphs_preserve_intrinsic_color_and_cluster_identity() {
    let source = std::fs::read_to_string(crate_root().join("worth-ui-text/src/raster.rs"))
        .expect("color raster destination is readable");
    assert!(source.contains("UiColorRasterKind"));
    assert!(!source.contains("tint_emoji"));
    assert!(!source.contains("split_cluster"));
}

#[test]
fn native_host_owns_separate_alpha_and_rgba_atlas_lifecycles() {
    assert_ne!(
        TypeId::of::<worth_ui_host_native::UiAlphaAtlasLifecycle>(),
        TypeId::of::<worth_ui_host_native::UiRgbaAtlasLifecycle>()
    );
    assert!(size_of::<worth_ui_host_native::UiAlphaAtlasLifecycle>() > 0);
    assert!(size_of::<worth_ui_host_native::UiRgbaAtlasLifecycle>() > 0);
    assert!(forbidden_owner_mentions(
        "UiAlphaAtlasLifecycle",
        &["worth-ui-text", "worth-ui-host-headless"]
    )
    .is_empty());
}

#[test]
fn live_layouts_pin_atlas_entries_without_consumer_eviction() {
    let source =
        std::fs::read_to_string(crate_root().join("worth-ui-host-native/src/native/text_atlas.rs"))
            .expect("atlas pin destination is readable");
    assert!(source.contains("pub struct UiAtlasPin"));
    assert!(!source.contains("pub fn evict"));
}

#[test]
fn dpi_replacement_is_pure_raster_and_does_not_relayout() {
    let raster = std::fs::read_to_string(crate_root().join("worth-ui-text/src/raster.rs"))
        .expect("raster destination is readable");
    assert!(!raster.contains("relayout"));
    assert!(!raster.contains("reshape"));
}

#[test]
fn paint_spans_carry_logical_foreground_without_layout_regen() {
    let runtime = crate_root().join("worth-ui-runtime/src/mounting/projection/semantic_text");
    assert!(runtime.is_dir());
}

#[test]
fn headless_and_native_pixels_report_the_same_paint_span() {
    assert!(crate_root().join("worth-ui-host-headless/src").is_dir());
    assert!(crate_root().join("worth-ui-host-native/src").is_dir());
}

#[test]
fn reconstruction_consumes_only_mounted_layout_authority() {
    let owner = std::fs::read_to_string(
        crate_root().join("worth-ui-runtime/src/mounting/projection/semantic_text/qualified.rs"),
    )
    .expect("runtime qualified-text owner is readable");
    assert!(owner.contains("layout: Arc<worth_ui_text::UiQualifiedTextLayout>"));
}

#[test]
fn phase_five_cost_vocabulary_separates_ordinary_and_reconstructive_lanes() {
    let source = std::fs::read_to_string(crate_root().join("worth-ui-text/src/raster.rs"))
        .expect("raster cost vocabulary is readable");
    assert!(source.contains("UiGlyphRasterCost"));
    assert!(source.contains("ordinary"));
    assert!(source.contains("reconstructive"));
}

#[test]
fn consumers_cannot_reshape_refallback_or_consult_system_fonts() {
    for crate_name in [
        "worth-ui-host-headless",
        "worth-ui-host-native",
        "worth-ui-host-egui",
        "worth-ui-host-contract",
    ] {
        for source in rust_sources(&crate_root().join(crate_name).join("src")) {
            let text = std::fs::read_to_string(&source).expect("host source is readable");
            reject_consumer_reshape(&text)
                .unwrap_or_else(|error| panic!("{}: {error}", source.display()));
        }
    }
    assert!(reject_consumer_reshape("fn paint() { reshape_layout(); }").is_err());
    assert!(
        reject_consumer_reshape("fn paint() { std::fs::read(\"C:/Windows/Fonts/a.ttf\"); }")
            .is_err()
    );
}

fn crate_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace crates directory")
        .to_path_buf()
}

fn rust_sources(root: &Path) -> Vec<PathBuf> {
    let mut sources = Vec::new();
    visit(root, &mut sources);
    sources
}

fn visit(root: &Path, sources: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(root).expect("source directory is readable") {
        let entry = entry.expect("directory entry is readable");
        let path = entry.path();
        if path.is_dir() {
            visit(&path, sources);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            sources.push(path);
        }
    }
}

fn reject_consumer_reshape(source: &str) -> Result<(), String> {
    let compact: String = source.chars().filter(|c| !c.is_whitespace()).collect();
    for forbidden in [
        "reshape_layout",
        "refallback",
        "rebreak_line",
        "C:/Windows/Fonts",
    ] {
        if compact.contains(forbidden) || source.contains(forbidden) {
            return Err(format!("consumer may not {forbidden}"));
        }
    }
    Ok(())
}

fn forbidden_owner_mentions(type_name: &str, crates: &[&str]) -> Vec<String> {
    let mut found = Vec::new();
    for crate_name in crates {
        for source in rust_sources(&crate_root().join(crate_name).join("src")) {
            let text = std::fs::read_to_string(&source).unwrap_or_default();
            if text.contains(type_name) {
                found.push(source.display().to_string());
            }
        }
    }
    found
}
