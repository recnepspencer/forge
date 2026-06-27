mod dependency_audit;
mod legacy_surface_residue;
mod lifecycle_propagation;

pub use dependency_audit::{
    audit_host_egui_dependency_boundary, audit_no_cross_crate_deep_imports,
};
pub use legacy_surface_residue::{
    audit_legacy_crate_dispositions, audit_legacy_public_surface_narrowing,
    audit_legacy_shim_honesty, audit_no_parallel_legacy_authority,
};
pub use lifecycle_propagation::{
    expected_phase3_lifecycle_subsystems, lifecycle_propagation_fixture_paths,
};
