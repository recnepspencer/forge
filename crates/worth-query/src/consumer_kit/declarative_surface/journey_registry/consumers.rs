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
            phase_chain:
                "package -> runtime installation -> handle -> domain vocabulary -> Query execution",
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
        source: "workspaces/worth-ui/crates/worth-ui-query-binding/src/installed_measurements.rs",
        probe: "pub trait WorthUiQueryExt",
        family: Family::DomainExtension,
        meaning: JourneyMeaning {
            intent:
                "UI measurement read, live observation, record workflow, projection, and inspection",
            context:
                "installed UI domain plus ordinary read, live, workflow, or inspection context",
            capability: "downstream extension over WorthQueryInstalledDomainHandle",
            phase_chain:
                "package -> runtime installation -> handle -> UI vocabulary -> Query execution",
        },
        evidence: JourneyEvidence {
            result: "installed-domain read, live, workflow, projection, and inspection outcomes",
            receipts: "installed authority linked to Query-owned operational receipts",
            diagnostics: "typed installed-capability stops plus linked inspection",
            counters: "read, live lifecycle, workflow, and inspection counters",
        },
        cutover: JourneyCutover {
            local_ceremony: "extend one installed handle with UI-native measurement vocabulary",
            replacement: "facade::domain installed-handle grammar",
        },
    }),
];
