    use super::RuntimeBridge;
    use crate::builder::RuntimeBridgeBuilder;
    use crate::input::envelope::{
        BridgeCommittedPatchEnvelope, BridgeCommittedPatchItem, BridgeProducerMetadata,
        RawCommittedPatchEnvelope, TruthBranchIdentity, TruthCommitIdentity,
        TruthPatchIdentity,
    };
    use crate::mapping::{
        BridgeMappingId, BridgeMappingRegistration, CoarseRoutingMode, MappingSelector,
        SignalInvalidationScope, TruthPatchScope,
    };
    use crate::policy::{BridgeDiagnosticsTier, BridgeRuntimePolicy};
    use crate::snapshot::{
        BridgeDeliveryIntent, BridgeReplayMode, BridgeTruthViewPolicyResolution,
        BridgeTruthViewSelector, HistoricalEvaluationDeclaration, SnapshotReadPacket,
        TruthSnapshotIdentity,
    };
    use crate::facade::BridgeHistoricalMaterializationPath;

    #[derive(Clone)]
    struct StaticSource;
    impl crate::adapter::CommittedPatchSource for StaticSource {
        fn load_committed_patch(
            &self,
            request: crate::adapter::RelationalCommittedPatchRequest,
        ) -> Result<crate::input::envelope::RawCommittedPatchEnvelope, crate::adapter::RelationalBridgeSourceError> {
            Ok(crate::input::envelope::RawCommittedPatchEnvelope::new(
                crate::input::envelope::TruthCommitIdentity::new(request.commit_identity()),
                crate::input::envelope::TruthPatchIdentity::new(format!(
                    "patch-for-{}",
                    request.commit_identity()
                )),
                TruthSnapshotIdentity::new("snapshot-a"),
                TruthBranchIdentity::new("analysis"),
                vec![],
            ))
        }
    }

    #[derive(Clone)]
    struct StaticSnapshotReader;
    impl crate::snapshot::TruthSnapshotReader for StaticSnapshotReader {
        fn snapshot_identity(&self) -> TruthSnapshotIdentity {
            TruthSnapshotIdentity::new("snapshot-a")
        }

        fn read_packet(
            &self,
            request: &SnapshotReadPacket,
        ) -> Result<crate::snapshot::SnapshotReadPacketResult, crate::snapshot::BridgeSnapshotReadError> {
            Ok(crate::snapshot::SnapshotReadPacketResult::new(
                TruthSnapshotIdentity::new("snapshot-a"),
                request
                    .reads()
                    .iter()
                    .map(|read| {
                        crate::snapshot::SnapshotReadRecord::new(
                            read.request_key(),
                            b"fixture-value".to_vec(),
                        )
                    })
                    .collect(),
            ))
        }
    }

    impl crate::adapter::SnapshotReadSource for StaticSource {
        fn open_snapshot(
            &self,
            identity: &TruthSnapshotIdentity,
        ) -> Result<Box<dyn crate::snapshot::TruthSnapshotReader>, crate::adapter::RelationalBridgeSourceError> {
            if identity.as_str() == "snapshot-a" {
                Ok(Box::new(StaticSnapshotReader))
            } else {
                Err(crate::adapter::RelationalBridgeSourceError::new(format!(
                    "unknown snapshot `{}`",
                    identity.as_str()
                )))
            }
        }
    }

    impl crate::adapter::TruthBranchHeadSource for StaticSource {
        fn load_branch_head_patch(
            &self,
            branch_identity: &TruthBranchIdentity,
        ) -> Result<crate::input::envelope::RawCommittedPatchEnvelope, crate::adapter::RelationalBridgeSourceError> {
            Ok(crate::input::envelope::RawCommittedPatchEnvelope::new(
                crate::input::envelope::TruthCommitIdentity::new(format!(
                    "head-{}",
                    branch_identity.as_str()
                )),
                crate::input::envelope::TruthPatchIdentity::new(format!(
                    "patch-{}",
                    branch_identity.as_str()
                )),
                TruthSnapshotIdentity::new("snapshot-a"),
                branch_identity.clone(),
                vec![],
            ))
        }
    }

    struct StaticSink;
    impl crate::adapter::InvalidationSink for StaticSink {
        fn deliver_invalidation(
            &self,
            delivery: crate::routing::BridgeSignalInvalidationDelivery,
        ) -> Result<crate::delivery::BridgeDeliveryReceipt, crate::adapter::SignalBridgeSinkError> {
            Ok(crate::delivery::BridgeDeliveryReceipt::new(
                delivery.invalidation_targets().len(),
                delivery.source_snapshot().clone(),
            ))
        }
    }

    fn runtime(policy: BridgeRuntimePolicy) -> RuntimeBridge {
        RuntimeBridgeBuilder::new()
            .with_policy(policy)
            .with_relational_source(StaticSource)
            .with_truth_branch_head_source(StaticSource)
            .with_signal_sink(StaticSink)
            .register_mapping(BridgeMappingRegistration::new(
                BridgeMappingId::new("mapping"),
                TruthPatchScope::new(
                    MappingSelector::exact("entity-1"),
                    MappingSelector::exact("profile"),
                    MappingSelector::exact("name"),
                ),
                SignalInvalidationScope::new("signal:profile"),
                CoarseRoutingMode::Direct,
            ))
            .build()
            .expect("runtime should build for policy-resolution tests")
    }

    fn canonical_envelope(
        branch: &str,
        commit: &str,
        patch: &str,
        snapshot: &str,
    ) -> BridgeCommittedPatchEnvelope {
        let raw = RawCommittedPatchEnvelope::new_with_metadata(
            BridgeProducerMetadata::bridge_harness_fixture(),
            TruthCommitIdentity::new(commit),
            TruthPatchIdentity::new(patch),
            TruthSnapshotIdentity::new(snapshot),
            TruthBranchIdentity::new(branch),
            vec![BridgeCommittedPatchItem::new("entity-1", "profile", "name")],
        );
        let normalized = crate::input::normalization::normalize_raw_envelope(raw);
        crate::input::validation::validate_normalized_envelope(normalized)
            .expect("fixture envelopes should validate")
    }

mod policy_and_materialization;
mod replay;
mod stream;
mod stream_protocol;
