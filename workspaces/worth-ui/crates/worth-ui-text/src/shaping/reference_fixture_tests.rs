use std::{path::Path, sync::Arc};

use serde::Deserialize;
use sha2::{Digest, Sha256};
use worth_ui_host_contract::{
    UiFontCollectionGeneration, UiTextProfileGeneration, UiTextScaleGeneration,
};

use crate::{
    font_collection::profile_inputs_from_repository, UiAdmittedTextParagraph,
    UiAnalyzedTextParagraph, UiFallbackTextParagraph, UiGlobalFontCollection,
    UiQualifiedTextLayout, UiShapedTextParagraph, UiTextAlignment, UiTextBaseDirection,
    UiTextOverflow, UiTextParagraphAdmissionInput, UiTextParagraphConstraints,
    UiTextParagraphConstraintsInput, UiTextStyleSpan, UiTextWrap,
};

const FIXTURE: &str =
    include_str!("../../../../profiles/worth-ui-global-text-v2/fixtures/hb-shape-13.0.0.json");

#[derive(Deserialize)]
struct ReferenceFixture {
    schema: String,
    tool: ReferenceTool,
    profile_manifest_sha256: String,
    artifact_inventory_sha256: String,
    cases: Vec<ReferenceCase>,
}

#[derive(Deserialize)]
struct ReferenceTool {
    name: String,
    version: String,
    executable_sha256: String,
    release_zip_sha256: String,
    implementation: String,
    arguments: Vec<String>,
}

#[derive(Deserialize)]
struct ReferenceCase {
    name: String,
    source: String,
    font_path: String,
    font_sha256: String,
    direction: String,
    script: String,
    language: String,
    glyphs: Vec<ReferenceGlyph>,
}

#[derive(Deserialize)]
struct ReferenceGlyph {
    g: u32,
    cl: u32,
    dx: i32,
    dy: i32,
    ax: i32,
    ay: i32,
    range: [u32; 2],
    #[serde(default)]
    production_delta: [i32; 4],
}

#[test]
pub(crate) fn pinned_reference_harfbuzz_records_match_full_production_shaping() {
    let fixture: ReferenceFixture = serde_json::from_str(FIXTURE).expect("valid frozen fixture");
    validate_fixture_authority(&fixture);

    let generation = UiFontCollectionGeneration::new(1).unwrap();
    let (fonts, _) =
        UiGlobalFontCollection::admit_profile(generation, profile_inputs_from_repository())
            .unwrap();
    let fonts = Arc::new(fonts);
    for case in &fixture.cases {
        let layout = layout(case, generation, Arc::clone(&fonts));
        let view = layout.artifact().view();
        assert_eq!(
            view.logical_runs().len(),
            1,
            "{} logical run count",
            case.name
        );
        let run = view.logical_runs()[0];
        assert_eq!(
            run.script_tag(),
            script_tag(&case.script),
            "{} script",
            case.name
        );
        let resource = layout
            .artifact()
            .face_resource(run.face())
            .expect("selected run retains its exact font bytes");
        assert_eq!(
            sha256(resource.bytes()),
            case.font_sha256,
            "{} selected font",
            case.name
        );
        let actual = layout.glyphs();
        assert_eq!(actual.len(), case.glyphs.len(), "{} glyph count", case.name);
        for (index, (actual, expected)) in actual.iter().zip(&case.glyphs).enumerate() {
            let range = actual.original_range();
            assert_eq!(
                actual.glyph_id(),
                expected.g,
                "{} glyph {index} id",
                case.name
            );
            assert_eq!(
                range.start(),
                expected.range[0],
                "{} glyph {index} range start",
                case.name
            );
            assert_eq!(
                range.end(),
                expected.range[1],
                "{} glyph {index} range end",
                case.name
            );
            assert_eq!(
                range.start(),
                expected.cl,
                "{} glyph {index} cluster",
                case.name
            );
            assert_eq!(
                actual.x_advance_font_units(),
                expected.ax + expected.production_delta[2],
                "{} glyph {index} x advance",
                case.name
            );
            assert_eq!(
                actual.y_advance_font_units(),
                expected.ay + expected.production_delta[3],
                "{} glyph {index} y advance",
                case.name
            );
            assert_eq!(
                actual.x_offset_font_units(),
                expected.dx + expected.production_delta[0],
                "{} glyph {index} x offset",
                case.name
            );
            assert_eq!(
                actual.y_offset_font_units(),
                expected.dy + expected.production_delta[1],
                "{} glyph {index} y offset",
                case.name
            );
        }
    }
}

fn layout(
    case: &ReferenceCase,
    generation: UiFontCollectionGeneration,
    fonts: Arc<UiGlobalFontCollection>,
) -> UiQualifiedTextLayout {
    let base_direction = match case.direction.as_str() {
        "ltr" => UiTextBaseDirection::LeftToRight,
        "rtl" => UiTextBaseDirection::RightToLeft,
        other => panic!("unsupported fixture direction {other}"),
    };
    let constraints = UiTextParagraphConstraints::new(UiTextParagraphConstraintsInput {
        language: Arc::from(case.language.as_str()),
        base_direction,
        wrap: UiTextWrap::None,
        alignment: UiTextAlignment::Start,
        overflow: UiTextOverflow::Clip,
        font_size_millipoints: 14_000,
        width_millipoints: 1_000_000,
        line_height_millipoints: 18_000,
        letter_spacing_millipoints: 0,
        word_spacing_millipoints: 0,
        tab_interval_millipoints: 56_000,
        maximum_lines: 1,
    })
    .expect("fixture language and constraints are qualified");
    let styles = Box::new([
        UiTextStyleSpan::whole_paragraph(&case.source, &constraints).expect("nonempty fixture")
    ]);
    let (admitted, _) = UiAdmittedTextParagraph::admit(UiTextParagraphAdmissionInput {
        source: Arc::from(case.source.as_str()),
        constraints,
        profile_generation: UiTextProfileGeneration::new(1).unwrap(),
        font_collection_generation: generation,
        text_scale_generation: UiTextScaleGeneration::new(1).unwrap(),
        styles,
    })
    .unwrap();
    let analyzed = UiAnalyzedTextParagraph::analyze(admitted);
    let fallback = UiFallbackTextParagraph::select(analyzed, fonts).unwrap();
    let shaped = UiShapedTextParagraph::shape(fallback).unwrap();
    UiQualifiedTextLayout::layout(shaped).unwrap()
}

fn validate_fixture_authority(fixture: &ReferenceFixture) {
    assert_eq!(fixture.schema, "worth-ui-harfbuzz-reference-shaping-v1");
    assert_eq!(fixture.tool.name, "hb-shape");
    assert_eq!(fixture.tool.version, "13.0.0");
    assert_eq!(
        fixture.tool.implementation,
        "harfrust=0.12.0 tracks HarfBuzz 13.0.0"
    );
    assert_eq!(
        fixture.tool.arguments,
        [
            "--shapers=ot",
            "--output-format=json",
            "--no-glyph-names",
            "--utf8-clusters",
            "--cluster-level=0",
            "--variations=wght=400,wdth=100,ital=0,slnt=0",
        ]
    );
    assert_eq!(
        fixture.tool.executable_sha256,
        "2e5f548717d0bafff240f19a579d70bd69d8d9f73a7481d069397ef08d06f5d1"
    );
    assert_eq!(
        fixture.tool.release_zip_sha256,
        "d7ec9b71946f68fadc0d64d54238c66d7f17cfd80a840fcf2cbde05a5b0dd271"
    );
    let deltas = fixture
        .cases
        .iter()
        .enumerate()
        .flat_map(|(case_index, case)| {
            case.glyphs
                .iter()
                .enumerate()
                .filter(|(_, glyph)| glyph.production_delta != [0; 4])
                .map(move |(glyph_index, glyph)| (case_index, glyph_index, glyph.production_delta))
        })
        .collect::<Vec<_>>();
    assert_eq!(deltas, [(2, 4, [-1, 0, 0, 0])]);
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let profile = root.join("profiles/worth-ui-global-text-v2");
    assert_eq!(
        sha256_file(&profile.join("manifest.toml")),
        fixture.profile_manifest_sha256
    );
    assert_eq!(
        sha256_file(&profile.join("artifact-inventory.toml")),
        fixture.artifact_inventory_sha256
    );
    for case in &fixture.cases {
        assert_eq!(
            sha256_file(&profile.join(&case.font_path)),
            case.font_sha256,
            "{} frozen font",
            case.name
        );
    }
}

fn sha256_file(path: &Path) -> String {
    sha256(&std::fs::read(path).unwrap_or_else(|error| panic!("{}: {error}", path.display())))
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn script_tag(script: &str) -> [u8; 4] {
    script
        .as_bytes()
        .try_into()
        .unwrap_or_else(|_| panic!("fixture script tag must have four bytes: {script}"))
}
