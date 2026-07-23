use worth_foundational::facade::ContractValidatedAspectArtifact;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryConditionalObservationEvidence {
    pub(crate) dependency_ordinal: usize,
    pub(crate) previous: Option<ContractValidatedAspectArtifact>,
    pub(crate) current: Option<ContractValidatedAspectArtifact>,
}

impl WorthQueryConditionalObservationEvidence {
    pub const fn dependency_ordinal(&self) -> usize {
        self.dependency_ordinal
    }

    pub fn previous(&self) -> Option<&ContractValidatedAspectArtifact> {
        self.previous.as_ref()
    }

    pub fn current(&self) -> Option<&ContractValidatedAspectArtifact> {
        self.current.as_ref()
    }

    pub const fn was_observed(&self) -> bool {
        self.current.is_some()
    }
}
