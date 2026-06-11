use topology::facade::{
    TopologySeed, TopologySeedNeighborhoodReceipt, TopologySeedReceipt, TopologySeedRecipe,
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
    PlanarEdgeCarrierSet, PlanarFaceCarrierSet, PlanarLoopCarrierSet,
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
use crate::workload_composition::{WorthWorkload, WorthWorkloadParts};

pub(crate) struct CatalogWorkloadBuild {
    workload: WorthWorkload,
    topology_neighborhood: Option<TopologySeedNeighborhoodReceipt>,
}

impl CatalogWorkloadBuild {
    pub(crate) fn workload(self) -> WorthWorkload {
        self.workload
    }

    pub(crate) fn topology_neighborhood(&self) -> Option<&TopologySeedNeighborhoodReceipt> {
        self.topology_neighborhood.as_ref()
    }
}

pub(crate) fn build_catalog_workload(
    recipe: WorkloadCatalogRecipeKind,
    declaration: &str,
    transform_recipe: TransformRecipe,
    retained_replay_recipe: RetainedReplayRecipe,
    topology_breadth: WorkloadTopologyBreadth,
) -> Result<CatalogWorkloadBuild, WorkloadCatalogError> {
    let topology = build_topology_seed(recipe, declaration, topology_breadth)?;
    let topology_neighborhood = topology.neighborhood().cloned();
    let bound_geometry = bind_seed_geometry(&topology, declaration)?;
    let geometry_receipts = bound_geometry.receipts().clone();
    let surface_support = certify_surface_support(bound_geometry, declaration)?;
    let support_receipts = surface_support.receipts().clone();
    let projected = project_supported_geometry(surface_support, declaration)?;
    let projection_receipts = projected.receipts().clone();
    let transformed = transform_projected_geometry(projected, declaration, transform_recipe)?;
    let transform_receipts = transformed.receipts().clone();
    let retained_replay =
        replay_transformed_geometry(transformed, declaration, retained_replay_recipe)?;
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
        diagnostics,
        response,
        evidence_ledger,
    })
    .map_err(WorkloadCatalogError::from)?;

    Ok(CatalogWorkloadBuild {
        workload,
        topology_neighborhood,
    })
}

fn build_topology_seed(
    recipe: WorkloadCatalogRecipeKind,
    declaration: &str,
    topology_breadth: WorkloadTopologyBreadth,
) -> Result<TopologySeedReceipt, WorkloadCatalogError> {
    let seed = match topology_breadth {
        WorkloadTopologyBreadth::Default => default_topology_seed(recipe),
        WorkloadTopologyBreadth::MultiFaceShell { face_count }
            if recipe == WorkloadCatalogRecipeKind::CoplanarOverlapStorm =>
        {
            TopologySeed::multi_face_shell(face_count)
        }
        WorkloadTopologyBreadth::HighValenceVertex { valence }
            if recipe == WorkloadCatalogRecipeKind::HighValenceVertex =>
        {
            TopologySeed::high_valence_vertex_with_valence(valence)
        }
        WorkloadTopologyBreadth::MultiFaceShell { .. } => {
            return Err(WorkloadCatalogError::UnsupportedRecipe {
                recipe,
                reason: "explicit multi-face shell breadth is only admitted for the coplanar overlap storm recipe".to_string(),
            });
        }
        WorkloadTopologyBreadth::HighValenceVertex { .. } => {
            return Err(WorkloadCatalogError::UnsupportedRecipe {
                recipe,
                reason: "explicit high-valence vertex breadth is only admitted for the high-valence vertex recipe".to_string(),
            });
        }
    };
    seed.with_declaration(format!("topology seed for {declaration}"))
        .build()
        .map_err(WorkloadCatalogError::from)
}

fn default_topology_seed(recipe: WorkloadCatalogRecipeKind) -> TopologySeedRecipe {
    match recipe {
        WorkloadCatalogRecipeKind::Cube
        | WorkloadCatalogRecipeKind::TransformCycle
        | WorkloadCatalogRecipeKind::RetainedCancellationChain => TopologySeed::cube(),
        WorkloadCatalogRecipeKind::CoplanarOverlapStorm => TopologySeed::multi_face_shell(64),
        WorkloadCatalogRecipeKind::Tetrahedron => TopologySeed::tetrahedron(),
        WorkloadCatalogRecipeKind::SingleFaceLoop => TopologySeed::single_face_loop(4),
        WorkloadCatalogRecipeKind::ThinFeatureWall => TopologySeed::single_face_loop(64),
        WorkloadCatalogRecipeKind::HighValenceVertex => TopologySeed::high_valence_vertex(),
        WorkloadCatalogRecipeKind::OpenSheet => TopologySeed::open_sheet(),
        WorkloadCatalogRecipeKind::DirtySelfIntersectingLoop => {
            TopologySeed::self_intersecting_loop()
        }
    }
}

fn bind_seed_geometry(
    topology: &TopologySeedReceipt,
    declaration: &str,
) -> Result<BoundGeometryWorkload, WorkloadCatalogError> {
    GeometryBindingWorkload::for_topology_seed(topology)
        .declared(format!("bind catalog geometry for {declaration}"))
        .with_planar_faces(PlanarFaceCarrierSet::for_seed_faces(topology))
        .with_planar_edges(PlanarEdgeCarrierSet::for_seed_edges(topology))
        .with_planar_loops(PlanarLoopCarrierSet::for_seed_loops(topology))
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
