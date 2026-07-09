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

    #[cfg(test)]
    pub(crate) fn with_runtime_basis_for_test(
        mut self,
        runtime_basis: WorthUiRuntimeEquivalenceBasis,
    ) -> Self {
        self.runtime_basis = runtime_basis;
        self
    }

    pub fn compare_admitted(
        self,
        admitted: &WorthUiAdmittedReplacementCandidate,
    ) -> Result<WorthUiRuntimeArtifactComparison, WorthUiRuntimeArtifactComparisonDenial> {
        reject_changed_admission_receipts(admitted)?;
        require_matching_runtime_equivalence_basis(self.runtime_basis, admitted)?;

        let mut counters = WorthUiRuntimeArtifactComparisonCounters::default();
        counters.record_artifact_comparison();
        let artifact_equivalence = self.compare_artifact_equivalence(admitted);

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
    ) -> WorthUiArtifactEquivalence {
        WorthUiArtifactEquivalenceComparator::compare(
            self.active_artifact.artifact(),
            admitted.artifact_bundle().artifact(),
            self.runtime_basis.artifact_equivalence_basis(),
        )
    }
}

fn reject_changed_admission_receipts(
    admitted: &WorthUiAdmittedReplacementCandidate,
) -> Result<(), WorthUiRuntimeArtifactComparisonDenial> {
    admitted.verify_receipts_unchanged().map_err(|denial| {
        WorthUiRuntimeArtifactComparisonDenial::AdmissionReceiptChanged {
            denial,
            counters: WorthUiRuntimeArtifactComparisonCounters::default(),
        }
    })
}

fn require_matching_runtime_equivalence_basis(
    runtime_basis: WorthUiRuntimeEquivalenceBasis,
    admitted: &WorthUiAdmittedReplacementCandidate,
) -> Result<(), WorthUiRuntimeArtifactComparisonDenial> {
    let candidate_basis = admitted.candidate().basis();
    let candidate_query_support_status = admitted.report().query_support_receipt().status();
    if candidate_artifact_equivalence_basis_matches_runtime(candidate_basis, runtime_basis)
        && candidate_query_support_status_matches_runtime(
            candidate_query_support_status,
            runtime_basis,
        )
    {
        Ok(())
    } else {
        Err(
            WorthUiRuntimeArtifactComparisonDenial::EquivalenceBasisMismatch {
                runtime_basis,
                candidate_basis,
                candidate_query_support_status,
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

fn candidate_query_support_status_matches_runtime(
    candidate_query_support_status: crate::runtime::WorthUiQuerySupportStatus,
    runtime_basis: WorthUiRuntimeEquivalenceBasis,
) -> bool {
    candidate_query_support_status == runtime_basis.required_query_support_status()
}
