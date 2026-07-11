use crate::{PhysicalScenarioObserverKind, PhysicalScenarioObserverRequirement};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioObserverTrace {
    observed_observers: Vec<PhysicalScenarioObserverKind>,
}

impl ScenarioObserverTrace {
    pub(crate) fn from_requirements(
        observer_requirements: &[PhysicalScenarioObserverRequirement],
    ) -> Self {
        Self {
            observed_observers: observer_requirements
                .iter()
                .map(PhysicalScenarioObserverRequirement::kind)
                .collect(),
        }
    }

    pub fn observed_observers(&self) -> &[PhysicalScenarioObserverKind] {
        &self.observed_observers
    }

    pub fn contains(&self, observer: PhysicalScenarioObserverKind) -> bool {
        self.observed_observers.contains(&observer)
    }
}
