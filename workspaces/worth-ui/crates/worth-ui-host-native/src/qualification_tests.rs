use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

use super::{
    UiBodyDefaultAtlasCapacities, UiNativeMechanicsCapacities, WORTH_UI_BODY_DEFAULT_FONT,
    WORTH_UI_BODY_DEFAULT_LICENSE, WORTH_UI_NATIVE_PROFILE_MANIFEST,
    WORTH_UI_TEXT_PROFILE_MANIFEST,
};

mod font_coverage;

#[test]
fn qualified_asset_license_and_manifests_have_exact_digests() {
    assert_eq!(
        sha256(WORTH_UI_BODY_DEFAULT_FONT),
        "478c558ea716033cd60c03438f628dfa75694dcf6b5f6d505a2f05fd2b4f3823"
    );
    assert_eq!(
        sha256(WORTH_UI_BODY_DEFAULT_LICENSE.as_bytes()),
        "cee9892f9f0cc8fe882c9e9537ee6a89621d86ee7ceaf70b02e2b2b1c25c061a"
    );
    assert_eq!(
        sha256(WORTH_UI_TEXT_PROFILE_MANIFEST.as_bytes()),
        "6f140249866e6815e9284fe1c8c959a8bb1b8cab252cfbe8c7c397f9a7eb9b01"
    );
    assert_eq!(
        sha256(WORTH_UI_NATIVE_PROFILE_MANIFEST.as_bytes()),
        "93121321d608b95e496f5e7defe63f0493f90ebf965202a64160da03de24d0fe"
    );
}

#[test]
fn qualified_capacity_types_match_the_canonical_manifests() {
    let text = manifest(WORTH_UI_TEXT_PROFILE_MANIFEST);
    let platform = manifest(WORTH_UI_NATIVE_PROFILE_MANIFEST);
    let atlas = UiBodyDefaultAtlasCapacities::QUALIFIED;
    let native = UiNativeMechanicsCapacities::QUALIFIED;
    assert_eq!(
        (
            atlas.pages,
            atlas.page_width,
            atlas.page_height,
            atlas.entries,
            atlas.texel_bytes,
            atlas.glyph_width,
            atlas.glyph_height,
            atlas.staged_upload_bytes,
        ),
        (
            integer(&text, "atlas_pages") as u8,
            integer(&text, "atlas_page_width") as u16,
            integer(&text, "atlas_page_height") as u16,
            integer(&text, "atlas_entries") as u16,
            integer(&text, "atlas_texel_bytes") as u32,
            integer(&text, "glyph_max_width") as u16,
            integer(&text, "glyph_max_height") as u16,
            integer(&text, "staged_upload_bytes") as u32,
        )
    );
    assert_eq!(
        (
            native.retained_commands,
            native.rectangle_commands,
            native.text_commands,
            native.damage_regions,
            native.order_edits,
            native.text_bytes,
            native.readiness_owners,
            native.causes_per_owner,
            native.ready_owner_slots,
            native.presentation_slots,
            native.readback_slots,
            native.readback_bytes,
        ),
        (
            integer(&platform, "retained_commands") as u16,
            integer(&platform, "rectangle_commands") as u16,
            integer(&platform, "text_commands") as u16,
            integer(&platform, "damage_regions") as u16,
            integer(&platform, "order_edits") as u16,
            integer(&platform, "text_bytes") as u32,
            integer(&platform, "readiness_owners") as u8,
            integer(&platform, "causes_per_owner") as u8,
            integer(&platform, "ready_owner_slots") as u8,
            integer(&platform, "presentation_slots") as u8,
            integer(&platform, "readback_slots") as u8,
            integer(&platform, "readback_bytes") as u32,
        )
    );
}

#[test]
fn every_qualified_semantic_and_dependency_pin_matches_the_closed_record() {
    let text = manifest(WORTH_UI_TEXT_PROFILE_MANIFEST);
    let platform = manifest(WORTH_UI_NATIVE_PROFILE_MANIFEST);
    assert_exact_manifest(
        &text,
        TEXT_STRING_FIELDS,
        TEXT_INTEGER_FIELDS,
        TEXT_BOOL_FIELDS,
    );
    assert_exact_manifest(
        &platform,
        NATIVE_STRING_FIELDS,
        NATIVE_INTEGER_FIELDS,
        NATIVE_BOOL_FIELDS,
    );
    assert_eq!(
        integer(&text, "asset_bytes"),
        WORTH_UI_BODY_DEFAULT_FONT.len() as i64
    );
    assert_eq!(
        text.get("subpixel").and_then(toml::Value::as_bool),
        Some(false)
    );
    assert_eq!(integer(&platform, "windows"), 1);
    assert_eq!(integer(&platform, "surfaces"), 1);
    assert_eq!(integer(&platform, "sample_count"), 1);
    assert_qualified_dependencies();
}

fn assert_qualified_dependencies() {
    let crate_manifest = manifest(include_str!("../Cargo.toml"));
    let workspace_manifest = manifest(include_str!("../../../Cargo.toml"));
    let qualified = crate_manifest
        .get("package")
        .and_then(|package| package.get("metadata"))
        .and_then(|metadata| metadata.get("worth-ui-qualified-dependencies"))
        .and_then(toml::Value::as_table)
        .expect("qualified dependency metadata");
    let declarations = crate_manifest
        .get("dependencies")
        .and_then(toml::Value::as_table)
        .expect("native dependency declarations");
    let workspace_declarations = workspace_manifest["workspace"]["dependencies"]
        .as_table()
        .expect("workspace dependency declarations");
    for &(name, version) in QUALIFIED_DEPENDENCIES {
        let exact_version = format!("={version}");
        assert_eq!(
            qualified.get(name).and_then(toml::Value::as_str),
            Some(version)
        );
        let workspace = workspace_declarations.get(name).expect("workspace pin");
        let observed_version = workspace
            .as_str()
            .or_else(|| workspace.get("version").and_then(toml::Value::as_str));
        assert_eq!(observed_version, Some(exact_version.as_str()));
        let declaration = declarations.get(name).expect("native direct pin");
        let direct_version = declaration
            .as_str()
            .or_else(|| declaration.get("version").and_then(toml::Value::as_str));
        assert_eq!(direct_version, Some(exact_version.as_str()));
    }
    assert_workspace_dependency_features(declarations, "winit", &["rwh_06"]);
    assert_workspace_dependency_features(
        declarations,
        "wgpu",
        &["std", "parking_lot", "dx12", "wgsl"],
    );
    assert_workspace_dependency_features(workspace_declarations, "winit", &["rwh_06"]);
    assert_workspace_dependency_features(
        workspace_declarations,
        "wgpu",
        &["std", "parking_lot", "dx12", "wgsl"],
    );
    assert_eq!(qualified["winit-features"].as_str(), Some("rwh_06"));
    assert_eq!(
        qualified["wgpu-features"].as_str(),
        Some("std,parking_lot,dx12,wgsl")
    );
    assert_eq!(qualified["wgpu-device-features"].as_str(), Some("empty"));
    assert_eq!(
        qualified["wgpu-limits"].as_str(),
        Some("wgpu-29.0.4-Limits::downlevel_defaults")
    );
}

const QUALIFIED_DEPENDENCIES: &[(&str, &str)] = &[
    ("winit", "0.30.13"),
    ("wgpu", "29.0.4"),
    ("pollster", "0.4.0"),
    ("rustybuzz", "0.20.1"),
    ("swash", "0.2.10"),
];

const TEXT_STRING_FIELDS: &[(&str, &str)] = &[
    ("identity", "worth-ui-body-default-v1"),
    ("asset", "NotoSans-Regular.ttf"),
    (
        "asset_sha256",
        "478c558ea716033cd60c03438f628dfa75694dcf6b5f6d505a2f05fd2b4f3823",
    ),
    ("upstream_release", "NotoSans-v2.015"),
    (
        "upstream_commit",
        "c4a321e123e4d4ff315f57f4e0adf294fe3a95be",
    ),
    (
        "upstream_asset_path",
        "NotoSans/hinted/ttf/NotoSans-Regular.ttf",
    ),
    (
        "archive_sha256",
        "0c34df072a3fa7efbb7cbf34950e1f971a4447cffe365d3a359e2d4089b958f5",
    ),
    ("license", "SIL Open Font License 1.1"),
    ("license_file", "OFL.txt"),
    (
        "license_sha256",
        "cee9892f9f0cc8fe882c9e9537ee6a89621d86ee7ceaf70b02e2b2b1c25c061a",
    ),
    ("support_start", "U+0020"),
    ("support_end", "U+007E"),
    ("normalization", "none"),
    ("direction", "horizontal-ltr"),
    ("language", "none"),
    ("script", "none"),
    ("fallback", "none"),
    ("shaper", "rustybuzz-0.20.1"),
    ("rasterizer", "swash-0.2.10"),
    ("baseline", "alphabetic"),
    ("wrap", "clip"),
    ("hinting", "hinted"),
    ("coverage", "grayscale"),
    (
        "rounding",
        "origin-nearest-ties-even;bounds-floor-ceil;clip-half-open",
    ),
    (
        "dpi_basis",
        "event-time-logical-size-times-scale-generation",
    ),
    ("unsupported", "typed-denial-before-effects"),
    (
        "unsupported_preserves",
        "semantic-projection-and-predecessor-publication",
    ),
    ("live_glyph_posture", "retained-commands-pin-entries"),
    ("candidate_eviction", "unpinned-candidate-only"),
    ("saturation", "deny-before-upload-no-growth-no-fallback"),
    (
        "qualification_observation",
        "asset-license-profile-dependency-digest-v1",
    ),
];

const TEXT_INTEGER_FIELDS: &[(&str, i64)] = &[
    ("asset_bytes", 621_572),
    ("archive_bytes", 117_491_253),
    ("license_bytes", 4_396),
    ("run_count", 1),
    ("size_millipoints", 14_000),
    ("weight", 400),
    ("atlas_pages", 4),
    ("atlas_page_width", 1_024),
    ("atlas_page_height", 1_024),
    ("atlas_entries", 4_096),
    ("atlas_texel_bytes", 4_194_304),
    ("glyph_max_width", 256),
    ("glyph_max_height", 256),
    ("staged_upload_bytes", 1_048_576),
];

const TEXT_BOOL_FIELDS: &[(&str, bool)] = &[("subpixel", false)];

const NATIVE_STRING_FIELDS: &[(&str, &str)] = &[
    ("identity", "worth-ui-windows-dx12-v1"),
    ("platform", "windows-11-x86_64"),
    ("desktop", "composition-enabled"),
    ("event_backend", "winit-0.30.13"),
    ("event_backend_features", "rwh_06"),
    ("graphics_backend", "wgpu-29.0.4-dx12-only"),
    ("graphics_backend_features", "std;parking_lot;dx12;wgsl"),
    ("device_features", "empty"),
    ("required_limits", "wgpu-29.0.4-Limits::downlevel_defaults"),
    ("runtime_backend_selection", "Backends::DX12"),
    ("joiner", "pollster-0.4.0"),
    (
        "adapter_order",
        "discrete;integrated;virtual;vendor-id;device-id;name;driver-info",
    ),
    ("cpu_adapter", "deny"),
    ("other_adapter", "deny"),
    ("surface_format", "Bgra8UnormSrgb"),
    ("target_format", "Rgba8UnormSrgb"),
    ("present_mode", "Fifo"),
    ("composite_alpha", "PreMultiplied"),
    ("shader_input", "logical-straight-rgba"),
    ("shader_output", "premultiplied-rgb-and-alpha"),
    ("blend", "src-One;dst-OneMinusSrcAlpha;op-Add"),
    ("filled_rect_antialiasing", "none"),
    ("text_antialiasing", "qualified-grayscale-coverage"),
    ("coordinate_rounding", "min-floor;max-ceil;half-open"),
    ("baseline_rgba", "0;0;0;0"),
    ("baseline_authority", "same-surface-binding-profile-runtime-receipt"),
    ("unsupported_mode", "typed-denial-before-effects-no-fallback"),
    ("initial_logical_size", "application-profile"),
    ("client_background", "transparent"),
    ("qualification_observations", "os-build;adapter-name;vendor-id;device-id;driver-info;scale-factors;required-modes;required-limits"),
    ("presented_source_observation", "retained-target-readback"),
    ("client_area_observation", "pulse-executable-world-xcap-0.9.7-wgc"),
    ("client_window_observation", "winsafe-0.0.28-dwm-kernel-user"),
    ("client_input_observation", "uiautomation-0.25.0-control-input"),
];

const NATIVE_INTEGER_FIELDS: &[(&str, i64)] = &[
    ("sample_count", 1),
    ("windows", 1),
    ("surfaces", 1),
    ("retained_commands", 4_096),
    ("rectangle_commands", 2_048),
    ("text_commands", 2_048),
    ("damage_regions", 4_096),
    ("order_edits", 4_096),
    ("text_bytes", 1_048_576),
    ("readiness_owners", 8),
    ("causes_per_owner", 64),
    ("ready_owner_slots", 8),
    ("presentation_slots", 2),
    ("readback_slots", 4),
    ("readback_bytes", 16_777_216),
];

const NATIVE_BOOL_FIELDS: &[(&str, bool)] = &[("required_surface_compatibility", true)];

fn assert_exact_manifest(
    manifest: &toml::Value,
    strings: &[(&str, &str)],
    integers: &[(&str, i64)],
    booleans: &[(&str, bool)],
) {
    let expected = strings
        .iter()
        .map(|(key, _)| *key)
        .chain(integers.iter().map(|(key, _)| *key))
        .chain(booleans.iter().map(|(key, _)| *key))
        .collect::<BTreeSet<_>>();
    let observed = manifest
        .as_table()
        .expect("qualified manifest table")
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    assert_eq!(
        observed, expected,
        "canonical manifest field inventory drifted"
    );
    for (key, value) in strings {
        assert_eq!(manifest[*key].as_str(), Some(*value), "string {key}");
    }
    for (key, value) in integers {
        assert_eq!(manifest[*key].as_integer(), Some(*value), "integer {key}");
    }
    for (key, value) in booleans {
        assert_eq!(manifest[*key].as_bool(), Some(*value), "boolean {key}");
    }
}

fn assert_workspace_dependency_features(
    declarations: &toml::map::Map<String, toml::Value>,
    name: &str,
    expected: &[&str],
) {
    let dependency = &declarations[name];
    assert_eq!(dependency["default-features"].as_bool(), Some(false));
    let features = dependency["features"]
        .as_array()
        .expect("qualified dependency feature list")
        .iter()
        .map(|feature| feature.as_str().expect("feature string"))
        .collect::<Vec<_>>();
    assert_eq!(features, expected, "{name} feature posture drifted");
}

fn manifest(text: &str) -> toml::Value {
    text.parse().expect("qualified manifest parses")
}

fn integer(manifest: &toml::Value, key: &str) -> i64 {
    manifest
        .get(key)
        .and_then(toml::Value::as_integer)
        .unwrap_or_else(|| panic!("qualified integer `{key}`"))
}

fn sha256(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
