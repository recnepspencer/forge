use crate::data::comparator::VersionComparatorPolicy;
use crate::data::node::EvaluationCondition;

use super::super::{InstalledSignalConditionalContract, SignalConditionalArtifactReusePolicy};
use super::{
    SignalConditionalComparisonWork, SignalConditionalExecutionAffinityComparisonMismatch,
    SignalConditionalSemanticContinuity, SignalConditionalSemanticMismatch,
};

/// Opaque proof that two semantically continuous contracts also name the exact
/// same installed Signal execution subject.
#[must_use]
pub struct SignalConditionalExecutionAffinity<'contract> {
    semantic_continuity: SignalConditionalSemanticContinuity<'contract>,
    work: SignalConditionalComparisonWork,
}

impl std::fmt::Debug for SignalConditionalExecutionAffinity<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SignalConditionalExecutionAffinity")
            .finish_non_exhaustive()
    }
}

impl<'contract> SignalConditionalExecutionAffinity<'contract> {
    pub const fn semantic_continuity(&self) -> &SignalConditionalSemanticContinuity<'contract> {
        &self.semantic_continuity
    }

    pub const fn work(&self) -> SignalConditionalComparisonWork {
        self.work
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignalConditionalExecutionAffinityMismatch {
    Semantic(SignalConditionalSemanticMismatch),
    GraphInstance { current: u64, candidate: u64 },
    NodeIndex { current: u32, candidate: u32 },
    NodeGeneration { current: u32, candidate: u32 },
    InstalledConditionIdentity,
    InstalledDependencyComparatorIdentity,
    InstalledOutputComparatorIdentity,
    InstalledArtifactReuseIdentity,
}

impl From<SignalConditionalSemanticMismatch> for SignalConditionalExecutionAffinityMismatch {
    fn from(mismatch: SignalConditionalSemanticMismatch) -> Self {
        Self::Semantic(mismatch)
    }
}

impl InstalledSignalConditionalContract {
    pub fn compare_execution_affinity<'contract>(
        &'contract self,
        candidate: &'contract Self,
    ) -> Result<
        SignalConditionalExecutionAffinity<'contract>,
        SignalConditionalExecutionAffinityComparisonMismatch,
    > {
        let semantic_continuity =
            self.compare_semantic_continuity(candidate)
                .map_err(|denial| {
                    SignalConditionalExecutionAffinityComparisonMismatch::new(
                        SignalConditionalExecutionAffinityMismatch::Semantic(
                            denial.mismatch().clone(),
                        ),
                        denial.work(),
                    )
                })?;
        let mut work = semantic_continuity.work();
        let (current, candidate) = semantic_continuity.contracts();

        work.inspect_affinity();
        if current.graph_instance_id() != candidate.graph_instance_id() {
            return Err(SignalConditionalExecutionAffinityComparisonMismatch::new(
                SignalConditionalExecutionAffinityMismatch::GraphInstance {
                    current: current.graph_instance_id(),
                    candidate: candidate.graph_instance_id(),
                },
                work,
            ));
        }
        work.inspect_affinity();
        if current.node().index() != candidate.node().index() {
            return Err(SignalConditionalExecutionAffinityComparisonMismatch::new(
                SignalConditionalExecutionAffinityMismatch::NodeIndex {
                    current: current.node().index(),
                    candidate: candidate.node().index(),
                },
                work,
            ));
        }
        work.inspect_affinity();
        if current.node().generation() != candidate.node().generation() {
            return Err(SignalConditionalExecutionAffinityComparisonMismatch::new(
                SignalConditionalExecutionAffinityMismatch::NodeGeneration {
                    current: current.node().generation(),
                    candidate: candidate.node().generation(),
                },
                work,
            ));
        }
        inspect_affinity(
            compare_installed_condition_identity(current.condition(), candidate.condition()),
            &mut work,
        )?;
        inspect_affinity(
            compare_installed_comparator_identity(
                current.dependency_comparator(),
                candidate.dependency_comparator(),
                SignalConditionalExecutionAffinityMismatch::InstalledDependencyComparatorIdentity,
            ),
            &mut work,
        )?;
        inspect_affinity(
            compare_installed_comparator_identity(
                current.output_comparator(),
                candidate.output_comparator(),
                SignalConditionalExecutionAffinityMismatch::InstalledOutputComparatorIdentity,
            ),
            &mut work,
        )?;
        inspect_affinity(
            compare_installed_reuse_identity(current.artifact_reuse(), candidate.artifact_reuse()),
            &mut work,
        )?;

        Ok(SignalConditionalExecutionAffinity {
            semantic_continuity,
            work,
        })
    }
}

fn inspect_affinity(
    result: Result<(), SignalConditionalExecutionAffinityMismatch>,
    work: &mut SignalConditionalComparisonWork,
) -> Result<(), SignalConditionalExecutionAffinityComparisonMismatch> {
    work.inspect_affinity();
    result.map_err(|mismatch| {
        SignalConditionalExecutionAffinityComparisonMismatch::new(mismatch, *work)
    })
}

fn compare_installed_condition_identity(
    current: &EvaluationCondition,
    candidate: &EvaluationCondition,
) -> Result<(), SignalConditionalExecutionAffinityMismatch> {
    match (current, candidate) {
        (EvaluationCondition::Installed(current), EvaluationCondition::Installed(candidate))
            if !current.is_same_installed_identity(candidate) =>
        {
            Err(SignalConditionalExecutionAffinityMismatch::InstalledConditionIdentity)
        }
        _ => Ok(()),
    }
}

fn compare_installed_comparator_identity(
    current: &VersionComparatorPolicy,
    candidate: &VersionComparatorPolicy,
    mismatch: SignalConditionalExecutionAffinityMismatch,
) -> Result<(), SignalConditionalExecutionAffinityMismatch> {
    match (current, candidate) {
        (
            VersionComparatorPolicy::Installed { identity: current },
            VersionComparatorPolicy::Installed {
                identity: candidate,
            },
        ) if !current.is_same_installed_identity(candidate) => Err(mismatch),
        _ => Ok(()),
    }
}

fn compare_installed_reuse_identity(
    current: &SignalConditionalArtifactReusePolicy,
    candidate: &SignalConditionalArtifactReusePolicy,
) -> Result<(), SignalConditionalExecutionAffinityMismatch> {
    match (current, candidate) {
        (
            SignalConditionalArtifactReusePolicy::Installed(current),
            SignalConditionalArtifactReusePolicy::Installed(candidate),
        ) if !current.is_same_installed_identity(candidate) => {
            Err(SignalConditionalExecutionAffinityMismatch::InstalledArtifactReuseIdentity)
        }
        _ => Ok(()),
    }
}
