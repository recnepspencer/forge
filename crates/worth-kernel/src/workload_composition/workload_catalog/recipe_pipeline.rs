use topology::facade::{
    NmtTopologyConstructionReceipt, TopologySeedNeighborhoodReceipt, TopologySeedReceipt,
};
use worth_spatial::facade::projection_workload::{
    LocalFrameBasis, ProjectedPlanarWorkload, ProjectionReceiptSet, ProjectionWorkload,
};
use worth_spatial::facade::retained_replay_workload::{
    canonical_retained_cancellation_chain_capture, ReplayReceiptSet, ReplayWorkload,
};
use worth_spatial::facade::surface_support::{
    CertifiedSurfaceSupport, SurfaceFamily, SurfaceSupportReceiptSet, SurfaceSupportWorkload,
};
use worth_spatial::facade::transform_workload::{
    TransformReceiptSet, TransformWorkload, TransformedWorkload,
};
use worth_spatial::facade::workload_binding::{
    BoundGeometryWorkload, GeometryBindingReceiptSet, GeometryBindingWorkload,
    PlanarEdgeCarrierSet, PlanarFaceCarrierSet, PlanarLoopBoundaryCatalogProfile,
    PlanarLoopCarrierSet,
};
use worth_spatial::facade::workload_vocabulary::{
    DiagnosticWorkload, DiagnosticWorkloadReceipt, ResponseWorkload, ResponseWorkloadReceipt,
    RetainedReplayWorkload as RetainedReplayStageWorkload, RetainedReplayWorkloadReceipt,
    WorkloadEvidenceLedger, WorkloadEvidenceRow,
};

use super::error::WorkloadCatalogError;
use super::recipe_kind::{
    RetainedReplayRecipe, TransformRecipe, WorkloadCatalogRecipeKind, WorkloadTopologyBreadth,
};
use super::recipe_seed::build_topology_seed;
use crate::workload_composition::{WorthWorkload, WorthWorkloadParts};

pub(crate) struct CatalogWorkloadBuild {
    workload: WorthWorkload,
    topology_neighborhood: Option<TopologySeedNeighborhoodReceipt>,
    topology_construction: Option<NmtTopologyConstructionReceipt>,
    bound_geometry: BoundGeometryWorkload,
    surface_support: CertifiedSurfaceSupport,
    projected: ProjectedPlanarWorkload,
    transform_receipts: TransformReceiptSet,
    replay_receipts: Option<ReplayReceiptSet>,
}

impl CatalogWorkloadBuild {
    pub(crate) fn workload(self) -> WorthWorkload {
        self.workload
    }

    pub(crate) fn topology_neighborhood(&self) -> Option<&TopologySeedNeighborhoodReceipt> {
        self.topology_neighborhood.as_ref()
    }

    pub(crate) fn topology_construction(&self) -> Option<&NmtTopologyConstructionReceipt> {
        self.topology_construction.as_ref()
    }

    pub(crate) fn projected(&self) -> &ProjectedPlanarWorkload {
        &self.projected
    }

    pub(crate) fn surface_support(&self) -> &CertifiedSurfaceSupport {
        &self.surface_support
    }

    pub(crate) fn bound_geometry(&self) -> &BoundGeometryWorkload {
        &self.bound_geometry
    }

    pub(crate) fn transform_receipts(&self) -> &TransformReceiptSet {
        &self.transform_receipts
    }

    pub(crate) fn replay_receipts(&self) -> Option<&ReplayReceiptSet> {
        self.replay_receipts.as_ref()
    }
}

pub(crate) fn build_catalog_workload(
    recipe: WorkloadCatalogRecipeKind,
    declaration: &str,
    transform_recipe: TransformRecipe,
    retained_replay_recipe: RetainedReplayRecipe,
    topology_breadth: WorkloadTopologyBreadth,
    planar_loop_boundary_profile: PlanarLoopBoundaryCatalogProfile,
    topology_construction: Option<NmtTopologyConstructionReceipt>,
) -> Result<CatalogWorkloadBuild, WorkloadCatalogError> {
    let topology = build_topology_seed(
        recipe,
        declaration,
        topology_breadth,
        topology_construction.as_ref(),
    )?;
    let topology_neighborhood = topology.neighborhood().cloned();
    let bound_geometry = bind_seed_geometry(&topology, declaration, planar_loop_boundary_profile)?;
    let bound_geometry_for_catalog = bound_geometry.clone();
    let geometry_receipts = bound_geometry.receipts().clone();
    let surface_support = certify_surface_support(bound_geometry, declaration)?;
    let support_receipts = surface_support.receipts().clone();
    let projected = project_supported_geometry(surface_support.clone(), declaration)?;
    let projection_receipts = projected.receipts().clone();
    let transformed =
        transform_projected_geometry(projected.clone(), declaration, transform_recipe)?;
    let transform_receipts = transformed.receipts().clone();
    let retained_replay =
        replay_transformed_geometry(transformed, declaration, retained_replay_recipe)?;
    let replay_receipts = retained_replay.replay_receipts().cloned();
    let diagnostics = DiagnosticWorkload::for_retained_replay(retained_replay.stage_receipt())
        .declared(format!("catalog diagnostics for {declaration}"))
        .admit()
        .map_err(|error| {
            WorkloadCatalogError::QueryAdmissionFailed(error.human_reason().to_string())
        })?;
    let response = ResponseWorkload::for_diagnostics(&diagnostics)
        .declared(format!("catalog response for {declaration}"))
        .admit()
        .map_err(|error| {
            WorkloadCatalogError::QueryAdmissionFailed(error.human_reason().to_string())
        })?;
    let evidence_ledger = complete_catalog_evidence_ledger(CatalogEvidenceParts {
        topology: &topology,
        geometry: &geometry_receipts,
        support: &support_receipts,
        projection: &projection_receipts,
        transform: &transform_receipts,
        retained_replay: &retained_replay,
        replay_receipts: retained_replay.replay_receipts(),
        diagnostics: &diagnostics,
        response: &response,
    })?;

    let workload = WorthWorkload::compose(WorthWorkloadParts {
        topology: topology.query_receipts().declaration_receipt().clone(),
        geometry_binding: geometry_receipts.stage_receipt().clone(),
        surface_support: support_receipts.stage_receipt().clone(),
        projection: projection_receipts.stage_receipt().clone(),
        transform: transform_receipts.stage_receipt().clone(),
        retained_replay: retained_replay.into_stage_receipt(),
        batch_admission_execution: None,
        diagnostics,
        response,
        evidence_ledger,
    })
    .map_err(WorkloadCatalogError::from)?;

    Ok(CatalogWorkloadBuild {
        workload,
        topology_neighborhood,
        topology_construction,
        bound_geometry: bound_geometry_for_catalog,
        surface_support,
        projected,
        transform_receipts,
        replay_receipts,
    })
}

fn bind_seed_geometry(
    topology: &TopologySeedReceipt,
    declaration: &str,
    planar_loop_boundary_profile: PlanarLoopBoundaryCatalogProfile,
) -> Result<BoundGeometryWorkload, WorkloadCatalogError> {
    GeometryBindingWorkload::for_topology_seed(topology)
        .declared(format!("bind catalog geometry for {declaration}"))
        .with_planar_faces(PlanarFaceCarrierSet::for_seed_faces(topology))
        .with_planar_edges(PlanarEdgeCarrierSet::for_seed_edges(topology))
        .with_planar_loops(PlanarLoopCarrierSet::for_seed_loops_with_profile(
            topology,
            planar_loop_boundary_profile,
        ))
        .admit()
        .map_err(WorkloadCatalogError::from)
}

fn certify_surface_support(
    bound_geometry: BoundGeometryWorkload,
    declaration: &str,
) -> Result<CertifiedSurfaceSupport, WorkloadCatalogError> {
    SurfaceSupportWorkload::for_bound_geometry(bound_geometry)
        .declared(format!("certify catalog support for {declaration}"))
        .with_surface_family(SurfaceFamily::Plane)
        .certify()
        .map_err(WorkloadCatalogError::from)
}

fn project_supported_geometry(
    surface_support: CertifiedSurfaceSupport,
    declaration: &str,
) -> Result<ProjectedPlanarWorkload, WorkloadCatalogError> {
    ProjectionWorkload::for_certified_surface_support(surface_support)
        .declared(format!("project catalog geometry for {declaration}"))
        .with_local_frame(LocalFrameBasis::from_certified_plane())
        .project()
        .map_err(WorkloadCatalogError::from)
}

fn transform_projected_geometry(
    projected: ProjectedPlanarWorkload,
    declaration: &str,
    transform_recipe: TransformRecipe,
) -> Result<TransformedWorkload, WorkloadCatalogError> {
    TransformWorkload::for_projected_workload(projected)
        .declared(format!("transform catalog geometry for {declaration}"))
        .with_transform_sequence(transform_recipe.sequence())
        .transform()
        .map_err(WorkloadCatalogError::from)
}

struct CatalogRetainedReplay {
    stage_receipt: RetainedReplayWorkloadReceipt,
    replay_receipts: Option<ReplayReceiptSet>,
}

impl CatalogRetainedReplay {
    fn from_stage_receipt(stage_receipt: RetainedReplayWorkloadReceipt) -> Self {
        Self {
            stage_receipt,
            replay_receipts: None,
        }
    }

    fn from_replay_receipts(replay_receipts: ReplayReceiptSet) -> Self {
        Self {
            stage_receipt: replay_receipts.stage_receipt().clone(),
            replay_receipts: Some(replay_receipts),
        }
    }

    fn stage_receipt(&self) -> &RetainedReplayWorkloadReceipt {
        &self.stage_receipt
    }

    fn replay_receipts(&self) -> Option<&ReplayReceiptSet> {
        self.replay_receipts.as_ref()
    }

    fn into_stage_receipt(self) -> RetainedReplayWorkloadReceipt {
        self.stage_receipt
    }
}

fn replay_transformed_geometry(
    transformed: TransformedWorkload,
    declaration: &str,
    retained_replay_recipe: RetainedReplayRecipe,
) -> Result<CatalogRetainedReplay, WorkloadCatalogError> {
    match retained_replay_recipe {
        RetainedReplayRecipe::StageReceiptOnly => {
            RetainedReplayStageWorkload::for_transform(transformed.receipts().stage_receipt())
                .declared(format!("catalog replay stage for {declaration}"))
                .admit()
                .map(CatalogRetainedReplay::from_stage_receipt)
                .map_err(|error| {
                    WorkloadCatalogError::QueryAdmissionFailed(error.human_reason().to_string())
                })
        }
        RetainedReplayRecipe::RetainedCancellationChain => {
            let captured = canonical_retained_cancellation_chain_capture(
                "workload-catalog-retained-cancellation-chain",
            )?;
            let replayed = ReplayWorkload::for_transformed_workload(transformed)
                .declared(format!("catalog retained replay for {declaration}"))
                .with_captured_retained_workload(captured)
                .replay()?;
            Ok(CatalogRetainedReplay::from_replay_receipts(
                replayed.receipts().clone(),
            ))
        }
    }
}

struct CatalogEvidenceParts<'a> {
    topology: &'a TopologySeedReceipt,
    geometry: &'a GeometryBindingReceiptSet,
    support: &'a SurfaceSupportReceiptSet,
    projection: &'a ProjectionReceiptSet,
    transform: &'a TransformReceiptSet,
    retained_replay: &'a CatalogRetainedReplay,
    replay_receipts: Option<&'a ReplayReceiptSet>,
    diagnostics: &'a DiagnosticWorkloadReceipt,
    response: &'a ResponseWorkloadReceipt,
}

fn complete_catalog_evidence_ledger(
    parts: CatalogEvidenceParts<'_>,
) -> Result<
    worth_spatial::facade::workload_vocabulary::CompleteWorkloadEvidenceLedger,
    WorkloadCatalogError,
> {
    WorkloadEvidenceLedger::from_rows(vec![
        WorkloadEvidenceRow::from_topology_seed_receipt(parts.topology),
        WorkloadEvidenceRow::from_geometry_binding_receipt_set(parts.geometry),
        WorkloadEvidenceRow::from_surface_support_receipt_set(parts.support),
        WorkloadEvidenceRow::from_projection_receipt_set(parts.projection),
        WorkloadEvidenceRow::from_transform_receipt_set(parts.transform),
        retained_replay_evidence_row(parts.retained_replay, parts.replay_receipts),
        WorkloadEvidenceRow::from_diagnostic_receipt(parts.diagnostics),
        WorkloadEvidenceRow::from_response_receipt(parts.response),
    ])?
    .certify_complete()
    .map_err(WorkloadCatalogError::from)
}

fn retained_replay_evidence_row(
    retained_replay: &CatalogRetainedReplay,
    replay_receipts: Option<&ReplayReceiptSet>,
) -> WorkloadEvidenceRow {
    replay_receipts.map_or_else(
        || WorkloadEvidenceRow::from_retained_replay_receipt(retained_replay.stage_receipt()),
        WorkloadEvidenceRow::from_replay_receipt_set,
    )
}
