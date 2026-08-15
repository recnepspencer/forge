use worth_proof::{
    AssumptionBasis, ContextualTransition, CurrentValidity, FreshnessScopedBasis, Recipe,
    RecipeResolutionContext, ResolveRecipeTransition, Resolved, TransitionOutcome, Unresolved,
};

use super::super::output_commit::{CommittedProducedAspectDelta, ProducedAspectDelta};
use super::super::revalidation::CanonicalInvalidationOrigin;
use super::resolved::InvalidationWorkBatch;
use super::{InvalidationOriginBindingAxes, PreparedDirectInvalidation};
use crate::data::graph::OutputCommitPublicationReceipt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DirectPublicationBinding {
    producer: crate::data::handle::NodeId,
    ordinal: super::super::binding::OutputCommitOrdinal,
}

type CurrentDirectPublicationBasis =
    FreshnessScopedBasis<CurrentValidity, AssumptionBasis<DirectPublicationBinding>>;

worth_proof::authority_marker!(DependencyPublicationAuthority);

/// Proof that one prepared producer delta crossed the atomic publication door.
#[derive(Debug)]
pub(crate) struct CommittedDirectInvalidation {
    _recipe: Recipe<Resolved, ProducedAspectDelta, CurrentDirectPublicationBasis>,
    publication: CommittedProducedAspectDelta,
}

impl CommittedDirectInvalidation {
    pub(crate) fn after_publication(
        prepared: PreparedDirectInvalidation,
        publication: CommittedProducedAspectDelta,
        _receipt: &OutputCommitPublicationReceipt,
    ) -> Self {
        debug_assert_eq!(prepared.delta(), publication.delta());
        let binding = DirectPublicationBinding {
            producer: publication.delta().producer,
            ordinal: publication.delta().output_commit_ordinal,
        };
        let recipe = ResolveRecipeTransition
            .transition(
                prepared.into_recipe(),
                RecipeResolutionContext::new(binding, DependencyPublicationAuthority::witness()),
            )
            .into_value();
        Self {
            _recipe: recipe,
            publication,
        }
    }

    pub(crate) fn publication(&self) -> &CommittedProducedAspectDelta {
        &self.publication
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DependencyAdmissionDenial {
    WrongOrigin,
    RebindRequired,
    StaleRevision,
    StaleGraphInstance,
    StaleCommitOrdinals,
}

/// Current consumer invalidation admitted from canonical performed causes.
#[derive(Debug)]
pub(crate) struct AdmittedDependencyRecompute {
    recipe: Recipe<Unresolved, InvalidationWorkBatch>,
    binding: InvalidationOriginBindingAxes,
}

impl AdmittedDependencyRecompute {
    pub(super) fn admit(
        batch: InvalidationWorkBatch,
        binding: InvalidationOriginBindingAxes,
    ) -> TransitionOutcome<Self, DependencyAdmissionDenial> {
        let item = batch.first();
        if item.input().origin() != CanonicalInvalidationOrigin::DependencyCommit {
            return TransitionOutcome::denied(DependencyAdmissionDenial::WrongOrigin);
        }
        if item.target() != binding.target {
            return TransitionOutcome::denied(DependencyAdmissionDenial::RebindRequired);
        }
        if item.dependency_revision() != binding.dependency_revision {
            return TransitionOutcome::denied(DependencyAdmissionDenial::StaleRevision);
        }
        let super::InvalidationOriginBinding::DependencyCommit {
            producer_commit_ordinals,
            ..
        } = &binding.origin
        else {
            return TransitionOutcome::denied(DependencyAdmissionDenial::WrongOrigin);
        };
        let causes = item
            .input()
            .dependency_causes()
            .expect("dependency origin retains canonical causes");
        if causes.iter().any(|cause| {
            cause.key.graph_instance != binding.graph_instance
                || cause.key.consumer != binding.target
        }) {
            return TransitionOutcome::denied(DependencyAdmissionDenial::StaleGraphInstance);
        }
        if causes
            .iter()
            .any(|cause| cause.key.dependency_revision != binding.dependency_revision)
        {
            return TransitionOutcome::denied(DependencyAdmissionDenial::StaleRevision);
        }
        let mut actual = causes
            .iter()
            .map(|cause| cause.binding_axes.output_commit_ordinal)
            .collect::<Vec<_>>();
        actual.sort_unstable();
        actual.dedup();
        if actual != *producer_commit_ordinals {
            return TransitionOutcome::denied(DependencyAdmissionDenial::StaleCommitOrdinals);
        }
        TransitionOutcome::success(Self {
            recipe: Recipe::new(batch),
            binding,
        })
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        Recipe<Unresolved, InvalidationWorkBatch>,
        InvalidationOriginBindingAxes,
    ) {
        (self.recipe, self.binding)
    }
}
