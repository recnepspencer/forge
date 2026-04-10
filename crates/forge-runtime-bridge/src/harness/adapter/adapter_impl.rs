use std::collections::{BTreeMap, BTreeSet};

use forge_harness::facade::{
    run_id, snapshot_id, AdapterSupport, CaptureDepth, DiagnosticsLevel, ExecutionMode,
    ExecutionProfile, ExecutionRequest, HarnessAdapter, HarnessCapabilities, MutationBatch,
    ObservationStatus, RecordSchemaVersion, RunOutcome, RunRecord, RunStatus, ScenarioFixture,
    SnapshotObservation, SnapshotPayload, SnapshotRecord, StructuredValue, TargetStatusRecord,
};
use serde_json::json;

use crate::facade::{
    BridgeDeliveryIntent, BridgeHistoricalEvaluationExplanation, BridgeMappingContext,
    BridgeReplayMode, BridgeRouteRequest, BridgeTruthViewSelector, HistoricalEvaluationDeclaration,
    RuntimeBridgeBuilder, SnapshotReadPacket, TruthBranchIdentity, TruthCommitIdentity,
};
use crate::source::BridgeSourceCapabilitySet;

use super::super::fixtures::BridgeHarnessFixture;
use super::support::route_record_json;
use super::types::{
    BridgeHarnessAdapter, BridgeHarnessError, BridgeHarnessMutation, BridgeHarnessSession,
    PolicyBuilderLoadOrder, SourceAdapterBehavior, SourceAdapterShape, SourceBuilderLoadOrder,
};
use merge::execute_merge_request;
use policy::execute_policy_request;
use source::execute_source_request;
use speculation::execute_speculation_request;
use stream::execute_stream_request;
use structural::execute_structural_request;

#[derive(Clone)]
struct FixtureSourceAdapter {
    capabilities: BridgeSourceCapabilitySet,
    source: super::super::fixtures::InMemoryRelationalBridgeSource,
}

#[derive(Clone)]
struct WrappedFixtureSourceAdapter {
    inner: SessionSourceAdapter,
}

struct DriftFixtureSnapshotReader {
    inner: Box<dyn crate::facade::TruthSnapshotReader>,
}

impl crate::adapter::BridgeSourceAdapter for FixtureSourceAdapter {
    fn declared_capabilities(&self) -> BridgeSourceCapabilitySet {
        self.capabilities.clone()
    }

    fn open_snapshot(
        &self,
        identity: &crate::facade::TruthSnapshotIdentity,
    ) -> Result<
        Box<dyn crate::facade::TruthSnapshotReader>,
        crate::adapter::RelationalBridgeSourceError,
    > {
        crate::adapter::SnapshotReadSource::open_snapshot(&self.source, identity)
    }
}

impl crate::facade::TruthSnapshotReader for DriftFixtureSnapshotReader {
    fn snapshot_identity(&self) -> crate::facade::TruthSnapshotIdentity {
        crate::facade::TruthSnapshotIdentity::new("snapshot-drift")
    }

    fn read_packet(
        &self,
        request: &SnapshotReadPacket,
    ) -> Result<crate::facade::SnapshotReadPacketResult, crate::facade::BridgeSnapshotReadError>
    {
        self.inner.read_packet(request)
    }
}

impl HarnessAdapter for BridgeHarnessAdapter {
    type Runtime = BridgeHarnessSession;
    type Fixture = BridgeHarnessFixture;
    type Mutation = BridgeHarnessMutation;
    type TargetId = String;
    type Error = BridgeHarnessError;

    fn adapter_name(&self) -> &'static str {
        "forge-runtime-bridge"
    }

    fn capabilities(&self) -> HarnessCapabilities {
        let mut capabilities = HarnessCapabilities::default();
        capabilities.execution_modes =
            BTreeSet::from([ExecutionMode::RuntimeDefault, ExecutionMode::Serial]);
        capabilities.diagnostics_levels = BTreeSet::from([
            DiagnosticsLevel::Operational,
            DiagnosticsLevel::Development,
            DiagnosticsLevel::Forensic,
        ]);
        capabilities.capture_depths = BTreeSet::from([
            CaptureDepth::Minimal,
            CaptureDepth::Standard,
            CaptureDepth::Rich,
        ]);
        capabilities.replay_support = AdapterSupport::Supported;
        capabilities.rich_record_kinds = BTreeSet::from([
            "bridge_route_record".to_string(),
            "bridge_replay_record".to_string(),
            "bridge_delivery_receipt".to_string(),
            "bridge_merge_record".to_string(),
            "bridge_historical_evaluation_record".to_string(),
            "bridge_source_materialization_record".to_string(),
            "bridge_speculation_record".to_string(),
            "bridge_structural_remap_record".to_string(),
            "bridge_structural_branch_comparison_record".to_string(),
        ]);
        capabilities
    }

    fn create_runtime(&self) -> Result<Self::Runtime, Self::Error> {
        Ok(BridgeHarnessSession::default())
    }

    fn prepare_runtime(
        &self,
        runtime: &mut Self::Runtime,
        profile: &ExecutionProfile,
    ) -> Result<(), Self::Error> {
        runtime.source_adapter_shape = match profile
            .metadata
            .get("source_adapter_shape")
            .map(String::as_str)
        {
            Some("wrapped") => SourceAdapterShape::Wrapped,
            _ => SourceAdapterShape::Direct,
        };
        runtime.source_builder_load_order = match profile
            .metadata
            .get("source_builder_load_order")
            .map(String::as_str)
        {
            Some("sources_first") => SourceBuilderLoadOrder::SourcesBeforeAdapter,
            _ => SourceBuilderLoadOrder::AdapterBeforeSources,
        };
        runtime.policy_builder_load_order = match profile
            .metadata
            .get("policy_builder_load_order")
            .map(String::as_str)
        {
            Some("sections_canonical") => PolicyBuilderLoadOrder::SectionsCanonical,
            Some("sections_reverse") => PolicyBuilderLoadOrder::SectionsReverse,
            _ => PolicyBuilderLoadOrder::WholePolicy,
        };
        runtime.source_adapter_behavior = match profile
            .metadata
            .get("source_adapter_behavior")
            .map(String::as_str)
        {
            Some("reject_open_snapshot") => SourceAdapterBehavior::RejectOpenSnapshot,
            Some("drift_snapshot_identity") => SourceAdapterBehavior::DriftSnapshotIdentity,
            _ => SourceAdapterBehavior::Honest,
        };
        match profile.execution_mode {
            ExecutionMode::RuntimeDefault | ExecutionMode::Serial => Ok(()),
            mode => Err(BridgeHarnessError::new(format!(
                "bridge harness does not support execution mode `{mode:?}`"
            ))),
        }
    }

    fn load_fixture(
        &self,
        runtime: &mut Self::Runtime,
        fixture: &ScenarioFixture<Self::Fixture>,
    ) -> Result<(), Self::Error> {
        for patch in fixture.fixture.committed_patches() {
            runtime.source.insert_committed_patch(patch.clone());
        }
        for snapshot in fixture.fixture.snapshots() {
            runtime.source.insert_snapshot(snapshot.clone());
        }
        for (entity_identity, authority) in fixture.fixture.continuity_authorities() {
            runtime
                .source
                .insert_continuity_authority(entity_identity.clone(), authority.clone());
        }

        let builder = apply_policy_builder_load_order(
            RuntimeBridgeBuilder::new()
            .with_relational_source(runtime.source.clone())
            .with_truth_branch_head_source(runtime.source.clone())
            .with_signal_sink(runtime.sink.clone())
            .with_continuity_lineage_source(runtime.source.clone()),
            runtime,
            fixture.fixture.policy(),
        );
        let (first_mapping, remaining_mappings) =
            fixture.fixture.mappings().split_first().ok_or_else(|| {
                BridgeHarnessError::new("bridge harness fixture requires at least one mapping")
            })?;
        let mut builder = builder.register_mapping(first_mapping.clone());
        for mapping in remaining_mappings {
            builder = builder.register_mapping(mapping.clone());
        }
        for aspect_mapping in fixture.fixture.aspect_mappings() {
            builder = builder.register_aspect_mapping(aspect_mapping.clone());
        }
        for structural_declaration in fixture.fixture.structural_declarations() {
            builder = builder.register_structural(structural_declaration.clone());
        }
        for merge_declaration in fixture.fixture.merge_declarations() {
            builder = builder.register_merge(merge_declaration.clone());
        }
        if !fixture.fixture.source_declarations().is_empty() {
            let adapter_capabilities = fixture
                .fixture
                .source_adapter_capabilities()
                .cloned()
                .unwrap_or_else(|| {
                    let mut capabilities = Vec::new();
                    for declaration in fixture.fixture.source_declarations() {
                        capabilities
                            .extend_from_slice(declaration.required_capabilities().capabilities());
                    }
                    BridgeSourceCapabilitySet::new(capabilities)
                });
            let source_adapter = FixtureSourceAdapter {
                capabilities: adapter_capabilities,
                source: runtime.source.clone(),
            };
            builder = match runtime.source_builder_load_order {
                SourceBuilderLoadOrder::AdapterBeforeSources => {
                    let mut builder = with_session_source_adapter(builder, runtime, source_adapter);
                    for declaration in fixture.fixture.source_declarations() {
                        builder = builder.register_source(declaration.clone());
                    }
                    builder
                }
                SourceBuilderLoadOrder::SourcesBeforeAdapter => {
                    let mut builder = builder;
                    for declaration in fixture.fixture.source_declarations() {
                        builder = builder.register_source(declaration.clone());
                    }
                    with_session_source_adapter(builder, runtime, source_adapter)
                }
            };
        }
        runtime.runtime = Some(builder.build().map_err(|error| {
            BridgeHarnessError::new(format!("failed to build bridge runtime: {error}"))
        })?);
        Ok(())
    }

    fn apply_mutation_batch(
        &self,
        runtime: &mut Self::Runtime,
        batch: &MutationBatch<Self::Mutation>,
    ) -> Result<(), Self::Error> {
        for operation in &batch.operations {
            match operation {
                BridgeHarnessMutation::PublishCommittedPatch(patch) => {
                    runtime.source.insert_committed_patch(patch.clone());
                }
                BridgeHarnessMutation::PublishSnapshot(snapshot) => {
                    runtime.source.insert_snapshot(snapshot.clone());
                }
            }
        }
        Ok(())
    }

    fn execute(
        &self,
        runtime: &mut Self::Runtime,
        fixture: &ScenarioFixture<Self::Fixture>,
        request: &ExecutionRequest<Self::TargetId>,
        profile: &ExecutionProfile,
    ) -> Result<RunRecord<Self::TargetId>, Self::Error> {
        let runtime_bridge = runtime
            .runtime
            .as_ref()
            .ok_or_else(|| BridgeHarnessError::new("bridge runtime not loaded"))?;
        let target = request
            .targets
            .first()
            .ok_or_else(|| BridgeHarnessError::new("bridge execution requires one target"))?;
        let harness_target = parse_harness_target(target)?;
        let mapping_context = fixture
            .fixture
            .lineage_context()
            .cloned()
            .map(|lineage_context| {
                BridgeMappingContext::default().with_lineage_context(lineage_context)
            })
            .unwrap_or_default();
        let scenario_id_value = forge_harness::facade::scenario_id(&fixture.name);
        let run_id_value = run_id(&scenario_id_value, &profile.name, &request.name);

        let execution = match harness_target {
            HarnessTarget::CommittedRoute { commit_identity } => {
                let route = runtime_bridge
                    .plan_committed_patch_with_mapping_context(
                        BridgeRouteRequest::for_commit(commit_identity.clone()),
                        mapping_context,
                    )
                    .map_err(|error| {
                        BridgeHarnessError::new(format!("bridge planning failed: {error}"))
                    })?;
                let result = runtime_bridge
                    .deliver_invalidation(route)
                    .map_err(|error| {
                        BridgeHarnessError::new(format!("bridge delivery failed: {error}"))
                    })?;
                let continuity_summary =
                    runtime_bridge
                        .diagnostics()
                        .last_route_record()
                        .and_then(|route_record| {
                            let requests = runtime_bridge
                                .plan_continuity_requests(&route_record)
                                .ok()?;
                            let packet = runtime_bridge
                                .plan_historical_lineage_packet(&requests)
                                .ok()?;
                            let resolved =
                                runtime_bridge.resolve_lineage_continuity(&packet).ok()?;
                            let artifact = runtime_bridge.lower_continuity_artifact(&resolved);
                            let canonical = runtime_bridge.canonicalize_continuity_record(
                                &route_record,
                                &requests,
                                &artifact,
                            );
                            Some((artifact, canonical))
                        });
                HarnessExecution::Route {
                    result,
                    continuity_summary,
                }
            }
            HarnessTarget::Stream(stream_target) => {
                HarnessExecution::Stream(execute_stream_request(runtime_bridge, stream_target)?)
            }
            HarnessTarget::Source(source_target) => HarnessExecution::Source(
                execute_source_request(runtime_bridge, &fixture.fixture, source_target)?,
            ),
            HarnessTarget::Merge(merge_target) => HarnessExecution::Merge(execute_merge_request(
                runtime_bridge,
                &fixture.fixture,
                merge_target,
            )?),
            HarnessTarget::Policy(policy_target) => HarnessExecution::Policy(
                execute_policy_request(runtime_bridge, &fixture.fixture, policy_target)?,
            ),
            HarnessTarget::Speculation(speculation_target) => HarnessExecution::Speculation(
                execute_speculation_request(runtime_bridge, &fixture.fixture, speculation_target)?,
            ),
            HarnessTarget::Structural(structural_target) => HarnessExecution::Structural(
                execute_structural_request(runtime_bridge, &fixture.fixture, structural_target)?,
            ),
            HarnessTarget::HistoricalCommit {
                branch_identity,
                commit_identity,
            } => execute_historical_request(
                runtime_bridge,
                HistoricalEvaluationDeclaration::new(
                    BridgeTruthViewSelector::historical_commit(branch_identity, commit_identity),
                    BridgeReplayMode::Enabled,
                    fixture.fixture.policy().diagnostics_tier(),
                    BridgeDeliveryIntent::PrepareSignalEvaluation,
                ),
            )?,
            HarnessTarget::BranchHead { branch_identity } => execute_historical_request(
                runtime_bridge,
                HistoricalEvaluationDeclaration::new(
                    BridgeTruthViewSelector::branch_head(branch_identity),
                    BridgeReplayMode::Enabled,
                    fixture.fixture.policy().diagnostics_tier(),
                    BridgeDeliveryIntent::PrepareSignalEvaluation,
                ),
            )?,
            HarnessTarget::BranchCommit {
                branch_identity,
                commit_identity,
            } => execute_historical_request(
                runtime_bridge,
                HistoricalEvaluationDeclaration::new(
                    BridgeTruthViewSelector::branch_commit(branch_identity, commit_identity),
                    BridgeReplayMode::Enabled,
                    fixture.fixture.policy().diagnostics_tier(),
                    BridgeDeliveryIntent::PrepareSignalEvaluation,
                ),
            )?,
        };

        Ok(RunRecord {
            schema_version: RecordSchemaVersion::V1,
            run_id: run_id_value,
            scenario_id: scenario_id_value,
            adapter_name: self.adapter_name().to_string(),
            scenario_name: fixture.name.clone(),
            profile_name: profile.name.clone(),
            time_marker: profile.time_marker.clone(),
            feed_batch: request.feed_batch.clone(),
            execution_mode: profile.execution_mode,
            diagnostics_level: profile.diagnostics_level,
            status: RunStatus::Succeeded,
            outcome: RunOutcome::Completed,
            budget_usage: None,
            requested_targets: request.targets.clone(),
            target_statuses: request
                .targets
                .iter()
                .map(|target| TargetStatusRecord {
                    target: target.clone(),
                    status: ObservationStatus::Validated,
                    detail: None,
                })
                .collect(),
            changed_targets: request.targets.clone(),
            attachments: Vec::new(),
            summary: execution.summary_json(),
            extensions: execution.extensions_json(runtime_bridge),
        })
    }

    fn capture_snapshot(
        &self,
        runtime: &Self::Runtime,
        fixture: &ScenarioFixture<Self::Fixture>,
        request: &ExecutionRequest<Self::TargetId>,
        profile: &ExecutionProfile,
    ) -> Result<SnapshotRecord<Self::TargetId>, Self::Error> {
        let scenario_id_value = forge_harness::facade::scenario_id(&fixture.name);
        let run_id_value = run_id(&scenario_id_value, &profile.name, &request.name);
        let runtime_bridge = runtime
            .runtime
            .as_ref()
            .ok_or_else(|| BridgeHarnessError::new("bridge runtime not loaded"))?;
        let (status, detail, value) = if let Some(record) = runtime_bridge
            .diagnostics()
            .last_historical_evaluation_record()
        {
            (
                ObservationStatus::Clean,
                Some("historical evaluation record captured".to_string()),
                Some(SnapshotPayload::Structured(StructuredValue::Json(json!({
                    "snapshot_identity": record.decision_log().snapshot_identity().as_str(),
                    "record_identity": record.record_identity().as_str(),
                })))),
            )
        } else {
            let last_route_record = runtime_bridge.diagnostics().last_route_record();
            match last_route_record {
                Some(record) => (
                    ObservationStatus::Clean,
                    Some(format!(
                        "{} snapshot records captured",
                        record.counters().snapshot_read_count()
                    )),
                    Some(SnapshotPayload::Structured(StructuredValue::Json(json!({
                        "snapshot_identity": record.source_snapshot().as_str(),
                        "read_count": record.counters().snapshot_read_count(),
                    })))),
                ),
                None => (
                    ObservationStatus::Clean,
                    Some("bridge delivery has not run yet".to_string()),
                    Some(SnapshotPayload::Structured(StructuredValue::Json(json!({
                        "snapshot_identity": null,
                        "read_count": 0,
                    })))),
                ),
            }
        };
        Ok(SnapshotRecord {
            schema_version: RecordSchemaVersion::V1,
            snapshot_id: snapshot_id(&run_id_value, "capture"),
            run_id: run_id_value,
            adapter_name: self.adapter_name().to_string(),
            scenario_name: fixture.name.clone(),
            profile_name: profile.name.clone(),
            time_marker: profile.time_marker.clone(),
            observations: vec![SnapshotObservation {
                target: request
                    .targets
                    .first()
                    .cloned()
                    .unwrap_or_else(|| "bridge".to_string()),
                status,
                detail,
                value,
            }],
            attachments: Vec::new(),
            extensions: BTreeMap::new(),
        })
    }
}

mod merge;
mod policy;
mod source;
mod speculation;
mod stream;
mod structural;
use execution::{
    execute_historical_request, parse_harness_target, HarnessExecution, HarnessTarget,
};
mod execution;

fn with_session_source_adapter<
    PatchState,
    SnapshotState,
    SignalState,
    BranchHeadState,
    MappingState,
>(
    builder: RuntimeBridgeBuilder<
        PatchState,
        SnapshotState,
        SignalState,
        BranchHeadState,
        MappingState,
    >,
    runtime: &BridgeHarnessSession,
    adapter: FixtureSourceAdapter,
) -> RuntimeBridgeBuilder<PatchState, SnapshotState, SignalState, BranchHeadState, MappingState> {
    let adapter = SessionSourceAdapter {
        runtime: runtime.clone(),
        inner: adapter,
    };
    match runtime.source_adapter_shape {
        SourceAdapterShape::Direct => builder.with_source_adapter(adapter),
        SourceAdapterShape::Wrapped => {
            builder.with_source_adapter(WrappedFixtureSourceAdapter { inner: adapter })
        }
    }
}

fn apply_policy_builder_load_order<
    PatchState,
    SnapshotState,
    SignalState,
    BranchHeadState,
    MappingState,
>(
    builder: RuntimeBridgeBuilder<
        PatchState,
        SnapshotState,
        SignalState,
        BranchHeadState,
        MappingState,
    >,
    runtime: &BridgeHarnessSession,
    policy: crate::facade::BridgeRuntimePolicy,
) -> RuntimeBridgeBuilder<PatchState, SnapshotState, SignalState, BranchHeadState, MappingState> {
    match runtime.policy_builder_load_order {
        PolicyBuilderLoadOrder::WholePolicy => builder.with_policy(policy),
        PolicyBuilderLoadOrder::SectionsCanonical => builder
            .with_execution_policy_baseline(policy.execution())
            .with_diagnostics_policy_baseline(policy.diagnostics())
            .with_artifact_policy_baseline(policy.artifacts()),
        PolicyBuilderLoadOrder::SectionsReverse => builder
            .with_artifact_policy_baseline(policy.artifacts())
            .with_diagnostics_policy_baseline(policy.diagnostics())
            .with_execution_policy_baseline(policy.execution()),
    }
}

#[derive(Clone)]
struct SessionSourceAdapter {
    runtime: BridgeHarnessSession,
    inner: FixtureSourceAdapter,
}

impl crate::adapter::BridgeSourceAdapter for SessionSourceAdapter {
    fn declared_capabilities(&self) -> BridgeSourceCapabilitySet {
        self.inner.declared_capabilities()
    }

    fn open_snapshot(
        &self,
        identity: &crate::facade::TruthSnapshotIdentity,
    ) -> Result<
        Box<dyn crate::facade::TruthSnapshotReader>,
        crate::adapter::RelationalBridgeSourceError,
    > {
        match self.runtime.source_adapter_behavior {
            SourceAdapterBehavior::Honest => self.inner.open_snapshot(identity),
            SourceAdapterBehavior::RejectOpenSnapshot => {
                Err(crate::adapter::RelationalBridgeSourceError::new(format!(
                    "session source adapter refused snapshot `{}`",
                    identity.as_str()
                )))
            }
            SourceAdapterBehavior::DriftSnapshotIdentity => {
                let reader = self.inner.open_snapshot(identity)?;
                Ok(Box::new(DriftFixtureSnapshotReader { inner: reader }))
            }
        }
    }
}

impl crate::adapter::BridgeSourceAdapter for WrappedFixtureSourceAdapter {
    fn declared_capabilities(&self) -> BridgeSourceCapabilitySet {
        self.inner.declared_capabilities()
    }

    fn open_snapshot(
        &self,
        identity: &crate::facade::TruthSnapshotIdentity,
    ) -> Result<
        Box<dyn crate::facade::TruthSnapshotReader>,
        crate::adapter::RelationalBridgeSourceError,
    > {
        self.inner.open_snapshot(identity)
    }
}
