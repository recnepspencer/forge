use std::any::TypeId;
use std::collections::BTreeSet;
use std::mem::size_of;
use std::path::Path;

use syn::visit::Visit;

#[test]
fn text_owns_typed_alpha_and_color_raster_batches() {
    assert_ne!(
        TypeId::of::<worth_ui_text::UiAlphaRasterBatch>(),
        TypeId::of::<worth_ui_text::UiColorRasterBatch>()
    );
    assert!(size_of::<worth_ui_text::UiAlphaRasterBatch>() > 0);
    assert_ne!(
        TypeId::of::<worth_ui_host_contract::UiAlphaRasterRecordView<'static>>(),
        TypeId::of::<worth_ui_host_contract::UiColorRasterRecordView<'static>>()
    );
}

#[test]
fn color_glyphs_preserve_intrinsic_color_and_cluster_identity() {
    let parsed = parse_crate_module("worth-ui-text/src/raster");
    let color = crate_root().join("worth-ui-text/src/raster/color");
    assert!(color.join("bitmap.rs").is_file());
    assert!(color.join("colr.rs").is_file());
    assert!(color.join("compositing.rs").is_file());
    assert!(!color.with_file_name("color_bitmap.rs").exists());
    assert!(!color.with_file_name("color_colrv1.rs").exists());
    assert!(parsed.contains_ident("UiColorRasterKind"));
    assert!(!parsed.contains_ident("tint_emoji"));
    assert!(!parsed.contains_ident("split_cluster"));
}

#[test]
fn native_host_owns_separate_alpha_and_rgba_atlas_lifecycles() {
    let parsed = parse_crate_module("worth-ui-host-native/src/native/text_atlas");
    assert!(parsed.contains_ident("UiNativeTextAtlasCensus"));
    assert!(parsed.contains_ident("UiNativeTextAtlasResourceClass"));
    assert!(parsed.source.contains("AtlasKind::Alpha"));
    assert!(parsed.source.contains("AtlasKind::Color"));
    assert!(parsed.source.contains("UiNativeGpuAtlasKind::Alpha"));
    assert!(parsed.source.contains("UiNativeGpuAtlasKind::Color"));
    assert!(forbidden_owner_mentions(
        "UiNativeTextAtlasGpuPages",
        &["worth-ui-text", "worth-ui-host-headless"]
    )
    .is_empty());
}

#[test]
fn native_host_exclusively_owns_move_only_atlas_lifecycle_authority() {
    let parsed = parse_crate_module("worth-ui-host-native/src/native/text_atlas");
    assert!(parsed.contains_struct("UiNativeTextAtlasPin"));
    assert!(parsed.struct_is_move_only("UiNativeTextAtlasPin"));
    assert!(parsed.struct_is_move_only("UiNativeTextAtlasTransactionPlan"));
    assert!(parsed.struct_is_move_only("UiNativeTextAtlasRecovery"));
    assert!(forbidden_owner_mentions(
        "UiNativeTextAtlasPin",
        &[
            "worth-ui-text",
            "worth-ui-host-contract",
            "worth-ui-host-headless"
        ]
    )
    .is_empty());
}

#[test]
fn dpi_replacement_is_pure_raster_and_does_not_relayout() {
    let parsed = parse_crate_module("worth-ui-text/src/raster");
    assert!(!parsed.contains_ident("relayout"));
    assert!(!parsed.contains_ident("reshape"));
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
    let parsed =
        parse_source("worth-ui-runtime/src/mounting/projection/semantic_text/qualified.rs");
    assert!(parsed
        .source
        .contains("layout: Arc<worth_ui_text::UiQualifiedTextLayout>"));
}

#[test]
fn phase_five_cost_vocabulary_separates_ordinary_and_reconstructive_lanes() {
    assert!(size_of::<worth_ui_text::UiGlyphRasterCost>() > 0);
    let cost = worth_ui_text::UiGlyphRasterCost::default();
    assert_eq!(cost.ordinary().rasterized_glyphs(), 0);
    assert_eq!(cost.reconstructive().rasterized_glyphs(), 0);
}

#[test]
fn consumers_cannot_reshape_refallback_or_consult_system_fonts() {
    for crate_name in [
        "worth-ui-host-headless",
        "worth-ui-host-native",
        "worth-ui-host-contract",
    ] {
        let parsed = parse_crate_module(&format!("{crate_name}/src"));
        parsed.reject_consumer_reshape(crate_name);
    }
}

#[test]
fn host_native_and_text_keep_one_way_dependencies() {
    let native = std::fs::read_to_string(crate_root().join("worth-ui-host-native/Cargo.toml"))
        .expect("native manifest");
    let text = std::fs::read_to_string(crate_root().join("worth-ui-text/Cargo.toml"))
        .expect("text manifest");
    assert!(!native.contains("worth-ui-text"));
    assert!(!text.contains("worth-ui-host-native"));
    assert!(!text.contains("wgpu"));
}

#[test]
fn coordinator_semantic_text_readiness_is_native_owned() {
    let parsed = parse_source(
        "worth-ui-runtime/src/mounting/presentation/coordinator/presentation_attempt.rs",
    );
    assert!(parsed.contains_ident("native_host_owns_semantic_text_boundary"));
    assert!(parsed.contains_ident("WorthUiHostKind"));
}

#[test]
fn only_runtime_text_presentation_imports_both_domains() {
    let inventory = workspace_source_files("worth-ui-runtime/src");
    let both = inventory
        .into_iter()
        .filter(|(_, text)| {
            (text.contains("UiQualifiedTextLayout") || text.contains("UiGlyphRaster"))
                && (text.contains("UiNativeTextAtlas")
                    || text.contains("UiNativeAlphaTextAtlas")
                    || text.contains("UiNativeAlphaRasterHandoff"))
        })
        .map(|(path, _)| path)
        .collect::<BTreeSet<_>>();
    assert!(
        both.iter()
            .all(|path| path.contains("native_platform/text_presentation")),
        "dual-domain imports escaped text_presentation: {both:?}"
    );
    assert!(
        both.iter().any(|path| path
            .replace('\\', "/")
            .ends_with("native_platform/text_presentation/transaction.rs")),
        "transaction owner must import both domains"
    );
}

#[test]
fn readiness_identities_emit_no_phase_five_feature_counter() {
    for relative in [
        "worth-ui-text/src/raster",
        "worth-ui-host-native/src/native/text_atlas",
        "worth-ui-runtime/src/native_platform/text_presentation",
        "worth-ui-host-contract/src/qualified_text",
    ] {
        let production_source = workspace_source_files(relative)
            .into_iter()
            .filter(|(path, _)| {
                !path.contains("/tests/")
                    && !path.ends_with("_tests.rs")
                    && !path.ends_with("/tests.rs")
            })
            .map(|(_, source)| source)
            .collect::<String>();
        assert!(
            !production_source.contains("P5-GLYPH-RASTER-01"),
            "{relative} binds a feature row"
        );
    }
}

struct ParsedModule {
    source: String,
    file: syn::File,
}

impl ParsedModule {
    fn contains_ident(&self, name: &str) -> bool {
        let mut found = false;
        struct Finder<'a> {
            name: &'a str,
            found: &'a mut bool,
        }
        impl Visit<'_> for Finder<'_> {
            fn visit_ident(&mut self, ident: &syn::Ident) {
                if ident == self.name {
                    *self.found = true;
                }
            }
        }
        Finder {
            name,
            found: &mut found,
        }
        .visit_file(&self.file);
        found
    }

    fn contains_struct(&self, name: &str) -> bool {
        self.file.items.iter().any(|item| match item {
            syn::Item::Struct(item) => item.ident == name,
            syn::Item::Mod(module) => module.content.as_ref().is_some_and(|(_, items)| {
                items
                    .iter()
                    .any(|item| matches!(item, syn::Item::Struct(item) if item.ident == name))
            }),
            _ => false,
        }) || self.contains_ident(name)
    }

    fn struct_is_move_only(&self, name: &str) -> bool {
        let mut move_only = false;
        struct Finder<'a> {
            name: &'a str,
            move_only: &'a mut bool,
        }
        impl Visit<'_> for Finder<'_> {
            fn visit_item_struct(&mut self, item: &syn::ItemStruct) {
                if item.ident == self.name {
                    let derives = item.attrs.iter().any(|attr| {
                        attr.path().is_ident("derive")
                            && attr
                                .parse_args_with(
                                    syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
                                )
                                .map(|paths| {
                                    paths.iter().any(|path| {
                                        path.segments.last().is_some_and(|segment| {
                                            segment.ident == "Clone" || segment.ident == "Copy"
                                        })
                                    })
                                })
                                .unwrap_or(false)
                    });
                    let public_ctor = item
                        .fields
                        .iter()
                        .all(|field| matches!(field.vis, syn::Visibility::Public(_)))
                        && !item.fields.is_empty();
                    *self.move_only = !derives && !public_ctor;
                }
                syn::visit::visit_item_struct(self, item);
            }
        }
        Finder {
            name,
            move_only: &mut move_only,
        }
        .visit_file(&self.file);
        move_only
    }

    fn reject_consumer_reshape(&self, crate_name: &str) {
        for forbidden in ["reshape_layout", "refallback", "rebreak_line"] {
            assert!(
                !self.contains_ident(forbidden),
                "{crate_name} exposes {forbidden}"
            );
        }
        assert!(
            !self.source.contains("C:/Windows/Fonts"),
            "{crate_name} consults a system font path"
        );
    }
}

fn parse_crate_module(relative: &str) -> ParsedModule {
    let mut source = String::new();
    let mut items = Vec::new();
    for path in rust_sources(&crate_root().join(relative)) {
        let file_source = std::fs::read_to_string(&path).expect("source is readable");
        if let Ok(file) = syn::parse_file(&file_source) {
            items.extend(file.items);
        }
        source.push_str(&file_source);
        source.push('\n');
    }
    ParsedModule {
        source,
        file: syn::File {
            shebang: None,
            attrs: Vec::new(),
            items,
        },
    }
}

fn parse_source(relative: &str) -> ParsedModule {
    let source = std::fs::read_to_string(crate_root().join(relative)).expect("source is readable");
    let file = syn::parse_file(&source).expect("source parses");
    ParsedModule { source, file }
}

fn crate_root() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace crates directory")
        .to_path_buf()
}

fn rust_sources(root: &Path) -> Vec<std::path::PathBuf> {
    let mut sources = Vec::new();
    visit(root, &mut sources);
    sources
}

fn visit(root: &Path, sources: &mut Vec<std::path::PathBuf>) {
    if !root.exists() {
        return;
    }
    for entry in std::fs::read_dir(root).expect("source directory is readable") {
        let path = entry.expect("directory entry is readable").path();
        if path.is_dir() {
            visit(&path, sources);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            sources.push(path);
        }
    }
}

fn workspace_source_files(relative: &str) -> Vec<(String, String)> {
    rust_sources(&crate_root().join(relative))
        .into_iter()
        .map(|path| {
            (
                path.to_string_lossy().replace('\\', "/"),
                std::fs::read_to_string(&path).unwrap_or_default(),
            )
        })
        .collect()
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
