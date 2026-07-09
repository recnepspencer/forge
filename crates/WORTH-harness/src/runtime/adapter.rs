use std::future::Future;
use std::pin::Pin;

use serde_json::Value;

use crate::capture::{
    DiagnosticsRecord, EventRecord, EventStreamRecord, ExplanationRecord, ProvenanceRecord,
    RunRecord, ScenarioRecord, SnapshotRecord,
};
use crate::identity::scenario_id;
use crate::replay::{ReplayRecord, ReplayRequest};
use crate::scenario::{ExecutionProfile, ExecutionRequest, MutationBatch, ScenarioFixture};

use super::capability::HarnessCapabilities;

pub type HarnessFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;

pub trait HarnessAdapter {
    type Runtime;
    type Fixture;
    type Mutation;
    type TargetId: Clone;
    type Error;

    fn adapter_name(&self) -> &'static str;
    fn capabilities(&self) -> HarnessCapabilities;
    fn create_runtime(&self) -> Result<Self::Runtime, Self::Error>;
    fn prepare_runtime(
        &self,
        _runtime: &mut Self::Runtime,
        _profile: &ExecutionProfile,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
    fn load_fixture(
        &self,
        runtime: &mut Self::Runtime,
        fixture: &ScenarioFixture<Self::Fixture>,
    ) -> Result<(), Self::Error>;
    fn apply_mutation_batch(
        &self,
        runtime: &mut Self::Runtime,
        batch: &MutationBatch<Self::Mutation>,
    ) -> Result<(), Self::Error>;
    fn execute(
        &self,
        runtime: &mut Self::Runtime,
        fixture: &ScenarioFixture<Self::Fixture>,
        request: &ExecutionRequest<Self::TargetId>,
        profile: &ExecutionProfile,
    ) -> Result<RunRecord<Self::TargetId>, Self::Error>;
    fn capture_snapshot(
        &self,
        runtime: &Self::Runtime,
        fixture: &ScenarioFixture<Self::Fixture>,
        request: &ExecutionRequest<Self::TargetId>,
        profile: &ExecutionProfile,
    ) -> Result<SnapshotRecord<Self::TargetId>, Self::Error>;

    fn scenario_record(&self, fixture: &ScenarioFixture<Self::Fixture>) -> ScenarioRecord {
        ScenarioRecord::new(
            scenario_id(&fixture.name),
            fixture.name.clone(),
            fixture.declared_inputs.clone(),
            fixture.declared_observations.clone(),
            fixture.metadata.clone(),
        )
    }
}

pub trait DiagnosticsHarnessAdapter: HarnessAdapter {
    fn capture_diagnostics(
        &self,
        runtime: &Self::Runtime,
        fixture: &ScenarioFixture<Self::Fixture>,
        profile: &ExecutionProfile,
    ) -> Result<DiagnosticsRecord, Self::Error>;
}

pub trait ExplanationHarnessAdapter: HarnessAdapter {
    fn capture_explanations(
        &self,
        runtime: &Self::Runtime,
        fixture: &ScenarioFixture<Self::Fixture>,
        request: &ExecutionRequest<Self::TargetId>,
        profile: &ExecutionProfile,
    ) -> Result<Vec<ExplanationRecord<Self::TargetId>>, Self::Error>;
}

pub trait ProvenanceHarnessAdapter: HarnessAdapter {
    fn capture_provenance(
        &self,
        runtime: &Self::Runtime,
        fixture: &ScenarioFixture<Self::Fixture>,
        request: &ExecutionRequest<Self::TargetId>,
        profile: &ExecutionProfile,
    ) -> Result<Vec<ProvenanceRecord<Self::TargetId>>, Self::Error>;
}

pub trait EventHarnessAdapter: HarnessAdapter {
    fn capture_events(
        &self,
        runtime: &Self::Runtime,
        fixture: &ScenarioFixture<Self::Fixture>,
        request: &ExecutionRequest<Self::TargetId>,
        profile: &ExecutionProfile,
    ) -> Result<Vec<EventRecord<Self::TargetId>>, Self::Error>;
}

pub trait EventStreamHarnessAdapter: HarnessAdapter {
    fn capture_event_streams(
        &self,
        runtime: &Self::Runtime,
        fixture: &ScenarioFixture<Self::Fixture>,
        request: &ExecutionRequest<Self::TargetId>,
        profile: &ExecutionProfile,
    ) -> Result<Vec<EventStreamRecord<Self::TargetId>>, Self::Error>;
}

pub trait PerformanceHarnessAdapter: HarnessAdapter {
    fn capture_performance(
        &self,
        runtime: &Self::Runtime,
        fixture: &ScenarioFixture<Self::Fixture>,
        profile: &ExecutionProfile,
    ) -> Result<Value, Self::Error>;
}

pub trait ReplayHarnessAdapter: HarnessAdapter {
    fn capture_replay(
        &self,
        runtime: &Self::Runtime,
        fixture: &ScenarioFixture<Self::Fixture>,
        replay: &ReplayRequest<Self::TargetId>,
    ) -> Result<ReplayRecord<Self::TargetId>, Self::Error>;
}

pub trait HarnessAdapterAsync {
    type Runtime;
    type Fixture;
    type Mutation;
    type TargetId: Clone;
    type Error;

    fn adapter_name(&self) -> &'static str;
    fn capabilities(&self) -> HarnessCapabilities;
    fn create_runtime_async(&self) -> HarnessFuture<'_, Result<Self::Runtime, Self::Error>>;
    fn prepare_runtime_async(
        &self,
        _runtime: &mut Self::Runtime,
        _profile: &ExecutionProfile,
    ) -> HarnessFuture<'_, Result<(), Self::Error>> {
        Box::pin(async { Ok(()) })
    }
    fn load_fixture_async(
        &self,
        runtime: &mut Self::Runtime,
        fixture: &ScenarioFixture<Self::Fixture>,
    ) -> HarnessFuture<'_, Result<(), Self::Error>>;
    fn apply_mutation_batch_async(
        &self,
        runtime: &mut Self::Runtime,
        batch: &MutationBatch<Self::Mutation>,
    ) -> HarnessFuture<'_, Result<(), Self::Error>>;
    fn execute_async(
        &self,
        runtime: &mut Self::Runtime,
        fixture: &ScenarioFixture<Self::Fixture>,
        request: &ExecutionRequest<Self::TargetId>,
        profile: &ExecutionProfile,
    ) -> HarnessFuture<'_, Result<RunRecord<Self::TargetId>, Self::Error>>;
    fn capture_snapshot_async(
        &self,
        runtime: &Self::Runtime,
        fixture: &ScenarioFixture<Self::Fixture>,
        request: &ExecutionRequest<Self::TargetId>,
        profile: &ExecutionProfile,
    ) -> HarnessFuture<'_, Result<SnapshotRecord<Self::TargetId>, Self::Error>>;

    fn scenario_record(&self, fixture: &ScenarioFixture<Self::Fixture>) -> ScenarioRecord {
        ScenarioRecord::new(
            scenario_id(&fixture.name),
            fixture.name.clone(),
            fixture.declared_inputs.clone(),
            fixture.declared_observations.clone(),
            fixture.metadata.clone(),
        )
    }
}
