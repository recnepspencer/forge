use super::{row, Family, JourneyCutover, JourneyEntry, JourneyEvidence, JourneyMeaning, Row};

pub(super) fn rows() -> &'static [Row] {
    QUERY_CAPABILITY_JOURNEYS
}

const QUERY_CAPABILITY_JOURNEYS: &[Row] = &[
    standard_read("query-detail-read", "typed detail declaration"),
    standard_read(
        "query-collection-read",
        "typed collection declaration with bounded shape",
    ),
    graph_read(),
    row(JourneyEntry {
        id: "query-count",
        consumer: "Query reference consumer",
        source: "crates/worth-query/src/ordinary/count/declaration.rs",
        probe: "pub fn declare_count(",
        family: Family::Aggregate,
        meaning: JourneyMeaning {
            intent: "sealed count declaration derived from a supported read family",
            context: "explicit read context",
            capability: "WorthQueryCountRequest",
            phase_chain: "author -> canonicalize -> bind -> validate -> admit -> aggregate plan -> lower -> execute",
        },
        evidence: JourneyEvidence {
            result: "WorthQueryCountOutcome",
            receipts: "context and count receipts",
            diagnostics: "typed declaration/count stops",
            counters: "WorthQueryReadJourneyCounters",
        },
        cutover: JourneyCutover {
            local_ceremony: "aggregate::declare -> using(context) -> run(workspace)",
            replacement: "facade::aggregate::declare",
        },
    }),
    row(JourneyEntry {
        id: "query-managed-live",
        consumer: "Query reference consumer",
        source: "crates/worth-query/src/ordinary/live/declaration.rs",
        probe: "pub fn declare_live(",
        family: Family::Live,
        meaning: JourneyMeaning {
            intent: "typed live declaration",
            context: "explicit read context and lifecycle authority",
            capability: "WorthQueryLiveRequest",
            phase_chain: "author -> canonicalize -> admit -> plan -> activate -> maintain -> close",
        },
        evidence: JourneyEvidence {
            result: "WorthQueryLiveOpenOutcome",
            receipts: "open and close receipts",
            diagnostics: "managed lifecycle observation and typed stops",
            counters: "live admission, maintenance, and disposal counters",
        },
        cutover: JourneyCutover {
            local_ceremony: "declare -> using(context) -> open(workspace); handle.close(workspace)",
            replacement: "facade::live::declare",
        },
    }),
    row(JourneyEntry {
        id: "query-historical",
        consumer: "ordinary historical consumer",
        source: "crates/worth-query/src/ordinary/history/declaration.rs",
        probe: "pub fn declare(",
        family: Family::Historical,
        meaning: JourneyMeaning {
            intent: "canonical read declaration refined by an explicit historical path",
            context: "sealed retained basis or an explicit replay/reconstruction request",
            capability: "WorthQueryHistoricalRequest",
            phase_chain: "declare -> bind retained basis -> admit path -> plan -> materialize -> execute",
        },
        evidence: JourneyEvidence {
            result: "WorthQueryHistoricalOutcome",
            receipts: "read context receipt and historical materialization metadata",
            diagnostics: "typed history stop and next action",
            counters: "WorthQueryHistoricalJourneyCounters",
        },
        cutover: JourneyCutover {
            local_ceremony: "declare -> retained_snapshot/replay/reconstruction -> using(context) -> run",
            replacement: "facade::history::declare",
        },
    }),
    row(JourneyEntry {
        id: "query-correspondence",
        consumer: "ordinary comparison consumer",
        source: "crates/worth-query/src/ordinary/comparison/declaration.rs",
        probe: "pub fn declare(",
        family: Family::Comparison,
        meaning: JourneyMeaning {
            intent: "canonical query refined as diff, lineage, or bounded correspondence",
            context: "sealed structural current/retained basis pair",
            capability: "WorthQueryComparisonRequest",
            phase_chain: "declare -> bind basis pair -> execute both bases -> assemble diff or correspondence",
        },
        evidence: JourneyEvidence {
            result: "WorthQueryComparisonOutcome",
            receipts: "both read receipts plus retained materialization proof",
            diagnostics: "typed stop or correspondence posture",
            counters: "WorthQueryComparisonJourneyCounters",
        },
        cutover: JourneyCutover {
            local_ceremony: "declare -> diff/lineage/correspondence -> using(context) -> run",
            replacement: "facade::comparison::declare",
        },
    }),
    row(JourneyEntry {
        id: "query-preview",
        consumer: "ordinary preview consumer",
        source: "crates/worth-query/src/ordinary/preview/declaration.rs",
        probe: "pub fn declare(label: WorthQuerySessionLabel)",
        family: Family::Preview,
        meaning: JourneyMeaning {
            intent: "preview intent and lifecycle posture",
            context: "scoped observation basis and preview session binding",
            capability: "WorthQueryReadOnlyPreviewRequest or WorthQueryPromotionEligiblePreviewRequest",
            phase_chain: "declare -> bind explicit preview context -> admit -> execute -> inspect aftermath",
        },
        evidence: JourneyEvidence {
            result: "WorthQueryPreviewJourneyOutcome",
            receipts: "preview completion and execution evidence",
            diagnostics: "typed preview stop and next action",
            counters: "preview journey counters",
        },
        cutover: JourneyCutover {
            local_ceremony: "declare -> using(context) -> run(workspace)",
            replacement: "facade::preview::declare",
        },
    }),
    row(JourneyEntry {
        id: "query-mutation",
        consumer: "ordinary mutation consumer",
        source: "crates/worth-query/src/ordinary/mutation/declaration.rs",
        probe: "pub fn declare(",
        family: Family::Mutation,
        meaning: JourneyMeaning {
            intent: "typed mutation intent declaration",
            context: "admitted workflow context, mutation authority, and freshness binding",
            capability: "WorthQueryMutationRequest",
            phase_chain: "declare -> bind explicit mutation context -> admit -> lower -> execute -> inspect aftermath",
        },
        evidence: JourneyEvidence {
            result: "WorthQueryMutationOutcome",
            receipts: "authoritative mutation receipt and aftermath evidence",
            diagnostics: "typed authority, freshness, and execution denials",
            counters: "WorthQueryMutationCounters",
        },
        cutover: JourneyCutover {
            local_ceremony: "declare -> using(context) -> run(workspace)",
            replacement: "facade::mutation::declare",
        },
    }),
    row(JourneyEntry {
        id: "query-workflow",
        consumer: "ordinary workflow consumer",
        source: "crates/worth-query/src/ordinary/workflow/declaration.rs",
        probe: "pub fn declare(",
        family: Family::Workflow,
        meaning: JourneyMeaning {
            intent: "mutation, merge, or writeback workflow declaration",
            context: "workflow context binding and admitted basis",
            capability: "WorthQueryWorkflowRequest",
            phase_chain: "declare -> bind explicit workflow context -> admit -> capability-specific lower -> execute -> inspect aftermath",
        },
        evidence: JourneyEvidence {
            result: "WorthQueryWorkflowOutcome",
            receipts: "workflow execution receipt and aftermath evidence",
            diagnostics: "typed workflow stop, violation, advisory, and next action",
            counters: "WorthQueryWorkflowCounters",
        },
        cutover: JourneyCutover {
            local_ceremony: "declare -> using(context) -> run(workspace)",
            replacement: "facade::workflow::declare",
        },
    }),
    row(JourneyEntry {
        id: "query-inspection",
        consumer: "ordinary inspection consumer",
        source: "crates/worth-query/src/ordinary/inspection/declaration.rs",
        probe: "pub fn declare(completion: &WorthQueryReadCompletion)",
        family: Family::Inspection,
        meaning: JourneyMeaning {
            intent: "typed inspection request",
            context: "admitted inspection basis and target",
            capability: "WorthQueryInspectionRequest",
            phase_chain: "declare from authentic completion -> bind scoped inspection basis -> admit -> derive evidence",
        },
        evidence: JourneyEvidence {
            result: "WorthQueryInspectionOutcome",
            receipts: "WorthQueryInspectionReceipt",
            diagnostics: "typed inspection materialization, stop, unavailable, and next action",
            counters: "WorthQueryInspectionCounters",
        },
        cutover: JourneyCutover {
            local_ceremony: "declare(completion) -> using(inspection_basis) -> run(workspace)",
            replacement: "facade::inspection::declare",
        },
    }),
];

const fn standard_read(id: &'static str, intent: &'static str) -> Row {
    read(
        id,
        intent,
        "explicit current or policy-tenant read context",
        "author -> canonicalize -> bind -> validate -> admit -> plan -> lower -> execute",
    )
}

const fn graph_read() -> Row {
    read(
        "query-graph-read",
        "typed graph declaration with traversal bounds",
        "policy-tenant context plus relationship proofs when required",
        "author -> canonicalize -> bind -> graph authority admission -> plan -> lower -> execute",
    )
}

const fn read(
    id: &'static str,
    intent: &'static str,
    context: &'static str,
    phase_chain: &'static str,
) -> Row {
    row(JourneyEntry {
        id,
        consumer: "Query reference consumer",
        source: "crates/worth-query/src/ordinary/read/declaration.rs",
        probe: "pub fn declare(",
        family: Family::Read,
        meaning: JourneyMeaning {
            intent,
            context,
            capability: "WorthQueryReadRequest",
            phase_chain,
        },
        evidence: JourneyEvidence {
            result: "WorthQueryReadOutcome",
            receipts: "context and read receipts",
            diagnostics: "typed read stops and inspection evidence",
            counters: "WorthQueryReadJourneyCounters",
        },
        cutover: JourneyCutover {
            local_ceremony: "declare -> using(context) -> run(workspace)",
            replacement: "facade::read::declare",
        },
    })
}
