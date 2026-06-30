mod dependency_audit;
mod inspection_boundary_audit;
mod inspection_boundary_certification;
mod inspection_topology_audit;
mod legacy_surface_residue;
mod lifecycle_propagation;
mod ownership_audit;
mod public_surface_audit;

pub use dependency_audit::{
    audit_host_egui_dependency_boundary, audit_no_cross_crate_deep_imports,
};
pub use inspection_boundary_audit::audit_consumers_route_inspection_through_worth_ui_facade;
pub use inspection_boundary_certification::certify_consumers_route_inspection_through_worth_ui_facade;
pub use inspection_topology_audit::{
    audit_inspection_future_artifact_seed_topology, audit_inspection_public_module_names,
    audit_inspection_public_module_role_purity,
};
pub use legacy_surface_residue::{
    audit_legacy_crate_dispositions, audit_legacy_public_surface_narrowing,
    audit_legacy_shim_honesty, audit_no_parallel_legacy_authority,
};
pub use lifecycle_propagation::{
    audit_phase3_lifecycle_public_surface, expected_phase3_lifecycle_subsystems,
    lifecycle_propagation_fixture_paths,
};
pub use ownership_audit::{
    audit_non_dsl_crates_do_not_reach_dsl_internals,
    audit_preboundary_receipt_and_posture_files_do_not_lower_to_foundational,
    audit_public_surfaces_do_not_recreate_query_owned_lanes,
    audit_required_runtime_lifecycle_aggregates_do_not_cheat_with_default_or_option,
};
