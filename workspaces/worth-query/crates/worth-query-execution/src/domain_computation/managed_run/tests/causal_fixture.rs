use std::collections::BTreeMap;
use std::sync::Arc;

use worth_foundational::facade::{AspectKey, FieldKey, ScalarAspectType};
use worth_relational::facade::bridge::RuntimeBridgeRelationalSource;
use worth_relational::facade::identity::{KindId, PartitionId};
use worth_relational::facade::runtime::{
    RelationalExecutionBasisLease, RelationalRuntime, RelationalRuntimeApi,
};
use worth_relational::facade::schema::{
    EntityKindRegistration, KindAspectContractDeclarations, RelationalSchemaRegistry, SchemaId,
    SchemaVersionId,
};
use worth_relational::facade::symbols::ClientKey;
use worth_relational::facade::transactions::{
    AspectFieldPatch, CreateIntent, EntitySpec, MutationIntent, TransactionOptions,
    WorkerIntentBatch,
};
use worth_runtime_bridge::facade::{
    BridgeAsyncRequestTruthViewBasis, BridgeAuthoritativeSourceProfile, BridgeBoundExecutionBasis,
    BridgeCommittedPatchEnvelope, BridgeDeliveryIntent, BridgeDeliveryReceipt,
    BridgeDiagnosticsTier, BridgeManagedExecutionIntent,
    BridgeManagedExecutionPartialEffectPosture, BridgeManagedExecutionStepContract,
    BridgeManagedExecutionStepLimits, BridgeMappingId, BridgeMappingRegistration, BridgeReplayMode,
    BridgeRuntimePolicy, BridgeSourceAdapter, BridgeSourceCapability, BridgeSourceCapabilitySet,
    BridgeTruthViewSelector, CoarseRoutingMode, CommittedPatchSource,
    HistoricalEvaluationDeclaration, InvalidationSink, MappingSelector,
    RelationalBridgeSnapshotIdentityParts, RelationalBridgeSourceError,
    RelationalCommittedPatchRequest, RuntimeBridge, RuntimeBridgeBuilder, SignalBridgeSinkError,
    SignalInvalidationScope, SnapshotReadContract, SnapshotReadPacket, SnapshotReadSource,
    SourceDeclaration, SourceDeclarationIdentity, TruthBranchIdentity, TruthPatchScope,
    TruthSnapshotIdentity, TruthSnapshotReader,
};

use super::super::WorthQueryManagedTruthReadRequest;

pub(super) struct CausalLowerExecutionBasis {
    pub bridge: BridgeBoundExecutionBasis,
    pub relational: RelationalExecutionBasisLease,
}

pub(crate) struct CausalManagedAdmissionContext {
    pub bridge: RuntimeBridge,
    pub relational: RuntimeBridgeRelationalSource,
    pub version_id: worth_relational::facade::identity::VersionId,
    pub branch: TruthBranchIdentity,
}

pub(super) struct SourceProfileSubstitutionContext {
    pub foreign_bridge: RuntimeBridge,
    pub exact_bridge: RuntimeBridge,
    pub relational: RuntimeBridgeRelationalSource,
    pub version_id: worth_relational::facade::identity::VersionId,
    pub branch: TruthBranchIdentity,
}

impl CausalManagedAdmissionContext {
    pub fn read_request(&self) -> WorthQueryManagedTruthReadRequest {
        WorthQueryManagedTruthReadRequest::new(
            self.version_id,
            self.branch.clone(),
            SnapshotReadPacket::new(vec![]),
        )
    }
}

pub(crate) fn managed_admission_context() -> CausalManagedAdmissionContext {
    let mut runtime = relational_runtime();
    let committed = create_fixture_entity(&mut runtime);
    let version_id = committed.version_id;
    let registration_snapshot =
        worth_relational::facade::bridge::bridge_snapshot_identity_for_commit(
            committed.commit.commit_id,
            committed.commit.version_id,
        );
    assert!(runtime.snapshots().release_snapshot(&committed.snapshot));
    let relational = RuntimeBridgeRelationalSource::for_graph_role(Arc::new(runtime), "model")
        .expect("model should be a valid graph role");
    let branch = TruthBranchIdentity::from_relational_branch_id("main");
    let bridge = bridge_for_source(relational.clone(), branch.clone(), registration_snapshot);
    CausalManagedAdmissionContext {
        bridge,
        relational,
        version_id,
        branch,
    }
}

pub(super) fn source_profile_substitution_context() -> SourceProfileSubstitutionContext {
    let mut runtime = relational_runtime();
    let committed = create_fixture_entity(&mut runtime);
    let version_id = committed.version_id;
    let registration_snapshot =
        worth_relational::facade::bridge::bridge_snapshot_identity_for_commit(
            committed.commit.commit_id,
            committed.commit.version_id,
        );
    assert!(runtime.snapshots().release_snapshot(&committed.snapshot));
    let relational = RuntimeBridgeRelationalSource::for_graph_role(Arc::new(runtime), "model")
        .expect("model should be a valid graph role");
    let branch = TruthBranchIdentity::from_relational_branch_id("main");
    let exact_bridge = bridge_for_source(
        relational.clone(),
        branch.clone(),
        registration_snapshot.clone(),
    );
    let foreign_bridge =
        bridge_for_foreign_profile(relational.clone(), branch.clone(), registration_snapshot);
    SourceProfileSubstitutionContext {
        foreign_bridge,
        exact_bridge,
        relational,
        version_id,
        branch,
    }
}

pub(super) fn causal_lower_execution_basis(
    operation_binding_identity: &str,
    resource_attempt_identity: &str,
) -> CausalLowerExecutionBasis {
    causal_lower_execution_basis_with_snapshot_match(
        operation_binding_identity,
        resource_attempt_identity,
        true,
    )
}

pub(super) fn mismatched_snapshot_lower_execution_basis(
    operation_binding_identity: &str,
    resource_attempt_identity: &str,
) -> CausalLowerExecutionBasis {
    causal_lower_execution_basis_with_snapshot_match(
        operation_binding_identity,
        resource_attempt_identity,
        false,
    )
}

fn causal_lower_execution_basis_with_snapshot_match(
    operation_binding_identity: &str,
    resource_attempt_identity: &str,
    matching_snapshot: bool,
) -> CausalLowerExecutionBasis {
    let (source, bridge_relational, substitute, branch, snapshot) =
        relational_source_and_lease(matching_snapshot);
    let bridge = bridge_for_source(source, branch.clone(), snapshot.clone());
    let planned = bridge
        .plan_truth_view_packet(
            HistoricalEvaluationDeclaration::new(
                BridgeTruthViewSelector::branch_snapshot(branch.clone(), snapshot.clone()),
                BridgeReplayMode::Enabled,
                BridgeDiagnosticsTier::Standard,
                BridgeDeliveryIntent::PrepareSignalEvaluation,
            ),
            SnapshotReadPacket::new(vec![]),
        )
        .expect("active Relational snapshot should plan");
    let bridge = bridge
        .admit_managed_execution_basis(
            BridgeManagedExecutionIntent::new(
                operation_binding_identity,
                resource_attempt_identity,
            ),
            BridgeManagedExecutionStepContract::new(
                "managed-run-safe-point",
                BridgeManagedExecutionStepLimits::new(8, 8, 8).with_memory_ceilings(8, 8),
                BridgeManagedExecutionPartialEffectPosture::None,
            )
            .expect("managed test step contract should be bounded"),
            BridgeAsyncRequestTruthViewBasis::branch_head(branch, snapshot),
            planned,
        )
        .expect("Bridge should mint Signal authority for the exact managed intent");
    let relational = substitute.unwrap_or(bridge_relational);
    CausalLowerExecutionBasis { bridge, relational }
}

fn relational_source_and_lease(
    matching_snapshot: bool,
) -> (
    RuntimeBridgeRelationalSource,
    RelationalExecutionBasisLease,
    Option<RelationalExecutionBasisLease>,
    TruthBranchIdentity,
    TruthSnapshotIdentity,
) {
    let mut runtime = relational_runtime();
    let committed = create_fixture_entity(&mut runtime);
    let version_id = committed.snapshot.version_id;
    let branch_id = committed.snapshot.branch_id.clone();
    assert!(runtime.snapshots().release_snapshot(&committed.snapshot));
    let source = RuntimeBridgeRelationalSource::for_graph_role(Arc::new(runtime), "model")
        .expect("model should be a valid graph role");
    let bridge_basis = source
        .admit_execution_basis(&branch_id, version_id)
        .expect("Relational source should retain the bridge execution basis");
    let substitute = if matching_snapshot {
        None
    } else {
        Some(
            source
                .admit_execution_basis(&branch_id, version_id)
                .expect("same version should admit an independent execution basis"),
        )
    };
    let snapshot = TruthSnapshotIdentity::from_relational_snapshot(
        RelationalBridgeSnapshotIdentityParts::new(
            bridge_basis.identity().snapshot_id().0,
            version_id.0,
        ),
    );
    let branch = TruthBranchIdentity::from_relational_branch_id("main");
    (source, bridge_basis, substitute, branch, snapshot)
}

fn relational_runtime() -> RelationalRuntime {
    RelationalRuntimeApi::builder()
        .schema_registry(
            RelationalSchemaRegistry::new()
                .register_entity_kind(EntityKindRegistration {
                    kind_id: KindId(1),
                    kind_name: "managed.run.fixture".into(),
                    schema_id: SchemaId("managed-run-fixture".into()),
                    schema_version_id: SchemaVersionId(1),
                    aspect_contract_declarations: KindAspectContractDeclarations::new(vec![]),
                })
                .expect("managed-run fixture schema should register"),
        )
        .build()
}

fn create_fixture_entity(
    runtime: &mut RelationalRuntime,
) -> worth_relational::facade::transactions::CommitResult {
    let mut transaction = runtime.begin_transaction(TransactionOptions::default());
    transaction.push_batch(WorkerIntentBatch::new("managed-run-fixture").push(
        MutationIntent::Create(CreateIntent::Entity(EntitySpec {
            partition_id: PartitionId::main(),
            kind_id: KindId(1),
            client_key: ClientKey::raw("managed-run-entity"),
            fields: AspectFieldPatch::new(BTreeMap::new()),
        })),
    ));
    transaction
        .commit()
        .expect("managed-run fixture entity should commit")
}

#[derive(Clone)]
struct SameRuntimeSourceAdapter {
    source: RuntimeBridgeRelationalSource,
}

#[derive(Clone)]
struct ForeignProfileRelationalSource {
    source: RuntimeBridgeRelationalSource,
}

impl CommittedPatchSource for ForeignProfileRelationalSource {
    fn authoritative_source_profile(&self) -> Option<BridgeAuthoritativeSourceProfile> {
        Some(
            BridgeAuthoritativeSourceProfile::new(
                self.source
                    .authoritative_source_profile()
                    .runtime_instance_id(),
                "foreign-managed-run-adapter",
            )
            .expect("hostile adapter profile should remain structurally valid"),
        )
    }

    fn load_committed_patch(
        &self,
        request: RelationalCommittedPatchRequest,
    ) -> Result<BridgeCommittedPatchEnvelope, RelationalBridgeSourceError> {
        CommittedPatchSource::load_committed_patch(&self.source, request)
    }
}

impl SnapshotReadSource for ForeignProfileRelationalSource {
    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError> {
        SnapshotReadSource::open_snapshot(&self.source, identity)
    }
}

impl BridgeSourceAdapter for SameRuntimeSourceAdapter {
    fn declared_capabilities(&self) -> BridgeSourceCapabilitySet {
        BridgeSourceCapabilitySet::new(vec![
            BridgeSourceCapability::SnapshotRead,
            BridgeSourceCapability::BranchRead,
        ])
    }

    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError> {
        SnapshotReadSource::open_snapshot(&self.source, identity)
    }
}

struct ManagedRunSink;

impl InvalidationSink for ManagedRunSink {
    fn deliver_invalidation(
        &self,
        delivery: worth_runtime_bridge::facade::BridgeSignalInvalidationDelivery,
    ) -> Result<BridgeDeliveryReceipt, SignalBridgeSinkError> {
        Ok(BridgeDeliveryReceipt::new(
            delivery.invalidation_targets().len(),
            delivery.source_snapshot().clone(),
        ))
    }
}

fn bridge_for_source(
    source: RuntimeBridgeRelationalSource,
    branch: TruthBranchIdentity,
    snapshot: TruthSnapshotIdentity,
) -> RuntimeBridge {
    bridge_for_truth_source(source.clone(), source, branch, snapshot)
}

fn bridge_for_foreign_profile(
    source: RuntimeBridgeRelationalSource,
    branch: TruthBranchIdentity,
    snapshot: TruthSnapshotIdentity,
) -> RuntimeBridge {
    bridge_for_truth_source(
        ForeignProfileRelationalSource {
            source: source.clone(),
        },
        source,
        branch,
        snapshot,
    )
}

fn bridge_for_truth_source<S>(
    truth_source: S,
    adapter_source: RuntimeBridgeRelationalSource,
    branch: TruthBranchIdentity,
    snapshot: TruthSnapshotIdentity,
) -> RuntimeBridge
where
    S: CommittedPatchSource + SnapshotReadSource,
{
    let selector = BridgeTruthViewSelector::branch_snapshot(branch, snapshot);
    RuntimeBridgeBuilder::new()
        .with_policy(BridgeRuntimePolicy::development())
        .with_relational_source(truth_source)
        .with_source_adapter(SameRuntimeSourceAdapter {
            source: adapter_source,
        })
        .with_signal_sink(ManagedRunSink)
        .register_source(SourceDeclaration::new(
            SourceDeclarationIdentity::from_stable_name("managed-run-source"),
            selector,
            BridgeSourceCapabilitySet::new(vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::BranchRead,
            ]),
        ))
        .register_mapping(managed_run_mapping())
        .build()
        .expect("managed-run Bridge should build")
}

fn managed_run_mapping() -> BridgeMappingRegistration {
    let aspect = AspectKey::new("managed-run").expect("valid aspect key");
    BridgeMappingRegistration::new(
        BridgeMappingId::from_stable_name("managed-run-mapping"),
        TruthPatchScope::for_entity_field(
            MappingSelector::exact("managed-run-entity"),
            aspect.clone(),
            FieldKey::new("value".to_owned()).expect("valid field key"),
        ),
        SnapshotReadContract::scalar(aspect, ScalarAspectType::String),
        SignalInvalidationScope::from_stable_name("managed-run-signal"),
        CoarseRoutingMode::Direct,
    )
}
