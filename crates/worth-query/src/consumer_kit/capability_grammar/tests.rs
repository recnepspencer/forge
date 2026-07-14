use std::collections::BTreeSet;

use super::audit::audit_capability_grammar;
use super::{
    current_capability_grammar_audit, worth_query_capability_grammar,
    WorthQueryCapabilityFacadeNamespace as Namespace, WorthQueryCapabilityGrammarFindingKind,
    WorthQueryCapabilityOutcomeContract as Outcome, WorthQueryCapabilityTranscriptOwner as Owner,
};
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
            row.transcript_path(),
            row.transcript_probe(),
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

#[test]
fn every_frozen_transcript_has_its_implementation_phase_owner() {
    let owners = worth_query_capability_grammar()
        .iter()
        .map(|row| (row.family(), row.transcript_owner()))
        .collect::<BTreeSet<_>>();

    assert!(owners.contains(&(Family::Read, Owner::PhaseFiveReadExecution)));
    assert!(owners.contains(&(Family::Aggregate, Owner::PhaseFiveReadExecution)));
    assert!(owners.contains(&(Family::Live, Owner::PhaseSixManagedLive)));
    assert!(owners.contains(&(Family::Historical, Owner::PhaseSevenHistoricalComparison)));
    assert!(owners.contains(&(Family::Comparison, Owner::PhaseSevenHistoricalComparison)));
    for family in [
        Family::Preview,
        Family::Mutation,
        Family::Workflow,
        Family::DomainExtension,
    ] {
        assert!(owners.contains(&(family, Owner::PhaseEightWorkflowOrchestration)));
    }
    assert!(owners.contains(&(Family::Inspection, Owner::PhaseNineOutcomeInspection)));
}

#[test]
fn a_transcript_assigned_to_the_wrong_phase_fails_the_grammar_audit() {
    let mut rows = worth_query_capability_grammar().to_vec();
    rows[0] = rows[0].with_transcript_owner_for_test(Owner::PhaseNineOutcomeInspection);

    let audit = audit_capability_grammar(&rows);

    assert!(audit.findings().iter().any(|finding| {
        finding.kind() == WorthQueryCapabilityGrammarFindingKind::TranscriptOwnerMismatch
            && finding.reference_journey() == "query-detail-read"
    }));
}

#[test]
fn namespace_and_outcome_drift_fail_the_grammar_audit() {
    let mut rows = worth_query_capability_grammar().to_vec();
    rows[0] = rows[0]
        .with_namespace_for_test(Namespace::Workflow)
        .with_outcome_for_test(Outcome::Comparison);

    let audit = audit_capability_grammar(&rows);
    assert!(audit.findings().iter().any(|finding| {
        finding.kind() == WorthQueryCapabilityGrammarFindingKind::NamespaceFamilyMismatch
    }));
    assert!(audit.findings().iter().any(|finding| {
        finding.kind() == WorthQueryCapabilityGrammarFindingKind::OutcomeFamilyMismatch
    }));
}
