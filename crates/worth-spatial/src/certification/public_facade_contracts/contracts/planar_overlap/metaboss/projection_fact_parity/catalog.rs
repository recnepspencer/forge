use worth_kernel::workload_composition::{TransformRecipe, WorkloadCatalog};
use worth_spatial::facade::workload_vocabulary::CompleteWorkloadEvidenceLedger;

#[derive(Clone, Copy)]
pub(crate) enum ProjectionParityCatalog {
    Cube,
    CoplanarOverlapStorm,
    ThinFeatureWall,
    RetainedCancellationChain,
    OpenWire,
    OpenSheet,
    OpenRadialFan(usize),
}

pub(crate) fn projection_parity_workload_ledger(
    world: &'static str,
    catalog: ProjectionParityCatalog,
) -> CompleteWorkloadEvidenceLedger {
    let recipe = match catalog {
        ProjectionParityCatalog::Cube => WorkloadCatalog::cube(),
        ProjectionParityCatalog::CoplanarOverlapStorm => WorkloadCatalog::coplanar_overlap_storm(),
        ProjectionParityCatalog::ThinFeatureWall => WorkloadCatalog::thin_feature_wall(),
        ProjectionParityCatalog::RetainedCancellationChain => {
            WorkloadCatalog::retained_cancellation_chain()
        }
        ProjectionParityCatalog::OpenWire => WorkloadCatalog::open_wire(),
        ProjectionParityCatalog::OpenSheet => WorkloadCatalog::open_sheet(),
        ProjectionParityCatalog::OpenRadialFan(incident_faces) => {
            WorkloadCatalog::open_shell_nmt_edge_fan(incident_faces)
        }
    };
    recipe
        .with_transform(TransformRecipe::HostileCancellation)
        .with_retained_replay_artifacts()
        .declared(format!("MB-M6-7 catalog workload {world}"))
        .build()
        .expect("catalog workload")
        .workload()
        .evidence_ledger()
        .clone()
}
