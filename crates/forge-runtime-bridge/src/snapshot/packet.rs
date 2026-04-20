use std::collections::BTreeMap;
use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::error::BridgeMessageError;
use crate::mapping::SubscriptionSliceKind;
use crate::snapshot::TruthSnapshotIdentity;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BridgeSnapshotReadErrorTag {}
pub type BridgeSnapshotReadError = BridgeMessageError<BridgeSnapshotReadErrorTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotReadRequest {
    request_key: Arc<str>,
    entity_identity: Arc<str>,
    aspect_label: Arc<str>,
    shape: SnapshotReadShape,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SnapshotReadShape {
    Coarse,
    SubscriptionSlice {
        surface_label: Arc<str>,
        slice_kind: SubscriptionSliceKind,
    },
}

impl SnapshotReadRequest {
    pub fn for_coarse(
        entity_identity: impl Into<Arc<str>>,
        aspect_label: impl Into<Arc<str>>,
    ) -> Self {
        let entity_identity = entity_identity.into();
        let aspect_label = aspect_label.into();
        Self {
            request_key: format!("{}:{}", entity_identity.as_ref(), aspect_label.as_ref()).into(),
            entity_identity,
            aspect_label,
            shape: SnapshotReadShape::Coarse,
        }
    }

    pub fn for_subscription_slice(
        entity_identity: impl Into<Arc<str>>,
        aspect_label: impl Into<Arc<str>>,
        surface_label: impl Into<Arc<str>>,
        slice_kind: SubscriptionSliceKind,
    ) -> Self {
        let entity_identity = entity_identity.into();
        let aspect_label = aspect_label.into();
        let surface_label = surface_label.into();
        Self {
            request_key: format!(
                "{}:{}:{}:{}",
                entity_identity.as_ref(),
                aspect_label.as_ref(),
                canonical_subscription_slice_kind_label(&slice_kind),
                surface_label.as_ref()
            )
            .into(),
            entity_identity,
            aspect_label,
            shape: SnapshotReadShape::SubscriptionSlice {
                surface_label,
                slice_kind,
            },
        }
    }

    pub fn request_key(&self) -> &str {
        self.request_key.as_ref()
    }

    pub fn entity_identity(&self) -> &str {
        self.entity_identity.as_ref()
    }

    pub fn aspect_label(&self) -> &str {
        self.aspect_label.as_ref()
    }

    pub fn surface_label(&self) -> Option<&str> {
        match &self.shape {
            SnapshotReadShape::Coarse => None,
            SnapshotReadShape::SubscriptionSlice { surface_label, .. } => {
                Some(surface_label.as_ref())
            }
        }
    }

    pub fn slice_kind(&self) -> Option<&SubscriptionSliceKind> {
        match &self.shape {
            SnapshotReadShape::Coarse => None,
            SnapshotReadShape::SubscriptionSlice { slice_kind, .. } => Some(slice_kind),
        }
    }

    pub(crate) fn canonical_basis(&self) -> Arc<str> {
        match &self.shape {
            SnapshotReadShape::Coarse => Arc::from(format!(
                "snapshot-read-request|key={}|entity={}|aspect={}|shape=coarse",
                self.request_key(),
                self.entity_identity(),
                self.aspect_label(),
            )),
            SnapshotReadShape::SubscriptionSlice {
                surface_label,
                slice_kind,
            } => Arc::from(format!(
                "snapshot-read-request|key={}|entity={}|aspect={}|shape=subscription-slice|surface={}|slice-kind={}",
                self.request_key(),
                self.entity_identity(),
                self.aspect_label(),
                surface_label.as_ref(),
                canonical_subscription_slice_kind_label(slice_kind),
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotReadPacket {
    reads: Vec<SnapshotReadRequest>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl SnapshotReadPacket {
    pub fn new(reads: Vec<SnapshotReadRequest>) -> Self {
        let read_basis = reads
            .iter()
            .map(SnapshotReadRequest::canonical_basis)
            .collect::<Vec<_>>()
            .join("|");
        let canonical_basis = Arc::<str>::from(format!(
            "snapshot-read-packet|count={}|reads={read_basis}",
            reads.len(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            reads,
            canonical_basis,
            digest: Arc::from(format!("snapshot-read-packet:sha256:{digest:x}")),
        }
    }

    pub fn reads(&self) -> &[SnapshotReadRequest] {
        &self.reads
    }

    pub(crate) fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotReadRecord {
    request_key: Arc<str>,
    payload: Arc<[u8]>,
}

impl SnapshotReadRecord {
    pub fn new(request_key: impl Into<Arc<str>>, payload: impl Into<Arc<[u8]>>) -> Self {
        Self {
            request_key: request_key.into(),
            payload: payload.into(),
        }
    }

    pub fn request_key(&self) -> &str {
        self.request_key.as_ref()
    }

    pub fn payload(&self) -> &[u8] {
        self.payload.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotReadPacketResult {
    snapshot_identity: TruthSnapshotIdentity,
    records: Vec<SnapshotReadRecord>,
}

impl SnapshotReadPacketResult {
    pub fn new(snapshot_identity: TruthSnapshotIdentity, records: Vec<SnapshotReadRecord>) -> Self {
        Self {
            snapshot_identity,
            records,
        }
    }

    pub fn snapshot_identity(&self) -> &TruthSnapshotIdentity {
        &self.snapshot_identity
    }

    pub fn records(&self) -> &[SnapshotReadRecord] {
        &self.records
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedSnapshotReadPacketResult {
    raw: SnapshotReadPacketResult,
}

impl ValidatedSnapshotReadPacketResult {
    pub(crate) fn validated(raw: SnapshotReadPacketResult) -> Self {
        Self { raw }
    }

    pub fn snapshot_identity(&self) -> &TruthSnapshotIdentity {
        self.raw.snapshot_identity()
    }

    pub fn records(&self) -> &[SnapshotReadRecord] {
        self.raw.records()
    }
}

pub(crate) fn validate_snapshot_read_result_contract(
    packet: &SnapshotReadPacket,
    raw_result: SnapshotReadPacketResult,
) -> Result<ValidatedSnapshotReadPacketResult, BridgeSnapshotReadError> {
    let snapshot_identity = raw_result.snapshot_identity;
    let mut record_lookup = BTreeMap::new();
    for record in raw_result.records {
        let key = record.request_key().to_string();
        if record_lookup.insert(key.clone(), record).is_some() {
            return Err(BridgeSnapshotReadError::new(format!(
                "Snapshot read result contained duplicate record for request key `{key}`."
            )));
        }
    }

    if record_lookup.len() != packet.reads().len() {
        return Err(BridgeSnapshotReadError::new(format!(
            "Snapshot read result returned {} records for {} requested reads.",
            record_lookup.len(),
            packet.reads().len()
        )));
    }

    let mut canonical_records = Vec::with_capacity(packet.reads().len());
    for read in packet.reads() {
        let Some(record) = record_lookup.remove(read.request_key()) else {
            return Err(BridgeSnapshotReadError::new(format!(
                "Snapshot read result omitted required request key `{}`.",
                read.request_key()
            )));
        };
        canonical_records.push(record);
    }

    if let Some(extra_key) = record_lookup.into_keys().next() {
        return Err(BridgeSnapshotReadError::new(format!(
            "Snapshot read result returned undeclared request key `{extra_key}`."
        )));
    }

    Ok(ValidatedSnapshotReadPacketResult::validated(
        SnapshotReadPacketResult::new(snapshot_identity, canonical_records),
    ))
}

pub(crate) fn canonical_subscription_slice_kind_label(
    slice_kind: &SubscriptionSliceKind,
) -> &'static str {
    match slice_kind {
        SubscriptionSliceKind::SignalField => "signal-field",
        SubscriptionSliceKind::SignalLens => "signal-lens",
        SubscriptionSliceKind::SignalRegion => "signal-region",
        SubscriptionSliceKind::SignalPartition => "signal-partition",
        SubscriptionSliceKind::SignalFacet => "signal-facet",
        SubscriptionSliceKind::RegisteredCoarseFallback => "registered-coarse-fallback",
    }
}

#[cfg(test)]
mod tests {
    use crate::mapping::SubscriptionSliceKind;
    use crate::snapshot::{
        validate_snapshot_read_result_contract, BridgeSnapshotReadError, SnapshotReadPacket,
        SnapshotReadRecord, SnapshotReadRequest, TruthSnapshotIdentity,
    };

    use super::SnapshotReadPacketResult;

    #[test]
    fn packet_preserves_declared_read_order() {
        let packet = SnapshotReadPacket::new(vec![
            SnapshotReadRequest::for_coarse("user-1", "profile"),
            SnapshotReadRequest::for_subscription_slice(
                "user-2",
                "profile",
                "name",
                SubscriptionSliceKind::SignalField,
            ),
        ]);

        assert_eq!(packet.reads()[0].request_key(), "user-1:profile");
        assert_eq!(
            packet.reads()[1].request_key(),
            "user-2:profile:signal-field:name"
        );
        assert_eq!(packet.reads()[0].surface_label(), None);
        assert_eq!(packet.reads()[0].slice_kind(), None);
        assert_eq!(packet.reads()[1].surface_label(), Some("name"));
        assert_eq!(
            packet.reads()[1].slice_kind(),
            Some(&SubscriptionSliceKind::SignalField)
        );
        assert!(packet.digest().starts_with("snapshot-read-packet:sha256:"));
    }

    #[test]
    fn packet_digest_changes_when_declared_reads_change() {
        let left =
            SnapshotReadPacket::new(vec![SnapshotReadRequest::for_coarse("user-1", "profile")]);
        let right =
            SnapshotReadPacket::new(vec![SnapshotReadRequest::for_coarse("user-2", "profile")]);

        assert_ne!(left.digest(), right.digest());
    }

    #[test]
    fn packet_result_retains_snapshot_identity() {
        let result = SnapshotReadPacketResult::new(
            TruthSnapshotIdentity::new("snapshot-a"),
            vec![SnapshotReadRecord::new("a", b"alice".to_vec())],
        );

        assert_eq!(result.snapshot_identity().as_str(), "snapshot-a");
        assert_eq!(result.records()[0].request_key(), "a");
    }

    #[test]
    fn validation_rejects_missing_required_record() {
        let packet = SnapshotReadPacket::new(vec![
            SnapshotReadRequest::for_coarse("user-1", "profile"),
            SnapshotReadRequest::for_coarse("user-2", "profile"),
        ]);

        let error = validate_snapshot_read_result_contract(
            &packet,
            SnapshotReadPacketResult::new(
                TruthSnapshotIdentity::new("snapshot-a"),
                vec![SnapshotReadRecord::new("a", b"alice".to_vec())],
            ),
        )
        .expect_err("missing records must fail the bridge snapshot contract");

        assert!(matches!(error, BridgeSnapshotReadError { .. }));
        assert!(error.to_string().contains("returned 1 records"));
    }

    #[test]
    fn validation_rejects_duplicate_result_keys() {
        let packet =
            SnapshotReadPacket::new(vec![SnapshotReadRequest::for_coarse("user-1", "profile")]);

        let error = validate_snapshot_read_result_contract(
            &packet,
            SnapshotReadPacketResult::new(
                TruthSnapshotIdentity::new("snapshot-a"),
                vec![
                    SnapshotReadRecord::new("a", b"alice".to_vec()),
                    SnapshotReadRecord::new("a", b"alice-2".to_vec()),
                ],
            ),
        )
        .expect_err("duplicate result keys must fail the bridge snapshot contract");

        assert!(error.to_string().contains("duplicate record"));
    }
}
