use super::{row, Family, JourneyCutover, JourneyEntry, JourneyEvidence, JourneyMeaning, Row};

pub(super) fn rows() -> &'static [Row] {
    REFERENCE_CONSUMER_JOURNEYS
}

const REFERENCE_CONSUMER_JOURNEYS: &[Row] = &[
    row(JourneyEntry {
        id: "hadwiger-domain-entry",
        consumer: "Hadwiger Research",
        source: "crates/hadwiger-research/src/query_entry/admitted_handle.rs",
        probe: ".orchestrate_declaration_entry_outcome(input)",
        family: Family::DomainExtension,
        meaning: JourneyMeaning {
            intent: "typed research declaration plus contributions",
            context: "admitted domain handle and world basis",
            capability: "progressed declaration entry",
            phase_chain: "local handle -> Query declaration orchestration -> local outcome mapping",
        },
        evidence: JourneyEvidence {
            result: "WorthQueryDeclarationEntryOutcome",
            receipts: "declaration receipt/envelope",
            diagnostics: "inventory, readiness, checked, and proof projections",
            counters: "declaration-entry counters",
        },
        cutover: JourneyCutover {
            local_ceremony: "consumer wrapper exposes several Query phase/product variants",
            replacement: "one Query-owned domain declaration capability",
        },
    }),
    row(JourneyEntry {
        id: "worth-ui-query-authority",
        consumer: "Worth UI",
        source: "workspaces/worth-ui/crates/worth-ui-query-binding/src/prerequisites/query_prerequisite_boundary.rs",
        probe: "pub fn bind_query_authority(",
        family: Family::Read,
        meaning: JourneyMeaning {
            intent: "UI measurement fact consumption",
            context: "resolved basis plus Query-owned projection-consumption authority",
            capability: "WorthUiQueryMeasurementFactEligibility",
            phase_chain: "assemble prerequisites -> bind Query authority -> settle -> reconstruct UI receipt",
        },
        evidence: JourneyEvidence {
            result: "WorthUiQueryMeasurementFactSettlement",
            receipts: "UI-local measurement fact receipt",
            diagnostics: "UI-local prerequisite and warning evidence",
            counters: "settlement-local counters and digests",
        },
        cutover: JourneyCutover {
            local_ceremony: "consumer owns binding, validation, settlement, and receipt assembly",
            replacement: "Query-owned admitted read/inspection outcome",
        },
    }),
];
