use worth_proof::{Recipe, TransitionOutcome, Unresolved};

use super::resolved::InvalidationWorkBatch;
use super::InvalidationOriginBindingAxes;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SourceAdmissionDenial {
    WrongOrigin,
    RebindRequired,
    StaleRevision,
    StaleGeneration,
}

/// A current persisted source obligation admitted for resolution.
#[derive(Debug)]
pub(crate) struct AdmittedSourceRecompute {
    recipe: Recipe<Unresolved, InvalidationWorkBatch>,
    binding: InvalidationOriginBindingAxes,
}

impl AdmittedSourceRecompute {
    pub(super) fn admit(
        batch: InvalidationWorkBatch,
        binding: InvalidationOriginBindingAxes,
    ) -> TransitionOutcome<Self, SourceAdmissionDenial> {
        let item = batch.first();
        if !item.input().is_source_recompute() {
            return TransitionOutcome::denied(SourceAdmissionDenial::WrongOrigin);
        }
        if item.target() != binding.target {
            return TransitionOutcome::denied(SourceAdmissionDenial::RebindRequired);
        }
        if item.dependency_revision() != binding.dependency_revision {
            return TransitionOutcome::denied(SourceAdmissionDenial::StaleRevision);
        }
        let super::InvalidationOriginBinding::SourceAdmission { generation } = &binding.origin
        else {
            return TransitionOutcome::denied(SourceAdmissionDenial::WrongOrigin);
        };
        if item.input().origin_generation() != Some(*generation) {
            return TransitionOutcome::denied(SourceAdmissionDenial::StaleGeneration);
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
