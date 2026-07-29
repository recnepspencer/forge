use crate::runtime::active::WorthUiActiveArtifact;
use crate::runtime::{
    WorthUiAdmittedReplacementCandidate, WorthUiRuntimeArtifactComparison,
    WorthUiRuntimeArtifactComparisonCounters, WorthUiRuntimeArtifactComparisonDenial,
    WorthUiRuntimeEquivalenceBasis,
};
use crate::source::{WorthUiArtifactEquivalence, WorthUiArtifactEquivalenceComparator};

#[derive(Debug)]
pub struct WorthUiRuntimeArtifactComparator<'a> {
    active_artifact: &'a WorthUiActiveArtifact,
    runtime_basis: WorthUiRuntimeEquivalenceBasis,
}

impl<'a> WorthUiRuntimeArtifactComparator<'a> {
    pub(crate) fn for_active_artifact(active_artifact: &'a WorthUiActiveArtifact) -> Self {
        Self {
            active_artifact,
            runtime_basis: WorthUiRuntimeEquivalenceBasis::semantic_artifact_meaning(),
        }
    }

    pub fn compare_admitted(
        self,
        admitted: &WorthUiAdmittedReplacementCandidate,
    ) -> Result<WorthUiRuntimeArtifactComparison, WorthUiRuntimeArtifactComparisonDenial> {
        self.compare_admitted_bounded(admitted, usize::MAX)
    }

    pub(crate) fn compare_admitted_bounded(
        self,
        admitted: &WorthUiAdmittedReplacementCandidate,
        structural_entry_limit: usize,
    ) -> Result<WorthUiRuntimeArtifactComparison, WorthUiRuntimeArtifactComparisonDenial> {
        require_matching_runtime_equivalence_basis(self.runtime_basis, admitted)?;

        let mut counters = WorthUiRuntimeArtifactComparisonCounters::default();
        counters.record_artifact_comparison();
        let artifact_equivalence = self
            .compare_artifact_equivalence(admitted, structural_entry_limit)
            .map_err(|denial| {
                WorthUiRuntimeArtifactComparisonDenial::StructuralCapacityExceeded {
                    limit: denial.structural_entry_limit(),
                    observed: denial.observed_structural_entries(),
                    counters,
                }
            })?;

        Ok(WorthUiRuntimeArtifactComparison::new(
            self.runtime_basis,
            self.active_artifact.digest(),
            admitted.artifact_bundle().artifact_digest(),
            artifact_equivalence,
            counters,
        ))
    }

    fn compare_artifact_equivalence(
        &self,
        admitted: &WorthUiAdmittedReplacementCandidate,
        structural_entry_limit: usize,
    ) -> Result<WorthUiArtifactEquivalence, crate::source::WorthUiArtifactEquivalenceDenial> {
        WorthUiArtifactEquivalenceComparator::compare_bounded(
            self.active_artifact.artifact(),
            admitted.artifact_bundle().artifact(),
            self.runtime_basis.artifact_equivalence_basis(),
            structural_entry_limit,
        )
    }
}

fn require_matching_runtime_equivalence_basis(
    runtime_basis: WorthUiRuntimeEquivalenceBasis,
    admitted: &WorthUiAdmittedReplacementCandidate,
) -> Result<(), WorthUiRuntimeArtifactComparisonDenial> {
    let candidate_basis = admitted.candidate().basis();
    if candidate_artifact_equivalence_basis_matches_runtime(candidate_basis, runtime_basis) {
        Ok(())
    } else {
        Err(
            WorthUiRuntimeArtifactComparisonDenial::EquivalenceBasisMismatch {
                runtime_basis,
                candidate_basis,
                counters: WorthUiRuntimeArtifactComparisonCounters::default(),
            },
        )
    }
}

fn candidate_artifact_equivalence_basis_matches_runtime(
    candidate_basis: crate::runtime::WorthUiReplacementCandidateBasis,
    runtime_basis: WorthUiRuntimeEquivalenceBasis,
) -> bool {
    candidate_basis.artifact_equivalence_basis() == runtime_basis.artifact_equivalence_basis()
}
