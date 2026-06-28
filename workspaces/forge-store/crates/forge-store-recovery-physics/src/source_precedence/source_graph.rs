use super::{
    source_admission_accumulator::RecoverySourceAdmissionAccumulator,
    source_selection::select_admitted_recovery_source, AdmittedRecoverySource,
    RecoverySourceCandidate,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoverySourcePrecedenceGraph {
    profile: String,
    candidates: Vec<RecoverySourceCandidate>,
}

impl RecoverySourcePrecedenceGraph {
    pub fn new(profile: impl Into<String>) -> Self {
        Self {
            profile: profile.into(),
            candidates: Vec::new(),
        }
    }

    pub fn discover(mut self, candidate: RecoverySourceCandidate) -> Self {
        self.candidates.push(candidate);
        self
    }

    pub fn candidate_count(&self) -> usize {
        self.candidates.len()
    }

    pub fn admit_sources(self) -> AdmittedRecoverySource {
        let admission = RecoverySourceAdmissionAccumulator::from_candidates(self.candidates);
        select_admitted_recovery_source(self.profile, admission.into_selection_input())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source_precedence::RecoverySourceDecisionKind;

    #[test]
    fn source_precedence_empty_graph_is_no_valid_checkpoint() {
        let admitted = RecoverySourcePrecedenceGraph::new("unit-profile").admit_sources();

        assert_eq!(
            admitted.trace().kind(),
            RecoverySourceDecisionKind::NoValidCheckpoint
        );
    }
}
