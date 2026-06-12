use topology::facade::{
    NmtTopologyConstructionReceipt, TopologySeedCleanFailReceipt, TopologySeedNeighborhoodReceipt,
};
use worth_spatial::facade::projection_workload::ProjectedPlanarWorkload;
use worth_spatial::facade::retained_replay_workload::ReplayReceiptSet;
use worth_spatial::facade::transform_workload::TransformReceiptSet;
use worth_spatial::facade::workload_binding::BoundGeometryWorkload;

use super::recipe_kind::WorkloadCatalogRecipeKind;
use super::support_receipt::{WorkloadCatalogDeclarationReceipt, WorkloadCatalogSupportReceipt};
use crate::workload_composition::WorthWorkload;

#[derive(Clone, Debug, PartialEq)]
pub struct BuiltWorkloadCatalogRecipe {
    recipe: WorkloadCatalogRecipeKind,
    declaration: WorkloadCatalogDeclarationReceipt,
    support: WorkloadCatalogSupportReceipt,
    workload: WorthWorkload,
    topology_neighborhood: Option<TopologySeedNeighborhoodReceipt>,
    topology_construction: Option<NmtTopologyConstructionReceipt>,
    bound_geometry: BoundGeometryWorkload,
    projected: ProjectedPlanarWorkload,
    transform_receipts: TransformReceiptSet,
    replay_receipts: Option<ReplayReceiptSet>,
}

impl BuiltWorkloadCatalogRecipe {
    pub(crate) fn new(
        recipe: WorkloadCatalogRecipeKind,
        declaration: WorkloadCatalogDeclarationReceipt,
        support: WorkloadCatalogSupportReceipt,
        workload: WorthWorkload,
        topology_neighborhood: Option<TopologySeedNeighborhoodReceipt>,
        topology_construction: Option<NmtTopologyConstructionReceipt>,
        bound_geometry: BoundGeometryWorkload,
        projected: ProjectedPlanarWorkload,
        transform_receipts: TransformReceiptSet,
        replay_receipts: Option<ReplayReceiptSet>,
    ) -> Self {
        Self {
            recipe,
            declaration,
            support,
            workload,
            topology_neighborhood,
            topology_construction,
            bound_geometry,
            projected,
            transform_receipts,
            replay_receipts,
        }
    }

    pub fn recipe(&self) -> WorkloadCatalogRecipeKind {
        self.recipe
    }

    pub fn declaration(&self) -> &WorkloadCatalogDeclarationReceipt {
        &self.declaration
    }

    pub fn support(&self) -> &WorkloadCatalogSupportReceipt {
        &self.support
    }

    pub fn workload(&self) -> &WorthWorkload {
        &self.workload
    }

    pub fn topology_neighborhood(&self) -> Option<&TopologySeedNeighborhoodReceipt> {
        self.topology_neighborhood.as_ref()
    }

    pub fn topology_construction(&self) -> Option<&NmtTopologyConstructionReceipt> {
        self.topology_construction.as_ref()
    }

    pub fn projected_workload(&self) -> &ProjectedPlanarWorkload {
        &self.projected
    }

    pub fn bound_geometry(&self) -> &BoundGeometryWorkload {
        &self.bound_geometry
    }

    pub fn transform_receipts(&self) -> &TransformReceiptSet {
        &self.transform_receipts
    }

    pub fn replay_receipts(&self) -> Option<&ReplayReceiptSet> {
        self.replay_receipts.as_ref()
    }

    pub fn into_workload(self) -> WorthWorkload {
        self.workload
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BuiltCleanFailCatalogRecipe {
    recipe: WorkloadCatalogRecipeKind,
    declaration: WorkloadCatalogDeclarationReceipt,
    support: WorkloadCatalogSupportReceipt,
    topology_clean_fail: TopologySeedCleanFailReceipt,
}

impl BuiltCleanFailCatalogRecipe {
    pub(crate) fn new(
        recipe: WorkloadCatalogRecipeKind,
        declaration: WorkloadCatalogDeclarationReceipt,
        support: WorkloadCatalogSupportReceipt,
        topology_clean_fail: TopologySeedCleanFailReceipt,
    ) -> Self {
        Self {
            recipe,
            declaration,
            support,
            topology_clean_fail,
        }
    }

    pub fn recipe(&self) -> WorkloadCatalogRecipeKind {
        self.recipe
    }

    pub fn declaration(&self) -> &WorkloadCatalogDeclarationReceipt {
        &self.declaration
    }

    pub fn support(&self) -> &WorkloadCatalogSupportReceipt {
        &self.support
    }

    pub fn topology_clean_fail(&self) -> &TopologySeedCleanFailReceipt {
        &self.topology_clean_fail
    }

    pub fn into_topology_clean_fail(self) -> TopologySeedCleanFailReceipt {
        self.topology_clean_fail
    }
}
