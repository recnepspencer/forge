use std::sync::Arc;

use worth_ui_host_contract::{UiFontCollectionGeneration, UiFontSlant};

use super::application_test_world::{
    assert_pack_denial, face, fallback_result, layout, profile_collection_and_sources,
};
use super::*;
use crate::{UiFontFamilyStack, UiTextFaceRequest};

#[test]
pub(super) fn collection_indices_and_pack_generations_are_exact_and_old_layouts_pin_bytes() {
    let (profile, sources) = profile_collection_and_sources();
    let cjk = sources["noto-sans-cjk-jp"].clone();
    let definition = |index| UiApplicationFontPackDefinition {
        name: Arc::from(format!("CJK generation {index}")),
        faces: Box::new([face(
            "Application CJK",
            cjk.clone(),
            index,
            UiFontSlant::Upright,
        )]),
    };
    let (generation_two, receipt_two, _) = profile
        .register_application_pack(UiFontCollectionGeneration::new(2).unwrap(), definition(0))
        .unwrap();
    let family_two = receipt_two.family("Application CJK").unwrap();
    let face_two = receipt_two.faces()[0].identity();
    assert_eq!(face_two.face_index(), 0);
    let generation_two = Arc::new(generation_two);
    let old_layout = layout(Arc::clone(&generation_two), family_two, "\u{6F22}\u{5B57}");
    assert_eq!(old_layout.view().logical_runs()[0].face(), face_two);
    assert_eq!(old_layout.artifact().identity(), old_layout.identity());
    assert!(Arc::ptr_eq(
        old_layout
            .artifact()
            .face_resource(face_two)
            .unwrap()
            .bytes(),
        &cjk
    ));
    assert!(old_layout.artifact().coverage().iter().all(|coverage| {
        coverage.attempted_collection() == UiFontCollectionGeneration::new(2).unwrap()
    }));

    let (generation_three, receipt_three, _) = generation_two
        .replace_application_pack(
            receipt_two.identity(),
            UiFontCollectionGeneration::new(3).unwrap(),
            definition(1),
        )
        .unwrap();
    let family_three = receipt_three.family("Application CJK").unwrap();
    let face_three = receipt_three.faces()[0].identity();
    assert_eq!(face_three.face_index(), 1);
    assert_ne!(face_two, face_three);
    let stale_denial = match fallback_result(
        Arc::clone(&generation_two),
        UiFontFamilyStack::new(Box::new([family_two])).unwrap(),
        UiTextFaceRequest::regular(),
        "fresh work after successor publication",
    ) {
        Ok(_) => panic!("retired collection generation admitted fresh text"),
        Err(denial) => denial,
    };
    assert_eq!(
        stale_denial,
        crate::UiTextFallbackDenial::StaleFontCollectionGeneration
    );
    let new_layout = layout(Arc::new(generation_three), family_three, "\u{6F22}\u{5B57}");
    assert_eq!(new_layout.view().logical_runs()[0].face(), face_three);
    assert_eq!(old_layout.pinned_font_collection().generation().get(), 2);
    assert!(old_layout.pinned_font_collection().contains_face(face_two));

    let removed = new_layout
        .pinned_font_collection()
        .remove_application_pack(
            receipt_three.identity(),
            UiFontCollectionGeneration::new(4).unwrap(),
        )
        .unwrap();
    assert!(removed.application_packs().is_empty());
    assert_eq!(removed.application_font_bytes(), 0);
    assert!(matches!(
        fallback_result(
            Arc::new(removed),
            UiFontFamilyStack::new(Box::new([family_three])).unwrap(),
            UiTextFaceRequest::regular(),
            "removed family cannot open profile fallback",
        ),
        Err(crate::UiTextFallbackDenial::ForeignFontFamily)
    ));
    let denial = match generation_two
        .register_application_pack(UiFontCollectionGeneration::new(4).unwrap(), definition(0))
    {
        Ok(_) => panic!("stale collection transition was admitted"),
        Err(denial) => denial,
    };
    assert_eq!(
        denial,
        UiFontCollectionAdmissionDenial::StaleCollectionGeneration
    );
}

#[test]
pub(super) fn malformed_ambiguous_unsupported_and_over_capacity_packs_deny_atomically() {
    let (profile, sources) = profile_collection_and_sources();
    let mismatched_face = UiApplicationFontFaceDefinition {
        weight: 700,
        ..face(
            "Static Math",
            sources["noto-sans-math"].clone(),
            0,
            UiFontSlant::Upright,
        )
    };
    assert!(matches!(
        application_pack::validate_face_definition_for_test(&mismatched_face),
        Err(UiFontCollectionAdmissionDenial::FaceMetadataMismatch)
    ));
    let malformed_metadata = UiApplicationFontPackDefinition {
        name: Arc::from("metadata mismatch"),
        faces: Box::new([mismatched_face]),
    };
    assert_pack_denial(
        &profile,
        malformed_metadata,
        UiFontCollectionAdmissionDenial::FaceMetadataMismatch,
    );

    let ambiguous = UiApplicationFontPackDefinition {
        name: Arc::from("ambiguous family"),
        faces: Box::new([
            face(
                "Ambiguous",
                sources["noto-sans-roman"].clone(),
                0,
                UiFontSlant::Upright,
            ),
            face(
                "Ambiguous",
                sources["noto-sans-math"].clone(),
                0,
                UiFontSlant::Upright,
            ),
        ]),
    };
    assert_pack_denial(
        &profile,
        ambiguous,
        UiFontCollectionAdmissionDenial::AmbiguousFaceSelection,
    );

    let svg_bytes = rename_table(sources["noto-sans-roman"].clone(), *b"name", *b"SVG ");
    assert_pack_denial(
        &profile,
        UiApplicationFontPackDefinition {
            name: Arc::from("unsupported SVG"),
            faces: Box::new([face("SVG", svg_bytes, 0, UiFontSlant::Upright)]),
        },
        UiFontCollectionAdmissionDenial::UnsupportedColorFontTable,
    );

    let faces = (0..=crate::UiGlobalTextProfile::MAX_APPLICATION_FONT_FACES)
        .map(|index| {
            face(
                &format!("Family {index}"),
                sources["noto-sans-roman"].clone(),
                0,
                UiFontSlant::Upright,
            )
        })
        .collect::<Vec<_>>()
        .into_boxed_slice();
    assert_pack_denial(
        &profile,
        UiApplicationFontPackDefinition {
            name: Arc::from("too many faces"),
            faces,
        },
        UiFontCollectionAdmissionDenial::ApplicationFaceCapacityExceeded,
    );

    let malformed_over_capacity = (0..=crate::UiGlobalTextProfile::MAX_APPLICATION_FONT_FACES)
        .map(|index| UiApplicationFontFaceDefinition {
            family: Arc::from(format!("Malformed {index}")),
            bytes: Arc::from(&b"not a font"[..]),
            face_index: 0,
            weight: 400,
            width_milli_percent: 100_000,
            slant: UiFontSlant::Upright,
            license: UiApplicationFontLicenseRecord {
                identifier: Arc::from("OFL-1.1"),
                notice: Arc::from("Capacity must deny before these bytes are parsed."),
            },
        })
        .collect::<Vec<_>>()
        .into_boxed_slice();
    assert_pack_denial(
        &profile,
        UiApplicationFontPackDefinition {
            name: Arc::from("malformed but over capacity"),
            faces: malformed_over_capacity,
        },
        UiFontCollectionAdmissionDenial::ApplicationFaceCapacityExceeded,
    );

    let color = UiApplicationFontPackDefinition {
        name: Arc::from("application color emoji"),
        faces: Box::new([face(
            "Application Emoji",
            sources["noto-color-emoji"].clone(),
            0,
            UiFontSlant::Upright,
        )]),
    };
    let (with_color, receipt, _) = profile
        .register_application_pack(UiFontCollectionGeneration::new(2).unwrap(), color)
        .unwrap();
    assert!(with_color.contains_face(receipt.faces()[0].identity()));
}

#[test]
pub(super) fn exhausted_collection_generation_cannot_alias_its_successor() {
    let (profile, sources) = profile_collection_and_sources();
    let (maximum, _) = UiGlobalFontCollection::admit_profile(
        UiFontCollectionGeneration::new(u64::MAX).unwrap(),
        profile_inputs_from_repository(),
    )
    .unwrap();
    let denial = match maximum.register_application_pack(
        UiFontCollectionGeneration::new(u64::MAX).unwrap(),
        UiApplicationFontPackDefinition {
            name: Arc::from("generation exhaustion"),
            faces: Box::new([face(
                "No aliased successor",
                sources["noto-sans-roman"].clone(),
                0,
                UiFontSlant::Upright,
            )]),
        },
    ) {
        Ok(_) => panic!("maximum collection generation was reused"),
        Err(denial) => denial,
    };
    assert_eq!(
        denial,
        UiFontCollectionAdmissionDenial::CollectionGenerationExhausted
    );
    assert!(profile.is_current_for_admission());
}

fn rename_table(bytes: Arc<[u8]>, from: [u8; 4], to: [u8; 4]) -> Arc<[u8]> {
    let mut mutated = bytes.to_vec();
    let table_count = usize::from(u16::from_be_bytes([mutated[4], mutated[5]]));
    let record = (0..table_count)
        .map(|index| 12 + index * 16)
        .find(|start| mutated[*start..*start + 4] == from)
        .expect("fixture owns the substituted table");
    mutated[record..record + 4].copy_from_slice(&to);
    let mut directory = (0..table_count)
        .map(|index| {
            let start = 12 + index * 16;
            <[u8; 16]>::try_from(&mutated[start..start + 16]).unwrap()
        })
        .collect::<Vec<_>>();
    directory.sort_by_key(|entry| <[u8; 4]>::try_from(&entry[..4]).unwrap());
    for (index, entry) in directory.into_iter().enumerate() {
        let start = 12 + index * 16;
        mutated[start..start + 16].copy_from_slice(&entry);
    }
    Arc::from(mutated)
}
