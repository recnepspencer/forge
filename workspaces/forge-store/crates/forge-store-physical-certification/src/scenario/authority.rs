#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalScenarioAuthorityWitness {
    _private: (),
}

impl PhysicalScenarioAuthorityWitness {
    pub(crate) const fn from_store_scenario_authority() -> Self {
        Self { _private: () }
    }
}
