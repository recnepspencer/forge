use forge_proof::raw::{
    AssumptionBasis, ContextualTransition, CurrentValidity, FreshnessScopedBasis,
    LowerRecipeTransition, Lowered, Recipe, RecipeResolutionContext, ResolveRecipeTransition,
    Transition,
};

use crate::planning::{AccessPlanIdentity, SelectedBTreeLookup};

use super::authority::{lowering_capability, readiness_authority};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BTreeLookupLoweringBasis {
    fingerprint: AccessPlanIdentity,
}

impl BTreeLookupLoweringBasis {
    fn new(selected: SelectedBTreeLookup) -> Self {
        Self {
            fingerprint: selected.fingerprint().clone(),
        }
    }

    pub const fn fingerprint(&self) -> &AccessPlanIdentity {
        &self.fingerprint
    }
}

pub(in crate::access::execution::btree_lookup) type CurrentBTreeLookupRecipe = Recipe<
    Lowered,
    SelectedBTreeLookup,
    FreshnessScopedBasis<CurrentValidity, AssumptionBasis<BTreeLookupLoweringBasis>>,
>;
#[derive(Debug, PartialEq, Eq)]
pub struct LoweredBTreeLookup {
    recipe: CurrentBTreeLookupRecipe,
}

impl LoweredBTreeLookup {
    fn issue(selected: SelectedBTreeLookup) -> Self {
        let basis = BTreeLookupLoweringBasis::new(selected.clone());
        let resolved = ResolveRecipeTransition.transition(
            Recipe::new(selected),
            RecipeResolutionContext::new(basis, readiness_authority()),
        );
        let recipe = LowerRecipeTransition::new(lowering_capability())
            .transition(resolved.into_value())
            .into_value();
        Self { recipe }
    }

    pub(super) fn into_recipe(self) -> CurrentBTreeLookupRecipe {
        self.recipe
    }
    pub fn selected(&self) -> &SelectedBTreeLookup {
        self.recipe.payload()
    }
}

pub(in crate::access::execution::btree_lookup) fn lower(
    selected: SelectedBTreeLookup,
) -> LoweredBTreeLookup {
    LoweredBTreeLookup::issue(selected)
}
