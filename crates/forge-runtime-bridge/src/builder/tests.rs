use crate::adapter::{
    BridgeHistoricalLineageAuthority, BridgeHistoricalLineageRequest, BridgeSourceAdapter,
    CommittedPatchSource, ContinuityLineageSource, InvalidationSink, RelationalBridgeSourceError,
    SignalBridgeSinkError, SnapshotReadSource, TruthBranchHeadSource, TruthWritebackAuthority,
    TruthWritebackAuthorityError, TruthWritebackReceipt, TruthWritebackRequest,
};
use crate::continuity::BridgeContinuityAuthorityBasis;
use crate::delivery::BridgeDeliveryReceipt;
use crate::diagnostics::BridgeDiagnosticsFacade;
use crate::error::{BridgeBuildErrorKind, BridgeLineageSourceError, BridgeLineageSourceErrorKind};
use crate::facade::RuntimeBridgeBuilder;
use crate::input::envelope::RawCommittedPatchEnvelope;
use crate::mapping::{
    BridgeAspectRegistration, BridgeAspectRegistrationId, BridgeMappingId,
    BridgeMappingRegistration, CoarseRoutingMode, MappingSelector, SignalInvalidationScope,
    SliceFallbackPolicy, SubscriptionSliceKind, TruthDeltaSurfaceKind, TruthPatchScope,
};
use crate::policy::{
    BridgeArtifactPolicyBaseline, BridgeDiagnosticsPolicyBaseline, BridgeDiagnosticsTier,
    BridgeExecutionPolicyBaseline, BridgeExecutionPolicyClass, BridgeRuntimePolicy,
    BridgeRuntimePosture,
};
use crate::snapshot::{
    BridgeSnapshotReadError, BridgeTruthViewSelector, SnapshotReadPacket, SnapshotReadPacketResult,
    TruthSnapshotIdentity, TruthSnapshotReader,
};
use crate::source::{
    BridgeSourceCapability, BridgeSourceCapabilitySet, SourceDeclaration, SourceDeclarationIdentity,
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
struct TestSourceAdapter {
    capabilities: BridgeSourceCapabilitySet,
}
struct TestWritebackAuthority;

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

impl BridgeSourceAdapter for TestSourceAdapter {
    fn declared_capabilities(&self) -> BridgeSourceCapabilitySet {
        self.capabilities.clone()
    }

    fn open_snapshot(
        &self,
        _identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError> {
        unreachable!("builder tests do not materialize source snapshots")
    }
}

impl TruthWritebackAuthority for TestWritebackAuthority {
    fn execute_writeback(
        &self,
        request: TruthWritebackRequest,
    ) -> Result<TruthWritebackReceipt, TruthWritebackAuthorityError> {
        Ok(TruthWritebackReceipt::new(
            crate::facade::BridgeWritebackOutcomeClass::AuthoritativeCommit,
            format!("authoritative-artifact:{}", request.digest()),
            &request,
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

fn source_declaration(
    declaration_id: &str,
    snapshot_id: &str,
    capabilities: Vec<BridgeSourceCapability>,
) -> SourceDeclaration {
    SourceDeclaration::new(
        SourceDeclarationIdentity::new(declaration_id),
        BridgeTruthViewSelector::committed_snapshot(
            crate::input::envelope::TruthBranchIdentity::new("main"),
            TruthSnapshotIdentity::new(snapshot_id),
        ),
        BridgeSourceCapabilitySet::new(capabilities),
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

    assert_eq!(
        runtime.policy(),
        &crate::policy::BridgeRuntimePolicy::default()
    );
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

    assert_eq!(
        runtime.policy(),
        &crate::policy::BridgeRuntimePolicy::default()
    );
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

    assert!(authority
        .lineage_digest()
        .starts_with("historical-lineage-authority:sha256:"));
}

#[test]
fn build_accepts_optional_writeback_authority() {
    let runtime = RuntimeBridgeBuilder::new()
        .with_relational_source(TestSource)
        .with_signal_sink(TestSink)
        .with_writeback_authority(TestWritebackAuthority)
        .register_mapping(exact_registration("user-profile-name"))
        .build()
        .expect("builder should accept optional writeback authority");

    assert!(runtime.writeback_authority().is_some());
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

#[test]
fn build_rejects_duplicate_source_declarations() {
    let error = RuntimeBridgeBuilder::new()
        .with_relational_source(TestSource)
        .with_signal_sink(TestSink)
        .with_source_adapter(TestSourceAdapter {
            capabilities: BridgeSourceCapabilitySet::new(vec![
                BridgeSourceCapability::SnapshotRead,
            ]),
        })
        .register_source(source_declaration(
            "source:profile",
            "snapshot-a",
            vec![BridgeSourceCapability::SnapshotRead],
        ))
        .register_source(source_declaration(
            "source:profile",
            "snapshot-a",
            vec![BridgeSourceCapability::SnapshotRead],
        ))
        .register_mapping(exact_registration("user-profile-name"))
        .build()
        .expect_err("duplicate source declarations should fail");

    assert_eq!(
        error.kind(),
        BridgeBuildErrorKind::DuplicateSourceDeclaration
    );
}

#[test]
fn build_rejects_source_declarations_without_source_adapter() {
    let error = RuntimeBridgeBuilder::new()
        .with_relational_source(TestSource)
        .with_signal_sink(TestSink)
        .register_source(source_declaration(
            "source:profile",
            "snapshot-a",
            vec![BridgeSourceCapability::SnapshotRead],
        ))
        .register_mapping(exact_registration("user-profile-name"))
        .build()
        .expect_err("source declarations without source adapter should fail");

    assert_eq!(error.kind(), BridgeBuildErrorKind::MissingSourceAdapter);
}

#[test]
fn build_rejects_multiple_source_adapters() {
    let error = RuntimeBridgeBuilder::new()
        .with_relational_source(TestSource)
        .with_signal_sink(TestSink)
        .with_source_adapter(TestSourceAdapter {
            capabilities: BridgeSourceCapabilitySet::new(vec![
                BridgeSourceCapability::SnapshotRead,
            ]),
        })
        .with_source_adapter(TestSourceAdapter {
            capabilities: BridgeSourceCapabilitySet::new(vec![
                BridgeSourceCapability::SnapshotRead,
            ]),
        })
        .register_source(source_declaration(
            "source:profile",
            "snapshot-a",
            vec![BridgeSourceCapability::SnapshotRead],
        ))
        .register_mapping(exact_registration("user-profile-name"))
        .build()
        .expect_err("multiple source adapters should fail");

    assert_eq!(
        error.kind(),
        BridgeBuildErrorKind::BuilderConfigurationConflict
    );
}

#[test]
fn build_source_registry_digest_is_order_invariant() {
    let first = RuntimeBridgeBuilder::new()
        .with_relational_source(TestSource)
        .with_signal_sink(TestSink)
        .with_source_adapter(TestSourceAdapter {
            capabilities: BridgeSourceCapabilitySet::new(vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::HistoricalRead,
            ]),
        })
        .register_source(source_declaration(
            "source:a",
            "snapshot-a",
            vec![BridgeSourceCapability::SnapshotRead],
        ))
        .register_source(source_declaration(
            "source:b",
            "snapshot-b",
            vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::HistoricalRead,
            ],
        ))
        .register_mapping(exact_registration("user-profile-name"))
        .build()
        .expect("first builder order should succeed");

    let second = RuntimeBridgeBuilder::new()
        .with_relational_source(TestSource)
        .with_signal_sink(TestSink)
        .with_source_adapter(TestSourceAdapter {
            capabilities: BridgeSourceCapabilitySet::new(vec![
                BridgeSourceCapability::HistoricalRead,
                BridgeSourceCapability::SnapshotRead,
            ]),
        })
        .register_source(source_declaration(
            "source:b",
            "snapshot-b",
            vec![
                BridgeSourceCapability::HistoricalRead,
                BridgeSourceCapability::SnapshotRead,
            ],
        ))
        .register_source(source_declaration(
            "source:a",
            "snapshot-a",
            vec![BridgeSourceCapability::SnapshotRead],
        ))
        .register_mapping(exact_registration("user-profile-name"))
        .build()
        .expect("second builder order should succeed");

    assert_eq!(
        first.source_registry().digest(),
        second.source_registry().digest()
    );
}

#[test]
fn build_rejects_source_capability_mismatch_before_runtime_construction() {
    let error = RuntimeBridgeBuilder::new()
        .with_relational_source(TestSource)
        .with_signal_sink(TestSink)
        .with_source_adapter(TestSourceAdapter {
            capabilities: BridgeSourceCapabilitySet::new(vec![
                BridgeSourceCapability::SnapshotRead,
            ]),
        })
        .register_source(source_declaration(
            "source:profile-history",
            "snapshot-a",
            vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::HistoricalRead,
            ],
        ))
        .register_mapping(exact_registration("user-profile-name"))
        .build()
        .expect_err("unsupported source capability should fail before runtime construction");

    assert_eq!(error.kind(), BridgeBuildErrorKind::SourceCapabilityMismatch);
}

#[test]
fn build_accepts_policy_sections_without_losing_canonical_runtime_policy() {
    let runtime = RuntimeBridgeBuilder::new()
        .with_execution_policy_baseline(BridgeExecutionPolicyBaseline::new(
            BridgeExecutionPolicyClass::DeterministicCanonical,
            BridgeRuntimePosture::Development,
        ))
        .with_diagnostics_policy_baseline(
            BridgeDiagnosticsPolicyBaseline::for_tier(BridgeDiagnosticsTier::Standard)
                .with_route_record_limit(17)
                .with_failure_record_limit(9),
        )
        .with_artifact_policy_baseline(BridgeArtifactPolicyBaseline::new(true, false))
        .with_relational_source(TestSource)
        .with_signal_sink(TestSink)
        .register_mapping(exact_registration("user-profile-name"))
        .build()
        .expect("sectioned policy configuration should build");

    assert_eq!(
        runtime.policy(),
        &BridgeRuntimePolicy::from_sections(
            BridgeExecutionPolicyBaseline::new(
                BridgeExecutionPolicyClass::DeterministicCanonical,
                BridgeRuntimePosture::Development,
            ),
            BridgeDiagnosticsPolicyBaseline::for_tier(BridgeDiagnosticsTier::Standard)
                .with_route_record_limit(17)
                .with_failure_record_limit(9),
            BridgeArtifactPolicyBaseline::new(true, false),
        )
    );
}

#[test]
fn build_policy_sections_are_order_invariant() {
    let first = RuntimeBridgeBuilder::new()
        .with_execution_policy_baseline(BridgeExecutionPolicyBaseline::new(
            BridgeExecutionPolicyClass::DeterministicCanonical,
            BridgeRuntimePosture::Development,
        ))
        .with_diagnostics_policy_baseline(
            BridgeDiagnosticsPolicyBaseline::for_tier(BridgeDiagnosticsTier::Standard)
                .with_route_record_limit(23)
                .with_failure_record_limit(11),
        )
        .with_artifact_policy_baseline(BridgeArtifactPolicyBaseline::new(true, false))
        .with_relational_source(TestSource)
        .with_signal_sink(TestSink)
        .register_mapping(exact_registration("user-profile-name"))
        .build()
        .expect("first policy order should build");

    let second = RuntimeBridgeBuilder::new()
        .with_artifact_policy_baseline(BridgeArtifactPolicyBaseline::new(true, false))
        .with_diagnostics_policy_baseline(
            BridgeDiagnosticsPolicyBaseline::for_tier(BridgeDiagnosticsTier::Standard)
                .with_failure_record_limit(11)
                .with_route_record_limit(23),
        )
        .with_execution_policy_baseline(BridgeExecutionPolicyBaseline::new(
            BridgeExecutionPolicyClass::DeterministicCanonical,
            BridgeRuntimePosture::Development,
        ))
        .with_relational_source(TestSource)
        .with_signal_sink(TestSink)
        .register_mapping(exact_registration("user-profile-name"))
        .build()
        .expect("second policy order should build");

    assert_eq!(first.policy(), second.policy());
}
