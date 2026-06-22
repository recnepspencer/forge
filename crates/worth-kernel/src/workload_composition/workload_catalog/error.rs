use topology::facade::{NmtTopologyConstructionDenial, TopologySeedCleanFailReceipt};
use worth_spatial::facade::projection_workload::UnsupportedProjectionWorkload;
use worth_spatial::facade::retained_replay_workload::UnsupportedReplayWorkload;
use worth_spatial::facade::surface_support::UnsupportedSurfaceSupport;
use worth_spatial::facade::transform_workload::UnsupportedTransformWorkload;
use worth_spatial::facade::workload_binding::UnsupportedGeometryBinding;
use worth_spatial::facade::workload_vocabulary::WorkloadEvidenceLedgerError;

use super::recipe_kind::WorkloadCatalogRecipeKind;
use crate::workload_composition::WorkloadCompositionError;

#[derive(Clone, Debug, PartialEq)]
pub enum WorkloadCatalogError {
    MissingDeclaration,
    QueryAdmissionFailed(String),
    UnsupportedRecipe {
        recipe: WorkloadCatalogRecipeKind,
        reason: String,
    },
    NmtTopologyConstructionDenied(NmtTopologyConstructionDenial),
    TopologySeedDenied(TopologySeedCleanFailReceipt),
    GeometryBindingDenied(UnsupportedGeometryBinding),
    SurfaceSupportDenied(UnsupportedSurfaceSupport),
    ProjectionDenied(UnsupportedProjectionWorkload),
    TransformDenied(UnsupportedTransformWorkload),
    RetainedReplayDenied(UnsupportedReplayWorkload),
    EvidenceLedgerDenied(WorkloadEvidenceLedgerError),
    WorkloadCompositionDenied(WorkloadCompositionError),
}

impl WorkloadCatalogError {
    pub fn human_reason(&self) -> String {
        match self {
            Self::MissingDeclaration => {
                "workload catalog recipe requires a human-readable declaration".to_string()
            }
            Self::QueryAdmissionFailed(reason) => {
                format!("workload catalog recipe could not be admitted by Forge Query: {reason}")
            }
            Self::UnsupportedRecipe { recipe, reason } => {
                format!("{} is not admitted: {reason}", recipe.human_name())
            }
            Self::NmtTopologyConstructionDenied(denial) => denial.reason().to_string(),
            Self::TopologySeedDenied(denial) => denial.reason().to_string(),
            Self::GeometryBindingDenied(denial) => denial.human_reason().to_string(),
            Self::SurfaceSupportDenied(denial) => denial.human_reason().to_string(),
            Self::ProjectionDenied(denial) => denial.human_reason().to_string(),
            Self::TransformDenied(denial) => denial.human_reason().to_string(),
            Self::RetainedReplayDenied(denial) => denial.human_reason().to_string(),
            Self::EvidenceLedgerDenied(error) => error.human_reason(),
            Self::WorkloadCompositionDenied(error) => error.human_reason(),
        }
    }
}

impl From<NmtTopologyConstructionDenial> for WorkloadCatalogError {
    fn from(value: NmtTopologyConstructionDenial) -> Self {
        Self::NmtTopologyConstructionDenied(value)
    }
}

impl From<TopologySeedCleanFailReceipt> for WorkloadCatalogError {
    fn from(value: TopologySeedCleanFailReceipt) -> Self {
        Self::TopologySeedDenied(value)
    }
}

impl From<UnsupportedGeometryBinding> for WorkloadCatalogError {
    fn from(value: UnsupportedGeometryBinding) -> Self {
        Self::GeometryBindingDenied(value)
    }
}

impl From<UnsupportedSurfaceSupport> for WorkloadCatalogError {
    fn from(value: UnsupportedSurfaceSupport) -> Self {
        Self::SurfaceSupportDenied(value)
    }
}

impl From<UnsupportedProjectionWorkload> for WorkloadCatalogError {
    fn from(value: UnsupportedProjectionWorkload) -> Self {
        Self::ProjectionDenied(value)
    }
}

impl From<UnsupportedTransformWorkload> for WorkloadCatalogError {
    fn from(value: UnsupportedTransformWorkload) -> Self {
        Self::TransformDenied(value)
    }
}

impl From<UnsupportedReplayWorkload> for WorkloadCatalogError {
    fn from(value: UnsupportedReplayWorkload) -> Self {
        Self::RetainedReplayDenied(value)
    }
}

impl From<WorkloadEvidenceLedgerError> for WorkloadCatalogError {
    fn from(value: WorkloadEvidenceLedgerError) -> Self {
        Self::EvidenceLedgerDenied(value)
    }
}

impl From<WorkloadCompositionError> for WorkloadCatalogError {
    fn from(value: WorkloadCompositionError) -> Self {
        Self::WorkloadCompositionDenied(value)
    }
}
