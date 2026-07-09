use worth_proof::TransitionOutcome;

use super::super::super::{
    AspectKey, AuthoritativePatchConstructionDenial, AuthoritativeRecordAspectPatch,
    ContractValidatedAspectArtifact,
};

#[derive(Debug, Clone, Default)]
pub struct WholeAspectPatchBuilder {
    sets: Vec<ContractValidatedAspectArtifact>,
    clears: Vec<AspectKey>,
}

impl WholeAspectPatchBuilder {
    pub fn set(mut self, artifact: ContractValidatedAspectArtifact) -> Self {
        self.sets.push(artifact);
        self
    }

    pub fn clear(mut self, key: AspectKey) -> Self {
        self.clears.push(key);
        self
    }

    pub fn finish(
        self,
    ) -> TransitionOutcome<AuthoritativeRecordAspectPatch, AuthoritativePatchConstructionDenial>
    {
        AuthoritativeRecordAspectPatch::whole_aspect(self.sets, self.clears)
    }
}
