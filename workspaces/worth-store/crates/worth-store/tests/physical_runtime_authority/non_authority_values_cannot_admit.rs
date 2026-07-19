use worth_store::physical_runtime::{
    ObservationHandle, PhysicalStore, ProcessRuntimeCounterSnapshot, RuntimeIdentity,
};
use worth_store_physical_format::{
    InMemoryPhysicalFormatModel, InMemoryPhysicalFormatReplayArtifact, PersistedPhysicalLayout,
};

fn admit_identity(identity: RuntimeIdentity) {
    let _runtime = PhysicalStore::admit(identity);
}

fn admit_observation(observation: ObservationHandle) {
    let _runtime = PhysicalStore::admit(observation);
}

fn admit_diagnostics(diagnostics: ProcessRuntimeCounterSnapshot) {
    let _runtime = PhysicalStore::admit(diagnostics);
}

fn admit_model(model: InMemoryPhysicalFormatModel) {
    let _runtime = PhysicalStore::admit(model);
}

fn admit_replay(replay: InMemoryPhysicalFormatReplayArtifact) {
    let _runtime = PhysicalStore::admit(replay);
}

fn admit_layout(layout: PersistedPhysicalLayout) {
    let _runtime = PhysicalStore::admit(layout);
}

fn main() {}
