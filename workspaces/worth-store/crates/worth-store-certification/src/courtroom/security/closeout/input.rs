use worth_store_physical_certification::{
    SecurityScopeHarnessEvidence, SecurityScopeHarnessReplayTranscript,
};
use worth_store_security::StoreAdmittedSecurityScope;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S51CertificationEvidencePolicy {
    counter_backed_foundational: bool,
}

#[derive(Debug)]
pub struct S51CertificationCloseoutInput {
    scenario_evidence: Vec<SecurityScopeHarnessEvidence>,
    replay_transcripts: Vec<SecurityScopeHarnessReplayTranscript>,
    security_scope: StoreAdmittedSecurityScope,
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
    pub fn from_replay_and_security_scope(
        scenario_evidence: impl IntoIterator<Item = SecurityScopeHarnessEvidence>,
        replay_transcripts: impl IntoIterator<Item = SecurityScopeHarnessReplayTranscript>,
        security_scope: StoreAdmittedSecurityScope,
        policy: S51CertificationEvidencePolicy,
    ) -> Self {
        Self {
            scenario_evidence: scenario_evidence.into_iter().collect(),
            replay_transcripts: replay_transcripts.into_iter().collect(),
            security_scope,
            policy,
        }
    }

    pub fn scenario_evidence(&self) -> &[SecurityScopeHarnessEvidence] {
        &self.scenario_evidence
    }

    pub fn replay_transcripts(&self) -> &[SecurityScopeHarnessReplayTranscript] {
        &self.replay_transcripts
    }

    pub const fn security_scope(&self) -> &StoreAdmittedSecurityScope {
        &self.security_scope
    }

    pub const fn policy(&self) -> S51CertificationEvidencePolicy {
        self.policy
    }
}
