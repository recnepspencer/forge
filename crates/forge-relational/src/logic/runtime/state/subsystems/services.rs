use std::collections::BTreeMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::logic::runtime::state::subsystems::RuntimeSubsystem;
use crate::logic::runtime::RuntimeComplexityCounters;
use crate::simulation::data::CompiledExecutionArtifact;
use crate::symbols::data::StringInterner;

#[derive(Debug, Clone, Default)]
struct RuntimeSequenceState {
    runtime_instance_id: u64,
    next_transaction_id: u64,
    next_savepoint_id: u64,
}

impl RuntimeSequenceState {
    fn new() -> Self {
        static NEXT_RUNTIME_INSTANCE_ID: AtomicU64 = AtomicU64::new(1);
        Self {
            runtime_instance_id: NEXT_RUNTIME_INSTANCE_ID.fetch_add(1, Ordering::Relaxed),
            next_transaction_id: 1,
            next_savepoint_id: 1,
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct RuntimeInstrumentation {
    pub(crate) complexity_counters: Mutex<RuntimeComplexityCounters>,
}

impl RuntimeInstrumentation {
    pub(crate) fn new() -> Self {
        Self {
            complexity_counters: Mutex::new(RuntimeComplexityCounters::default()),
        }
    }

    pub(crate) fn fork(&self) -> Self {
        Self {
            complexity_counters: Mutex::new(
                self.complexity_counters
                    .lock()
                    .expect("complexity counter lock poisoned")
                    .clone(),
            ),
        }
    }

    pub(crate) fn count(&self, update: impl FnOnce(&mut RuntimeComplexityCounters)) {
        update(
            &mut self
                .complexity_counters
                .lock()
                .expect("complexity counter lock poisoned"),
        );
    }
}

#[derive(Debug, Clone, Default)]
struct SimulationState {
    compiled_artifacts: BTreeMap<u64, CompiledExecutionArtifact>,
    next_compiled_artifact_id: u64,
}

impl SimulationState {
    fn new() -> Self {
        Self {
            compiled_artifacts: BTreeMap::new(),
            next_compiled_artifact_id: 1,
        }
    }
}

#[derive(Debug)]
pub(crate) struct RuntimeServices {
    sequence: RuntimeSequenceState,
    pub(crate) instrumentation: RuntimeInstrumentation,
    simulation: SimulationState,
    pub(crate) symbols: StringInterner,
}

impl RuntimeServices {
    fn empty() -> Self {
        Self {
            sequence: RuntimeSequenceState::new(),
            instrumentation: RuntimeInstrumentation::new(),
            simulation: SimulationState::new(),
            symbols: StringInterner::default(),
        }
    }

    pub(crate) fn next_transaction_id(&mut self) -> crate::transactions::data::TransactionId {
        let transaction_id =
            crate::transactions::data::TransactionId(self.sequence.next_transaction_id);
        self.sequence.next_transaction_id += 1;
        transaction_id
    }

    pub(crate) fn next_savepoint_id(&mut self) -> crate::transactions::data::SavepointId {
        let savepoint_id = crate::transactions::data::SavepointId(self.sequence.next_savepoint_id);
        self.sequence.next_savepoint_id += 1;
        savepoint_id
    }

    pub(crate) fn runtime_instance_id(&self) -> u64 {
        self.sequence.runtime_instance_id
    }

    pub(crate) fn compiled_artifact(
        &self,
        compiled_artifact_id: u64,
    ) -> Option<&CompiledExecutionArtifact> {
        self.simulation
            .compiled_artifacts
            .get(&compiled_artifact_id)
    }

    pub(crate) fn next_compiled_artifact_id(&self) -> u64 {
        self.simulation.next_compiled_artifact_id
    }

    pub(crate) fn store_compiled_artifact(&mut self, artifact: CompiledExecutionArtifact) -> u64 {
        let artifact_id = self.next_compiled_artifact_id();
        self.simulation.next_compiled_artifact_id += 1;
        self.simulation
            .compiled_artifacts
            .insert(artifact_id, artifact);
        artifact_id
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
