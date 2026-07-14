use crate::consumer_kit::WorthQueryDeclarativeCapabilityFamily;

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
    namespace: &'static str,
    declare: &'static str,
    refine: &'static str,
    terminal: &'static str,
    outcome: &'static str,
    stop: &'static str,
    next_action: &'static str,
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
    pub(super) namespace: &'static str,
    pub(super) declare: &'static str,
    pub(super) refine: &'static str,
    pub(super) terminal: &'static str,
}

pub(super) struct WorthQueryCapabilityGrammarBoundary {
    pub(super) outcome: &'static str,
    pub(super) stop: &'static str,
    pub(super) next_action: &'static str,
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
            stop: boundary.stop,
            next_action: boundary.next_action,
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
        self.namespace
    }
    pub fn declare(&self) -> &'static str {
        self.declare
    }
    pub fn refine(&self) -> &'static str {
        self.refine
    }
    pub fn terminal(&self) -> &'static str {
        self.terminal
    }
    pub fn outcome(&self) -> &'static str {
        self.outcome
    }
    pub fn stop(&self) -> &'static str {
        self.stop
    }
    pub fn next_action(&self) -> &'static str {
        self.next_action
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
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryCapabilityGrammarFindingKind {
    MissingJourney,
    JourneyFamilyMismatch,
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
