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
            local_ceremony: "declare_count -> using(context) -> run(workspace)",
            replacement: "facade::read::declare_count",
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
            local_ceremony: "declare_live -> using(context) -> open(workspace); handle.close(workspace)",
            replacement: "facade::read::declare_live",
        },
    }),
    row(JourneyEntry {
        id: "query-historical",
        consumer: "advanced historical consumer",
        source: "crates/worth-query/src/historical/request.rs",
        probe: "impl HistoricalEvaluationRequest {\n    pub fn retained_snapshot(",
        family: Family::Historical,
        meaning: JourneyMeaning {
            intent: "historical evaluation request and bounded path",
            context: "resolved historical basis and explicit replay/reconstruction budget",
            capability: "HistoricalPathAdmission",
            phase_chain: "request -> path admission -> resolution -> materialization",
        },
        evidence: JourneyEvidence {
            result: "historical evaluation outcome",
            receipts: "historical materialization metadata",
            diagnostics: "historical path report and typed denial",
            counters: "HistoricalCounterSnapshot",
        },
        cutover: JourneyCutover {
            local_ceremony: "construct request -> admit path -> resolve materialization",
            replacement: "ordinary historical declaration",
        },
    }),
    row(JourneyEntry {
        id: "query-correspondence",
        consumer: "advanced comparison consumer",
        source: "crates/worth-query/src/correspondence/request.rs",
        probe: "pub fn mixed(",
        family: Family::Comparison,
        meaning: JourneyMeaning {
            intent: "lineage, structural, or mixed correspondence request",
            context: "lineage authority, discovery plan, and candidate budget",
            capability: "CorrespondenceRequest",
            phase_chain: "request -> bounded discovery -> resolve evidence",
        },
        evidence: JourneyEvidence {
            result: "CorrespondenceResolution",
            receipts: "resolution evidence identity",
            diagnostics: "correspondence report and ambiguity/denial",
            counters: "CorrespondenceCounterSnapshot",
        },
        cutover: JourneyCutover {
            local_ceremony: "construct request -> resolve evidence",
            replacement: "ordinary correspondence declaration",
        },
    }),
    row(JourneyEntry {
        id: "query-preview",
        consumer: "advanced preview consumer",
        source: "crates/worth-query/src/preview/scoped.rs",
        probe: "pub fn admit_scoped_preview_session_plan_binding(",
        family: Family::Preview,
        meaning: JourneyMeaning {
            intent: "preview intent and lifecycle posture",
            context: "scoped observation basis and preview session binding",
            capability: "ScopedPreviewSessionPlanBinding",
            phase_chain: "bind -> admit scoped preview -> plan live session -> execute -> close",
        },
        evidence: JourneyEvidence {
            result: "preview execution outcome",
            receipts: "preview execution and closeout evidence",
            diagnostics: "preview admission report and lifecycle diagnostics",
            counters: "PreviewBindingCounters",
        },
        cutover: JourneyCutover {
            local_ceremony: "bind -> admit -> execute using public phase functions",
            replacement: "ordinary preview declaration",
        },
    }),
    row(JourneyEntry {
        id: "query-mutation",
        consumer: "advanced mutation consumer",
        source: "crates/worth-query/src/workflow/lowering/mutation.rs",
        probe: "pub fn lower_mutation_intent_declaration(",
        family: Family::Mutation,
        meaning: JourneyMeaning {
            intent: "typed mutation intent declaration",
            context: "admitted workflow context, mutation authority, and freshness binding",
            capability: "LoweredMutationIntentDeclaration",
            phase_chain: "bind -> admit -> lower mutation -> execute authoritative effect",
        },
        evidence: JourneyEvidence {
            result: "mutation workflow outcome",
            receipts: "authoritative mutation and workflow receipts",
            diagnostics: "typed authority, freshness, and execution denials",
            counters: "WorkflowLoweringCounters and effect execution counters",
        },
        cutover: JourneyCutover {
            local_ceremony: "bind workflow -> admit declaration -> choose mutation lowerer -> execute",
            replacement: "ordinary mutation declaration",
        },
    }),
    row(JourneyEntry {
        id: "query-workflow",
        consumer: "advanced workflow consumer",
        source: "crates/worth-query/src/workflow/foundation.rs",
        probe: "pub fn bind_workflow_context(",
        family: Family::Workflow,
        meaning: JourneyMeaning {
            intent: "mutation, merge, or writeback workflow declaration",
            context: "workflow context binding and admitted basis",
            capability: "AdmittedQueryWorkflowDeclaration",
            phase_chain: "bind -> admit -> capability-specific lower -> execute -> inspect",
        },
        evidence: JourneyEvidence {
            result: "workflow outcome",
            receipts: "workflow receipt and decision evidence",
            diagnostics: "workflow inspection result",
            counters: "workflow phase counters",
        },
        cutover: JourneyCutover {
            local_ceremony: "bind -> admit -> select lowerer -> execute",
            replacement: "ordinary capability-specific workflow declaration",
        },
    }),
    row(JourneyEntry {
        id: "query-inspection",
        consumer: "advanced inspection consumer",
        source: "crates/worth-query/src/runtime/workspace_queries.rs",
        probe: "pub fn inspect<'a, T>(",
        family: Family::Inspection,
        meaning: JourneyMeaning {
            intent: "typed inspection request",
            context: "admitted inspection basis and target",
            capability: "QueryInspectionResult",
            phase_chain: "request -> admit inspection -> derive evidence",
        },
        evidence: JourneyEvidence {
            result: "typed inspection result",
            receipts: "inspection evidence identity",
            diagnostics: "inspection report and causal explanation",
            counters: "inspection scale counters",
        },
        cutover: JourneyCutover {
            local_ceremony: "select workspace inspection method and assemble target",
            replacement: "ordinary inspection declaration",
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
