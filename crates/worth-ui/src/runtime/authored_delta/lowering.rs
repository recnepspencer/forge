use std::collections::BTreeSet;

use crate::runtime::authoring_snapshot::WorthUiAuthoringCatalogEntry;
use crate::runtime::{
    WorthUiCandidateRuntimeAuthoringSnapshot, WorthUiRuntimeAuthoringSnapshot,
    WorthUiSemanticSliceId, WorthUiSemanticSliceInventory,
};
use crate::source::WorthUiParsedSourcePackage;

use super::layout_delta::{layout_gap_changed, layout_padding_changed, layout_topology_changed};
use super::lowering_support::{
    authored_delta_digest, candidate_content_slots_by_page, candidate_layout_topology_by_page,
    catalog_map, change_posture, content_slots_by_page, ensure_inventory_slice,
    insert_page_semantic_row, insert_surface_semantic_row, layout_topology_by_page,
};
use super::surface_delta::{lower_surface_component_delta, lower_surface_props_delta};
use super::{
    WorthUiAuthoredDeclarationKind, WorthUiAuthoredDeltaChangePosture,
    WorthUiAuthoredDeltaCounters, WorthUiAuthoredDeltaSummary, WorthUiAuthoredSemanticSubject,
    WorthUiTouchedAuthoredDeclarationRow, WorthUiTouchedAuthoredSemanticSliceRow,
};

pub(crate) fn lower_authored_delta_summary(
    observed_modules: usize,
    parsed: &WorthUiParsedSourcePackage,
    active: Option<&WorthUiRuntimeAuthoringSnapshot>,
    candidate: &WorthUiCandidateRuntimeAuthoringSnapshot,
    inventory: &WorthUiSemanticSliceInventory,
) -> WorthUiAuthoredDeltaSummary {
    let mut declaration_rows = BTreeSet::new();
    let mut semantic_rows = BTreeSet::new();

    lower_catalog_delta(
        WorthUiAuthoredDeclarationKind::Workspace,
        active
            .map(|snapshot| snapshot.workspace_shell().entries())
            .unwrap_or_default(),
        candidate.workspace_shell().entries(),
        WorthUiSemanticSliceId::ShellSlotAssignment,
        inventory,
        &mut declaration_rows,
        &mut semantic_rows,
        |name| WorthUiAuthoredSemanticSubject::Workspace {
            workspace_name: name.to_owned(),
        },
    );
    lower_catalog_delta(
        WorthUiAuthoredDeclarationKind::Appearance,
        active
            .map(|snapshot| snapshot.appearance_recipes().entries())
            .unwrap_or_default(),
        candidate.appearance_recipes().entries(),
        WorthUiSemanticSliceId::AppearanceRecipe,
        inventory,
        &mut declaration_rows,
        &mut semantic_rows,
        |name| WorthUiAuthoredSemanticSubject::AppearanceRecipe {
            recipe_name: name.to_owned(),
        },
    );
    lower_catalog_delta(
        WorthUiAuthoredDeclarationKind::RuntimeBinding,
        active
            .map(|snapshot| snapshot.runtime_bindings().entries())
            .unwrap_or_default(),
        candidate.runtime_bindings().entries(),
        WorthUiSemanticSliceId::AuthoredQueryBindingShape,
        inventory,
        &mut declaration_rows,
        &mut semantic_rows,
        |name| WorthUiAuthoredSemanticSubject::RuntimeBinding {
            binding_name: name.to_owned(),
        },
    );
    lower_surface_component_delta(
        active,
        candidate,
        inventory,
        &mut declaration_rows,
        &mut semantic_rows,
    );
    lower_surface_props_delta(
        active,
        candidate,
        inventory,
        &mut declaration_rows,
        &mut semantic_rows,
    );

    let active_page_declarations = active
        .map(|snapshot| catalog_map(snapshot.page_templates().entries()))
        .unwrap_or_default();
    let candidate_page_declarations = catalog_map(candidate.page_templates().entries());
    let page_names = active_page_declarations
        .keys()
        .chain(candidate_page_declarations.keys())
        .cloned()
        .collect::<BTreeSet<_>>();
    let active_layouts = active.map(layout_topology_by_page).unwrap_or_default();
    let candidate_layouts = candidate_layout_topology_by_page(candidate);
    let active_content_slots = active.map(content_slots_by_page).unwrap_or_default();
    let candidate_content_slots = candidate_content_slots_by_page(candidate);

    for page_name in page_names {
        let active_digest = active_page_declarations.get(&page_name).copied();
        let candidate_digest = candidate_page_declarations.get(&page_name).copied();
        let declaration_change_posture = change_posture(active_digest, candidate_digest);
        let layout_topology_changed = layout_topology_changed(
            active_layouts.get(&page_name).copied(),
            candidate_layouts.get(&page_name).copied(),
        );
        let layout_gap_changed = layout_gap_changed(
            active_layouts.get(&page_name).copied(),
            candidate_layouts.get(&page_name).copied(),
        );
        let layout_padding_changed = layout_padding_changed(
            active_layouts.get(&page_name).copied(),
            candidate_layouts.get(&page_name).copied(),
        );
        let layout_changed =
            layout_topology_changed || layout_gap_changed || layout_padding_changed;
        let active_slots_for_page = active_content_slots.get(&page_name);
        let candidate_slots_for_page = candidate_content_slots.get(&page_name);
        let slot_names = active_slots_for_page
            .into_iter()
            .flat_map(|slots| slots.keys())
            .chain(
                candidate_slots_for_page
                    .into_iter()
                    .flat_map(|slots| slots.keys()),
            )
            .cloned()
            .collect::<BTreeSet<_>>();
        let content_changed = slot_names.iter().any(|slot_name| {
            let active_slot = active_slots_for_page
                .and_then(|slots| slots.get(slot_name))
                .copied();
            let candidate_slot = candidate_slots_for_page
                .and_then(|slots| slots.get(slot_name))
                .copied();
            active_slot != candidate_slot
        });
        let Some(change_posture) = declaration_change_posture.or({
            if layout_changed || content_changed {
                Some(WorthUiAuthoredDeltaChangePosture::Changed)
            } else {
                None
            }
        }) else {
            continue;
        };
        if let Some(declaration_change_posture) = declaration_change_posture {
            declaration_rows.insert(WorthUiTouchedAuthoredDeclarationRow::new(
                WorthUiAuthoredDeclarationKind::Page,
                &page_name,
                declaration_change_posture,
            ));
        }

        if active_digest.is_none() || candidate_digest.is_none() {
            insert_page_semantic_row(
                &mut semantic_rows,
                inventory,
                WorthUiSemanticSliceId::PageTemplateDeclaration,
                &page_name,
                change_posture,
            );
            insert_page_semantic_row(
                &mut semantic_rows,
                inventory,
                WorthUiSemanticSliceId::PageInstanceDeclaration,
                &page_name,
                change_posture,
            );
            insert_page_semantic_row(
                &mut semantic_rows,
                inventory,
                WorthUiSemanticSliceId::PageTemplateBinding,
                &page_name,
                change_posture,
            );
        }

        if layout_changed {
            declaration_rows.insert(WorthUiTouchedAuthoredDeclarationRow::new(
                WorthUiAuthoredDeclarationKind::Layout,
                &page_name,
                change_posture,
            ));
            if layout_topology_changed {
                insert_page_semantic_row(
                    &mut semantic_rows,
                    inventory,
                    WorthUiSemanticSliceId::LayoutTopology,
                    &page_name,
                    change_posture,
                );
            }
            if layout_gap_changed {
                insert_page_semantic_row(
                    &mut semantic_rows,
                    inventory,
                    WorthUiSemanticSliceId::LayoutGapRule,
                    &page_name,
                    change_posture,
                );
            }
            if layout_padding_changed {
                insert_page_semantic_row(
                    &mut semantic_rows,
                    inventory,
                    WorthUiSemanticSliceId::LayoutPaddingRule,
                    &page_name,
                    change_posture,
                );
            }
        }

        for slot_name in slot_names {
            let active_slot = active_slots_for_page
                .and_then(|slots| slots.get(&slot_name))
                .copied();
            let candidate_slot = candidate_slots_for_page
                .and_then(|slots| slots.get(&slot_name))
                .copied();
            if active_slot == candidate_slot {
                continue;
            }
            ensure_inventory_slice(inventory, WorthUiSemanticSliceId::ContentSlotAssignment);
            semantic_rows.insert(WorthUiTouchedAuthoredSemanticSliceRow::new(
                WorthUiSemanticSliceId::ContentSlotAssignment,
                WorthUiAuthoredSemanticSubject::PageSlot {
                    page_name: page_name.clone(),
                    slot_name: slot_name.clone(),
                },
                change_posture,
            ));
            for surface_id in [active_slot, candidate_slot].into_iter().flatten() {
                insert_surface_semantic_row(
                    &mut semantic_rows,
                    inventory,
                    WorthUiSemanticSliceId::SurfaceMountTarget,
                    surface_id.surface_id(),
                    change_posture,
                );
                insert_surface_semantic_row(
                    &mut semantic_rows,
                    inventory,
                    WorthUiSemanticSliceId::AuthoredMountComponentSelection,
                    surface_id.surface_id(),
                    change_posture,
                );
            }
        }
        if content_changed {
            declaration_rows.insert(WorthUiTouchedAuthoredDeclarationRow::new(
                WorthUiAuthoredDeclarationKind::Content,
                &page_name,
                change_posture,
            ));
        }
    }

    let touched_declaration_rows = declaration_rows.into_iter().collect::<Vec<_>>();
    let semantic_slice_rows = semantic_rows.into_iter().collect::<Vec<_>>();
    let counters = WorthUiAuthoredDeltaCounters::new(
        observed_modules,
        parsed.module_ids().len(),
        parsed
            .module_ids()
            .iter()
            .filter_map(|module_id| parsed.module(module_id))
            .map(|module| module.declarations().len())
            .sum(),
        touched_declaration_rows.len(),
        semantic_slice_rows.len(),
    );
    let digest = authored_delta_digest(&counters, &touched_declaration_rows, &semantic_slice_rows);
    WorthUiAuthoredDeltaSummary::new(
        digest,
        counters,
        touched_declaration_rows,
        semantic_slice_rows,
    )
}

fn lower_catalog_delta(
    declaration_kind: WorthUiAuthoredDeclarationKind,
    active_entries: &[WorthUiAuthoringCatalogEntry],
    candidate_entries: &[WorthUiAuthoringCatalogEntry],
    semantic_slice_id: WorthUiSemanticSliceId,
    inventory: &WorthUiSemanticSliceInventory,
    declaration_rows: &mut BTreeSet<WorthUiTouchedAuthoredDeclarationRow>,
    semantic_rows: &mut BTreeSet<WorthUiTouchedAuthoredSemanticSliceRow>,
    subject_for_name: impl Fn(&str) -> WorthUiAuthoredSemanticSubject,
) {
    let active = catalog_map(active_entries);
    let candidate = catalog_map(candidate_entries);
    let names = active
        .keys()
        .chain(candidate.keys())
        .cloned()
        .collect::<BTreeSet<_>>();
    ensure_inventory_slice(inventory, semantic_slice_id);

    for name in names {
        let Some(change_posture) =
            change_posture(active.get(&name).copied(), candidate.get(&name).copied())
        else {
            continue;
        };
        declaration_rows.insert(WorthUiTouchedAuthoredDeclarationRow::new(
            declaration_kind,
            &name,
            change_posture,
        ));
        semantic_rows.insert(WorthUiTouchedAuthoredSemanticSliceRow::new(
            semantic_slice_id,
            subject_for_name(&name),
            change_posture,
        ));
    }
}
