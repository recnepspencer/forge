use forge_proof::raw::{
    AssumptionBasis, BoundaryBridgedStaleReadableBasis, ContextualTransition, CurrentValidity,
    FreshnessScopedBasis, LowerRecipeTransition, Lowered, Recipe, RecipeResolutionContext,
    ResolveRecipeTransition, Transition,
};

use crate::planning::{AccessPlanIdentity, SelectedDegradedExactScan};

use super::super::transition_authority::{lowering_capability, readiness_authority};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DegradedScanLoweringBasis {
    fingerprint: AccessPlanIdentity,
}

impl DegradedScanLoweringBasis {
    fn new(selected: SelectedDegradedExactScan) -> Self {
        Self {
            fingerprint: selected.fingerprint().clone(),
        }
    }

    pub const fn fingerprint(&self) -> &AccessPlanIdentity {
        &self.fingerprint
    }
}

pub(in crate::access::execution::degraded_scan) type CurrentDegradedScanRecipe = Recipe<
    Lowered,
    SelectedDegradedExactScan,
    FreshnessScopedBasis<CurrentValidity, AssumptionBasis<DegradedScanLoweringBasis>>,
>;
pub(in crate::access::execution::degraded_scan) type StaleDegradedScanRecipe = Recipe<
    Lowered,
    SelectedDegradedExactScan,
    BoundaryBridgedStaleReadableBasis<DegradedScanLoweringBasis>,
>;
#[derive(Debug, PartialEq, Eq)]
pub struct LoweredDegradedExactScan {
    recipe: CurrentDegradedScanRecipe,
}

impl LoweredDegradedExactScan {
    pub(super) fn issue(selected: SelectedDegradedExactScan) -> Self {
        let basis = DegradedScanLoweringBasis::new(selected.clone());
        let resolved = ResolveRecipeTransition.transition(
            Recipe::new(selected),
            RecipeResolutionContext::new(basis, readiness_authority()),
        );
        let recipe = LowerRecipeTransition::new(lowering_capability())
            .transition(resolved.into_value())
            .into_value();
        Self { recipe }
    }

    pub(super) fn into_recipe(self) -> CurrentDegradedScanRecipe {
        self.recipe
    }
    pub fn selected(&self) -> &SelectedDegradedExactScan {
        self.recipe.payload()
    }
    pub fn basis(&self) -> &DegradedScanLoweringBasis {
        self.recipe.strong_basis().value()
    }

    pub(super) fn stale(
        &self,
        materialization: crate::StaleLayoutMaterialization,
    ) -> StaleDegradedExactScan {
        StaleDegradedExactScan {
            recipe: self.recipe.clone().bridge_trust_boundary(),
            materialization,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct StaleDegradedExactScan {
    recipe: StaleDegradedScanRecipe,
    materialization: crate::StaleLayoutMaterialization,
}

impl StaleDegradedExactScan {
    pub fn selected(&self) -> &SelectedDegradedExactScan {
        self.recipe.payload()
    }
    pub fn basis(&self) -> &DegradedScanLoweringBasis {
        self.recipe.basis().weakened_basis().basis().value()
    }
    pub(in crate::access::execution::degraded_scan) fn recipe(&self) -> StaleDegradedScanRecipe {
        self.recipe.clone()
    }
    pub const fn stale_materialization(&self) -> &crate::StaleLayoutMaterialization {
        &self.materialization
    }
}
