use super::{row, Family, JourneyCutover, JourneyEntry, JourneyEvidence, JourneyMeaning, Row};

pub(super) fn rows() -> &'static [Row] {
    REFERENCE_CONSUMER_JOURNEYS
}

const REFERENCE_CONSUMER_JOURNEYS: &[Row] = &[
    row(JourneyEntry {
        id: "hadwiger-domain-entry",
        consumer: "Hadwiger Research",
        source: "crates/hadwiger-research/src/query_entry/ordinary_query.rs",
        probe: "pub fn declare_candidate_promotion(",
        family: Family::DomainExtension,
        meaning: JourneyMeaning {
            intent: "candidate search and candidate-promotion workflow",
            context: "ordinary read context or explicit domain workflow context",
            capability: "ordinary read declaration plus WorthQueryDomainWorkflowDeclaration",
            phase_chain: "domain vocabulary -> Query declaration -> explicit context -> Query-owned execution",
        },
        evidence: JourneyEvidence {
            result: "WorthQueryReadOutcome or WorthQueryDomainWorkflowOutcome",
            receipts: "Query-owned read or workflow receipts",
            diagnostics: "typed read/workflow stops and aftermath",
            counters: "read journey or workflow counters",
        },
        cutover: JourneyCutover {
            local_ceremony: "declare domain vocabulary and hand it to one ordinary Query capability",
            replacement: "facade::read and facade::domain",
        },
    }),
    row(JourneyEntry {
        id: "worth-ui-query-authority",
        consumer: "Worth UI",
        source: "workspaces/worth-ui/crates/worth-ui-query-binding/src/ordinary_query.rs",
        probe: "pub fn declare_measurement_read(",
        family: Family::Read,
        meaning: JourneyMeaning {
            intent: "UI measurement read, live observation, history/comparison, and inspection",
            context: "ordinary read/live/history/comparison or scoped inspection context",
            capability: "ordinary Query capability declarations",
            phase_chain: "UI domain vocabulary -> Query declaration -> explicit context -> Query-owned execution/lifecycle",
        },
        evidence: JourneyEvidence {
            result: "Query-owned read, live, historical, comparison, and inspection outcomes",
            receipts: "Query-owned operational receipts",
            diagnostics: "typed capability stops plus optional inspection materialization",
            counters: "Query-owned journey, lifecycle, and inspection counters",
        },
        cutover: JourneyCutover {
            local_ceremony: "declare UI vocabulary and hand it to one ordinary Query capability",
            replacement: "facade::read/live/history/comparison/inspection",
        },
    }),
];
