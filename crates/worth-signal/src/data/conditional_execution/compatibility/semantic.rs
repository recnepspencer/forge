use crate::data::comparator::VersionComparatorPolicy;
use crate::data::node::EvaluationCondition;

use super::super::{
    InstalledSignalConditionalContract, SignalConditionalArtifactReusePolicy,
    SignalConditionalCondition, SignalDeltaThresholdContract,
};
use super::mismatch::{
    SignalConditionalArtifactReuseClass, SignalConditionalComparatorClass,
    SignalConditionalComparatorPosition, SignalConditionalConditionClass,
    SignalConditionalSemanticMismatch,
};
use super::{SignalConditionalComparisonWork, SignalConditionalSemanticComparisonMismatch};

/// Opaque proof that two installed contracts preserve the same conditional
/// meaning. It borrows the exact pair compared and cannot be constructed by a
/// consumer.
#[must_use]
pub struct SignalConditionalSemanticContinuity<'contract> {
    current: &'contract InstalledSignalConditionalContract,
    candidate: &'contract InstalledSignalConditionalContract,
    work: SignalConditionalComparisonWork,
}

impl std::fmt::Debug for SignalConditionalSemanticContinuity<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SignalConditionalSemanticContinuity")
            .finish_non_exhaustive()
    }
}

impl SignalConditionalSemanticContinuity<'_> {
    pub const fn work(&self) -> SignalConditionalComparisonWork {
        self.work
    }

    pub(super) const fn contracts(
        &self,
    ) -> (
        &InstalledSignalConditionalContract,
        &InstalledSignalConditionalContract,
    ) {
        (self.current, self.candidate)
    }
}

impl InstalledSignalConditionalContract {
    pub fn compare_semantic_continuity<'contract>(
        &'contract self,
        candidate: &'contract Self,
    ) -> Result<
        SignalConditionalSemanticContinuity<'contract>,
        SignalConditionalSemanticComparisonMismatch,
    > {
        let mut work = SignalConditionalComparisonWork::default();
        compare_condition(
            self.semantic_condition(),
            candidate.semantic_condition(),
            &mut work,
        )
        .map_err(|mismatch| SignalConditionalSemanticComparisonMismatch::new(mismatch, work))?;
        compare_installed_condition_meaning(self.condition(), candidate.condition(), &mut work)
            .map_err(|mismatch| SignalConditionalSemanticComparisonMismatch::new(mismatch, work))?;
        compare_mask(
            self.dependency_aspects(),
            candidate.dependency_aspects(),
            &mut work,
            |current, candidate| SignalConditionalSemanticMismatch::DependencyAspects {
                current,
                candidate,
            },
        )
        .map_err(|mismatch| SignalConditionalSemanticComparisonMismatch::new(mismatch, work))?;
        compare_mask(
            self.trigger_aspects(),
            candidate.trigger_aspects(),
            &mut work,
            |current, candidate| SignalConditionalSemanticMismatch::TriggerAspects {
                current,
                candidate,
            },
        )
        .map_err(|mismatch| SignalConditionalSemanticComparisonMismatch::new(mismatch, work))?;
        compare_comparator(
            SignalConditionalComparatorPosition::Dependency,
            self.dependency_comparator(),
            candidate.dependency_comparator(),
            &mut work,
        )
        .map_err(|mismatch| SignalConditionalSemanticComparisonMismatch::new(mismatch, work))?;
        compare_comparator(
            SignalConditionalComparatorPosition::Output,
            self.output_comparator(),
            candidate.output_comparator(),
            &mut work,
        )
        .map_err(|mismatch| SignalConditionalSemanticComparisonMismatch::new(mismatch, work))?;
        compare_artifact_reuse(self.artifact_reuse(), candidate.artifact_reuse(), &mut work)
            .map_err(|mismatch| SignalConditionalSemanticComparisonMismatch::new(mismatch, work))?;

        Ok(SignalConditionalSemanticContinuity {
            current: self,
            candidate,
            work,
        })
    }
}

fn compare_condition(
    current: &SignalConditionalCondition,
    candidate: &SignalConditionalCondition,
    work: &mut SignalConditionalComparisonWork,
) -> Result<(), SignalConditionalSemanticMismatch> {
    work.inspect_semantic();
    match (current, candidate) {
        (SignalConditionalCondition::Always, SignalConditionalCondition::Always)
        | (SignalConditionalCondition::OnDemand, SignalConditionalCondition::OnDemand)
        | (
            SignalConditionalCondition::RuntimePredicate,
            SignalConditionalCondition::RuntimePredicate,
        )
        | (SignalConditionalCondition::TemporalWake, SignalConditionalCondition::TemporalWake) => {
            Ok(())
        }
        (
            SignalConditionalCondition::AspectFilter(current),
            SignalConditionalCondition::AspectFilter(candidate),
        ) => compare_mask(*current, *candidate, work, |current, candidate| {
            SignalConditionalSemanticMismatch::AspectFilterMask { current, candidate }
        }),
        (
            SignalConditionalCondition::DeltaThreshold(current),
            SignalConditionalCondition::DeltaThreshold(candidate),
        ) => compare_threshold(current, candidate, work),
        _ => Err(SignalConditionalSemanticMismatch::ConditionClass {
            current: condition_class(current),
            candidate: condition_class(candidate),
        }),
    }
}

fn compare_threshold(
    current: &SignalDeltaThresholdContract,
    candidate: &SignalDeltaThresholdContract,
    work: &mut SignalConditionalComparisonWork,
) -> Result<(), SignalConditionalSemanticMismatch> {
    work.inspect_semantic();
    if current.threshold() != candidate.threshold() {
        return Err(SignalConditionalSemanticMismatch::ThresholdValue {
            current: current.threshold().clone(),
            candidate: candidate.threshold().clone(),
        });
    }
    work.inspect_semantic();
    if current.unit_identity() != candidate.unit_identity() {
        return Err(SignalConditionalSemanticMismatch::ThresholdUnitIdentity {
            current: current.unit_identity().to_owned(),
            candidate: candidate.unit_identity().to_owned(),
        });
    }
    work.inspect_semantic();
    if current.value_family() != candidate.value_family() {
        return Err(SignalConditionalSemanticMismatch::ThresholdValueFamily {
            current: current.value_family(),
            candidate: candidate.value_family(),
        });
    }
    work.inspect_semantic();
    if current.comparison_domain() != candidate.comparison_domain() {
        return Err(
            SignalConditionalSemanticMismatch::ThresholdComparisonDomain {
                current: current.comparison_domain(),
                candidate: candidate.comparison_domain(),
            },
        );
    }
    work.inspect_semantic();
    if current.boundary() != candidate.boundary() {
        return Err(SignalConditionalSemanticMismatch::ThresholdBoundary {
            current: current.boundary(),
            candidate: candidate.boundary(),
        });
    }
    Ok(())
}

fn compare_mask(
    current: crate::data::aspect::AspectMask,
    candidate: crate::data::aspect::AspectMask,
    work: &mut SignalConditionalComparisonWork,
    mismatch: impl FnOnce(
        crate::data::aspect::AspectMask,
        crate::data::aspect::AspectMask,
    ) -> SignalConditionalSemanticMismatch,
) -> Result<(), SignalConditionalSemanticMismatch> {
    work.inspect_semantic();
    if current == candidate {
        Ok(())
    } else {
        Err(mismatch(current, candidate))
    }
}

fn compare_comparator(
    position: SignalConditionalComparatorPosition,
    current: &VersionComparatorPolicy,
    candidate: &VersionComparatorPolicy,
    work: &mut SignalConditionalComparisonWork,
) -> Result<(), SignalConditionalSemanticMismatch> {
    work.inspect_semantic();
    let current_class = comparator_class(current);
    let candidate_class = comparator_class(candidate);
    if current_class != candidate_class {
        return Err(SignalConditionalSemanticMismatch::ComparatorClass {
            position,
            current: current_class,
            candidate: candidate_class,
        });
    }
    match (current, candidate) {
        (
            VersionComparatorPolicy::Tolerance {
                epsilon: current_epsilon,
            },
            VersionComparatorPolicy::Tolerance {
                epsilon: candidate_epsilon,
            },
        ) => {
            work.inspect_semantic();
            if current_epsilon == candidate_epsilon {
                return Ok(());
            }
            Err(SignalConditionalSemanticMismatch::ComparatorTolerance {
                position,
                current_epsilon: *current_epsilon,
                candidate_epsilon: *candidate_epsilon,
            })
        }
        (
            VersionComparatorPolicy::Custom { key: current },
            VersionComparatorPolicy::Custom { key: candidate },
        ) => {
            work.inspect_semantic();
            if current == candidate {
                return Ok(());
            }
            Err(SignalConditionalSemanticMismatch::ComparatorCustomKey {
                position,
                current: current.clone(),
                candidate: candidate.clone(),
            })
        }
        (
            VersionComparatorPolicy::Installed { identity: current },
            VersionComparatorPolicy::Installed {
                identity: candidate,
            },
        ) => {
            work.inspect_semantic();
            if current.role() == candidate.role() {
                return Ok(());
            }
            Err(SignalConditionalSemanticMismatch::InstalledComparatorMeaningUnproven { position })
        }
        _ => Ok(()),
    }
}

fn compare_artifact_reuse(
    current: &SignalConditionalArtifactReusePolicy,
    candidate: &SignalConditionalArtifactReusePolicy,
    work: &mut SignalConditionalComparisonWork,
) -> Result<(), SignalConditionalSemanticMismatch> {
    work.inspect_semantic();
    let current_class = artifact_reuse_class(current);
    let candidate_class = artifact_reuse_class(candidate);
    if current_class != candidate_class {
        return Err(SignalConditionalSemanticMismatch::ArtifactReuseClass {
            current: current_class,
            candidate: candidate_class,
        });
    }
    match (current, candidate) {
        (
            SignalConditionalArtifactReusePolicy::Installed(current),
            SignalConditionalArtifactReusePolicy::Installed(candidate),
        ) => {
            work.inspect_semantic();
            if current.role() == candidate.role() {
                return Ok(());
            }
            Err(SignalConditionalSemanticMismatch::InstalledArtifactReuseMeaningUnproven)
        }
        _ => Ok(()),
    }
}

fn compare_installed_condition_meaning(
    current: &EvaluationCondition,
    candidate: &EvaluationCondition,
    work: &mut SignalConditionalComparisonWork,
) -> Result<(), SignalConditionalSemanticMismatch> {
    work.inspect_semantic();
    match (current, candidate) {
        (EvaluationCondition::Installed(current), EvaluationCondition::Installed(candidate))
            if current.role() != candidate.role() =>
        {
            Err(SignalConditionalSemanticMismatch::InstalledConditionMeaningUnproven)
        }
        _ => Ok(()),
    }
}

fn condition_class(condition: &SignalConditionalCondition) -> SignalConditionalConditionClass {
    match condition {
        SignalConditionalCondition::Always => SignalConditionalConditionClass::Always,
        SignalConditionalCondition::AspectFilter(_) => {
            SignalConditionalConditionClass::AspectFilter
        }
        SignalConditionalCondition::DeltaThreshold(_) => {
            SignalConditionalConditionClass::DeltaThreshold
        }
        SignalConditionalCondition::OnDemand => SignalConditionalConditionClass::OnDemand,
        SignalConditionalCondition::RuntimePredicate => {
            SignalConditionalConditionClass::RuntimePredicate
        }
        SignalConditionalCondition::TemporalWake => SignalConditionalConditionClass::TemporalWake,
    }
}

fn comparator_class(comparator: &VersionComparatorPolicy) -> SignalConditionalComparatorClass {
    match comparator {
        VersionComparatorPolicy::Exact => SignalConditionalComparatorClass::Exact,
        VersionComparatorPolicy::Tolerance { .. } => SignalConditionalComparatorClass::Tolerance,
        VersionComparatorPolicy::OutputIdentity => SignalConditionalComparatorClass::OutputIdentity,
        VersionComparatorPolicy::Custom { .. } => SignalConditionalComparatorClass::Custom,
        VersionComparatorPolicy::Installed { .. } => {
            SignalConditionalComparatorClass::RuntimeResolved
        }
    }
}

fn artifact_reuse_class(
    reuse: &SignalConditionalArtifactReusePolicy,
) -> SignalConditionalArtifactReuseClass {
    match reuse {
        SignalConditionalArtifactReusePolicy::NotReusable => {
            SignalConditionalArtifactReuseClass::NotReusable
        }
        SignalConditionalArtifactReusePolicy::DependencyAndOutputEquivalent => {
            SignalConditionalArtifactReuseClass::DependencyAndOutputEquivalent
        }
        SignalConditionalArtifactReusePolicy::OutputEquivalent => {
            SignalConditionalArtifactReuseClass::OutputEquivalent
        }
        SignalConditionalArtifactReusePolicy::Installed(_) => {
            SignalConditionalArtifactReuseClass::RuntimeResolved
        }
    }
}
