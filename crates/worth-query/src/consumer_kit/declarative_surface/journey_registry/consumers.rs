use super::{row, Family, JourneyCutover, JourneyEntry, JourneyEvidence, JourneyMeaning, Row};

pub(super) fn rows() -> &'static [Row] {
    REFERENCE_CONSUMER_JOURNEYS
}

const REFERENCE_CONSUMER_JOURNEYS: &[Row] = &[
    row(JourneyEntry {
        id: "hadwiger-domain-entry",
        consumer: "Hadwiger Research",
        source: "crates/hadwiger-research/src/query_entry/ordinary_query.rs",
        probe: "pub trait HadwigerResearchQueryExt",
        family: Family::DomainExtension,
        meaning: JourneyMeaning {
            intent: "candidate search and candidate-promotion workflow",
            context: "installed domain plus ordinary read or workflow context",
            capability: "downstream extension over WorthQueryInstalledDomainHandle",
            phase_chain: "package -> runtime installation -> handle -> domain vocabulary -> Query execution",
        },
        evidence: JourneyEvidence {
            result: "installed-domain read or workflow outcome",
            receipts: "installed authority linked to Query-owned operational receipts",
            diagnostics: "typed read/workflow stops and aftermath",
            counters: "read journey or workflow counters",
        },
        cutover: JourneyCutover {
            local_ceremony: "extend one installed handle with domain-native vocabulary",
            replacement: "facade::domain installed-handle grammar",
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
