use crate::adapter::{
    BridgeHistoricalLineageAuthority, BridgeHistoricalLineageRequest, CommittedPatchSource,
    ContinuityLineageSource, InvalidationSink, RelationalBridgeSourceError,
    SignalBridgeSinkError, SnapshotReadSource, TruthBranchHeadSource,
};
use crate::continuity::BridgeContinuityAuthorityBasis;
use crate::delivery::BridgeDeliveryReceipt;
use crate::diagnostics::BridgeDiagnosticsFacade;
use crate::error::{BridgeLineageSourceError, BridgeLineageSourceErrorKind};
use crate::facade::RuntimeBridgeBuilder;
use crate::input::envelope::RawCommittedPatchEnvelope;
use crate::mapping::{
    BridgeAspectRegistration, BridgeAspectRegistrationId, BridgeMappingId, BridgeMappingRegistration,
    CoarseRoutingMode, MappingSelector, SignalInvalidationScope, SliceFallbackPolicy,
    SubscriptionSliceKind, TruthDeltaSurfaceKind, TruthPatchScope,
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
