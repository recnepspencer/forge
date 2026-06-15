use super::{HarnessScenarioId, HarnessScenarioIdError, HarnessScenarioStep};

#[derive(Debug)]
pub struct HarnessScenario {
    id: HarnessScenarioId,
    steps: Vec<HarnessScenarioStep>,
}

impl HarnessScenario {
    pub fn define(id: impl Into<String>) -> Result<Self, HarnessScenarioIdError> {
        Ok(Self {
            id: HarnessScenarioId::new(id)?,
            steps: Vec::new(),
        })
    }

    pub fn step(mut self, step: HarnessScenarioStep) -> Self {
        self.steps.push(step);
        self
    }

    pub fn id(&self) -> &HarnessScenarioId {
        &self.id
    }

    pub fn steps(&self) -> &[HarnessScenarioStep] {
        &self.steps
    }

    pub fn into_parts(self) -> (HarnessScenarioId, Vec<HarnessScenarioStep>) {
        (self.id, self.steps)
    }
}
