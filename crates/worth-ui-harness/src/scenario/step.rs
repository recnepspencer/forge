use crate::evidence::HarnessEvidenceRequirement;

use super::{HarnessExpectedObservation, HarnessScenarioOperation};

#[derive(Debug)]
pub struct HarnessScenarioStep {
    label: String,
    operation: HarnessScenarioOperation,
    requirements: Vec<HarnessEvidenceRequirement>,
    expectations: Vec<HarnessExpectedObservation>,
}

impl HarnessScenarioStep {
    pub fn new(label: impl Into<String>, operation: HarnessScenarioOperation) -> Self {
        Self {
            label: label.into(),
            operation,
            requirements: Vec::new(),
            expectations: Vec::new(),
        }
    }

    pub fn requires(mut self, requirement: HarnessEvidenceRequirement) -> Self {
        self.requirements.push(requirement);
        self
    }

    pub fn expects(mut self, expectation: HarnessExpectedObservation) -> Self {
        self.expectations.push(expectation);
        self
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn operation(&self) -> &HarnessScenarioOperation {
        &self.operation
    }

    pub fn into_operation(self) -> HarnessScenarioOperation {
        self.operation
    }

    pub fn requirements(&self) -> &[HarnessEvidenceRequirement] {
        &self.requirements
    }

    pub fn expectations(&self) -> &[HarnessExpectedObservation] {
        &self.expectations
    }
}
