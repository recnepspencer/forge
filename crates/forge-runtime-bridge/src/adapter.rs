//! Bridge-owned parent-runtime adapter contracts.

use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::delivery::BridgeDeliveryReceipt;
use crate::error::{
    BridgeDeliveryError, BridgeDeliveryErrorKind, BridgeErrorContext, BridgeLineageSourceError,
    BridgeLineageSourceErrorKind, BridgeMessageError,
};
use crate::input::envelope::{RawCommittedPatchEnvelope, TruthBranchIdentity};
use crate::routing::BridgeSignalInvalidationDelivery;
use crate::snapshot::{
    AdmittedSnapshotContext, BridgeSnapshotContext, BridgeSnapshotToken, BridgeTruthViewKind,
    MaterializedTruthViewObservation, TruthSnapshotIdentity, TruthSnapshotReader,
};
use crate::source::{
    BridgeSourceCapabilitySet, MaterializedTruthViewPacketSet, PlannedSourceReadPacketSet,
};
use crate::{continuity, input};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelationalBridgeSourceErrorTag {}
pub type RelationalBridgeSourceError = BridgeMessageError<RelationalBridgeSourceErrorTag>;

pub trait CommittedPatchSource: Send + Sync + 'static {
    fn load_committed_patch(
        &self,
        request: RelationalCommittedPatchRequest,
    ) -> Result<RawCommittedPatchEnvelope, RelationalBridgeSourceError>;
}

pub trait SnapshotReadSource: Send + Sync + 'static {
    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError>;
}

pub trait SnapshotReaderPool: Send + Sync + 'static {
    fn acquire(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError>;

    fn release(&self, reader: Box<dyn TruthSnapshotReader>);
}

pub trait TruthBranchHeadSource: Send + Sync + 'static {
    fn load_branch_head_patch(
        &self,
        branch_identity: &TruthBranchIdentity,
    ) -> Result<RawCommittedPatchEnvelope, RelationalBridgeSourceError>;
}

pub trait RelationalBridgeSource:
    CommittedPatchSource + SnapshotReadSource + TruthBranchHeadSource
{
}

impl<T> RelationalBridgeSource for T where
    T: CommittedPatchSource + SnapshotReadSource + TruthBranchHeadSource
{
}

pub trait BridgeSourceAdapter: Send + Sync + 'static {
    fn declared_capabilities(&self) -> BridgeSourceCapabilitySet;

    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError>;

    fn materialize_packet(
        &self,
        planned: crate::snapshot::PlannedTruthViewPacket,
    ) -> Result<MaterializedTruthViewObservation, BridgeDeliveryError> {
        let snapshot_identity = planned
            .authority_basis()
            .snapshot_identity()
            .cloned()
            .ok_or_else(|| {
                BridgeDeliveryError::new(
                    BridgeDeliveryErrorKind::HistoricalPolicyRejected,
                    format!(
                        "Planned source packet `{}` has no resolved snapshot authority.",
                        planned.digest()
                    ),
                )
            })?;

        let snapshot_reader = self.open_snapshot(&snapshot_identity).map_err(|error| {
            BridgeDeliveryError::new(
                BridgeDeliveryErrorKind::SnapshotAcquisitionFailure,
                format!(
                    "Bridge source adapter failed to open snapshot `{}`: {error}",
                    snapshot_identity.as_str()
                ),
            )
            .with_context(BridgeErrorContext::snapshot(snapshot_identity.clone()))
        })?;
        let snapshot = BridgeSnapshotContext::bind(snapshot_reader);
        let admitted = AdmittedSnapshotContext::admit_for(snapshot, &snapshot_identity).map_err(
            |bound_snapshot_identity| {
                BridgeDeliveryError::new(
                    BridgeDeliveryErrorKind::SnapshotIdentityMismatch,
                    format!(
                        "Source adapter bound snapshot `{}` but planned source packet required `{}`.",
                        bound_snapshot_identity.as_str(),
                        snapshot_identity.as_str()
                    ),
                )
                .with_context(BridgeErrorContext::snapshot(snapshot_identity.clone()))
            },
        )?;
        let snapshot_token = BridgeSnapshotToken::issued(
            snapshot_identity.clone(),
            format!(
                "source-truth-view-observation|planned={}|snapshot={}",
                planned.digest(),
                snapshot_identity.as_str()
            ),
        );

        Ok(MaterializedTruthViewObservation::new(
            planned.clone(),
            snapshot_token,
            source_materialization_path_for(&planned),
            admitted,
        ))
    }

    fn materialize_packets(
        &self,
        planned_packet_set: &PlannedSourceReadPacketSet,
    ) -> Result<MaterializedTruthViewPacketSet, BridgeDeliveryError> {
        let observations = planned_packet_set
            .packets()
            .iter()
            .cloned()
            .map(|planned| self.materialize_packet(planned))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(MaterializedTruthViewPacketSet::new(
            planned_packet_set.clone(),
            observations,
        ))
    }
}

fn source_materialization_path_for(
    planned: &crate::snapshot::PlannedTruthViewPacket,
) -> crate::diagnostics::BridgeHistoricalMaterializationPath {
    match planned.declaration().selector().view_kind() {
        BridgeTruthViewKind::CommittedSnapshot | BridgeTruthViewKind::BranchSnapshot => {
            crate::diagnostics::BridgeHistoricalMaterializationPath::DirectSnapshotRead
        }
        BridgeTruthViewKind::HistoricalCommit | BridgeTruthViewKind::BranchCommit => {
            crate::diagnostics::BridgeHistoricalMaterializationPath::CommitEnvelopeSnapshot
        }
        BridgeTruthViewKind::BranchHead => {
            crate::diagnostics::BridgeHistoricalMaterializationPath::BranchHeadEnvelopeSnapshot
        }
    }
}

pub trait ContinuityLineageSource: Send + Sync + 'static {
    fn historical_lineage(
        &self,
        request: BridgeHistoricalLineageRequest,
    ) -> Result<BridgeHistoricalLineageAuthority, BridgeLineageSourceError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeHistoricalLineageTopology {
    NoAuthoritativeSuccessor,
    UnsupportedWithoutSuccessor,
    SingleSuccessor,
    MergeLikeSuccessor,
    SplitSuccessors,
    AmbiguousSuccessor,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationalCommittedPatchRequest {
    commit_identity: Arc<str>,
}

impl RelationalCommittedPatchRequest {
    pub fn new(commit_identity: impl Into<Arc<str>>) -> Self {
        Self {
            commit_identity: commit_identity.into(),
        }
    }

    pub fn commit_identity(&self) -> &str {
        self.commit_identity.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignalBridgeSinkErrorTag {}
pub type SignalBridgeSinkError = BridgeMessageError<SignalBridgeSinkErrorTag>;

pub trait InvalidationSink: Send + Sync + 'static {
    fn deliver_invalidation(
        &self,
        delivery: BridgeSignalInvalidationDelivery,
    ) -> Result<BridgeDeliveryReceipt, SignalBridgeSinkError>;
}

pub trait SignalBridgeSink: InvalidationSink {}

impl<T> SignalBridgeSink for T where T: InvalidationSink {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeHistoricalLineageRequest {
    authority_basis: continuity::BridgeContinuityAuthorityBasis,
    prior_slice: continuity::PriorSubscriptionSlice,
}

impl BridgeHistoricalLineageRequest {
    pub fn new(
        authority_basis: continuity::BridgeContinuityAuthorityBasis,
        prior_slice: continuity::PriorSubscriptionSlice,
    ) -> Self {
        Self {
            authority_basis,
            prior_slice,
        }
    }

    pub fn authority_basis(&self) -> &continuity::BridgeContinuityAuthorityBasis {
        &self.authority_basis
    }

    pub fn prior_slice(&self) -> &continuity::PriorSubscriptionSlice {
        &self.prior_slice
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeHistoricalLineageAuthority {
    authority_basis: continuity::BridgeContinuityAuthorityBasis,
    canonical_resolved_lineage_keys: Arc<[Arc<str>]>,
    canonical_resolved_record_keys: Arc<[Arc<str>]>,
    traversed_event_ids: Arc<[u64]>,
    topology: BridgeHistoricalLineageTopology,
    lineage_digest: Arc<str>,
}

impl BridgeHistoricalLineageAuthority {
    pub fn try_new(
        authority_basis: continuity::BridgeContinuityAuthorityBasis,
        canonical_resolved_lineage_keys: Vec<Arc<str>>,
        canonical_resolved_record_keys: Vec<Arc<str>>,
        traversed_event_ids: Vec<u64>,
    ) -> Result<Self, BridgeLineageSourceError> {
        let mut sorted_lineage_keys = canonical_resolved_lineage_keys.clone();
        sorted_lineage_keys.sort_unstable();
        sorted_lineage_keys.dedup();
        if sorted_lineage_keys.len() != canonical_resolved_lineage_keys.len() {
            return Err(BridgeLineageSourceError::new(
                BridgeLineageSourceErrorKind::HistoricalResolutionFailure,
                "bridge historical lineage authority contained duplicate resolved lineage keys",
            ));
        }
        if sorted_lineage_keys != canonical_resolved_lineage_keys {
            return Err(BridgeLineageSourceError::new(
                BridgeLineageSourceErrorKind::HistoricalResolutionFailure,
                "bridge historical lineage authority must emit resolved lineage keys in canonical order",
            ));
        }

        let mut sorted_record_keys = canonical_resolved_record_keys.clone();
        sorted_record_keys.sort_unstable();
        sorted_record_keys.dedup();
        if sorted_record_keys.len() != canonical_resolved_record_keys.len() {
            return Err(BridgeLineageSourceError::new(
                BridgeLineageSourceErrorKind::HistoricalResolutionFailure,
                "bridge historical lineage authority contained duplicate resolved record keys",
            ));
        }
        if sorted_record_keys != canonical_resolved_record_keys {
            return Err(BridgeLineageSourceError::new(
                BridgeLineageSourceErrorKind::HistoricalResolutionFailure,
                "bridge historical lineage authority must emit resolved record keys in canonical order",
            ));
        }

        let mut sorted_event_ids = traversed_event_ids.clone();
        sorted_event_ids.sort_unstable();
        sorted_event_ids.dedup();
        if sorted_event_ids.len() != traversed_event_ids.len() {
            return Err(BridgeLineageSourceError::new(
                BridgeLineageSourceErrorKind::HistoricalResolutionFailure,
                "bridge historical lineage authority contained duplicate traversed event ids",
            ));
        }
        if sorted_event_ids != traversed_event_ids {
            return Err(BridgeLineageSourceError::new(
                BridgeLineageSourceErrorKind::HistoricalResolutionFailure,
                "bridge historical lineage authority must emit traversed event ids in canonical order",
            ));
        }

        let topology = match (
            canonical_resolved_lineage_keys.len(),
            canonical_resolved_record_keys.len(),
        ) {
            (0, 0) => BridgeHistoricalLineageTopology::NoAuthoritativeSuccessor,
            (_, 0) => BridgeHistoricalLineageTopology::UnsupportedWithoutSuccessor,
            (_, 1) if canonical_resolved_lineage_keys.len() > 1 => {
                BridgeHistoricalLineageTopology::MergeLikeSuccessor
            }
            (_, 1) => BridgeHistoricalLineageTopology::SingleSuccessor,
            (lineage_count, record_count) if lineage_count == record_count => {
                // The lineage source is the authority boundary for split correspondence.
                // Once canonicalized and validated here, downstream resolution consumes the
                // preclassified topology instead of re-heuristically rediscovering it.
                BridgeHistoricalLineageTopology::SplitSuccessors
            }
            _ => BridgeHistoricalLineageTopology::AmbiguousSuccessor,
        };

        let canonical_basis = format!(
            "historical-lineage-authority|authority={}|branch={}|snapshot={}|lineage-count={}|lineages={}|record-count={}|records={}|event-count={}|events={}",
            authority_basis.digest(),
            authority_basis.branch_identity().as_str(),
            authority_basis.snapshot_identity().as_str(),
            canonical_resolved_lineage_keys.len(),
            canonical_resolved_lineage_keys
                .iter()
                .map(|key| key.as_ref())
                .collect::<Vec<_>>()
                .join(","),
            canonical_resolved_record_keys.len(),
            canonical_resolved_record_keys
                .iter()
                .map(|key| key.as_ref())
                .collect::<Vec<_>>()
                .join(","),
            traversed_event_ids.len(),
            traversed_event_ids
                .iter()
                .map(u64::to_string)
                .collect::<Vec<_>>()
                .join(","),
        );
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Ok(Self {
            authority_basis,
            canonical_resolved_lineage_keys: Arc::from(canonical_resolved_lineage_keys),
            canonical_resolved_record_keys: Arc::from(canonical_resolved_record_keys),
            traversed_event_ids: Arc::from(traversed_event_ids),
            topology,
            lineage_digest: Arc::from(format!("historical-lineage-authority:sha256:{digest:x}")),
        })
    }

    pub fn authority_basis(&self) -> &continuity::BridgeContinuityAuthorityBasis {
        &self.authority_basis
    }

    pub fn branch_identity(&self) -> &input::envelope::TruthBranchIdentity {
        self.authority_basis.branch_identity()
    }

    pub fn snapshot_identity(&self) -> &TruthSnapshotIdentity {
        self.authority_basis.snapshot_identity()
    }

    pub fn canonical_resolved_lineage_keys(&self) -> &[Arc<str>] {
        &self.canonical_resolved_lineage_keys
    }

    pub fn canonical_resolved_record_keys(&self) -> &[Arc<str>] {
        &self.canonical_resolved_record_keys
    }

    pub fn traversed_event_ids(&self) -> &[u64] {
        &self.traversed_event_ids
    }

    pub fn topology(&self) -> BridgeHistoricalLineageTopology {
        self.topology
    }

    pub fn lineage_digest(&self) -> &str {
        self.lineage_digest.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::continuity::BridgeContinuityAuthorityBasis;
    use crate::input::envelope::TruthBranchIdentity;
    use crate::snapshot::TruthSnapshotIdentity;

    use super::BridgeHistoricalLineageAuthority;

    #[test]
    fn historical_lineage_authority_digest_is_canonical_for_same_inputs() {
        let authority_basis = BridgeContinuityAuthorityBasis::new(
            TruthBranchIdentity::new("main"),
            TruthSnapshotIdentity::new("snapshot-a"),
        );

        let left = BridgeHistoricalLineageAuthority::try_new(
            authority_basis.clone(),
            vec![Arc::from("lineage:1"), Arc::from("lineage:2")],
            vec![Arc::from("entity:0:4:2"), Arc::from("entity:0:5:2")],
            vec![3, 7],
        )
        .expect("canonical lineage authority should build");
        let right = BridgeHistoricalLineageAuthority::try_new(
            authority_basis,
            vec![Arc::from("lineage:1"), Arc::from("lineage:2")],
            vec![Arc::from("entity:0:4:2"), Arc::from("entity:0:5:2")],
            vec![3, 7],
        )
        .expect("canonical lineage authority should build");

        assert_eq!(left, right);
        assert!(left
            .lineage_digest()
            .starts_with("historical-lineage-authority:sha256:"));
    }

    #[test]
    fn historical_lineage_authority_rejects_noncanonical_inputs() {
        let authority_basis = BridgeContinuityAuthorityBasis::new(
            TruthBranchIdentity::new("main"),
            TruthSnapshotIdentity::new("snapshot-a"),
        );

        let error = BridgeHistoricalLineageAuthority::try_new(
            authority_basis,
            vec![Arc::from("lineage:2"), Arc::from("lineage:1")],
            vec![Arc::from("entity:0:5:2"), Arc::from("entity:0:4:2")],
            vec![7, 3],
        )
        .expect_err("noncanonical lineage authority should be rejected");

        assert!(error.to_string().contains("canonical order"));
    }
}
