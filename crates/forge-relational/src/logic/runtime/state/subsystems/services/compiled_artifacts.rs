use std::collections::BTreeMap;

use crate::simulation::data::CompiledExecutionArtifact;

#[derive(Debug, Clone, Default)]
pub(crate) struct CompiledArtifactStore {
    compiled_artifacts: BTreeMap<u64, CompiledExecutionArtifact>,
    next_compiled_artifact_id: u64,
}

impl CompiledArtifactStore {
    pub(crate) fn new() -> Self {
        Self {
            compiled_artifacts: BTreeMap::new(),
            next_compiled_artifact_id: 1,
        }
    }

    pub(crate) fn compiled_artifact(
        &self,
        compiled_artifact_id: u64,
    ) -> Option<&CompiledExecutionArtifact> {
        self.compiled_artifacts.get(&compiled_artifact_id)
    }

    pub(crate) fn next_compiled_artifact_id(&self) -> u64 {
        self.next_compiled_artifact_id
    }

    pub(crate) fn store_compiled_artifact(&mut self, artifact: CompiledExecutionArtifact) -> u64 {
        let artifact_id = self.next_compiled_artifact_id();
        self.next_compiled_artifact_id += 1;
        self.compiled_artifacts.insert(artifact_id, artifact);
        artifact_id
    }
}
