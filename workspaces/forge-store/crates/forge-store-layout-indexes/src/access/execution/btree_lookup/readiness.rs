use forge_proof::raw::{
    CheckedAdmitExecutionReadyRecipeTransition, ContextualTransition, ExecutionReadinessContext,
    ExecutionReadyAdmissionReadiness, ExecutionReadyRecipe, TransitionOutcome,
};

use super::{BTreeLookupLoweringBasis, LoweredBTreeLookup};
use crate::access::execution::transition_authority::{
    readiness_authority, ExecutionReadinessAuthority, ExecutionReadinessDeferred,
};
use crate::planning::SelectedBTreeLookup;

pub(super) type BTreeLookupReadyRecipe = ExecutionReadyRecipe<
    SelectedBTreeLookup,
    forge_proof::raw::FreshnessScopedBasis<
        forge_proof::raw::CurrentValidity,
        forge_proof::raw::AssumptionBasis<BTreeLookupLoweringBasis>,
    >,
>;

#[derive(Debug, PartialEq, Eq)]
pub struct BTreeLookupReady {
    recipe: BTreeLookupReadyRecipe,
    current_materialization: crate::materialization::CurrentLayoutMaterialization,
}

impl BTreeLookupReady {
    pub(super) fn issue(
        lowered: LoweredBTreeLookup,
        frontier: crate::CurrentMaterializationFrontier,
    ) -> super::BTreeLookupReadinessOutcome {
        let materialization = lowered
            .selected()
            .materialization()
            .expect("indexed selection retains admitted materialization")
            .clone();
        let current_materialization = match materialization.classify_freshness_at(frontier) {
            Ok(crate::MaterializationFreshness::Current(current)) => current,
            Ok(crate::MaterializationFreshness::Stale(stale)) => {
                return super::BTreeLookupReadinessOutcome::stale(lowered.bridge_to_stale(stale));
            }
            Err(denial) => unreachable!(
                "B-tree lookup selection retains exact admitted materialization: {denial:?}"
            ),
        };
        let outcome = CheckedAdmitExecutionReadyRecipeTransition.transition(
            lowered.into_recipe(),
            ExecutionReadyAdmissionReadiness::<
                SelectedBTreeLookup,
                BTreeLookupLoweringBasis,
                &'static str,
                ExecutionReadinessAuthority,
                ExecutionReadinessDeferred,
                ExecutionReadinessDeferred,
                ExecutionReadinessDeferred,
            >::ready(ExecutionReadinessContext::new(
                "indexed-ready",
                readiness_authority(),
            )),
        );
        match outcome {
            TransitionOutcome::Success(recipe) => super::BTreeLookupReadinessOutcome::ready(Self {
                recipe,
                current_materialization,
            }),
            _ => unreachable!("indexed readiness is issued only after owner classification"),
        }
    }

    pub fn selected(&self) -> &SelectedBTreeLookup {
        self.recipe.payload()
    }
    pub fn basis(&self) -> &BTreeLookupLoweringBasis {
        self.recipe.strong_basis().value()
    }
    pub const fn current_materialization(
        &self,
    ) -> &crate::materialization::CurrentLayoutMaterialization {
        &self.current_materialization
    }
}
