use worth_proof::{
    AssumptionBasis, ContextualTransition, CurrentValidity, FreshnessScopedBasis, NonEmpty, Recipe,
    RecipeResolutionContext, ResolveRecipeTransition, Resolved,
};

use crate::data::handle::NodeId;

use super::super::binding::DependencyRevision;
use super::super::revalidation::CanonicalDependencyCauseSet;
use super::{
    AdmittedDependencyRecompute, AdmittedSourceRecompute, AdmittedStructuralRecompute,
    InvalidationOriginBindingAxes,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct InvalidationWorkItem {
    target: NodeId,
    dependency_revision: DependencyRevision,
    input: CanonicalDependencyCauseSet,
}

impl InvalidationWorkItem {
    pub(crate) fn new(
        target: NodeId,
        dependency_revision: DependencyRevision,
        input: CanonicalDependencyCauseSet,
    ) -> Self {
        Self {
            target,
            dependency_revision,
            input,
        }
    }

    pub(crate) const fn target(&self) -> NodeId {
        self.target
    }

    pub(crate) const fn dependency_revision(&self) -> DependencyRevision {
        self.dependency_revision
    }

    pub(crate) const fn input(&self) -> &CanonicalDependencyCauseSet {
        &self.input
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct InvalidationWorkBatch(NonEmpty<InvalidationWorkItem>);

impl InvalidationWorkBatch {
    pub(crate) fn single(item: InvalidationWorkItem) -> Self {
        Self(NonEmpty::new(item, Vec::new()))
    }

    pub(crate) fn first(&self) -> &InvalidationWorkItem {
        self.0.first()
    }

    pub(crate) fn as_slice(&self) -> &[InvalidationWorkItem] {
        self.0.as_slice()
    }
}

pub(super) type CurrentOriginBasis =
    FreshnessScopedBasis<CurrentValidity, AssumptionBasis<InvalidationOriginBindingAxes>>;

worth_proof::authority_marker!(SourceResolutionAuthority);
worth_proof::authority_marker!(DependencyResolutionAuthority);
worth_proof::authority_marker!(StructuralResolutionAuthority);

/// The sealed convergence point for the three lawful invalidation origins.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ResolvedInvalidationWork {
    recipe: Recipe<Resolved, InvalidationWorkBatch, CurrentOriginBasis>,
}

impl ResolvedInvalidationWork {
    pub(super) fn from_source(admitted: AdmittedSourceRecompute) -> Self {
        let (recipe, binding) = admitted.into_parts();
        Self::resolve(recipe, binding, SourceResolutionAuthority::witness())
    }

    pub(super) fn from_dependency(admitted: AdmittedDependencyRecompute) -> Self {
        let (recipe, binding) = admitted.into_parts();
        Self::resolve(recipe, binding, DependencyResolutionAuthority::witness())
    }

    pub(super) fn from_structural(admitted: AdmittedStructuralRecompute) -> Self {
        let (recipe, binding) = admitted.into_parts();
        Self::resolve(recipe, binding, StructuralResolutionAuthority::witness())
    }

    fn resolve<Auth: worth_proof::AuthorityMarker>(
        recipe: Recipe<worth_proof::Unresolved, InvalidationWorkBatch>,
        binding: InvalidationOriginBindingAxes,
        authority: worth_proof::AuthorityWitness<Auth>,
    ) -> Self {
        Self {
            recipe: ResolveRecipeTransition
                .transition(recipe, RecipeResolutionContext::new(binding, authority))
                .into_value(),
        }
    }

    pub(super) fn into_recipe(self) -> Recipe<Resolved, InvalidationWorkBatch, CurrentOriginBasis> {
        self.recipe
    }

    pub(super) fn origin_binding(&self) -> &InvalidationOriginBindingAxes {
        self.recipe.basis().basis().value()
    }
}
