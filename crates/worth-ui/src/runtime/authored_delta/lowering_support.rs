use std::collections::BTreeMap;

use crate::runtime::authoring_snapshot::WorthUiAuthoringCatalogEntry;
use crate::runtime::{
    WorthUiAuthoredDeltaChangePosture, WorthUiAuthoredDeltaCounters, WorthUiAuthoredDeltaDigest,
    WorthUiAuthoredSemanticSubject, WorthUiCandidateRuntimeAuthoringSnapshot,
    WorthUiRuntimeAuthoringSnapshot, WorthUiSemanticSliceId, WorthUiSemanticSliceInventory,
    WorthUiTouchedAuthoredDeclarationRow, WorthUiTouchedAuthoredSemanticSliceRow,
};
use crate::source::{WorthUiPageContentSlots, WorthUiPageLayoutTopology};

pub(super) fn authored_delta_digest(
    counters: &WorthUiAuthoredDeltaCounters,
    declaration_rows: &[WorthUiTouchedAuthoredDeclarationRow],
    semantic_rows: &[WorthUiTouchedAuthoredSemanticSliceRow],
) -> WorthUiAuthoredDeltaDigest {
    let mut parts = vec![
        format!("observed_modules:{}", counters.observed_modules()),
        format!("parsed_modules:{}", counters.parsed_modules()),
        format!(
            "authored_declarations_inspected:{}",
            counters.authored_declarations_inspected()
        ),
        format!(
            "authored_declarations_touched:{}",
            counters.authored_declarations_touched()
        ),
        format!(
            "semantic_slices_emitted:{}",
            counters.semantic_slices_emitted()
        ),
    ];
    parts.extend(declaration_rows.iter().map(|row| {
        format!(
            "declaration:{:?}|{}|{:?}",
            row.kind(),
            row.declaration_name(),
            row.change_posture()
        )
    }));
    parts.extend(semantic_rows.iter().map(|row| {
        format!(
            "semantic:{:?}|{:?}|{:?}",
            row.slice_id(),
            row.subject(),
            row.change_posture()
        )
    }));
    parts.sort();
    WorthUiAuthoredDeltaDigest::from_basis(&parts)
}

pub(super) fn change_posture(
    active: Option<u64>,
    candidate: Option<u64>,
) -> Option<WorthUiAuthoredDeltaChangePosture> {
    match (active, candidate) {
        (None, None) => None,
        (None, Some(_)) => Some(WorthUiAuthoredDeltaChangePosture::Added),
        (Some(_), None) => Some(WorthUiAuthoredDeltaChangePosture::Removed),
        (Some(left), Some(right)) if left == right => None,
        (Some(_), Some(_)) => Some(WorthUiAuthoredDeltaChangePosture::Changed),
    }
}

pub(super) fn catalog_map(entries: &[WorthUiAuthoringCatalogEntry]) -> BTreeMap<String, u64> {
    entries
        .iter()
        .map(|entry| (entry.name().to_owned(), entry.digest()))
        .collect()
}

pub(super) fn layout_topology_by_page(
    snapshot: &WorthUiRuntimeAuthoringSnapshot,
) -> BTreeMap<String, &WorthUiPageLayoutTopology> {
    snapshot
        .layout_topology()
        .pages()
        .iter()
        .map(|page| (page.page_name().to_owned(), page))
        .collect()
}

pub(super) fn candidate_layout_topology_by_page(
    snapshot: &WorthUiCandidateRuntimeAuthoringSnapshot,
) -> BTreeMap<String, &WorthUiPageLayoutTopology> {
    snapshot
        .layout_topology()
        .pages()
        .iter()
        .map(|page| (page.page_name().to_owned(), page))
        .collect()
}

pub(super) fn content_slots_by_page(
    snapshot: &WorthUiRuntimeAuthoringSnapshot,
) -> BTreeMap<String, BTreeMap<String, &crate::source::WorthUiContentSlotAssignment>> {
    snapshot
        .content_slots()
        .pages()
        .iter()
        .map(|page| (page.page_name().to_owned(), assignments_by_slot(page)))
        .collect()
}

pub(super) fn candidate_content_slots_by_page(
    snapshot: &WorthUiCandidateRuntimeAuthoringSnapshot,
) -> BTreeMap<String, BTreeMap<String, &crate::source::WorthUiContentSlotAssignment>> {
    snapshot
        .content_slots()
        .pages()
        .iter()
        .map(|page| (page.page_name().to_owned(), assignments_by_slot(page)))
        .collect()
}

pub(super) fn insert_page_semantic_row(
    semantic_rows: &mut std::collections::BTreeSet<WorthUiTouchedAuthoredSemanticSliceRow>,
    inventory: &WorthUiSemanticSliceInventory,
    slice_id: WorthUiSemanticSliceId,
    page_name: &str,
    change_posture: WorthUiAuthoredDeltaChangePosture,
) {
    ensure_inventory_slice(inventory, slice_id);
    semantic_rows.insert(WorthUiTouchedAuthoredSemanticSliceRow::new(
        slice_id,
        WorthUiAuthoredSemanticSubject::Page {
            page_name: page_name.to_owned(),
        },
        change_posture,
    ));
}

pub(super) fn insert_surface_semantic_row(
    semantic_rows: &mut std::collections::BTreeSet<WorthUiTouchedAuthoredSemanticSliceRow>,
    inventory: &WorthUiSemanticSliceInventory,
    slice_id: WorthUiSemanticSliceId,
    surface_id: &str,
    change_posture: WorthUiAuthoredDeltaChangePosture,
) {
    ensure_inventory_slice(inventory, slice_id);
    semantic_rows.insert(WorthUiTouchedAuthoredSemanticSliceRow::new(
        slice_id,
        WorthUiAuthoredSemanticSubject::Surface {
            surface_id: surface_id.to_owned(),
        },
        change_posture,
    ));
}

pub(super) fn ensure_inventory_slice(
    inventory: &WorthUiSemanticSliceInventory,
    slice_id: WorthUiSemanticSliceId,
) {
    inventory
        .slice(slice_id)
        .expect("authored delta lowering only emits registered semantic slices");
}

fn assignments_by_slot(
    page: &WorthUiPageContentSlots,
) -> BTreeMap<String, &crate::source::WorthUiContentSlotAssignment> {
    page.assignments()
        .iter()
        .map(|assignment| (assignment.slot_name().to_owned(), assignment))
        .collect()
}
