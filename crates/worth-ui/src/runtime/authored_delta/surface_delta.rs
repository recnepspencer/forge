use std::collections::{BTreeMap, BTreeSet};

use crate::runtime::{
    appearance_state_prop_schema, event_geometry_prop_schema, flow_layout_prop_schema,
    interaction_prop_schema, primitive_authored_prop_schema, primitive_content_prop_schema,
    WorthUiCandidateRuntimeAuthoringSnapshot, WorthUiRuntimeAuthoringSnapshot,
    WorthUiSemanticSliceId, WorthUiSemanticSliceInventory,
};

use super::lowering_support::{change_posture, insert_surface_semantic_row};
use super::{
    WorthUiAuthoredDeclarationKind, WorthUiAuthoredDeltaChangePosture,
    WorthUiTouchedAuthoredDeclarationRow, WorthUiTouchedAuthoredSemanticSliceRow,
};

pub(crate) fn lower_surface_component_delta(
    active: Option<&WorthUiRuntimeAuthoringSnapshot>,
    candidate: &WorthUiCandidateRuntimeAuthoringSnapshot,
    inventory: &WorthUiSemanticSliceInventory,
    declaration_rows: &mut BTreeSet<WorthUiTouchedAuthoredDeclarationRow>,
    semantic_rows: &mut BTreeSet<WorthUiTouchedAuthoredSemanticSliceRow>,
) {
    let active_surfaces = active_surface_components(active);
    let candidate_surfaces = candidate_surface_components(candidate);
    let surface_ids = active_surfaces
        .keys()
        .chain(candidate_surfaces.keys())
        .cloned()
        .collect::<BTreeSet<_>>();

    for surface_id in surface_ids {
        let active_component = active_surfaces.get(&surface_id);
        let candidate_component = candidate_surfaces.get(&surface_id);
        let Some(change_posture) = change_posture(
            active_component.map(|value| digest_text(value)),
            candidate_component.map(|value| digest_text(value)),
        ) else {
            continue;
        };
        declaration_rows.insert(WorthUiTouchedAuthoredDeclarationRow::new(
            WorthUiAuthoredDeclarationKind::Surface,
            &surface_id,
            change_posture,
        ));
        insert_surface_semantic_row(
            semantic_rows,
            inventory,
            WorthUiSemanticSliceId::AuthoredMountComponentSelection,
            &surface_id,
            change_posture,
        );
    }
}

pub(crate) fn lower_surface_props_delta(
    active: Option<&WorthUiRuntimeAuthoringSnapshot>,
    candidate: &WorthUiCandidateRuntimeAuthoringSnapshot,
    inventory: &WorthUiSemanticSliceInventory,
    declaration_rows: &mut BTreeSet<WorthUiTouchedAuthoredDeclarationRow>,
    semantic_rows: &mut BTreeSet<WorthUiTouchedAuthoredSemanticSliceRow>,
) {
    let active_props = active_surface_prop_digests(active);
    let candidate_props = candidate_surface_prop_digests(candidate);
    let surface_ids = active_props
        .keys()
        .chain(candidate_props.keys())
        .map(|(surface_id, _)| surface_id.to_owned())
        .collect::<BTreeSet<_>>();

    for surface_id in surface_ids {
        let touched_keys =
            touched_prop_keys_for_surface(&active_props, &candidate_props, &surface_id);
        let Some(surface_change_posture) = surface_change_posture(&touched_keys) else {
            continue;
        };
        declaration_rows.insert(WorthUiTouchedAuthoredDeclarationRow::new(
            WorthUiAuthoredDeclarationKind::Surface,
            &surface_id,
            surface_change_posture,
        ));
        insert_surface_semantic_row(
            semantic_rows,
            inventory,
            WorthUiSemanticSliceId::AuthoredSurfaceInstanceProps,
            &surface_id,
            surface_change_posture,
        );
        for touched_key in touched_keys {
            insert_primitive_prop_semantic_rows(
                semantic_rows,
                inventory,
                &surface_id,
                touched_key.key.as_str(),
                touched_key.change_posture,
            );
        }
    }
}

struct TouchedSurfacePropKey {
    key: String,
    change_posture: WorthUiAuthoredDeltaChangePosture,
}

fn active_surface_components(
    active: Option<&WorthUiRuntimeAuthoringSnapshot>,
) -> BTreeMap<String, String> {
    active
        .map(|snapshot| {
            snapshot
                .authored_surfaces()
                .entries()
                .iter()
                .map(|entry| {
                    (
                        entry.surface_id().to_owned(),
                        entry.component_id().to_owned(),
                    )
                })
                .collect()
        })
        .unwrap_or_default()
}

fn candidate_surface_components(
    candidate: &WorthUiCandidateRuntimeAuthoringSnapshot,
) -> BTreeMap<String, String> {
    candidate
        .authored_surfaces()
        .entries()
        .iter()
        .map(|entry| {
            (
                entry.surface_id().to_owned(),
                entry.component_id().to_owned(),
            )
        })
        .collect()
}

fn active_surface_prop_digests(
    active: Option<&WorthUiRuntimeAuthoringSnapshot>,
) -> BTreeMap<(String, String), u64> {
    active
        .map(|snapshot| {
            snapshot
                .authored_surface_props()
                .entries()
                .iter()
                .map(|entry| {
                    (
                        (entry.surface_id().to_owned(), entry.key().to_owned()),
                        entry.digest(),
                    )
                })
                .collect()
        })
        .unwrap_or_default()
}

fn candidate_surface_prop_digests(
    candidate: &WorthUiCandidateRuntimeAuthoringSnapshot,
) -> BTreeMap<(String, String), u64> {
    candidate
        .authored_surface_props()
        .entries()
        .iter()
        .map(|entry| {
            (
                (entry.surface_id().to_owned(), entry.key().to_owned()),
                entry.digest(),
            )
        })
        .collect()
}

fn touched_prop_keys_for_surface(
    active_props: &BTreeMap<(String, String), u64>,
    candidate_props: &BTreeMap<(String, String), u64>,
    surface_id: &str,
) -> Vec<TouchedSurfacePropKey> {
    active_props
        .keys()
        .chain(candidate_props.keys())
        .filter(|(prop_surface_id, _)| prop_surface_id == surface_id)
        .map(|(_, key)| key.to_owned())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .filter_map(|key| {
            let identity = (surface_id.to_owned(), key.clone());
            change_posture(
                active_props.get(&identity).copied(),
                candidate_props.get(&identity).copied(),
            )
            .map(|change_posture| TouchedSurfacePropKey {
                key,
                change_posture,
            })
        })
        .collect()
}

fn surface_change_posture(
    touched_keys: &[TouchedSurfacePropKey],
) -> Option<WorthUiAuthoredDeltaChangePosture> {
    let first = touched_keys.first()?.change_posture;
    if touched_keys.iter().all(|key| key.change_posture == first) {
        Some(first)
    } else {
        Some(WorthUiAuthoredDeltaChangePosture::Changed)
    }
}

fn insert_primitive_prop_semantic_rows(
    semantic_rows: &mut BTreeSet<WorthUiTouchedAuthoredSemanticSliceRow>,
    inventory: &WorthUiSemanticSliceInventory,
    surface_id: &str,
    prop_key: &str,
    change_posture: WorthUiAuthoredDeltaChangePosture,
) {
    let slice_id = primitive_authored_prop_schema(prop_key)
        .map(|schema| schema.semantic_slice())
        .or_else(|| primitive_content_prop_schema(prop_key).map(|schema| schema.semantic_slice()))
        .or_else(|| flow_layout_prop_schema(prop_key).map(|schema| schema.semantic_slice()))
        .or_else(|| event_geometry_prop_schema(prop_key).map(|schema| schema.semantic_slice()))
        .or_else(|| interaction_prop_schema(prop_key).map(|schema| schema.semantic_slice()))
        .or_else(|| appearance_state_prop_schema(prop_key).map(|schema| schema.semantic_slice()));
    let Some(slice_id) = slice_id else {
        return;
    };
    {
        insert_surface_semantic_row(
            semantic_rows,
            inventory,
            slice_id,
            surface_id,
            change_posture,
        );
    }
}

fn digest_text(value: &str) -> u64 {
    value.bytes().fold(0xcbf2_9ce4_8422_2325, |digest, byte| {
        let digest = digest ^ u64::from(byte);
        digest.wrapping_mul(0x0000_0100_0000_01b3)
    })
}
