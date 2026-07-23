use super::super::super::WorthQueryBoundDomainOperation;
use super::super::conditional_comparison::WorthQueryConditionalAffinityEvidence;
use super::super::denial::{WorthQueryCompatibilityCounters, WorthQueryCompatibilityUseDenial};
use super::authority::{
    mint_pair_proof, WorthQueryCompatibilityProof, WorthQueryPortableAndBasisEvidence,
};
use crate::basis_lifecycle::BasisOperationLane;
use std::fmt;
use worth_proof::PhaseMarker;

#[derive(Debug)]
struct WorthQueryExecutionSharingPhase;
impl PhaseMarker for WorthQueryExecutionSharingPhase {}
pub(in crate::domain_installation::compatibility) struct WorthQueryExecutionSharingEvidence {
    common: WorthQueryPortableAndBasisEvidence,
    conditional: WorthQueryConditionalAffinityEvidence,
}
impl WorthQueryExecutionSharingEvidence {
    pub(in crate::domain_installation::compatibility) fn new(
        common: WorthQueryPortableAndBasisEvidence,
        conditional: WorthQueryConditionalAffinityEvidence,
    ) -> Self {
        Self {
            common,
            conditional,
        }
    }
}
pub struct WorthQueryExecutionSharingWitness {
    proof: WorthQueryCompatibilityProof<WorthQueryExecutionSharingPhase>,
    evidence: WorthQueryExecutionSharingEvidence,
    counters: WorthQueryCompatibilityCounters,
}
impl WorthQueryExecutionSharingWitness {
    pub(in crate::domain_installation::compatibility) fn mint<D, O, F, L: BasisOperationLane>(
        subject: &WorthQueryBoundDomainOperation<D, O, F, L>,
        candidate: &WorthQueryBoundDomainOperation<D, O, F, L>,
        evidence: WorthQueryExecutionSharingEvidence,
        counters: WorthQueryCompatibilityCounters,
    ) -> Self {
        Self {
            proof: mint_pair_proof("execution-sharing", subject, candidate),
            evidence,
            counters,
        }
    }

    pub(crate) fn readmit_for_pair<D, O, F, L: BasisOperationLane>(
        self,
        subject: &WorthQueryBoundDomainOperation<D, O, F, L>,
        candidate: &WorthQueryBoundDomainOperation<D, O, F, L>,
    ) -> Result<Self, WorthQueryCompatibilityUseDenial> {
        let pair = self.proof.basis().basis().value();
        require_execution_sharing_readmission(
            pair.matches(subject, candidate),
            || pair.matches_current_pair(subject, candidate),
            || self.evidence.conditional.both_are_live(),
        )?;
        Ok(self)
    }

    pub fn counters(&self) -> WorthQueryCompatibilityCounters {
        self.counters
    }
}

fn require_execution_sharing_readmission(
    exact_pair: bool,
    current_pair: impl FnOnce() -> bool,
    conditionals_live: impl FnOnce() -> bool,
) -> Result<(), WorthQueryCompatibilityUseDenial> {
    if !exact_pair {
        return Err(WorthQueryCompatibilityUseDenial::WrongCapabilityPair);
    }
    if !current_pair() {
        return Err(WorthQueryCompatibilityUseDenial::StaleAuthority);
    }
    if !conditionals_live() {
        return Err(WorthQueryCompatibilityUseDenial::StaleConditionalLowering);
    }
    Ok(())
}

impl fmt::Debug for WorthQueryExecutionSharingWitness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WorthQueryExecutionSharingWitness")
            .field("relationship", &self.proof.payload().relationship())
            .field(
                "retained_comparison_evidence",
                &(self.evidence.common.comparison_count()
                    + self.evidence.conditional.count() as u32),
            )
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::{require_execution_sharing_readmission, WorthQueryCompatibilityUseDenial};

    #[test]
    fn wrong_pair_denies_before_current_or_conditional_authority_is_read() {
        assert_eq!(
            require_execution_sharing_readmission(
                false,
                || panic!("wrong pairs must not inspect current authority"),
                || panic!("wrong pairs must not inspect conditional liveness"),
            ),
            Err(WorthQueryCompatibilityUseDenial::WrongCapabilityPair)
        );
    }

    #[test]
    fn stale_pair_denies_before_conditional_liveness_is_read() {
        assert_eq!(
            require_execution_sharing_readmission(
                true,
                || false,
                || panic!("stale pairs must not inspect conditional liveness"),
            ),
            Err(WorthQueryCompatibilityUseDenial::StaleAuthority)
        );
    }

    #[test]
    fn revoked_conditional_liveness_denies_a_current_exact_pair() {
        assert_eq!(
            require_execution_sharing_readmission(true, || true, || false),
            Err(WorthQueryCompatibilityUseDenial::StaleConditionalLowering)
        );
    }

    #[test]
    fn only_a_current_live_exact_pair_is_readmitted() {
        assert_eq!(
            require_execution_sharing_readmission(true, || true, || true),
            Ok(())
        );
    }
}
