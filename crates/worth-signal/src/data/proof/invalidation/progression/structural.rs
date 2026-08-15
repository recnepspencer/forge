use worth_proof::{Recipe, TransitionOutcome, Unresolved};

use super::super::revalidation::CanonicalInvalidationOrigin;
use super::resolved::InvalidationWorkBatch;
use super::InvalidationOriginBindingAxes;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum StructuralAdmissionDenial {
    WrongOrigin,
    RebindRequired,
    StaleRevision,
    StaleMutation,
}

/// A performed topology mutation admitted as structural recompute work.
#[derive(Debug)]
pub(crate) struct AdmittedStructuralRecompute {
    recipe: Recipe<Unresolved, InvalidationWorkBatch>,
    binding: InvalidationOriginBindingAxes,
}

impl AdmittedStructuralRecompute {
    pub(super) fn admit(
        batch: InvalidationWorkBatch,
        binding: InvalidationOriginBindingAxes,
    ) -> TransitionOutcome<Self, StructuralAdmissionDenial> {
        let item = batch.first();
        if item.input().origin() != CanonicalInvalidationOrigin::StructuralRecompute {
            return TransitionOutcome::denied(StructuralAdmissionDenial::WrongOrigin);
        }
        if item.target() != binding.target {
            return TransitionOutcome::denied(StructuralAdmissionDenial::RebindRequired);
        }
        if item.dependency_revision() != binding.dependency_revision {
            return TransitionOutcome::denied(StructuralAdmissionDenial::StaleRevision);
        }
        let super::InvalidationOriginBinding::StructuralMutation { ordinal } = &binding.origin
        else {
            return TransitionOutcome::denied(StructuralAdmissionDenial::WrongOrigin);
        };
        if item.input().origin_generation() != Some(*ordinal) {
            return TransitionOutcome::denied(StructuralAdmissionDenial::StaleMutation);
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
