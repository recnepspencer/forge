use worth_query_installation::facade::WorthQueryStructuralCounterReplayPosture;

use super::{
    WorthQueryAdmittedDomainEvidence, WorthQueryAdmittedStructuralCounter,
    WorthQueryDomainEvidenceCore,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainEvidenceReplayMeaning {
    contract_identity: String,
    core: WorthQueryDomainEvidenceCore,
}

impl WorthQueryAdmittedDomainEvidence {
    pub fn replay_meaning(&self) -> WorthQueryDomainEvidenceReplayMeaning {
        WorthQueryDomainEvidenceReplayMeaning {
            contract_identity: self.contract_identity().to_owned(),
            core: self.core().clone(),
        }
    }
}

impl WorthQueryDomainEvidenceReplayMeaning {
    pub fn semantic_replay_eq(&self, candidate: &Self) -> bool {
        self.contract_identity == candidate.contract_identity
            && self.core.semantic_replay_eq(&candidate.core)
    }

    pub fn semantic_material(&self) -> String {
        worth_query_execution::facade::domain_computation::canonical_operation_material(vec![
            ("contract", self.contract_identity.clone()),
            (
                "core",
                super::identity::domain_evidence_core_material(&self.core),
            ),
        ])
    }
}

impl WorthQueryDomainEvidenceCore {
    fn semantic_replay_eq(&self, candidate: &Self) -> bool {
        self.decisions == candidate.decisions
            && self.candidate_search == candidate.candidate_search
            && self.transformation == candidate.transformation
            && self.counters.len() == candidate.counters.len()
            && self
                .counters
                .iter()
                .zip(&candidate.counters)
                .all(|(original, candidate)| counter_replay_eq(original, candidate))
    }
}

fn counter_replay_eq(
    original: &WorthQueryAdmittedStructuralCounter,
    candidate: &WorthQueryAdmittedStructuralCounter,
) -> bool {
    if original.schema() != candidate.schema() {
        return false;
    }
    match original.schema().replay() {
        WorthQueryStructuralCounterReplayPosture::Exact => {
            original.initial() == candidate.initial() && original.observed() == candidate.observed()
        }
        WorthQueryStructuralCounterReplayPosture::NonDecreasing => {
            candidate.initial() >= original.initial() && candidate.observed() >= original.observed()
        }
        WorthQueryStructuralCounterReplayPosture::ProviderCertified => {
            original.provider_certification() == candidate.provider_certification()
        }
        WorthQueryStructuralCounterReplayPosture::NotCompared => true,
    }
}
