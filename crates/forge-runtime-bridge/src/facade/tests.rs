use super::RuntimeBridge;
use crate::builder::RuntimeBridgeBuilder;
use crate::facade::BridgeHistoricalMaterializationPath;
use crate::input::envelope::{
    BridgeCommittedPatchEnvelope, BridgeCommittedPatchItem, BridgeProducerMetadata,
    RawCommittedPatchEnvelope, TruthBranchIdentity, TruthCommitIdentity, TruthPatchIdentity,
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
use crate::source::{
    BridgeSourceCapability, BridgeSourceCapabilitySet, SourceDeclaration, SourceDeclarationIdentity,
};
use crate::structural::{
    StructuralFingerprintEquivalenceContract, StructuralFingerprintFamily,
    StructuralFingerprintNormalizationRule, StructuralFingerprintOmissionPolicy,
    StructuralFingerprintOrderingRule, StructuralIdentityDeclaration,
    StructuralIdentityDeclarationIdentity, StructuralSchemaIdentity, StructuralTruthViewBasis,
};

#[derive(Clone)]
struct StaticSource;
impl crate::adapter::CommittedPatchSource for StaticSource {
    fn load_committed_patch(
        &self,
        request: crate::adapter::RelationalCommittedPatchRequest,
    ) -> Result<
        crate::input::envelope::RawCommittedPatchEnvelope,
        crate::adapter::RelationalBridgeSourceError,
    > {
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
    ) -> Result<crate::snapshot::SnapshotReadPacketResult, crate::snapshot::BridgeSnapshotReadError>
    {
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
    ) -> Result<
        Box<dyn crate::snapshot::TruthSnapshotReader>,
        crate::adapter::RelationalBridgeSourceError,
    > {
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
    ) -> Result<
        crate::input::envelope::RawCommittedPatchEnvelope,
        crate::adapter::RelationalBridgeSourceError,
    > {
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

#[derive(Clone)]
struct StaticSourceAdapter;

#[derive(Clone)]
struct RejectingSourceAdapter;

#[derive(Clone)]
struct DriftSourceAdapter;

#[derive(Clone)]
struct ReorderingSourceAdapter;

#[derive(Clone)]
struct DriftSnapshotReader;

impl crate::adapter::BridgeSourceAdapter for StaticSourceAdapter {
    fn declared_capabilities(&self) -> BridgeSourceCapabilitySet {
        BridgeSourceCapabilitySet::new(vec![
            BridgeSourceCapability::SnapshotRead,
            BridgeSourceCapability::HistoricalRead,
            BridgeSourceCapability::BranchRead,
            BridgeSourceCapability::ReplayCompatibleRead,
        ])
    }

    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<
        Box<dyn crate::snapshot::TruthSnapshotReader>,
        crate::adapter::RelationalBridgeSourceError,
    > {
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

impl crate::snapshot::TruthSnapshotReader for DriftSnapshotReader {
    fn snapshot_identity(&self) -> TruthSnapshotIdentity {
        TruthSnapshotIdentity::new("snapshot-bad")
    }

    fn read_packet(
        &self,
        request: &SnapshotReadPacket,
    ) -> Result<crate::snapshot::SnapshotReadPacketResult, crate::snapshot::BridgeSnapshotReadError>
    {
        StaticSnapshotReader.read_packet(request)
    }
}

impl crate::adapter::BridgeSourceAdapter for RejectingSourceAdapter {
    fn declared_capabilities(&self) -> BridgeSourceCapabilitySet {
        StaticSourceAdapter.declared_capabilities()
    }

    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<
        Box<dyn crate::snapshot::TruthSnapshotReader>,
        crate::adapter::RelationalBridgeSourceError,
    > {
        Err(crate::adapter::RelationalBridgeSourceError::new(format!(
            "refused snapshot `{}`",
            identity.as_str()
        )))
    }
}

impl crate::adapter::BridgeSourceAdapter for DriftSourceAdapter {
    fn declared_capabilities(&self) -> BridgeSourceCapabilitySet {
        StaticSourceAdapter.declared_capabilities()
    }

    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<
        Box<dyn crate::snapshot::TruthSnapshotReader>,
        crate::adapter::RelationalBridgeSourceError,
    > {
        if identity.as_str() == "snapshot-a" {
            Ok(Box::new(DriftSnapshotReader))
        } else {
            Err(crate::adapter::RelationalBridgeSourceError::new(format!(
                "unknown snapshot `{}`",
                identity.as_str()
            )))
        }
    }
}

impl crate::adapter::BridgeSourceAdapter for ReorderingSourceAdapter {
    fn declared_capabilities(&self) -> BridgeSourceCapabilitySet {
        StaticSourceAdapter.declared_capabilities()
    }

    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<
        Box<dyn crate::snapshot::TruthSnapshotReader>,
        crate::adapter::RelationalBridgeSourceError,
    > {
        StaticSourceAdapter.open_snapshot(identity)
    }

    fn materialize_packets(
        &self,
        planned_packet_set: &crate::source::PlannedSourceReadPacketSet,
    ) -> Result<crate::source::MaterializedTruthViewPacketSet, crate::error::BridgeDeliveryError>
    {
        let observations = planned_packet_set
            .packets()
            .iter()
            .rev()
            .cloned()
            .map(|planned| {
                <StaticSourceAdapter as crate::adapter::BridgeSourceAdapter>::materialize_packet(
                    &StaticSourceAdapter,
                    planned,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(crate::source::MaterializedTruthViewPacketSet::new(
            planned_packet_set.clone(),
            observations,
        ))
    }
}

fn registered_source(
    id: &str,
    selector: BridgeTruthViewSelector,
    capabilities: Vec<BridgeSourceCapability>,
) -> SourceDeclaration {
    SourceDeclaration::new(
        SourceDeclarationIdentity::new(id),
        selector,
        BridgeSourceCapabilitySet::new(capabilities),
    )
}

fn registered_structural(
    id: &str,
    family: StructuralFingerprintFamily,
    truth_view_basis: StructuralTruthViewBasis,
) -> StructuralIdentityDeclaration {
    StructuralIdentityDeclaration::advisory_remap(
        StructuralIdentityDeclarationIdentity::new(id),
        StructuralSchemaIdentity::new("schema:geometry"),
        StructuralFingerprintEquivalenceContract::new(
            StructuralSchemaIdentity::new("schema:geometry"),
            family,
            "geometry-v1",
            StructuralFingerprintNormalizationRule::SchemaDeclaredCanonicalForm,
            StructuralFingerprintOrderingRule::SchemaDeclaredCanonicalOrder,
            StructuralFingerprintOmissionPolicy::SchemaDeclaredOmissionPolicy,
        ),
        truth_view_basis,
    )
}

fn runtime(policy: BridgeRuntimePolicy) -> RuntimeBridge {
    runtime_with_source_adapter(policy, StaticSourceAdapter)
}

fn runtime_with_source_adapter<A>(policy: BridgeRuntimePolicy, source_adapter: A) -> RuntimeBridge
where
    A: crate::adapter::BridgeSourceAdapter,
{
    RuntimeBridgeBuilder::new()
        .with_policy(policy)
        .with_relational_source(StaticSource)
        .with_source_adapter(source_adapter)
        .with_truth_branch_head_source(StaticSource)
        .with_signal_sink(StaticSink)
        .register_source(registered_source(
            "source:analysis-snapshot",
            BridgeTruthViewSelector::branch_snapshot(
                TruthBranchIdentity::new("analysis"),
                TruthSnapshotIdentity::new("snapshot-a"),
            ),
            vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::BranchRead,
            ],
        ))
        .register_source(registered_source(
            "source:analysis-history",
            BridgeTruthViewSelector::historical_commit(
                TruthBranchIdentity::new("analysis"),
                TruthCommitIdentity::new("commit-a"),
            ),
            vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::HistoricalRead,
                BridgeSourceCapability::BranchRead,
                BridgeSourceCapability::ReplayCompatibleRead,
            ],
        ))
        .register_structural(registered_structural(
            "structural:analysis-snapshot",
            StructuralFingerprintFamily::TopologyFingerprint,
            StructuralTruthViewBasis::explicit_snapshot(BridgeTruthViewSelector::branch_snapshot(
                TruthBranchIdentity::new("analysis"),
                TruthSnapshotIdentity::new("snapshot-a"),
            )),
        ))
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

mod merge;
mod policy_and_materialization;
mod replay;
mod stream;
mod stream_protocol;
mod structural;
