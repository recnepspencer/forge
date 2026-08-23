#[path = "policy.rs"]
mod policy;
#[path = "raster.rs"]
mod raster;

pub(super) fn validate(manifest: &toml::Value) -> Result<(), String> {
    exact_string(manifest, "schema", "worth-ui-global-text-profile-v2")?;
    exact_string(manifest, "profile", "worth-ui-global-text-v2")?;
    exact_string(manifest, "unicode_version", "17.0.0")?;
    exact_string(
        manifest,
        "font_catalog",
        "Noto monthly 2026.08.01 + Noto Sans CJK 2.004 + Noto Color Emoji 2.051 + Last Resort 17.000",
    )?;
    exact_bool(manifest, "ambient_system_fonts", false)?;
    exact_string(manifest, "normalization", "preserve-original-utf8")?;
    exact_string(
        manifest,
        "fallback",
        "complete-cluster-first-qualified-face",
    )?;
    exact_string(
        manifest,
        "missing_coverage",
        "one-last-resort-glyph-per-cluster",
    )?;
    exact_string(
        manifest,
        "dependency_qualification_manifest",
        "tools/worth-ui-text-profile-qualification/Cargo.toml",
    )?;
    exact_string(
        manifest,
        "dependency_qualification_lock_sha256",
        "84ef06e5d4609b183d1e484881c7d43fb3e51835d0ee3f71759c707e136b45c5",
    )?;
    validate_dependencies(table(manifest, "dependencies")?)?;
    policy::validate_application_fonts(table(manifest, "application_fonts")?)?;
    policy::validate_run_formation(table(manifest, "run_formation")?)?;
    validate_unicode(table(manifest, "unicode")?)?;
    validate_layout(table(manifest, "layout")?)?;
    policy::validate_layout_identity(table(manifest, "layout_identity")?)?;
    policy::validate_scale_generation(table(manifest, "scale_generation")?)?;
    raster::validate(table(manifest, "raster")?)?;
    validate_capacities(table(manifest, "capacity")?)?;
    validate_capacity_admission(table(manifest, "capacity_admission")?)?;
    validate_locality(table(manifest, "locality")?)?;
    policy::validate_saturation(table(manifest, "saturation")?)
}

fn validate_dependencies(dependencies: &toml::value::Table) -> Result<(), String> {
    validate_unicode_dependencies(dependencies)?;
    validate_shape_dependencies(dependencies)?;
    (dependencies.len() == 10)
        .then_some(())
        .ok_or_else(|| "text dependency inventory drifted".to_owned())
}

fn validate_unicode_dependencies(dependencies: &toml::value::Table) -> Result<(), String> {
    dependency(
        dependencies,
        "unicode_segmentation",
        Dependency {
            strings: &[("version", "=1.13.3"), ("unicode", "17.0.0")],
            requested_features: &[],
            resolved_features: &[],
            checksum: "c6f5d3c3b1bf09027a88a6bc961fc00497d651009560b5463668dc81b0fa87a8",
        },
    )?;
    dependency(
        dependencies,
        "unicode_bidi",
        Dependency {
            strings: &[
                ("version", "=0.3.18"),
                ("data", "repository-generated-unicode-17"),
            ],
            requested_features: &["std"],
            resolved_features: &["std"],
            checksum: "5c1cb5db39152898a79168971543b1cb5020dff7fe43c8dc468b0885f5e29df5",
        },
    )?;
    dependency(
        dependencies,
        "icu_segmenter",
        Dependency {
            strings: &[
                ("version", "=2.2.0"),
                ("cldr", "48.2.0"),
                ("icu", "78.1rc"),
                ("unicode", "17.0.0"),
            ],
            requested_features: &["compiled_data", "auto"],
            resolved_features: &["auto", "compiled_data", "lstm"],
            checksum: "5c0794db0b1a86193ac9c48768d0e6c52c54448e0870ad87907d456ee0dac964",
        },
    )
}

fn validate_shape_dependencies(dependencies: &toml::value::Table) -> Result<(), String> {
    dependency(
        dependencies,
        "harfrust",
        Dependency {
            strings: &[("version", "=0.12.0"), ("unicode", "17.0.0")],
            requested_features: &["std"],
            resolved_features: &["std"],
            checksum: "c03d949a14aa089bbb282f7dd76a498a7f684428e4257202efc119ec010376f9",
        },
    )?;
    dependency(
        dependencies,
        "read_fonts",
        Dependency {
            strings: &[("version", "=0.41.0")],
            requested_features: &["std", "experimental_traverse"],
            resolved_features: &["experimental_font_api", "experimental_traverse", "std"],
            checksum: "046a7d674daf459825b32f5062056d6882db0d2f5a479fbd76ccfc870ac18709",
        },
    )?;
    dependency(
        dependencies,
        "skrifa",
        Dependency {
            strings: &[("version", "=0.44.0")],
            requested_features: &["std"],
            resolved_features: &["std"],
            checksum: "819ab7d62b1d3e72d9d9dea5650bac30424f9111364bb94928dbf5ecad1baa68",
        },
    )?;
    dependency(
        dependencies,
        "kurbo",
        Dependency {
            strings: &[("version", "=0.13.1")],
            requested_features: &["std"],
            resolved_features: &["default", "serde", "std"],
            checksum: "4b60dfc32f652b926df6192e55525b16d186c69d47876c3ead4da5cc9f8450e2",
        },
    )?;
    dependency(
        dependencies,
        "linesweeper",
        Dependency {
            strings: &[("version", "=0.4.0")],
            requested_features: &[],
            resolved_features: &[],
            checksum: "9c19728333c060c6569a53c9a0e56c4be0df52cb4e6e07a8fbe16084cecce769",
        },
    )?;
    dependency(
        dependencies,
        "swash",
        Dependency {
            strings: &[("version", "=0.2.10")],
            requested_features: &["std", "scale", "render"],
            resolved_features: &["render", "scale", "std"],
            checksum: "6c2499c2d826531388872b2268718aed907a39bd785ab0dcfe57fab26283f92e",
        },
    )
}

struct Dependency<'a> {
    strings: &'a [(&'a str, &'a str)],
    requested_features: &'a [&'a str],
    resolved_features: &'a [&'a str],
    checksum: &'a str,
}

fn dependency(
    dependencies: &toml::value::Table,
    name: &str,
    contract: Dependency<'_>,
) -> Result<(), String> {
    let value = dependencies
        .get(name)
        .and_then(toml::Value::as_table)
        .ok_or_else(|| format!("text dependency missing: {name}"))?;
    exact_bool_table(value, "default_features", false)?;
    for (field, expected) in contract.strings {
        exact_string_table(value, field, expected)?;
    }
    if !contract.requested_features.is_empty() {
        exact_array_table(value, "features", contract.requested_features)?;
    } else if value.contains_key("features") {
        return Err(format!("text dependency feature posture drifted: {name}"));
    }
    exact_array_table(value, "resolved_features", contract.resolved_features)?;
    exact_string_table(value, "checksum", contract.checksum)?;
    let expected_fields =
        3 + contract.strings.len() + usize::from(!contract.requested_features.is_empty());
    if value.len() != expected_fields {
        return Err(format!("text dependency fields drifted: {name}"));
    }
    Ok(())
}

fn validate_unicode(value: &toml::value::Table) -> Result<(), String> {
    for (field, expected) in [
        ("grapheme", "unicode/ucd/auxiliary/GraphemeBreakTest.txt"),
        ("word", "unicode/ucd/auxiliary/WordBreakTest.txt"),
        ("line", "unicode/ucd/auxiliary/LineBreakTest.txt"),
        ("bidi", "unicode/ucd/BidiTest.txt"),
        ("bidi_character", "unicode/ucd/BidiCharacterTest.txt"),
        ("emoji_test", "unicode/emoji/emoji-test.txt"),
        ("emoji_sequences", "unicode/emoji/emoji-sequences.txt"),
        (
            "emoji_zwj_sequences",
            "unicode/emoji/emoji-zwj-sequences.txt",
        ),
        ("dictionary_segmentation", "icu-segmenter-2.2.0-auto-lstm"),
    ] {
        exact_string_table(value, field, expected)?;
    }
    (value.len() == 9)
        .then_some(())
        .ok_or_else(|| "Unicode corpus inventory drifted".to_owned())
}

fn validate_layout(value: &toml::value::Table) -> Result<(), String> {
    exact_array_table(value, "writing_modes", &["horizontal"])?;
    exact_array_table(value, "wrap_modes", &["none", "unicode-word", "grapheme"])?;
    exact_array_table(value, "alignments", &["start", "center", "end"])?;
    exact_array_table(value, "overflow", &["clip", "ellipsis"])?;
    exact_string_table(value, "whitespace", "preserved")?;
    exact_string_table(value, "tab_stops", "explicit")?;
    exact_string_table(value, "hard_line_breaks", "preserved")?;
    exact_string_table(value, "line_height", "explicit")?;
    exact_string_table(value, "letter_spacing", "explicit")?;
    exact_string_table(value, "word_spacing", "explicit")?;
    exact_string_table(value, "maximum_lines", "explicit")?;
    exact_string_table(value, "cluster_level", "monotone-graphemes")?;
    exact_string_table(
        value,
        "caret_boundaries",
        "grapheme-and-indivisible-shaping-cluster",
    )?;
    exact_string_table(
        value,
        "bidi_caret",
        "original-byte-boundary-plus-visual-edge-plus-upstream-downstream-affinity",
    )?;
    exact_string_table(
        value,
        "hit_testing",
        "point-to-line-to-visual-run-to-cluster",
    )?;
    exact_string_table(value, "selection", "discontiguous-visual-run-rectangles")?;
    exact_string_table(
        value,
        "bidi_boundary_rule",
        "two-affine-caret-positions-at-shared-visual-edge",
    )?;
    policy::exact_fields(value, 17, "text layout contract")
}

fn validate_capacity_admission(value: &toml::value::Table) -> Result<(), String> {
    exact_string_table(value, "input_reservation", "exact-before-analysis")?;
    exact_string_table(
        value,
        "derived_reservation",
        "conservative-qualified-font-expansion-bound",
    )?;
    exact_string_table(
        value,
        "staging",
        "bounded-effect-free-analysis-shaping-and-layout",
    )?;
    exact_string_table(
        value,
        "overflow",
        "typed-atomic-before-publication-raster-upload-or-retention",
    )?;
    exact_bool_table(value, "partial_publication", false)?;
    policy::exact_fields(value, 5, "capacity admission contract")
}

fn validate_locality(value: &toml::value::Table) -> Result<(), String> {
    exact_string_table(
        value,
        "content_edit",
        "one-paragraph-with-width-and-all-generations-fixed",
    )?;
    exact_string_table(value, "paragraph_width_edit", "one-paragraph-only")?;
    exact_string_table(
        value,
        "global_width_edit",
        "explicit-document-wide-layout-replacement",
    )?;
    exact_array_table(value, "proof_sizes", &["1", "32", "2048", "4096"])?;
    exact_string_table(
        value,
        "unchanged_siblings",
        "zero-analysis-zero-shaping-zero-layout",
    )?;
    policy::exact_fields(value, 5, "text locality contract")
}

fn validate_capacities(value: &toml::value::Table) -> Result<(), String> {
    let exact = [
        ("retained_paragraphs", 4_096),
        ("retained_utf8_bytes", 8_388_608),
        ("paragraph_utf8_bytes", 65_536),
        ("glyphs", 262_144),
        ("grapheme_cluster_records", 262_144),
        ("line_records", 65_536),
        ("runs_per_paragraph", 32),
        ("application_font_faces", 64),
        ("application_font_bytes", 67_108_864),
        ("alpha_atlas_pages", 4),
        ("alpha_atlas_width", 1_024),
        ("alpha_atlas_height", 1_024),
        ("color_atlas_pages", 2),
        ("color_atlas_width", 2_048),
        ("color_atlas_height", 2_048),
        ("atlas_entries", 8_192),
        ("maximum_glyph_width", 512),
        ("maximum_glyph_height", 512),
        ("atlas_texel_bytes", 37_748_736),
        ("staged_upload_bytes", 8_388_608),
    ];
    for (field, expected) in exact {
        if value.get(field).and_then(toml::Value::as_integer) != Some(expected) {
            return Err(format!("text capacity drifted: {field}"));
        }
    }
    (value.len() == exact.len())
        .then_some(())
        .ok_or_else(|| "text capacity inventory drifted".to_owned())
}

fn table<'a>(value: &'a toml::Value, key: &str) -> Result<&'a toml::value::Table, String> {
    value
        .get(key)
        .and_then(toml::Value::as_table)
        .ok_or_else(|| format!("missing text profile table: {key}"))
}

fn exact_string(value: &toml::Value, field: &str, expected: &str) -> Result<(), String> {
    value
        .get(field)
        .and_then(toml::Value::as_str)
        .filter(|actual| *actual == expected)
        .map(|_| ())
        .ok_or_else(|| format!("text profile field drifted: {field}"))
}

fn exact_bool(value: &toml::Value, field: &str, expected: bool) -> Result<(), String> {
    value
        .get(field)
        .and_then(toml::Value::as_bool)
        .filter(|actual| *actual == expected)
        .map(|_| ())
        .ok_or_else(|| format!("text profile field drifted: {field}"))
}

fn exact_string_table(
    value: &toml::value::Table,
    field: &str,
    expected: &str,
) -> Result<(), String> {
    value
        .get(field)
        .and_then(toml::Value::as_str)
        .filter(|actual| *actual == expected)
        .map(|_| ())
        .ok_or_else(|| format!("text profile field drifted: {field}"))
}

fn exact_bool_table(value: &toml::value::Table, field: &str, expected: bool) -> Result<(), String> {
    value
        .get(field)
        .and_then(toml::Value::as_bool)
        .filter(|actual| *actual == expected)
        .map(|_| ())
        .ok_or_else(|| format!("text profile field drifted: {field}"))
}

fn exact_array_table(
    value: &toml::value::Table,
    field: &str,
    expected: &[&str],
) -> Result<(), String> {
    let actual = value
        .get(field)
        .and_then(toml::Value::as_array)
        .ok_or_else(|| format!("text profile array missing: {field}"))?;
    let actual = actual
        .iter()
        .map(toml::Value::as_str)
        .collect::<Option<Vec<_>>>();
    (actual.as_deref() == Some(expected))
        .then_some(())
        .ok_or_else(|| format!("text profile array drifted: {field}"))
}
