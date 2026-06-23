#[path = "support/runtime_interaction_graph_registry.rs"]
mod runtime_interaction_graph_registry;

use runtime_interaction_graph_registry::{
    REQUIRED_INTERACTION_GRAPH_SURFACES, RUNTIME_INTERACTION_GRAPH_ROWS,
};

#[test]
fn runtime_interaction_graph_registry_has_no_blank_cells() {
    for row in RUNTIME_INTERACTION_GRAPH_ROWS {
        assert!(
            !row.surface_name.is_empty()
                && !row.interaction_entrypoint.is_empty()
                && !row.state_authority.is_empty()
                && !row.fact_family.is_empty()
                && !row.fact_constructor.is_empty()
                && !row.projection_dependency_site.is_empty()
                && !row.runtime_change_admission.is_empty()
                && !row.graph_rebind_entrypoint.is_empty()
                && !row.behavior_proof.is_empty(),
            "interaction graph registry contains a blank cell for {}",
            row.surface_name
        );
    }
}

#[test]
fn runtime_interaction_graph_registry_covers_required_surfaces() {
    for surface_name in REQUIRED_INTERACTION_GRAPH_SURFACES {
        assert!(
            RUNTIME_INTERACTION_GRAPH_ROWS
                .iter()
                .any(|row| row.surface_name == *surface_name),
            "interaction graph registry is missing required surface {surface_name}"
        );
    }
}

#[test]
fn runtime_interaction_graph_registry_keeps_interactions_on_graph_paths() {
    for row in RUNTIME_INTERACTION_GRAPH_ROWS {
        assert!(
            row.interaction_entrypoint
                .starts_with("WorthUiRuntimeHost::"),
            "interaction surface must enter through runtime host: {}",
            row.surface_name
        );
        assert!(
            row.runtime_change_admission
                .starts_with("WorthUiRuntimeHost::admit_"),
            "interaction surface must declare a runtime-change admission lane: {}",
            row.surface_name
        );
        assert!(
            row.graph_rebind_entrypoint
                .starts_with("WorthUiRuntimeHost::rebind_"),
            "interaction surface must re-enter projections through a runtime host rebind lane: {}",
            row.surface_name
        );
        assert!(
            row.fact_family.contains("RuntimeFactFamily::"),
            "interaction surface must declare a typed fact family: {}",
            row.surface_name
        );
        assert!(
            row.fact_constructor.contains("RuntimeFactId::"),
            "interaction surface must declare a typed fact constructor: {}",
            row.surface_name
        );
    }
}
