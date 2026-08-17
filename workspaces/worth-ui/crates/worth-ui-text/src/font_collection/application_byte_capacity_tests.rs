use std::sync::Arc;

use worth_ui_host_contract::{UiFontCollectionGeneration, UiFontSlant};

use super::{
    application_test_world::{face, profile_collection_and_sources},
    UiApplicationFontFaceDefinition, UiApplicationFontLicenseRecord,
    UiApplicationFontPackDefinition, UiFontCollectionAdmissionDenial,
};

#[test]
fn byte_capacity_precedes_parsing_deduplicates_and_subtracts_replaced_pack() {
    let (profile, sources) = profile_collection_and_sources();
    let limit = crate::UiGlobalTextProfile::MAX_APPLICATION_FONT_BYTES;
    assert_oversized_malformed_denied(&profile, limit);
    assert_aggregate_overflow_denied(&profile, &sources["noto-sans-roman"], limit);
    let exact_bytes = padded_font(&sources["noto-sans-roman"], limit, 0);
    let exact = UiApplicationFontPackDefinition {
        name: Arc::from("exact deduplicated capacity"),
        faces: Box::new([
            face("Exact Alpha", exact_bytes.clone(), 0, UiFontSlant::Upright),
            face("Exact Beta", exact_bytes, 0, UiFontSlant::Upright),
        ]),
    };
    let (generation_two, receipt, cost) = profile
        .register_application_pack(UiFontCollectionGeneration::new(2).unwrap(), exact)
        .unwrap();
    assert_eq!(generation_two.application_font_bytes(), limit);
    assert_eq!(cost.bytes_hashed(), (limit as u64) * 2);
    assert_replacement_subtracts_predecessor(
        generation_two,
        receipt.identity(),
        &sources["noto-sans-roman"],
        limit,
    );
}

fn assert_oversized_malformed_denied(profile: &super::UiGlobalFontCollection, limit: usize) {
    let oversized = UiApplicationFontPackDefinition {
        name: Arc::from("oversized malformed"),
        faces: Box::new([malformed_face(vec![0; limit + 1].into())]),
    };
    assert_eq!(
        profile
            .register_application_pack(UiFontCollectionGeneration::new(2).unwrap(), oversized)
            .err()
            .unwrap(),
        UiFontCollectionAdmissionDenial::ApplicationFontByteCapacityExceeded
    );
}

fn assert_aggregate_overflow_denied(
    profile: &super::UiGlobalFontCollection,
    source: &[u8],
    limit: usize,
) {
    let half_over = limit / 2 + 1;
    let aggregate_overflow = UiApplicationFontPackDefinition {
        name: Arc::from("aggregate overflow"),
        faces: Box::new([
            face(
                "Aggregate Alpha",
                padded_font(source, half_over, 1),
                0,
                UiFontSlant::Upright,
            ),
            face(
                "Aggregate Beta",
                padded_font(source, half_over, 2),
                0,
                UiFontSlant::Upright,
            ),
        ]),
    };
    assert_eq!(
        profile
            .register_application_pack(
                UiFontCollectionGeneration::new(2).unwrap(),
                aggregate_overflow,
            )
            .err()
            .unwrap(),
        UiFontCollectionAdmissionDenial::ApplicationFontByteCapacityExceeded
    );
}

fn assert_replacement_subtracts_predecessor(
    generation_two: super::UiGlobalFontCollection,
    predecessor: worth_ui_host_contract::UiQualifiedFontPackIdentity,
    source: &[u8],
    limit: usize,
) {
    let replacement_bytes = padded_font(source, limit, 1);
    let replacement = UiApplicationFontPackDefinition {
        name: Arc::from("replacement exact capacity"),
        faces: Box::new([face(
            "Replacement",
            replacement_bytes,
            0,
            UiFontSlant::Upright,
        )]),
    };
    let (generation_three, _, _) = generation_two
        .replace_application_pack(
            predecessor,
            UiFontCollectionGeneration::new(3).unwrap(),
            replacement,
        )
        .unwrap();
    assert_eq!(generation_three.application_font_bytes(), limit);
}

fn malformed_face(bytes: Arc<[u8]>) -> UiApplicationFontFaceDefinition {
    UiApplicationFontFaceDefinition {
        family: Arc::from("Malformed"),
        bytes,
        face_index: 0,
        weight: 400,
        width_milli_percent: 100_000,
        slant: UiFontSlant::Upright,
        license: UiApplicationFontLicenseRecord {
            identifier: Arc::from("OFL-1.1"),
            notice: Arc::from("Capacity denial must precede parsing."),
        },
    }
}

fn padded_font(source: &[u8], len: usize, trailing_marker: u8) -> Arc<[u8]> {
    let mut bytes = source.to_vec();
    bytes.resize(len, 0);
    *bytes.last_mut().unwrap() = trailing_marker;
    bytes.into()
}
