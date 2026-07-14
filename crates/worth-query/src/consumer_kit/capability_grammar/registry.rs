use super::model::{
    WorthQueryCapabilityCeremonyChange as CeremonyChange,
    WorthQueryCapabilityGrammarBoundary as Boundary,
    WorthQueryCapabilityGrammarIdentity as Identity, WorthQueryCapabilityGrammarWords as Words,
};
use super::{WorthQueryCapabilityCeremony as Ceremony, WorthQueryCapabilityGrammarRow as Row};
use crate::consumer_kit::WorthQueryDeclarativeCapabilityFamily as Family;

pub fn worth_query_capability_grammar() -> &'static [Row] {
    GRAMMAR
}

macro_rules! grammar {
    ($family:expr, $journey:expr, $namespace:expr, $declare:expr, $refine:expr,
     $terminal:expr, $outcome:expr, $stop:expr, $next_action:expr, $context:expr,
     $cost:expr, $baseline:expr, $target:expr) => {
        Row::new(
            Identity {
                family: $family,
                reference_journey: $journey,
            },
            Words {
                namespace: $namespace,
                declare: $declare,
                refine: $refine,
                terminal: $terminal,
            },
            Boundary {
                outcome: $outcome,
                stop: $stop,
                next_action: $next_action,
                explicit_context: $context,
                cost_disclosure: $cost,
            },
            CeremonyChange {
                baseline: $baseline,
                target: $target,
            },
        )
    };
}

#[rustfmt::skip]
const GRAMMAR: &[Row] = &[
    grammar!(Family::Read, "query-detail-read", "facade::read", "declare", "using", "run", "WorthQueryReadOutcome", "WorthQueryReadStop", "WorthQueryReadNextAction", "explicit basis plus optional policy, tenant, and relationship authority", "bounded result shape and traversal breadth", Ceremony::new(2, 2, 2, 0), Ceremony::new(2, 2, 2, 0)),
    grammar!(Family::Aggregate, "query-count", "facade::aggregate", "declare", "using", "run", "WorthQueryCountOutcome", "WorthQueryCountDeclarationStop / WorthQueryReadStop", "WorthQueryReadNextAction", "explicit read authority context", "aggregate family, source breadth, and materialization work", Ceremony::new(3, 3, 3, 1), Ceremony::new(2, 2, 2, 0)),
    grammar!(Family::Live, "query-managed-live", "facade::live", "declare", "using", "open/close", "WorthQueryLiveOpenOutcome / WorthQueryManagedLiveCloseOutcome", "WorthQueryLiveDeclarationStop / WorthQueryLiveOpenStop", "WorthQueryReadNextAction", "explicit read and lifecycle authority", "activation, maintenance, delivery, and disposal work", Ceremony::new(4, 4, 4, 1), Ceremony::new(2, 3, 3, 0)),
    grammar!(Family::Historical, "query-historical", "facade::history", "declare", "using", "run", "WorthQueryHistoricalOutcome", "WorthQueryHistoricalStop", "WorthQueryHistoricalNextAction", "explicit historical basis and retention or replay authority", "retained, replay, and reconstruction budgets", Ceremony::new(7, 6, 3, 2), Ceremony::new(2, 2, 2, 0)),
    grammar!(Family::Comparison, "query-correspondence", "facade::comparison", "declare", "using", "run", "WorthQueryComparisonOutcome", "WorthQueryComparisonStop", "WorthQueryComparisonNextAction", "explicit left/right basis and correspondence authority", "candidate width, discovery plan, and ambiguity posture", Ceremony::new(6, 4, 2, 1), Ceremony::new(2, 2, 2, 0)),
    grammar!(Family::Preview, "query-preview", "facade::preview", "declare", "using", "open/close", "WorthQueryPreviewOutcome", "WorthQueryPreviewStop", "WorthQueryPreviewNextAction", "explicit preview session, scoped basis, and lifecycle authority", "preview isolation, live maintenance, promotion, and closeout work", Ceremony::new(10, 9, 5, 3), Ceremony::new(2, 3, 3, 0)),
    grammar!(Family::Mutation, "query-mutation", "facade::mutation", "declare", "using", "run", "WorthQueryMutationOutcome", "WorthQueryMutationStop", "WorthQueryMutationNextAction", "explicit mutation basis, freshness, and write authority", "touch width, conflict inspection, and effect work", Ceremony::new(12, 10, 6, 3), Ceremony::new(2, 3, 2, 0)),
    grammar!(Family::Workflow, "query-workflow", "facade::workflow", "declare", "using", "run", "WorthQueryWorkflowOutcome", "WorthQueryWorkflowStop", "WorthQueryWorkflowNextAction", "explicit workflow basis and capability-specific authority", "workflow width, merge/writeback family, and delivery work", Ceremony::new(12, 10, 6, 3), Ceremony::new(2, 3, 2, 0)),
    grammar!(Family::Inspection, "query-inspection", "facade::inspection", "declare", "using", "run", "WorthQueryInspectionOutcome", "WorthQueryInspectionStop", "WorthQueryInspectionNextAction", "explicit inspection target and scoped inspection basis", "inspection breadth and evidence materialization work", Ceremony::new(5, 4, 2, 1), Ceremony::new(2, 2, 2, 0)),
    grammar!(Family::DomainExtension, "hadwiger-domain-entry", "facade::domain", "declare", "using", "run", "WorthQueryDomainOutcome", "WorthQueryDomainStop", "WorthQueryDomainNextAction", "admitted domain handle plus declared extension contracts", "domain contribution width and declared invariant work", Ceremony::new(7, 6, 4, 2), Ceremony::new(2, 3, 2, 0)),
];
