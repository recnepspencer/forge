//! Semantic integration suite. Individual responsibilities remain in named child modules.

use std::path::Path;
use std::sync::LazyLock;

use worth_ui_certification::topology::WorkspaceSourceInventory;

static WORKSPACE_SOURCE_INVENTORY: LazyLock<WorkspaceSourceInventory> = LazyLock::new(|| {
    WorkspaceSourceInventory::capture(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("crate parent")
            .parent()
            .expect("workspace root"),
    )
});

pub(crate) fn workspace_source_inventory() -> &'static WorkspaceSourceInventory {
    &WORKSPACE_SOURCE_INVENTORY
}

#[test]
fn topology_suite_uses_one_stable_workspace_source_inventory() {
    let first = workspace_source_inventory();
    let second = workspace_source_inventory();

    assert!(std::ptr::eq(first, second));
    assert!(first.rust_file_count() > 0);
}

#[path = "../admission_boundary_bypass.rs"]
mod admission_boundary_bypass;
#[path = "../admission_denial_topology_runtime.rs"]
mod admission_denial_topology_runtime;
#[path = "../admission_topology_audit.rs"]
mod admission_topology_audit;
#[path = "../application_authority_topology.rs"]
mod application_authority_topology;
#[path = "../declaration_residue_audit.rs"]
mod declaration_residue_audit;
#[path = "../declaration_topology_audit.rs"]
mod declaration_topology_audit;
#[path = "../graph_mutation_boundary_audit.rs"]
mod graph_mutation_boundary_audit;
#[path = "../graph_residue_audit.rs"]
mod graph_residue_audit;
#[path = "../graph_topology_authority_runtime.rs"]
mod graph_topology_authority_runtime;
#[path = "../graph_topology_runtime.rs"]
mod graph_topology_runtime;
#[path = "../inspection_boundary_bypass.rs"]
mod inspection_boundary_bypass;
#[path = "../inspection_boundary_purity_audit.rs"]
mod inspection_boundary_purity_audit;
#[path = "../inspection_contract_audit.rs"]
mod inspection_contract_audit;
#[path = "../inspection_growth_posture_audit.rs"]
mod inspection_growth_posture_audit;
#[path = "../legacy_surface_residue_audit.rs"]
mod legacy_surface_residue_audit;
#[path = "../measurement_boundary_purity_audit.rs"]
mod measurement_boundary_purity_audit;
#[path = "../measurement_growth_posture_audit.rs"]
mod measurement_growth_posture_audit;
#[path = "../milestone_37_structural_inventory_audit.rs"]
mod milestone_37_structural_inventory_audit;
#[path = "../obligation_boundary_bypass.rs"]
mod obligation_boundary_bypass;
#[path = "../obligation_boundary_residue_audit.rs"]
mod obligation_boundary_residue_audit;
#[path = "../obligation_selection_topology_runtime.rs"]
mod obligation_selection_topology_runtime;
#[path = "../query_reporting_projection_boundary.rs"]
mod query_reporting_projection_boundary;
#[path = "../responsibility_directory_topology.rs"]
mod responsibility_directory_topology;
#[path = "../topology_audit.rs"]
mod topology_audit;
