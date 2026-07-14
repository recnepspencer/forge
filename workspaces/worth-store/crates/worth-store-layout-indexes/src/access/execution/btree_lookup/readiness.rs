use worth_proof::raw::{
    CheckedAdmitExecutionReadyRecipeTransition, ContextualTransition, ExecutionReadinessContext,
    ExecutionReadyAdmissionReadiness, ExecutionReadyRecipe, TransitionOutcome,
};

use super::authority::{
    readiness_authority, BTreeLookupReadinessAuthority, BTreeLookupReadinessDeferred,
};
use super::{BTreeLookupLoweringBasis, LoweredBTreeLookup};
use crate::planning::SelectedBTreeLookup;

pub(super) type BTreeLookupReadyRecipe = ExecutionReadyRecipe<
    SelectedBTreeLookup,
    worth_proof::raw::FreshnessScopedBasis<
        worth_proof::raw::CurrentValidity,
        worth_proof::raw::AssumptionBasis<BTreeLookupLoweringBasis>,
    >,
>;

#[derive(Debug, PartialEq, Eq)]
pub struct BTreeLookupReady {
    recipe: BTreeLookupReadyRecipe,
    current_materialization: crate::materialization::CurrentLayoutMaterialization,
}

impl BTreeLookupReady {
    fn issue(
        lowered: LoweredBTreeLookup,
        current_materialization: crate::CurrentLayoutMaterialization,
    ) -> Self {
        let outcome = CheckedAdmitExecutionReadyRecipeTransition.transition(
            lowered.into_recipe(),
            ExecutionReadyAdmissionReadiness::<
                SelectedBTreeLookup,
                BTreeLookupLoweringBasis,
                &'static str,
                BTreeLookupReadinessAuthority,
                BTreeLookupReadinessDeferred,
                BTreeLookupReadinessDeferred,
                BTreeLookupReadinessDeferred,
            >::ready(ExecutionReadinessContext::new(
                "indexed-ready",
                readiness_authority(),
            )),
        );
        match outcome {
            TransitionOutcome::Success(recipe) => Self {
                recipe,
                current_materialization,
            },
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

#[derive(Debug, PartialEq, Eq)]
enum BTreeLookupReadinessCase {
    Ready(BTreeLookupReady),
    Stale(crate::StaleLayoutMaterialization),
}

#[derive(Debug, PartialEq, Eq)]
pub struct BTreeLookupReadinessOutcome {
    case: BTreeLookupReadinessCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BTreeLookupReadinessView<'a> {
    Ready(&'a BTreeLookupReady),
    Stale(&'a crate::StaleLayoutMaterialization),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BTreeLookupReadinessCaseId(&'static str);

impl BTreeLookupReadinessCaseId {
    pub const fn name(self) -> &'static str {
        self.0
    }
}

pub fn btree_lookup_readiness_cases() -> impl Iterator<Item = BTreeLookupReadinessCaseId> {
    [
        BTreeLookupReadinessCaseId("layout.btree_lookup.readiness.ready"),
        BTreeLookupReadinessCaseId("layout.btree_lookup.readiness.stale"),
    ]
    .into_iter()
}

impl BTreeLookupReadinessOutcome {
    fn ready(ready: BTreeLookupReady) -> Self {
        Self {
            case: BTreeLookupReadinessCase::Ready(ready),
        }
    }

    fn stale(stale: crate::StaleLayoutMaterialization) -> Self {
        Self {
            case: BTreeLookupReadinessCase::Stale(stale),
        }
    }

    pub const fn view(&self) -> BTreeLookupReadinessView<'_> {
        match &self.case {
            BTreeLookupReadinessCase::Ready(ready) => BTreeLookupReadinessView::Ready(ready),
            BTreeLookupReadinessCase::Stale(stale) => BTreeLookupReadinessView::Stale(stale),
        }
    }

    pub const fn case_id(&self) -> BTreeLookupReadinessCaseId {
        match self.case {
            BTreeLookupReadinessCase::Ready(_) => {
                BTreeLookupReadinessCaseId("layout.btree_lookup.readiness.ready")
            }
            BTreeLookupReadinessCase::Stale(_) => {
                BTreeLookupReadinessCaseId("layout.btree_lookup.readiness.stale")
            }
        }
    }

    pub fn into_ready(self) -> Result<BTreeLookupReady, Self> {
        match self.case {
            BTreeLookupReadinessCase::Ready(ready) => Ok(ready),
            case => Err(Self { case }),
        }
    }

    pub fn into_stale(self) -> Result<crate::StaleLayoutMaterialization, Self> {
        match self.case {
            BTreeLookupReadinessCase::Stale(stale) => Ok(stale),
            case => Err(Self { case }),
        }
    }
}

pub(in crate::access::execution::btree_lookup) fn admit_ready(
    lowered: LoweredBTreeLookup,
    frontier: crate::CurrentMaterializationFrontier,
) -> BTreeLookupReadinessOutcome {
    let materialization = lowered.selected().materialization().clone();
    match materialization.classify_freshness_at(frontier) {
        Ok(crate::MaterializationFreshness::Current(current)) => {
            BTreeLookupReadinessOutcome::ready(BTreeLookupReady::issue(lowered, current))
        }
        Ok(crate::MaterializationFreshness::Stale(stale)) => {
            BTreeLookupReadinessOutcome::stale(stale)
        }
        Err(denial) => unreachable!(
            "B-tree lookup selection retains exact admitted materialization: {denial:?}"
        ),
    }
}

#[cfg(test)]
mod tests {
    use worth_store_budgets::PreExecutionBudgetEnvelope;
    use worth_store_physical_format::{PhysicalPageId, PhysicalSegmentId};

    #[test]
    fn declared_cases_equal_cases_emitted_by_the_owner_operation() {
        let catalog = crate::bootstrap::test_support::bootstrap_catalog_read_admission();
        let advanced = crate::bootstrap::test_support::advanced_bootstrap_catalog_read_admission();
        let current = super::admit_ready(
            super::super::lower(selected(&catalog)),
            crate::CurrentMaterializationFrontier::from_catalog(&catalog),
        );
        let stale = super::admit_ready(
            super::super::lower(selected(&catalog)),
            crate::CurrentMaterializationFrontier::from_catalog(&advanced),
        );
        let observed = [current.case_id(), stale.case_id()]
            .into_iter()
            .collect::<std::collections::BTreeSet<_>>();
        let declared =
            super::btree_lookup_readiness_cases().collect::<std::collections::BTreeSet<_>>();

        assert_eq!(observed, declared);
    }

    fn selected(catalog: &crate::BootstrapCatalogReadAdmission) -> crate::SelectedBTreeLookup {
        let (family, key_domain) = crate::access::execution::tests_support::admit_page_scope();
        let concrete_key = crate::keyspace::admit_page_key(
            key_domain,
            PhysicalSegmentId::from_raw(1).unwrap(),
            PhysicalPageId::from_raw(1).unwrap(),
        )
        .expect("page identity must pass ordinary key admission");
        let materialization = crate::access_planning()
            .admit_current_catalog_root_materialization(family, catalog)
            .expect("physical catalog must admit exact root materialization");
        let request = crate::AccessPlanSelector
            .admit_read_request(
                family,
                concrete_key,
                materialization,
                crate::access_planning().point_access(),
            )
            .expect("ordinary B-tree request must pass request admission");
        crate::AccessPlanSelector
            .select_admitted_with_budget(request, PreExecutionBudgetEnvelope::foreground_default())
            .into_btree_lookup()
            .expect("ordinary page lookup must select B-tree authority")
    }
}
