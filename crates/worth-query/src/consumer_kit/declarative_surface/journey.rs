use super::WorthQueryDeclarativeCapabilityFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryConsumerJourneyRow {
    journey_id: &'static str,
    consumer: &'static str,
    source_path: &'static str,
    source_probe: &'static str,
    capability_family: WorthQueryDeclarativeCapabilityFamily,
    declared_intent: &'static str,
    required_context: &'static str,
    admitted_capability: &'static str,
    query_owned_phase_chain: &'static str,
    result: &'static str,
    receipts: &'static str,
    diagnostics: &'static str,
    cost_counters: &'static str,
    local_ceremony: &'static str,
    replacement: &'static str,
}

impl WorthQueryConsumerJourneyRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn new(
        journey_id: &'static str,
        consumer: &'static str,
        source_path: &'static str,
        source_probe: &'static str,
        capability_family: WorthQueryDeclarativeCapabilityFamily,
        declared_intent: &'static str,
        required_context: &'static str,
        admitted_capability: &'static str,
        query_owned_phase_chain: &'static str,
        result: &'static str,
        receipts: &'static str,
        diagnostics: &'static str,
        cost_counters: &'static str,
        local_ceremony: &'static str,
        replacement: &'static str,
    ) -> Self {
        Self {
            journey_id,
            consumer,
            source_path,
            source_probe,
            capability_family,
            declared_intent,
            required_context,
            admitted_capability,
            query_owned_phase_chain,
            result,
            receipts,
            diagnostics,
            cost_counters,
            local_ceremony,
            replacement,
        }
    }

    pub fn journey_id(&self) -> &'static str {
        self.journey_id
    }
    pub fn consumer(&self) -> &'static str {
        self.consumer
    }
    pub fn source_path(&self) -> &'static str {
        self.source_path
    }
    pub fn source_probe(&self) -> &'static str {
        self.source_probe
    }
    pub fn capability_family(&self) -> WorthQueryDeclarativeCapabilityFamily {
        self.capability_family
    }
    pub fn declared_intent(&self) -> &'static str {
        self.declared_intent
    }
    pub fn required_context(&self) -> &'static str {
        self.required_context
    }
    pub fn admitted_capability(&self) -> &'static str {
        self.admitted_capability
    }
    pub fn query_owned_phase_chain(&self) -> &'static str {
        self.query_owned_phase_chain
    }
    pub fn result(&self) -> &'static str {
        self.result
    }
    pub fn receipts(&self) -> &'static str {
        self.receipts
    }
    pub fn diagnostics(&self) -> &'static str {
        self.diagnostics
    }
    pub fn cost_counters(&self) -> &'static str {
        self.cost_counters
    }
    pub fn local_ceremony(&self) -> &'static str {
        self.local_ceremony
    }
    pub fn replacement(&self) -> &'static str {
        self.replacement
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryConsumerJourneySource<'a> {
    path: &'a str,
    text: &'a str,
}

impl<'a> WorthQueryConsumerJourneySource<'a> {
    pub const fn new(path: &'a str, text: &'a str) -> Self {
        Self { path, text }
    }
    pub fn path(&self) -> &str {
        self.path
    }
    pub fn text(&self) -> &str {
        self.text
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryConsumerJourneyFindingKind {
    MissingSource,
    MissingSourceProbe,
    AmbiguousSourceProbe,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryConsumerJourneyFinding {
    kind: WorthQueryConsumerJourneyFindingKind,
    journey_id: &'static str,
    source_path: &'static str,
}

impl WorthQueryConsumerJourneyFinding {
    pub(crate) fn new(
        kind: WorthQueryConsumerJourneyFindingKind,
        row: &WorthQueryConsumerJourneyRow,
    ) -> Self {
        Self {
            kind,
            journey_id: row.journey_id(),
            source_path: row.source_path(),
        }
    }
    pub fn kind(&self) -> WorthQueryConsumerJourneyFindingKind {
        self.kind
    }
    pub fn journey_id(&self) -> &'static str {
        self.journey_id
    }
    pub fn source_path(&self) -> &'static str {
        self.source_path
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryConsumerJourneyAudit {
    classified_journey_count: usize,
    findings: Vec<WorthQueryConsumerJourneyFinding>,
}

impl WorthQueryConsumerJourneyAudit {
    pub(crate) fn new(
        classified_journey_count: usize,
        findings: Vec<WorthQueryConsumerJourneyFinding>,
    ) -> Self {
        Self {
            classified_journey_count,
            findings,
        }
    }
    pub fn classified_journey_count(&self) -> usize {
        self.classified_journey_count
    }
    pub fn findings(&self) -> &[WorthQueryConsumerJourneyFinding] {
        &self.findings
    }
    pub fn is_complete(&self) -> bool {
        self.findings.is_empty()
    }
}
