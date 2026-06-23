use super::WorthUiAuthoredStructuralChangedFactRow;
use crate::runtime::{
    WorthUiAppearanceRecipeId, WorthUiAuthoredDeltaSummary, WorthUiAuthoredSemanticSubject,
    WorthUiContentSlotId, WorthUiPageInstanceId, WorthUiPageTemplateId, WorthUiRuntimeFactId,
    WorthUiRuntimeFactSet, WorthUiSemanticSliceId, WorthUiTouchedAuthoredSemanticSliceRow,
};

pub struct WorthUiAuthoredStructuralRuntimeFactLowering;

impl WorthUiAuthoredStructuralRuntimeFactLowering {
    pub fn from_authored_delta_summary(
        authored_delta_summary: &WorthUiAuthoredDeltaSummary,
    ) -> Vec<WorthUiAuthoredStructuralChangedFactRow> {
        authored_delta_summary
            .semantic_slice_rows()
            .iter()
            .cloned()
            .map(|semantic_row| {
                let changed_facts = changed_facts_for_row(&semantic_row);
                WorthUiAuthoredStructuralChangedFactRow::new(semantic_row, changed_facts)
            })
            .collect()
    }
}

fn changed_facts_for_row(
    semantic_row: &WorthUiTouchedAuthoredSemanticSliceRow,
) -> WorthUiRuntimeFactSet {
    match (semantic_row.slice_id(), semantic_row.subject()) {
        (
            WorthUiSemanticSliceId::LayoutTopology,
            WorthUiAuthoredSemanticSubject::Page { page_name },
        ) => WorthUiRuntimeFactSet::single(WorthUiRuntimeFactId::layout_topology(page_name)),
        (
            WorthUiSemanticSliceId::LayoutGapRule,
            WorthUiAuthoredSemanticSubject::Page { page_name },
        ) => WorthUiRuntimeFactSet::single(WorthUiRuntimeFactId::layout_gap(page_name)),
        (
            WorthUiSemanticSliceId::LayoutPaddingRule,
            WorthUiAuthoredSemanticSubject::Page { page_name },
        ) => WorthUiRuntimeFactSet::single(WorthUiRuntimeFactId::layout_padding(page_name)),
        (
            WorthUiSemanticSliceId::ShellSlotAssignment,
            WorthUiAuthoredSemanticSubject::Workspace { workspace_name },
        ) => WorthUiRuntimeFactSet::single(WorthUiRuntimeFactId::shell_slot_assignment(
            workspace_name,
        )),
        (
            WorthUiSemanticSliceId::ContentSlotAssignment,
            WorthUiAuthoredSemanticSubject::PageSlot {
                page_name,
                slot_name,
            },
        ) => content_slot_changed_facts(page_name, slot_name),
        (
            WorthUiSemanticSliceId::SurfaceMountTarget,
            WorthUiAuthoredSemanticSubject::Surface { surface_id },
        ) => WorthUiRuntimeFactSet::single(WorthUiRuntimeFactId::surface_mount_raw(surface_id)),
        (
            WorthUiSemanticSliceId::AuthoredMountComponentSelection,
            WorthUiAuthoredSemanticSubject::Surface { surface_id },
        ) => WorthUiRuntimeFactSet::single(
            WorthUiRuntimeFactId::authored_mount_component_selection(surface_id),
        ),
        (
            WorthUiSemanticSliceId::AuthoredSurfaceInstanceProps,
            WorthUiAuthoredSemanticSubject::Surface { surface_id },
        ) => {
            WorthUiRuntimeFactSet::single(WorthUiRuntimeFactId::authored_surface_props(surface_id))
        }
        (
            WorthUiSemanticSliceId::PrimitiveContent,
            WorthUiAuthoredSemanticSubject::Surface { surface_id },
        ) => WorthUiRuntimeFactSet::single(WorthUiRuntimeFactId::primitive_content(surface_id)),
        (
            WorthUiSemanticSliceId::PrimitiveContainer,
            WorthUiAuthoredSemanticSubject::Surface { surface_id },
        ) => WorthUiRuntimeFactSet::single(WorthUiRuntimeFactId::primitive_container(surface_id)),
        (
            WorthUiSemanticSliceId::PrimitiveMeasurement,
            WorthUiAuthoredSemanticSubject::Surface { surface_id },
        ) => WorthUiRuntimeFactSet::single(WorthUiRuntimeFactId::primitive_measurement(surface_id)),
        (
            WorthUiSemanticSliceId::PrimitiveAppearance,
            WorthUiAuthoredSemanticSubject::Surface { surface_id },
        ) => WorthUiRuntimeFactSet::single(WorthUiRuntimeFactId::primitive_appearance(surface_id)),
        (
            WorthUiSemanticSliceId::PrimitiveAppearanceState,
            WorthUiAuthoredSemanticSubject::Surface { surface_id },
        ) => WorthUiRuntimeFactSet::single(WorthUiRuntimeFactId::primitive_appearance_state(
            surface_id,
        )),
        (
            WorthUiSemanticSliceId::PrimitiveInteraction,
            WorthUiAuthoredSemanticSubject::Surface { surface_id },
        ) => WorthUiRuntimeFactSet::single(WorthUiRuntimeFactId::primitive_interaction(surface_id)),
        (
            WorthUiSemanticSliceId::PrimitiveMotion,
            WorthUiAuthoredSemanticSubject::Surface { surface_id },
        ) => WorthUiRuntimeFactSet::single(WorthUiRuntimeFactId::primitive_motion(surface_id)),
        (
            WorthUiSemanticSliceId::PrimitiveFlowLayout,
            WorthUiAuthoredSemanticSubject::Surface { surface_id },
        ) => WorthUiRuntimeFactSet::single(WorthUiRuntimeFactId::primitive_flow_layout(surface_id)),
        (
            WorthUiSemanticSliceId::PrimitiveEventGeometry,
            WorthUiAuthoredSemanticSubject::Surface { surface_id },
        ) => WorthUiRuntimeFactSet::single(WorthUiRuntimeFactId::primitive_event_geometry(
            surface_id,
        )),
        (
            WorthUiSemanticSliceId::PageTemplateDeclaration,
            WorthUiAuthoredSemanticSubject::Page { page_name },
        ) => page_template_changed_facts(page_name),
        (
            WorthUiSemanticSliceId::PageInstanceDeclaration,
            WorthUiAuthoredSemanticSubject::Page { page_name },
        ) => page_instance_changed_facts(page_name),
        (
            WorthUiSemanticSliceId::PageTemplateBinding,
            WorthUiAuthoredSemanticSubject::Page { page_name },
        ) => page_template_binding_changed_facts(page_name),
        (
            WorthUiSemanticSliceId::AppearanceRecipe,
            WorthUiAuthoredSemanticSubject::AppearanceRecipe { recipe_name },
        ) => appearance_recipe_changed_facts(recipe_name),
        (
            WorthUiSemanticSliceId::AuthoredQueryBindingShape,
            WorthUiAuthoredSemanticSubject::RuntimeBinding { binding_name },
        ) => WorthUiRuntimeFactSet::single(WorthUiRuntimeFactId::authored_query_binding_shape(
            binding_name,
        )),
        _ => WorthUiRuntimeFactSet::empty(),
    }
}

fn content_slot_changed_facts(page_name: &str, slot_name: &str) -> WorthUiRuntimeFactSet {
    let Ok(page_template_id) = WorthUiPageTemplateId::new(page_name) else {
        return WorthUiRuntimeFactSet::empty();
    };
    let Ok(content_slot_id) = WorthUiContentSlotId::new(slot_name) else {
        return WorthUiRuntimeFactSet::empty();
    };
    WorthUiRuntimeFactSet::empty()
        .with(WorthUiRuntimeFactId::page_content_slot(
            &page_template_id,
            &content_slot_id,
        ))
        .with(WorthUiRuntimeFactId::content_mount(format!(
            "{page_name}.{slot_name}"
        )))
}

fn page_template_changed_facts(page_name: &str) -> WorthUiRuntimeFactSet {
    let Ok(page_template_id) = WorthUiPageTemplateId::new(page_name) else {
        return WorthUiRuntimeFactSet::empty();
    };
    WorthUiRuntimeFactSet::single(WorthUiRuntimeFactId::page_template(&page_template_id))
}

fn page_instance_changed_facts(page_name: &str) -> WorthUiRuntimeFactSet {
    let Ok(page_instance_id) = WorthUiPageInstanceId::new(page_name) else {
        return WorthUiRuntimeFactSet::empty();
    };
    WorthUiRuntimeFactSet::single(WorthUiRuntimeFactId::page_instance(&page_instance_id))
}

fn page_template_binding_changed_facts(page_name: &str) -> WorthUiRuntimeFactSet {
    let Ok(page_instance_id) = WorthUiPageInstanceId::new(page_name) else {
        return WorthUiRuntimeFactSet::empty();
    };
    let Ok(page_template_id) = WorthUiPageTemplateId::new(page_name) else {
        return WorthUiRuntimeFactSet::empty();
    };
    WorthUiRuntimeFactSet::single(WorthUiRuntimeFactId::page_instance_template_binding(
        &page_instance_id,
        &page_template_id,
    ))
}

fn appearance_recipe_changed_facts(recipe_name: &str) -> WorthUiRuntimeFactSet {
    let Ok(recipe_id) = WorthUiAppearanceRecipeId::new(recipe_name) else {
        return WorthUiRuntimeFactSet::empty();
    };
    WorthUiRuntimeFactSet::single(WorthUiRuntimeFactId::appearance_recipe(&recipe_id))
}
