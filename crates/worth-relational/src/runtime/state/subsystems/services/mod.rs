mod compiled_artifacts;
mod instrumentation;
mod sequence;
#[cfg(test)]
mod tests;

use crate::runtime::state::subsystems::RuntimeSubsystem;
use crate::symbols::data::StringInterner;

pub(crate) use compiled_artifacts::CompiledArtifactStore;
pub(crate) use instrumentation::RuntimeInstrumentation;
use sequence::RuntimeSequenceState;

#[derive(Debug)]
pub(crate) struct RuntimeServices {
    sequence: RuntimeSequenceState,
    pub(crate) instrumentation: RuntimeInstrumentation,
    simulation: CompiledArtifactStore,
    pub(crate) symbols: StringInterner,
}

impl RuntimeServices {
    fn empty() -> Self {
        Self {
            sequence: RuntimeSequenceState::new(),
            instrumentation: RuntimeInstrumentation::new(),
            simulation: CompiledArtifactStore::new(),
            symbols: StringInterner::default(),
        }
    }

    pub(crate) fn next_transaction_id(&mut self) -> crate::transactions::data::TransactionId {
        self.sequence.next_transaction_id()
    }

    pub(crate) fn next_savepoint_id(&mut self) -> crate::transactions::data::SavepointId {
        self.sequence.next_savepoint_id()
    }

    pub(crate) fn next_proposal_ordinal(&mut self) -> Option<u64> {
        self.sequence.next_proposal_ordinal()
    }

    pub(crate) fn runtime_instance_id(&self) -> u64 {
        self.sequence.runtime_instance_id()
    }

    pub(crate) fn compiled_artifact(
        &self,
        compiled_artifact_id: u64,
    ) -> Option<&crate::simulation::data::CompiledExecutionArtifact> {
        self.simulation.compiled_artifact(compiled_artifact_id)
    }

    pub(crate) fn next_compiled_artifact_id(&self) -> u64 {
        self.simulation.next_compiled_artifact_id()
    }

    pub(crate) fn store_compiled_artifact(
        &mut self,
        artifact: crate::simulation::data::CompiledExecutionArtifact,
    ) -> u64 {
        self.simulation.store_compiled_artifact(artifact)
    }
}

impl RuntimeSubsystem for RuntimeServices {
    type Config = ();

    fn new(_: &Self::Config) -> Self {
        Self::empty()
    }

    fn fork(&self) -> Self {
        Self {
            sequence: RuntimeSequenceState::new(),
            instrumentation: self.instrumentation.fork(),
            simulation: self.simulation.clone(),
            symbols: self.symbols.clone(),
        }
    }
}
