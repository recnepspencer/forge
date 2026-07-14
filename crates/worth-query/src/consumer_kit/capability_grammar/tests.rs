use std::collections::BTreeSet;

use super::{current_capability_grammar_audit, worth_query_capability_grammar};
use crate::consumer_kit::WorthQueryDeclarativeCapabilityFamily as Family;

#[test]
fn every_declared_capability_family_has_one_complete_grammar() {
    let rows = worth_query_capability_grammar();
    let families = rows.iter().map(|row| row.family()).collect::<BTreeSet<_>>();

    assert_eq!(rows.len(), 10);
    for family in [
        Family::Read,
        Family::Aggregate,
        Family::Live,
        Family::Historical,
        Family::Comparison,
        Family::Preview,
        Family::Mutation,
        Family::Workflow,
        Family::Inspection,
        Family::DomainExtension,
    ] {
        assert!(families.contains(&family), "missing grammar for {family:?}");
    }
    for row in rows {
        for field in [
            row.reference_journey(),
            row.namespace(),
            row.declare(),
            row.refine(),
            row.terminal(),
            row.outcome(),
            row.stop(),
            row.next_action(),
            row.explicit_context(),
            row.cost_disclosure(),
        ] {
            assert!(
                !field.trim().is_empty(),
                "incomplete grammar for {:?}",
                row.family()
            );
        }
    }
}

#[test]
fn grammar_is_journey_backed_and_never_regresses_ceremony() {
    let audit = current_capability_grammar_audit();
    assert!(
        audit.is_complete(),
        "grammar findings: {:?}",
        audit.findings()
    );

    for row in worth_query_capability_grammar() {
        assert!(row.target().total() <= row.baseline().total());
        assert_eq!(row.target().local_adapter_count(), 0);
    }
}
