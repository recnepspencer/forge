use topology::facade::{
    NmtTopologyConstructionReceipt, TopologySeedCleanFailReceipt, TopologySeedNeighborhoodReceipt,
};
use worth_spatial::facade::projection_workload::ProjectedPlanarWorkload;
use worth_spatial::facade::retained_replay_workload::ReplayReceiptSet;
use worth_spatial::facade::transform_workload::TransformReceiptSet;
use worth_spatial::facade::workload_binding::BoundGeometryWorkload;

use super::recipe_kind::WorkloadCatalogRecipeKind;
use super::support_receipt::{WorkloadCatalogDeclarationReceipt, WorkloadCatalogSupportReceipt};
use crate::workload_composition::{PlanarBooleanOperandPairConstructionReceipt, WorthWorkload};

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

#[derive(Clone, Debug, PartialEq)]
pub struct BuiltBooleanOperandPairRecipe {
    recipe: WorkloadCatalogRecipeKind,
    declaration: WorkloadCatalogDeclarationReceipt,
    support: WorkloadCatalogSupportReceipt,
    operand_pair_identity: String,
    left: BuiltWorkloadCatalogRecipe,
    right: BuiltWorkloadCatalogRecipe,
}

impl BuiltBooleanOperandPairRecipe {
    pub(crate) fn new(
        recipe: WorkloadCatalogRecipeKind,
        declaration: WorkloadCatalogDeclarationReceipt,
        support: WorkloadCatalogSupportReceipt,
        operand_pair_identity: String,
        left: BuiltWorkloadCatalogRecipe,
        right: BuiltWorkloadCatalogRecipe,
    ) -> Self {
        Self {
            recipe,
            declaration,
            support,
            operand_pair_identity,
            left,
            right,
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

    pub fn operand_pair_identity(&self) -> &str {
        &self.operand_pair_identity
    }

    pub fn left(&self) -> &BuiltWorkloadCatalogRecipe {
        &self.left
    }

    pub fn right(&self) -> &BuiltWorkloadCatalogRecipe {
        &self.right
    }

    pub fn construction_receipt(&self) -> PlanarBooleanOperandPairConstructionReceipt {
        PlanarBooleanOperandPairConstructionReceipt::from_built_recipe(self)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BuiltBooleanCleanFailCatalogRecipe {
    recipe: WorkloadCatalogRecipeKind,
    declaration: WorkloadCatalogDeclarationReceipt,
    support: WorkloadCatalogSupportReceipt,
    operand_pair_identity: String,
    left_clean_fail: BuiltCleanFailCatalogRecipe,
    right_operand: BuiltWorkloadCatalogRecipe,
}

impl BuiltBooleanCleanFailCatalogRecipe {
    pub(crate) fn new(
        recipe: WorkloadCatalogRecipeKind,
        declaration: WorkloadCatalogDeclarationReceipt,
        support: WorkloadCatalogSupportReceipt,
        operand_pair_identity: String,
        left_clean_fail: BuiltCleanFailCatalogRecipe,
        right_operand: BuiltWorkloadCatalogRecipe,
    ) -> Self {
        Self {
            recipe,
            declaration,
            support,
            operand_pair_identity,
            left_clean_fail,
            right_operand,
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

    pub fn operand_pair_identity(&self) -> &str {
        &self.operand_pair_identity
    }

    pub fn left_clean_fail(&self) -> &BuiltCleanFailCatalogRecipe {
        &self.left_clean_fail
    }

    pub fn right_operand(&self) -> &BuiltWorkloadCatalogRecipe {
        &self.right_operand
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BuiltBooleanDeniedCatalogRecipe {
    recipe: WorkloadCatalogRecipeKind,
    declaration: WorkloadCatalogDeclarationReceipt,
    support: WorkloadCatalogSupportReceipt,
    operand_pair_identity: String,
    denial_reason: String,
    left_operand: BuiltWorkloadCatalogRecipe,
    right_operand: BuiltWorkloadCatalogRecipe,
}

impl BuiltBooleanDeniedCatalogRecipe {
    pub(crate) fn new(
        recipe: WorkloadCatalogRecipeKind,
        declaration: WorkloadCatalogDeclarationReceipt,
        support: WorkloadCatalogSupportReceipt,
        operand_pair_identity: String,
        denial_reason: String,
        left_operand: BuiltWorkloadCatalogRecipe,
        right_operand: BuiltWorkloadCatalogRecipe,
    ) -> Self {
        Self {
            recipe,
            declaration,
            support,
            operand_pair_identity,
            denial_reason,
            left_operand,
            right_operand,
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

    pub fn operand_pair_identity(&self) -> &str {
        &self.operand_pair_identity
    }

    pub fn denial_reason(&self) -> &str {
        &self.denial_reason
    }

    pub fn left_operand(&self) -> &BuiltWorkloadCatalogRecipe {
        &self.left_operand
    }

    pub fn right_operand(&self) -> &BuiltWorkloadCatalogRecipe {
        &self.right_operand
    }
}
