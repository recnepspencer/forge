use std::sync::Arc;

use crate::adapter::{
    CommittedPatchSource, ContinuityLineageSource, InvalidationSink, SnapshotReadSource,
    SnapshotReaderPool, TruthBranchHeadSource,
};
use crate::diagnostics::DiagnosticSink;
use crate::error::BridgeBuildError;
use crate::facade::RuntimeBridge;
use crate::mapping::{
    BridgeAspectRegistration, BridgeMappingRegistration, FrozenAspectMappingRegistry,
    FrozenMappingRegistry,
};
use crate::policy::BridgeRuntimePolicy;

#[derive(Clone)]
pub struct MissingCommittedPatchSource;

#[derive(Clone)]
pub struct PresentCommittedPatchSource(Arc<dyn CommittedPatchSource>);

#[derive(Clone)]
pub struct MissingSnapshotReadSource;

#[derive(Clone)]
pub struct PresentSnapshotReadSource(Arc<dyn SnapshotReadSource>);

#[derive(Clone)]
pub struct MissingSignalSink;

#[derive(Clone)]
pub struct PresentSignalSink(Arc<dyn InvalidationSink>);

#[derive(Clone)]
pub struct MissingTruthBranchHeadSource;

#[derive(Clone)]
pub struct PresentTruthBranchHeadSource(Arc<dyn TruthBranchHeadSource>);

#[derive(Clone)]
pub struct MissingMappingRegistrations;

#[derive(Clone)]
pub struct PresentMappingRegistrations(Vec<BridgeMappingRegistration>);

#[derive(Clone)]
pub struct RuntimeBridgeBuilder<
    PatchState = MissingCommittedPatchSource,
    SnapshotState = MissingSnapshotReadSource,
    SignalState = MissingSignalSink,
    BranchHeadState = MissingTruthBranchHeadSource,
    MappingState = MissingMappingRegistrations,
> {
    policy: BridgeRuntimePolicy,
    committed_patch_source: PatchState,
    snapshot_read_source: SnapshotState,
    signal_sink: SignalState,
    truth_branch_head_source: BranchHeadState,
    continuity_lineage_source: Option<Arc<dyn ContinuityLineageSource>>,
    snapshot_reader_pool: Option<Arc<dyn SnapshotReaderPool>>,
    diagnostic_sink: Option<Arc<dyn DiagnosticSink>>,
    mapping_registrations: MappingState,
    aspect_registrations: Vec<BridgeAspectRegistration>,
}

impl Default for RuntimeBridgeBuilder<MissingCommittedPatchSource, MissingSnapshotReadSource, MissingSignalSink, MissingTruthBranchHeadSource, MissingMappingRegistrations> {
    fn default() -> Self {
        Self::new()
    }
}

impl RuntimeBridgeBuilder<MissingCommittedPatchSource, MissingSnapshotReadSource, MissingSignalSink, MissingTruthBranchHeadSource, MissingMappingRegistrations> {
    pub fn new() -> Self {
        Self {
            policy: BridgeRuntimePolicy::default(),
            committed_patch_source: MissingCommittedPatchSource,
            snapshot_read_source: MissingSnapshotReadSource,
            signal_sink: MissingSignalSink,
            truth_branch_head_source: MissingTruthBranchHeadSource,
            continuity_lineage_source: None,
            snapshot_reader_pool: None,
            diagnostic_sink: None,
            mapping_registrations: MissingMappingRegistrations,
            aspect_registrations: Vec::new(),
        }
    }
}

impl<SnapshotState, SignalState, BranchHeadState, MappingState>
    RuntimeBridgeBuilder<MissingCommittedPatchSource, SnapshotState, SignalState, BranchHeadState, MappingState>
{
    pub fn with_committed_patch_source<S>(
        self,
        source: S,
    ) -> RuntimeBridgeBuilder<PresentCommittedPatchSource, SnapshotState, SignalState, BranchHeadState, MappingState>
    where
        S: CommittedPatchSource,
    {
        RuntimeBridgeBuilder {
            policy: self.policy,
            committed_patch_source: PresentCommittedPatchSource(Arc::new(source)),
            snapshot_read_source: self.snapshot_read_source,
            signal_sink: self.signal_sink,
            truth_branch_head_source: self.truth_branch_head_source,
            continuity_lineage_source: self.continuity_lineage_source,
            snapshot_reader_pool: self.snapshot_reader_pool,
            diagnostic_sink: self.diagnostic_sink,
            mapping_registrations: self.mapping_registrations,
            aspect_registrations: self.aspect_registrations,
        }
    }
}

impl<PatchState, SignalState, BranchHeadState, MappingState>
    RuntimeBridgeBuilder<PatchState, MissingSnapshotReadSource, SignalState, BranchHeadState, MappingState>
{
    pub fn with_snapshot_read_source<S>(
        self,
        source: S,
    ) -> RuntimeBridgeBuilder<PatchState, PresentSnapshotReadSource, SignalState, BranchHeadState, MappingState>
    where
        S: SnapshotReadSource,
    {
        RuntimeBridgeBuilder {
            policy: self.policy,
            committed_patch_source: self.committed_patch_source,
            snapshot_read_source: PresentSnapshotReadSource(Arc::new(source)),
            signal_sink: self.signal_sink,
            truth_branch_head_source: self.truth_branch_head_source,
            continuity_lineage_source: self.continuity_lineage_source,
            snapshot_reader_pool: self.snapshot_reader_pool,
            diagnostic_sink: self.diagnostic_sink,
            mapping_registrations: self.mapping_registrations,
            aspect_registrations: self.aspect_registrations,
        }
    }
}

impl<SignalState, MappingState>
    RuntimeBridgeBuilder<
        MissingCommittedPatchSource,
        MissingSnapshotReadSource,
        SignalState,
        MissingTruthBranchHeadSource,
        MappingState,
    >
{
    pub fn with_relational_source<S>(
        self,
        source: S,
    ) -> RuntimeBridgeBuilder<
        PresentCommittedPatchSource,
        PresentSnapshotReadSource,
        SignalState,
        MissingTruthBranchHeadSource,
        MappingState,
    >
    where
        S: CommittedPatchSource + SnapshotReadSource,
    {
        let source = Arc::new(source);
        RuntimeBridgeBuilder {
            policy: self.policy,
            committed_patch_source: PresentCommittedPatchSource(source.clone()),
            snapshot_read_source: PresentSnapshotReadSource(source.clone()),
            signal_sink: self.signal_sink,
            truth_branch_head_source: MissingTruthBranchHeadSource,
            continuity_lineage_source: self.continuity_lineage_source,
            snapshot_reader_pool: self.snapshot_reader_pool,
            diagnostic_sink: self.diagnostic_sink,
            mapping_registrations: self.mapping_registrations,
            aspect_registrations: self.aspect_registrations,
        }
    }
}

impl<PatchState, SnapshotState, BranchHeadState, MappingState>
    RuntimeBridgeBuilder<PatchState, SnapshotState, MissingSignalSink, BranchHeadState, MappingState>
{
    pub fn with_signal_sink<S>(
        self,
        sink: S,
    ) -> RuntimeBridgeBuilder<PatchState, SnapshotState, PresentSignalSink, BranchHeadState, MappingState>
    where
        S: InvalidationSink,
    {
        RuntimeBridgeBuilder {
            policy: self.policy,
            committed_patch_source: self.committed_patch_source,
            snapshot_read_source: self.snapshot_read_source,
            signal_sink: PresentSignalSink(Arc::new(sink)),
            truth_branch_head_source: self.truth_branch_head_source,
            continuity_lineage_source: self.continuity_lineage_source,
            snapshot_reader_pool: self.snapshot_reader_pool,
            diagnostic_sink: self.diagnostic_sink,
            mapping_registrations: self.mapping_registrations,
            aspect_registrations: self.aspect_registrations,
        }
    }
}

impl<PatchState, SnapshotState, SignalState, BranchHeadState, MappingState>
    RuntimeBridgeBuilder<PatchState, SnapshotState, SignalState, BranchHeadState, MappingState>
{
    pub fn with_policy(mut self, policy: BridgeRuntimePolicy) -> Self {
        self.policy = policy;
        self
    }

    pub fn with_snapshot_reader_pool<P>(mut self, pool: P) -> Self
    where
        P: SnapshotReaderPool,
    {
        self.snapshot_reader_pool = Some(Arc::new(pool));
        self
    }

    pub fn with_continuity_lineage_source<S>(mut self, source: S) -> Self
    where
        S: ContinuityLineageSource,
    {
        self.continuity_lineage_source = Some(Arc::new(source));
        self
    }

    pub fn with_truth_branch_head_source<S>(
        self,
        source: S,
    ) -> RuntimeBridgeBuilder<PatchState, SnapshotState, SignalState, PresentTruthBranchHeadSource, MappingState>
    where
        S: TruthBranchHeadSource,
    {
        RuntimeBridgeBuilder {
            policy: self.policy,
            committed_patch_source: self.committed_patch_source,
            snapshot_read_source: self.snapshot_read_source,
            signal_sink: self.signal_sink,
            truth_branch_head_source: PresentTruthBranchHeadSource(Arc::new(source)),
            continuity_lineage_source: self.continuity_lineage_source,
            snapshot_reader_pool: self.snapshot_reader_pool,
            diagnostic_sink: self.diagnostic_sink,
            mapping_registrations: self.mapping_registrations,
            aspect_registrations: self.aspect_registrations,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn with_diagnostics_sink(mut self, sink: Arc<dyn DiagnosticSink>) -> Self {
        self.diagnostic_sink = Some(sink);
        self
    }

    pub fn register_aspect_mapping(mut self, registration: BridgeAspectRegistration) -> Self {
        self.aspect_registrations.push(registration);
        self
    }
}

impl<PatchState, SnapshotState, SignalState, BranchHeadState>
    RuntimeBridgeBuilder<PatchState, SnapshotState, SignalState, BranchHeadState, MissingMappingRegistrations>
{
    pub fn register_mapping(
        self,
        registration: BridgeMappingRegistration,
    ) -> RuntimeBridgeBuilder<PatchState, SnapshotState, SignalState, BranchHeadState, PresentMappingRegistrations> {
        RuntimeBridgeBuilder {
            policy: self.policy,
            committed_patch_source: self.committed_patch_source,
            snapshot_read_source: self.snapshot_read_source,
            signal_sink: self.signal_sink,
            truth_branch_head_source: self.truth_branch_head_source,
            continuity_lineage_source: self.continuity_lineage_source,
            snapshot_reader_pool: self.snapshot_reader_pool,
            diagnostic_sink: self.diagnostic_sink,
            mapping_registrations: PresentMappingRegistrations(vec![registration]),
            aspect_registrations: self.aspect_registrations,
        }
    }
}

impl<PatchState, SnapshotState, SignalState, BranchHeadState>
    RuntimeBridgeBuilder<PatchState, SnapshotState, SignalState, BranchHeadState, PresentMappingRegistrations>
{
    pub fn register_mapping(mut self, registration: BridgeMappingRegistration) -> Self {
        self.mapping_registrations.0.push(registration);
        self
    }
}

impl RuntimeBridgeBuilder<
    PresentCommittedPatchSource,
    PresentSnapshotReadSource,
    PresentSignalSink,
    MissingTruthBranchHeadSource,
    PresentMappingRegistrations,
> {
    pub fn build(self) -> Result<RuntimeBridge, BridgeBuildError> {
        let mapping_registry = FrozenMappingRegistry::freeze(self.mapping_registrations.0)?;
        let aspect_registry = FrozenAspectMappingRegistry::freeze(self.aspect_registrations)?;
        Ok(RuntimeBridge::new(
            self.policy,
            self.committed_patch_source.0,
            self.snapshot_read_source.0,
            self.signal_sink.0,
            None,
            self.continuity_lineage_source,
            self.snapshot_reader_pool,
            self.diagnostic_sink,
            mapping_registry,
            aspect_registry,
        ))
    }
}

impl RuntimeBridgeBuilder<
    PresentCommittedPatchSource,
    PresentSnapshotReadSource,
    PresentSignalSink,
    PresentTruthBranchHeadSource,
    PresentMappingRegistrations,
> {
    pub fn build(self) -> Result<RuntimeBridge, BridgeBuildError> {
        let mapping_registry = FrozenMappingRegistry::freeze(self.mapping_registrations.0)?;
        let aspect_registry = FrozenAspectMappingRegistry::freeze(self.aspect_registrations)?;
        Ok(RuntimeBridge::new(
            self.policy,
            self.committed_patch_source.0,
            self.snapshot_read_source.0,
            self.signal_sink.0,
            Some(self.truth_branch_head_source.0),
            self.continuity_lineage_source,
            self.snapshot_reader_pool,
            self.diagnostic_sink,
            mapping_registry,
            aspect_registry,
        ))
    }
}

impl<PatchState, SnapshotState, SignalState, BranchHeadState, MappingState> std::fmt::Debug
    for RuntimeBridgeBuilder<PatchState, SnapshotState, SignalState, BranchHeadState, MappingState>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RuntimeBridgeBuilder")
            .field("policy", &self.policy)
            .field(
                "aspect_registration_count",
                &self.aspect_registrations.len(),
            )
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::adapter::{
        CommittedPatchSource, InvalidationSink, RelationalBridgeSourceError,
        SignalBridgeSinkError,
        SnapshotReadSource, TruthBranchHeadSource,
    };
    use crate::adapter::{
        BridgeHistoricalLineageAuthority, BridgeHistoricalLineageRequest, ContinuityLineageSource,
    };
    use crate::continuity::BridgeContinuityAuthorityBasis;
    use crate::error::{BridgeLineageSourceError, BridgeLineageSourceErrorKind};
    use crate::delivery::BridgeDeliveryReceipt;
    use crate::diagnostics::BridgeDiagnosticsFacade;
    use crate::facade::RuntimeBridgeBuilder;
    use crate::input::envelope::RawCommittedPatchEnvelope;
    use crate::mapping::{
        BridgeAspectRegistration, BridgeAspectRegistrationId, BridgeMappingId,
        BridgeMappingRegistration, CoarseRoutingMode, MappingSelector, SignalInvalidationScope,
        SliceFallbackPolicy, SubscriptionSliceKind, TruthDeltaSurfaceKind, TruthPatchScope,
    };
    use crate::snapshot::{
        BridgeSnapshotReadError, SnapshotReadPacket, SnapshotReadPacketResult, TruthSnapshotIdentity,
        TruthSnapshotReader,
    };

    struct TestSource;

    impl CommittedPatchSource for TestSource {
        fn load_committed_patch(
            &self,
            _request: crate::adapter::RelationalCommittedPatchRequest,
        ) -> Result<RawCommittedPatchEnvelope, RelationalBridgeSourceError> {
            unreachable!("builder tests do not load committed patch parts")
        }

    }

    impl SnapshotReadSource for TestSource {
        fn open_snapshot(
            &self,
            _identity: &TruthSnapshotIdentity,
        ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError> {
            Ok(Box::new(TestSnapshotReader))
        }
    }

    impl TruthBranchHeadSource for TestSource {
        fn load_branch_head_patch(
            &self,
            branch_identity: &crate::input::envelope::TruthBranchIdentity,
        ) -> Result<RawCommittedPatchEnvelope, RelationalBridgeSourceError> {
            Ok(RawCommittedPatchEnvelope::new(
                crate::input::envelope::TruthCommitIdentity::new(format!(
                    "head-{}",
                    branch_identity.as_str()
                )),
                crate::input::envelope::TruthPatchIdentity::new(format!(
                    "patch-{}",
                    branch_identity.as_str()
                )),
                TruthSnapshotIdentity::new("snapshot"),
                branch_identity.clone(),
                vec![],
            ))
        }
    }

    struct TestSnapshotReader;

    impl TruthSnapshotReader for TestSnapshotReader {
        fn snapshot_identity(&self) -> TruthSnapshotIdentity {
            TruthSnapshotIdentity::new("snapshot")
        }

        fn read_packet(
            &self,
            _request: &SnapshotReadPacket,
        ) -> Result<SnapshotReadPacketResult, BridgeSnapshotReadError> {
            unreachable!("builder tests do not read snapshots")
        }
    }

    struct TestSink;

    struct TestLineageSource;

    impl InvalidationSink for TestSink {
        fn deliver_invalidation(
            &self,
            delivery: crate::routing::BridgeSignalInvalidationDelivery,
        ) -> Result<BridgeDeliveryReceipt, SignalBridgeSinkError> {
            Ok(BridgeDeliveryReceipt::new(
                delivery.invalidation_targets().len(),
                delivery.source_snapshot().clone(),
            ))
        }
    }

    impl ContinuityLineageSource for TestLineageSource {
        fn historical_lineage(
            &self,
            request: BridgeHistoricalLineageRequest,
        ) -> Result<BridgeHistoricalLineageAuthority, BridgeLineageSourceError> {
            BridgeHistoricalLineageAuthority::try_new(
                request.authority_basis().clone(),
                vec![std::sync::Arc::from("lineage:test")],
                vec![std::sync::Arc::from("entity:test")],
                vec![],
            )
        }
    }

    struct TestUnsupportedLineageSource;

    impl ContinuityLineageSource for TestUnsupportedLineageSource {
        fn historical_lineage(
            &self,
            _request: BridgeHistoricalLineageRequest,
        ) -> Result<BridgeHistoricalLineageAuthority, BridgeLineageSourceError> {
            Err(BridgeLineageSourceError::new(
                BridgeLineageSourceErrorKind::UnsupportedContinuityClass,
                "unsupported continuity class",
            ))
        }
    }

    fn exact_registration(mapping_id: &str) -> BridgeMappingRegistration {
        BridgeMappingRegistration::new(
            BridgeMappingId::new(mapping_id),
            TruthPatchScope::new(
                MappingSelector::exact("user"),
                MappingSelector::exact("profile"),
                MappingSelector::exact("name"),
            ),
            SignalInvalidationScope::new("signal.user.profile"),
            CoarseRoutingMode::Direct,
        )
    }

    fn exact_aspect_registration(registration_id: &str) -> BridgeAspectRegistration {
        BridgeAspectRegistration::new(
            BridgeAspectRegistrationId::new(registration_id),
            TruthPatchScope::new(
                MappingSelector::exact("user"),
                MappingSelector::exact("profile"),
                MappingSelector::exact("name"),
            ),
            TruthDeltaSurfaceKind::EntityField,
            SubscriptionSliceKind::SignalField,
            SliceFallbackPolicy::Disallow,
        )
    }

    #[test]
    fn build_freezes_mapping_registry_before_runtime_construction() {
        let runtime = RuntimeBridgeBuilder::new()
            .with_relational_source(TestSource)
            .with_signal_sink(TestSink)
            .register_mapping(exact_registration("user-profile-name"))
            .register_aspect_mapping(exact_aspect_registration("user-profile-name-field"))
            .build()
            .expect("builder should freeze mapping registrations");

        assert_eq!(runtime.policy(), &crate::policy::BridgeRuntimePolicy::default());
    }

    #[test]
    fn build_accepts_custom_diagnostics_sink() {
        let diagnostics_sink = std::sync::Arc::new(BridgeDiagnosticsFacade::new(
            crate::policy::BridgeRuntimePolicy::default(),
        ));
        let runtime = RuntimeBridgeBuilder::new()
            .with_relational_source(TestSource)
            .with_signal_sink(TestSink)
            .with_diagnostics_sink(diagnostics_sink)
            .register_mapping(exact_registration("user-profile-name"))
            .build()
            .expect("builder should accept an injected diagnostics sink");

        assert_eq!(runtime.policy(), &crate::policy::BridgeRuntimePolicy::default());
    }

    #[test]
    fn build_accepts_optional_continuity_lineage_source() {
        let runtime = RuntimeBridgeBuilder::new()
            .with_relational_source(TestSource)
            .with_signal_sink(TestSink)
            .with_continuity_lineage_source(TestLineageSource)
            .register_mapping(exact_registration("user-profile-name"))
            .build()
            .expect("builder should accept continuity lineage source");

        let authority_basis = BridgeContinuityAuthorityBasis::new(
            crate::input::envelope::TruthBranchIdentity::new("main"),
            TruthSnapshotIdentity::new("snapshot"),
        );
        let source = runtime
            .continuity_lineage_source()
            .expect("continuity lineage source should be present");
        let authority = source
            .historical_lineage(BridgeHistoricalLineageRequest::new(
                authority_basis,
                crate::continuity::PriorSubscriptionSlice::from_parts(
                    crate::routing::BridgeSubscriptionSliceIdentity::new("slice:test"),
                    "entity:test",
                    "aspect:test",
                    "surface:test",
                    crate::mapping::SubscriptionSliceKind::SignalField,
                    crate::routing::FineGrainedMatchStatus::Matched,
                ),
            ))
            .expect("test lineage source should answer");

        assert!(authority.lineage_digest().starts_with("historical-lineage-authority:sha256:"));
    }

    #[test]
    fn continuity_lineage_source_can_return_typed_unsupported_class_failure() {
        let runtime = RuntimeBridgeBuilder::new()
            .with_relational_source(TestSource)
            .with_signal_sink(TestSink)
            .with_continuity_lineage_source(TestUnsupportedLineageSource)
            .register_mapping(exact_registration("user-profile-name"))
            .build()
            .expect("builder should accept continuity lineage source");

        let source = runtime
            .continuity_lineage_source()
            .expect("continuity lineage source should be present");
        let error = source
            .historical_lineage(BridgeHistoricalLineageRequest::new(
                BridgeContinuityAuthorityBasis::new(
                    crate::input::envelope::TruthBranchIdentity::new("main"),
                    TruthSnapshotIdentity::new("snapshot"),
                ),
                crate::continuity::PriorSubscriptionSlice::from_parts(
                    crate::routing::BridgeSubscriptionSliceIdentity::new("slice:test"),
                    "relation:test",
                    "aspect:test",
                    "surface:test",
                    crate::mapping::SubscriptionSliceKind::SignalField,
                    crate::routing::FineGrainedMatchStatus::Matched,
                ),
            ))
            .expect_err("unsupported continuity class should be typed");

        assert_eq!(
            error.kind(),
            BridgeLineageSourceErrorKind::UnsupportedContinuityClass
        );
    }
}
