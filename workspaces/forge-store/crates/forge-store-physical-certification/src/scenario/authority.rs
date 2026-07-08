/// Courtroom-only witness that certifies a scenario definition passed through
/// physical-certification scenario admission.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalScenarioAuthorityWitness {
    _private: (),
}

impl PhysicalScenarioAuthorityWitness {
    pub(crate) const fn from_store_scenario_authority() -> Self {
        Self { _private: () }
    }
}
