use crate::consumer_kit::WorthQueryDeclarativeCapabilityFamily;

use super::{
    WorthQueryCapabilityFacadeNamespace, WorthQueryCapabilityOutcomeContract,
    WorthQueryCapabilityTerminalVocabulary, WorthQueryCapabilityTranscriptOwner,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryCapabilityCeremony {
    import_count: usize,
    intermediate_value_count: usize,
    explicit_transition_count: usize,
    local_adapter_count: usize,
}

impl WorthQueryCapabilityCeremony {
    pub const fn new(
        import_count: usize,
        intermediate_value_count: usize,
        explicit_transition_count: usize,
        local_adapter_count: usize,
    ) -> Self {
        Self {
            import_count,
            intermediate_value_count,
            explicit_transition_count,
            local_adapter_count,
        }
    }

    pub fn import_count(&self) -> usize {
        self.import_count
    }
    pub fn intermediate_value_count(&self) -> usize {
        self.intermediate_value_count
    }
    pub fn explicit_transition_count(&self) -> usize {
        self.explicit_transition_count
    }
    pub fn local_adapter_count(&self) -> usize {
        self.local_adapter_count
    }
    pub fn total(&self) -> usize {
        self.import_count
            + self.intermediate_value_count
            + self.explicit_transition_count
            + self.local_adapter_count
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryCapabilityGrammarRow {
    family: WorthQueryDeclarativeCapabilityFamily,
    reference_journey: &'static str,
    namespace: WorthQueryCapabilityFacadeNamespace,
    declare: &'static str,
    refine: &'static str,
    terminal: WorthQueryCapabilityTerminalVocabulary,
    outcome: WorthQueryCapabilityOutcomeContract,
    transcript_owner: WorthQueryCapabilityTranscriptOwner,
    explicit_context: &'static str,
    cost_disclosure: &'static str,
    baseline: WorthQueryCapabilityCeremony,
    target: WorthQueryCapabilityCeremony,
}

pub(super) struct WorthQueryCapabilityGrammarIdentity {
    pub(super) family: WorthQueryDeclarativeCapabilityFamily,
    pub(super) reference_journey: &'static str,
}

pub(super) struct WorthQueryCapabilityGrammarWords {
    pub(super) namespace: WorthQueryCapabilityFacadeNamespace,
    pub(super) declare: &'static str,
    pub(super) refine: &'static str,
    pub(super) terminal: WorthQueryCapabilityTerminalVocabulary,
}

pub(super) struct WorthQueryCapabilityGrammarBoundary {
    pub(super) outcome: WorthQueryCapabilityOutcomeContract,
    pub(super) transcript_owner: WorthQueryCapabilityTranscriptOwner,
    pub(super) explicit_context: &'static str,
    pub(super) cost_disclosure: &'static str,
}

pub(super) struct WorthQueryCapabilityCeremonyChange {
    pub(super) baseline: WorthQueryCapabilityCeremony,
    pub(super) target: WorthQueryCapabilityCeremony,
}

impl WorthQueryCapabilityGrammarRow {
    pub(super) const fn new(
        identity: WorthQueryCapabilityGrammarIdentity,
        words: WorthQueryCapabilityGrammarWords,
        boundary: WorthQueryCapabilityGrammarBoundary,
        ceremony: WorthQueryCapabilityCeremonyChange,
    ) -> Self {
        Self {
            family: identity.family,
            reference_journey: identity.reference_journey,
            namespace: words.namespace,
            declare: words.declare,
            refine: words.refine,
            terminal: words.terminal,
            outcome: boundary.outcome,
            transcript_owner: boundary.transcript_owner,
            explicit_context: boundary.explicit_context,
            cost_disclosure: boundary.cost_disclosure,
            baseline: ceremony.baseline,
            target: ceremony.target,
        }
    }

    pub fn family(&self) -> WorthQueryDeclarativeCapabilityFamily {
        self.family
    }
    pub fn reference_journey(&self) -> &'static str {
        self.reference_journey
    }
    pub fn namespace(&self) -> &'static str {
        self.namespace.as_str()
    }
    pub fn namespace_contract(&self) -> WorthQueryCapabilityFacadeNamespace {
        self.namespace
    }
    pub fn declare(&self) -> &'static str {
        self.declare
    }
    pub fn refine(&self) -> &'static str {
        self.refine
    }
    pub fn terminal(&self) -> &'static str {
        self.terminal.as_str()
    }
    pub fn terminal_vocabulary(&self) -> WorthQueryCapabilityTerminalVocabulary {
        self.terminal
    }
    pub fn outcome(&self) -> &'static str {
        self.outcome.outcome()
    }
    pub fn outcome_contract(&self) -> WorthQueryCapabilityOutcomeContract {
        self.outcome
    }
    pub fn stop(&self) -> &'static str {
        self.outcome.stop()
    }
    pub fn next_action(&self) -> &'static str {
        self.outcome.next_action()
    }
    pub fn transcript_owner(&self) -> WorthQueryCapabilityTranscriptOwner {
        self.transcript_owner
    }
    pub fn explicit_context(&self) -> &'static str {
        self.explicit_context
    }
    pub fn cost_disclosure(&self) -> &'static str {
        self.cost_disclosure
    }
    pub fn baseline(&self) -> WorthQueryCapabilityCeremony {
        self.baseline
    }
    pub fn target(&self) -> WorthQueryCapabilityCeremony {
        self.target
    }

    #[cfg(test)]
    pub(super) fn with_transcript_owner_for_test(
        mut self,
        transcript_owner: WorthQueryCapabilityTranscriptOwner,
    ) -> Self {
        self.transcript_owner = transcript_owner;
        self
    }

    #[cfg(test)]
    pub(super) fn with_namespace_for_test(
        mut self,
        namespace: WorthQueryCapabilityFacadeNamespace,
    ) -> Self {
        self.namespace = namespace;
        self
    }

    #[cfg(test)]
    pub(super) fn with_outcome_for_test(
        mut self,
        outcome: WorthQueryCapabilityOutcomeContract,
    ) -> Self {
        self.outcome = outcome;
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryCapabilityGrammarFindingKind {
    MissingJourney,
    JourneyFamilyMismatch,
    NamespaceFamilyMismatch,
    OutcomeFamilyMismatch,
    TranscriptOwnerMismatch,
    CeremonyRegression,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryCapabilityGrammarFinding {
    kind: WorthQueryCapabilityGrammarFindingKind,
    reference_journey: &'static str,
}

impl WorthQueryCapabilityGrammarFinding {
    pub(super) fn new(
        kind: WorthQueryCapabilityGrammarFindingKind,
        reference_journey: &'static str,
    ) -> Self {
        Self {
            kind,
            reference_journey,
        }
    }
    pub fn kind(&self) -> WorthQueryCapabilityGrammarFindingKind {
        self.kind
    }
    pub fn reference_journey(&self) -> &'static str {
        self.reference_journey
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryCapabilityGrammarAudit {
    findings: Vec<WorthQueryCapabilityGrammarFinding>,
}

impl WorthQueryCapabilityGrammarAudit {
    pub(super) fn new(findings: Vec<WorthQueryCapabilityGrammarFinding>) -> Self {
        Self { findings }
    }
    pub fn findings(&self) -> &[WorthQueryCapabilityGrammarFinding] {
        &self.findings
    }
    pub fn is_complete(&self) -> bool {
        self.findings.is_empty()
    }
}
