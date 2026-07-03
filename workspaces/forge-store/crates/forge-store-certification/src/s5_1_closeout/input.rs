use forge_store_physical_certification::{
    S51SecurityScopeHarnessEvidence, S51SecurityScopeHarnessReplayTranscript,
};
use forge_store_readiness::S51SecurityFoundationHandoff;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S51CertificationEvidencePolicy {
    counter_backed_foundational: bool,
}

#[derive(Debug)]
pub struct S51CertificationCloseoutInput {
    scenario_evidence: Vec<S51SecurityScopeHarnessEvidence>,
    replay_transcripts: Vec<S51SecurityScopeHarnessReplayTranscript>,
    security_foundation_handoff: S51SecurityFoundationHandoff,
    policy: S51CertificationEvidencePolicy,
}

impl S51CertificationEvidencePolicy {
    pub const fn counter_backed_foundational() -> Self {
        Self {
            counter_backed_foundational: true,
        }
    }

    pub const fn is_counter_backed_foundational(self) -> bool {
        self.counter_backed_foundational
    }
}

impl S51CertificationCloseoutInput {
    pub fn from_phase10_replay_and_handoffs(
        scenario_evidence: impl IntoIterator<Item = S51SecurityScopeHarnessEvidence>,
        replay_transcripts: impl IntoIterator<Item = S51SecurityScopeHarnessReplayTranscript>,
        security_foundation_handoff: S51SecurityFoundationHandoff,
        policy: S51CertificationEvidencePolicy,
    ) -> Self {
        Self {
            scenario_evidence: scenario_evidence.into_iter().collect(),
            replay_transcripts: replay_transcripts.into_iter().collect(),
            security_foundation_handoff,
            policy,
        }
    }

    pub fn scenario_evidence(&self) -> &[S51SecurityScopeHarnessEvidence] {
        &self.scenario_evidence
    }

    pub fn replay_transcripts(&self) -> &[S51SecurityScopeHarnessReplayTranscript] {
        &self.replay_transcripts
    }

    pub const fn security_foundation_handoff(&self) -> &S51SecurityFoundationHandoff {
        &self.security_foundation_handoff
    }

    pub const fn policy(&self) -> S51CertificationEvidencePolicy {
        self.policy
    }
}
