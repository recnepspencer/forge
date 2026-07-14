use super::{
    worth_query_capability_grammar, WorthQueryCapabilityGrammarAudit,
    WorthQueryCapabilityGrammarFinding, WorthQueryCapabilityGrammarFindingKind as FindingKind,
};
use crate::consumer_kit::worth_query_consumer_journey_rows;

pub fn current_capability_grammar_audit() -> WorthQueryCapabilityGrammarAudit {
    let journeys = worth_query_consumer_journey_rows();
    let mut findings = Vec::new();

    for grammar in worth_query_capability_grammar() {
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
        if grammar.target().total() > grammar.baseline().total() {
            findings.push(WorthQueryCapabilityGrammarFinding::new(
                FindingKind::CeremonyRegression,
                grammar.reference_journey(),
            ));
        }
    }

    WorthQueryCapabilityGrammarAudit::new(findings)
}
