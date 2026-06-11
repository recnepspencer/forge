use topology::facade::{TopologySeedCleanFailReceipt, TopologySeedNeighborhoodReceipt};
use worth_spatial::facade::projection_workload::ProjectedPlanarWorkload;

use super::catalog::{WorkloadCatalogDeclarationReceipt, WorkloadCatalogSupportReceipt};
use super::recipe_kind::WorkloadCatalogRecipeKind;
use crate::workload_composition::WorthWorkload;

#[derive(Clone, Debug, PartialEq)]
pub struct BuiltWorkloadCatalogRecipe {
    recipe: WorkloadCatalogRecipeKind,
    declaration: WorkloadCatalogDeclarationReceipt,
    support: WorkloadCatalogSupportReceipt,
    workload: WorthWorkload,
    topology_neighborhood: Option<TopologySeedNeighborhoodReceipt>,
    projected: ProjectedPlanarWorkload,
}

impl BuiltWorkloadCatalogRecipe {
    pub(crate) fn new(
        recipe: WorkloadCatalogRecipeKind,
        declaration: WorkloadCatalogDeclarationReceipt,
        support: WorkloadCatalogSupportReceipt,
        workload: WorthWorkload,
        topology_neighborhood: Option<TopologySeedNeighborhoodReceipt>,
        projected: ProjectedPlanarWorkload,
    ) -> Self {
        Self {
            recipe,
            declaration,
            support,
            workload,
            topology_neighborhood,
            projected,
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

    pub fn projected_workload(&self) -> &ProjectedPlanarWorkload {
        &self.projected
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
