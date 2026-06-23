mod evidence;
mod expectation_match;
mod observed;
#[cfg(test)]
mod tests;

use crate::manual_flow::{
    validation_manual_flow_catalog, ValidationManualFlowDefinition,
    ValidationManualFlowExpectation, ValidationManualFlowId, ValidationManualFlowProof,
};
use crate::ValidationAppProofSnapshot;

use expectation_match::flow_expectation_matches;
use observed::observed_for_flow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationManualFlowMatrixSnapshot {
    rows: Vec<ValidationManualFlowVisibleRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationManualFlowVisibleRow {
    flow_id: ValidationManualFlowId,
    title: String,
    authored_input: String,
    expected_status: String,
    expected_visible_result: String,
    expected_counter_posture: String,
    expected_replay_posture: String,
    expected_changed_facts: Vec<String>,
    expected_rebuilt_projections: Vec<String>,
    expected_preserved_projections: Vec<String>,
    observed_status: String,
    observed_visible_result: String,
    observed_counter_posture: String,
    observed_counter_details: String,
    observed_replay_posture: String,
    observed_projection_digest: String,
    observed_changed_facts: Vec<String>,
    observed_rebuilt_projections: Vec<String>,
    observed_preserved_projections: Vec<String>,
    matches_expectation: bool,
}

impl ValidationManualFlowMatrixSnapshot {
    pub fn from_proof(
        proof: &ValidationAppProofSnapshot,
        last_executed_flow: Option<ValidationManualFlowId>,
    ) -> Self {
        let catalog = validation_manual_flow_catalog();
        Self {
            rows: catalog
                .definitions()
                .iter()
                .map(|definition| {
                    let expectation = catalog.expectation_for(definition.id());
                    ValidationManualFlowVisibleRow::from_definition(
                        *definition,
                        expectation,
                        proof,
                        last_executed_flow,
                    )
                })
                .collect(),
        }
    }

    pub fn rows(&self) -> &[ValidationManualFlowVisibleRow] {
        &self.rows
    }
}

impl ValidationManualFlowVisibleRow {
    fn from_definition(
        definition: ValidationManualFlowDefinition,
        expectation: ValidationManualFlowExpectation,
        proof: &ValidationAppProofSnapshot,
        last_executed_flow: Option<ValidationManualFlowId>,
    ) -> Self {
        let was_last_run = last_executed_flow == Some(definition.id());
        let observed = if was_last_run {
            observed_for_flow(definition.id(), proof)
        } else {
            ValidationManualFlowProof::not_run_yet()
        };
        let expected_changed_facts = expectation
            .changed_facts()
            .iter()
            .map(|fact| (*fact).to_owned())
            .collect::<Vec<_>>();
        let expected_rebuilt_projections = expectation
            .rebuilt_projections()
            .iter()
            .map(|projection| (*projection).to_owned())
            .collect::<Vec<_>>();
        let expected_preserved_projections = expectation
            .preserved_projections()
            .iter()
            .map(|projection| (*projection).to_owned())
            .collect::<Vec<_>>();
        let matches_expectation =
            flow_expectation_matches(definition.id(), expectation, &observed, proof, was_last_run);

        Self {
            flow_id: definition.id(),
            title: definition.title().to_owned(),
            authored_input: definition.authored_input().to_owned(),
            expected_status: expectation.status().to_owned(),
            expected_visible_result: expectation.visible_result().to_owned(),
            expected_counter_posture: expectation.counter_posture().to_owned(),
            expected_replay_posture: expectation.replay_posture().to_owned(),
            expected_changed_facts,
            expected_rebuilt_projections,
            expected_preserved_projections,
            observed_status: observed.status().to_owned(),
            observed_visible_result: observed.visible_result_label(),
            observed_counter_posture: observed.counter_posture_label().to_owned(),
            observed_counter_details: observed.counter_details().to_owned(),
            observed_replay_posture: observed.replay_posture_label().to_owned(),
            observed_projection_digest: observed.projection_digest().to_owned(),
            observed_changed_facts: observed.changed_facts().to_vec(),
            observed_rebuilt_projections: observed.rebuilt_projections().to_vec(),
            observed_preserved_projections: observed.preserved_projections().to_vec(),
            matches_expectation,
        }
    }

    pub fn flow_id(&self) -> ValidationManualFlowId {
        self.flow_id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn authored_input(&self) -> &str {
        &self.authored_input
    }

    pub fn expected_status(&self) -> &str {
        &self.expected_status
    }

    pub fn expected_visible_result(&self) -> &str {
        &self.expected_visible_result
    }

    pub fn expected_counter_posture(&self) -> &str {
        &self.expected_counter_posture
    }

    pub fn expected_replay_posture(&self) -> &str {
        &self.expected_replay_posture
    }

    pub fn expected_changed_facts(&self) -> &[String] {
        &self.expected_changed_facts
    }

    pub fn expected_rebuilt_projections(&self) -> &[String] {
        &self.expected_rebuilt_projections
    }

    pub fn expected_preserved_projections(&self) -> &[String] {
        &self.expected_preserved_projections
    }

    pub fn observed_status(&self) -> &str {
        &self.observed_status
    }

    pub fn observed_visible_result(&self) -> &str {
        &self.observed_visible_result
    }

    pub fn observed_counter_posture(&self) -> &str {
        &self.observed_counter_posture
    }

    pub fn observed_counter_details(&self) -> &str {
        &self.observed_counter_details
    }

    pub fn observed_replay_posture(&self) -> &str {
        &self.observed_replay_posture
    }

    pub fn observed_projection_digest(&self) -> &str {
        &self.observed_projection_digest
    }

    pub fn observed_changed_facts(&self) -> &[String] {
        &self.observed_changed_facts
    }

    pub fn observed_rebuilt_projections(&self) -> &[String] {
        &self.observed_rebuilt_projections
    }

    pub fn observed_preserved_projections(&self) -> &[String] {
        &self.observed_preserved_projections
    }

    pub fn matches_expectation(&self) -> bool {
        self.matches_expectation
    }
}
