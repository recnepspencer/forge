use super::model::{
    WorthQueryCapabilityCeremonyChange as CeremonyChange,
    WorthQueryCapabilityGrammarBoundary as Boundary,
    WorthQueryCapabilityGrammarIdentity as Identity, WorthQueryCapabilityGrammarWords as Words,
};
use super::{
    WorthQueryCapabilityCeremony as Ceremony, WorthQueryCapabilityFacadeNamespace as Namespace,
    WorthQueryCapabilityGrammarRow as Row, WorthQueryCapabilityOutcomeContract as Outcome,
    WorthQueryCapabilityTerminalVocabulary as Terminal,
    WorthQueryCapabilityTranscriptOwner as Owner,
};
use crate::consumer_kit::WorthQueryDeclarativeCapabilityFamily as Family;

pub fn worth_query_capability_grammar() -> &'static [Row] {
    GRAMMAR
}

macro_rules! grammar {
    ($family:expr, $journey:expr, $namespace:expr, $declare:expr, $refine:expr,
     $terminal:expr, $outcome:expr, $owner:expr, $probe:expr, $context:expr,
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
                transcript_owner: $owner,
                transcript_path:
                    "tests/declarative_product_boundary_certification/grammar_matrix.rs",
                transcript_probe: $probe,
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
    grammar!(Family::Read, "query-detail-read", Namespace::Read, "declare", "using", Terminal::Run, Outcome::Read, Owner::PhaseFiveReadExecution, "fn grammar_read_journey_executes()", "explicit basis plus optional policy, tenant, and relationship authority", "bounded result shape and traversal breadth", Ceremony::new(2, 2, 2, 0), Ceremony::new(2, 2, 2, 0)),
    grammar!(Family::Aggregate, "query-count", Namespace::Aggregate, "declare", "using", Terminal::Run, Outcome::Aggregate, Owner::PhaseFiveReadExecution, "fn grammar_aggregate_journey_executes()", "explicit read authority context", "aggregate family, source breadth, and materialization work", Ceremony::new(3, 3, 3, 1), Ceremony::new(2, 2, 2, 0)),
    grammar!(Family::Live, "query-managed-live", Namespace::Live, "declare", "using", Terminal::OpenAndClose, Outcome::Live, Owner::PhaseSixManagedLive, "fn grammar_live_journey_executes()", "explicit read and lifecycle authority", "activation, maintenance, delivery, and disposal work", Ceremony::new(4, 4, 4, 1), Ceremony::new(2, 3, 3, 0)),
    grammar!(Family::Historical, "query-historical", Namespace::History, "declare", "using", Terminal::Run, Outcome::Historical, Owner::PhaseSevenHistoricalComparison, "fn grammar_history_journey_executes()", "explicit historical basis and retention or replay authority", "retained, replay, and reconstruction budgets", Ceremony::new(7, 6, 3, 2), Ceremony::new(2, 2, 2, 0)),
    grammar!(Family::Comparison, "query-correspondence", Namespace::Comparison, "declare", "using", Terminal::Run, Outcome::Comparison, Owner::PhaseSevenHistoricalComparison, "fn grammar_comparison_journey_executes()", "explicit left/right basis and correspondence authority", "candidate width, discovery plan, and ambiguity posture", Ceremony::new(6, 4, 2, 1), Ceremony::new(2, 2, 2, 0)),
    grammar!(Family::Preview, "query-preview", Namespace::Preview, "declare", "using", Terminal::OpenAndClose, Outcome::Preview, Owner::PhaseEightWorkflowOrchestration, "fn grammar_preview_journey_executes()", "explicit preview session, scoped basis, and lifecycle authority", "preview isolation, live maintenance, promotion, and closeout work", Ceremony::new(10, 9, 5, 3), Ceremony::new(2, 3, 3, 0)),
    grammar!(Family::Mutation, "query-mutation", Namespace::Mutation, "declare", "using", Terminal::Run, Outcome::Mutation, Owner::PhaseEightWorkflowOrchestration, "fn grammar_mutation_journey_executes()", "explicit mutation basis, freshness, and write authority", "touch width, conflict inspection, and effect work", Ceremony::new(12, 10, 6, 3), Ceremony::new(2, 3, 2, 0)),
    grammar!(Family::Workflow, "query-workflow", Namespace::Workflow, "declare", "using", Terminal::Run, Outcome::Workflow, Owner::PhaseEightWorkflowOrchestration, "fn grammar_workflow_journey_executes()", "explicit workflow basis and capability-specific authority", "workflow width, merge/writeback family, and delivery work", Ceremony::new(12, 10, 6, 3), Ceremony::new(2, 3, 2, 0)),
    grammar!(Family::Inspection, "query-inspection", Namespace::Inspection, "declare", "using", Terminal::Run, Outcome::Inspection, Owner::PhaseNineOutcomeInspection, "fn grammar_inspection_journey_executes()", "explicit inspection target and scoped inspection basis", "inspection breadth and evidence materialization work", Ceremony::new(5, 4, 2, 1), Ceremony::new(2, 2, 2, 0)),
    grammar!(Family::DomainExtension, "hadwiger-domain-entry", Namespace::Domain, "declare", "using", Terminal::Run, Outcome::Domain, Owner::PhaseEightWorkflowOrchestration, "fn grammar_domain_journey_executes()", "admitted domain handle plus declared extension contracts", "domain contribution width and declared invariant work", Ceremony::new(7, 6, 4, 2), Ceremony::new(2, 3, 2, 0)),
];
