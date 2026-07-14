use super::{
    worth_query_capability_grammar, WorthQueryCapabilityGrammarAudit,
    WorthQueryCapabilityGrammarFinding, WorthQueryCapabilityGrammarFindingKind as FindingKind,
};
use crate::consumer_kit::worth_query_consumer_journey_rows;

pub fn current_capability_grammar_audit() -> WorthQueryCapabilityGrammarAudit {
    audit_capability_grammar(worth_query_capability_grammar())
}

pub(super) fn audit_capability_grammar(
    grammar_rows: &[super::WorthQueryCapabilityGrammarRow],
) -> WorthQueryCapabilityGrammarAudit {
    let journeys = worth_query_consumer_journey_rows();
    let mut findings = Vec::new();

    for grammar in grammar_rows {
        match journeys
            .iter()
            .find(|journey| journey.journey_id() == grammar.reference_journey())
        {
            None => findings.push(WorthQueryCapabilityGrammarFinding::new(
                FindingKind::MissingJourney,
                grammar.reference_journey(),
            )),
            Some(journey) if journey.capability_family() != grammar.family() => {
                findings.push(WorthQueryCapabilityGrammarFinding::new(
                    FindingKind::JourneyFamilyMismatch,
                    grammar.reference_journey(),
                ))
            }
            Some(_) => {}
        }
        if grammar.namespace_contract().family() != grammar.family() {
            findings.push(WorthQueryCapabilityGrammarFinding::new(
                FindingKind::NamespaceFamilyMismatch,
                grammar.reference_journey(),
            ));
        }
        if grammar.outcome_contract().family() != grammar.family() {
            findings.push(WorthQueryCapabilityGrammarFinding::new(
                FindingKind::OutcomeFamilyMismatch,
                grammar.reference_journey(),
            ));
        }
        if !grammar.transcript_owner().owns(grammar.family()) {
            findings.push(WorthQueryCapabilityGrammarFinding::new(
                FindingKind::TranscriptOwnerMismatch,
                grammar.reference_journey(),
            ));
        }
        if grammar.target().total() > grammar.baseline().total() {
            findings.push(WorthQueryCapabilityGrammarFinding::new(
                FindingKind::CeremonyRegression,
                grammar.reference_journey(),
            ));
        }
    }

    WorthQueryCapabilityGrammarAudit::new(findings)
}
