use worth_proof::raw::{
    AssumptionBasis, BoundaryBridgedRebindRequiredBasis, BoundaryBridgedStaleReadableBasis,
    ContextualTransition, CurrentValidity, FreshnessScopedBasis, LowerRecipeTransition, Lowered,
    Recipe, RecipeResolutionContext, ResolveRecipeTransition, Resolved, Transition,
};

use crate::planning::{S8PlanFingerprint, S8SelectedAccessPlan};

use super::freshness::{lowering_capability, readiness_authority};
use super::path_kind::S8AccessPathKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8AccessLoweringBasis {
    fingerprint: S8PlanFingerprint,
    path_kind: S8AccessPathKind,
}

impl S8AccessLoweringBasis {
    pub(crate) const fn new(fingerprint: S8PlanFingerprint, path_kind: S8AccessPathKind) -> Self {
        Self {
            fingerprint,
            path_kind,
        }
    }

    pub const fn fingerprint(self) -> S8PlanFingerprint {
        self.fingerprint
    }

    pub const fn path_kind(self) -> S8AccessPathKind {
        self.path_kind
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8LoweredAccessPayload {
    selected: S8SelectedAccessPlan,
    path_kind: S8AccessPathKind,
}

impl S8LoweredAccessPayload {
    pub(crate) const fn new(selected: S8SelectedAccessPlan, path_kind: S8AccessPathKind) -> Self {
        Self {
            selected,
            path_kind,
        }
    }

    pub const fn selected(self) -> S8SelectedAccessPlan {
        self.selected
    }

    pub const fn path_kind(self) -> S8AccessPathKind {
        self.path_kind
    }
}

type CurrentLoweredRecipe = Recipe<
    Lowered,
    S8LoweredAccessPayload,
    FreshnessScopedBasis<CurrentValidity, AssumptionBasis<S8AccessLoweringBasis>>,
>;
type BridgedStaleRecipe = Recipe<
    Lowered,
    S8LoweredAccessPayload,
    BoundaryBridgedStaleReadableBasis<S8AccessLoweringBasis>,
>;
type BridgedRebindRecipe = Recipe<
    Resolved,
    S8LoweredAccessPayload,
    BoundaryBridgedRebindRequiredBasis<S8AccessLoweringBasis>,
>;

#[derive(Debug, PartialEq, Eq)]
pub struct S8LoweredAccessReceipt {
    recipe: CurrentLoweredRecipe,
}

impl S8LoweredAccessReceipt {
    pub(crate) fn lower(selected: S8SelectedAccessPlan, path_kind: S8AccessPathKind) -> Self {
        let payload = S8LoweredAccessPayload::new(selected, path_kind);
        let basis = S8AccessLoweringBasis::new(selected.fingerprint(), path_kind);
        let resolved = ResolveRecipeTransition.transition(
            Recipe::new(payload),
            RecipeResolutionContext::new(basis, readiness_authority()),
        );
        let lowered = LowerRecipeTransition::new(lowering_capability())
            .transition(resolved.into_value())
            .into_value();

        Self { recipe: lowered }
    }

    pub(crate) fn into_recipe(self) -> CurrentLoweredRecipe {
        self.recipe
    }

    pub(crate) fn bridge_to_stale(&self) -> S8StaleLoweredAccessReceipt {
        S8StaleLoweredAccessReceipt {
            recipe: self.recipe.clone().bridge_trust_boundary(),
        }
    }

    pub(crate) fn bridge_to_rebind(&self) -> S8RebindRequiredAccessReceipt {
        let payload = *self.recipe.payload();
        let basis = *self.recipe.strong_basis().value();
        let resolved = ResolveRecipeTransition.transition(
            Recipe::new(payload),
            RecipeResolutionContext::new(basis, readiness_authority()),
        );

        S8RebindRequiredAccessReceipt {
            recipe: resolved.into_value().bridge_trust_boundary(),
        }
    }

    pub fn selected(&self) -> S8SelectedAccessPlan {
        self.recipe.payload().selected()
    }

    pub fn path_kind(&self) -> S8AccessPathKind {
        self.recipe.payload().path_kind()
    }

    pub fn basis(&self) -> S8AccessLoweringBasis {
        *self.recipe.strong_basis().value()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct S8StaleLoweredAccessReceipt {
    recipe: BridgedStaleRecipe,
}

impl S8StaleLoweredAccessReceipt {
    pub(crate) fn recipe(&self) -> BridgedStaleRecipe {
        self.recipe.clone()
    }

    pub fn selected(&self) -> S8SelectedAccessPlan {
        self.recipe.payload().selected()
    }

    pub fn path_kind(&self) -> S8AccessPathKind {
        self.recipe.payload().path_kind()
    }

    pub fn basis(&self) -> S8AccessLoweringBasis {
        *self.recipe.basis().weakened_basis().basis().value()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct S8RebindRequiredAccessReceipt {
    recipe: BridgedRebindRecipe,
}

impl S8RebindRequiredAccessReceipt {
    pub(crate) fn rebind(self) -> S8LoweredAccessReceipt {
        let basis = *self.recipe.basis().weakened_basis().basis().value();
        let rebound = self
            .recipe
            .rebind_with_authority(basis, readiness_authority());
        let lowered = LowerRecipeTransition::new(lowering_capability())
            .transition(rebound)
            .into_value();

        S8LoweredAccessReceipt { recipe: lowered }
    }

    pub fn selected(&self) -> S8SelectedAccessPlan {
        self.recipe.payload().selected()
    }

    pub fn path_kind(&self) -> S8AccessPathKind {
        self.recipe.payload().path_kind()
    }

    pub fn basis(&self) -> S8AccessLoweringBasis {
        *self.recipe.basis().weakened_basis().basis().value()
    }
}
