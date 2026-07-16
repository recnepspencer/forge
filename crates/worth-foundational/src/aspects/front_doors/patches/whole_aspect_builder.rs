use worth_proof::TransitionOutcome;

use super::super::super::{
    AspectContract, AuthoritativePatchConstructionDenial, AuthoritativeRecordAspectPatch,
    ContractValidatedAspectArtifact,
};

#[derive(Debug, Clone, Default)]
pub struct WholeAspectPatchBuilder {
    sets: Vec<ContractValidatedAspectArtifact>,
    clears: Vec<AspectContract>,
}

impl WholeAspectPatchBuilder {
    pub fn set(mut self, artifact: ContractValidatedAspectArtifact) -> Self {
        self.sets.push(artifact);
        self
    }

    pub fn clear(mut self, contract: AspectContract) -> Self {
        self.clears.push(contract);
        self
    }

    pub fn finish(
        self,
    ) -> TransitionOutcome<AuthoritativeRecordAspectPatch, AuthoritativePatchConstructionDenial>
    {
        AuthoritativeRecordAspectPatch::whole_aspect(self.sets, self.clears)
    }
}
