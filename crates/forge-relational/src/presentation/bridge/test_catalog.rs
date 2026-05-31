use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use crate::history::data::CommitId;
use crate::publication::patch::data::PublishedAuthoritativePatchEnvelope;
use forge_runtime_bridge::facade::{
    CommittedPatchSource, RawCommittedPatchEnvelope, RelationalBridgeSourceError,
    RelationalCommittedPatchRequest, SnapshotReadPacket, SnapshotReadPacketResult,
    SnapshotReadRecord, SnapshotReadSource, TruthBranchHeadSource, TruthBranchIdentity,
    TruthSnapshotIdentity, TruthSnapshotReader,
};

use super::patch_envelopes::publication_patch_to_bridge_envelope;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicationBridgeSnapshot {
    identity: TruthSnapshotIdentity,
    read_result_identity: TruthSnapshotIdentity,
    records: Vec<SnapshotReadRecord>,
}

impl PublicationBridgeSnapshot {
    pub fn new(identity: TruthSnapshotIdentity, records: Vec<SnapshotReadRecord>) -> Self {
        Self {
            read_result_identity: identity.clone(),
            identity,
            records,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PublicationBridgeCatalog {
    state: Arc<RwLock<PublicationBridgeCatalogState>>,
}

#[derive(Debug, Default)]
struct PublicationBridgeCatalogState {
    committed_patches: BTreeMap<String, RawCommittedPatchEnvelope>,
    snapshots: BTreeMap<String, PublicationBridgeSnapshot>,
}

impl PublicationBridgeCatalog {
    pub fn register_patch(
        &self,
        commit_id: CommitId,
        branch_identity: impl Into<String>,
        snapshot_identity: impl Into<String>,
        patch: &PublishedAuthoritativePatchEnvelope,
    ) {
        let envelope = publication_patch_to_bridge_envelope(
            commit_id,
            branch_identity,
            snapshot_identity,
            patch,
        );
        self.state
            .write()
            .expect("publication bridge catalog lock poisoned")
            .committed_patches
            .insert(envelope.commit_identity().as_str().to_string(), envelope);
    }

    pub fn register_snapshot(&self, snapshot: PublicationBridgeSnapshot) {
        self.state
            .write()
            .expect("publication bridge catalog lock poisoned")
            .snapshots
            .insert(snapshot.identity.as_str().to_string(), snapshot);
    }
}

impl CommittedPatchSource for PublicationBridgeCatalog {
    fn load_committed_patch(
        &self,
        request: RelationalCommittedPatchRequest,
    ) -> Result<RawCommittedPatchEnvelope, RelationalBridgeSourceError> {
        self.state
            .read()
            .expect("publication bridge catalog lock poisoned")
            .committed_patches
            .get(request.commit_identity())
            .cloned()
            .ok_or_else(|| {
                RelationalBridgeSourceError::new(format!(
                    "no publication bridge patch registered for commit `{}`",
                    request.commit_identity()
                ))
            })
    }
}

impl SnapshotReadSource for PublicationBridgeCatalog {
    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError> {
        let snapshot = self
            .state
            .read()
            .expect("publication bridge catalog lock poisoned")
            .snapshots
            .get(identity.as_str())
            .cloned()
            .ok_or_else(|| {
                RelationalBridgeSourceError::new(format!(
                    "no publication bridge snapshot registered for `{}`",
                    identity.as_str()
                ))
            })?;
        Ok(Box::new(PublicationSnapshotReader { snapshot }))
    }
}

impl TruthBranchHeadSource for PublicationBridgeCatalog {
    fn load_branch_head_patch(
        &self,
        branch_identity: &TruthBranchIdentity,
    ) -> Result<RawCommittedPatchEnvelope, RelationalBridgeSourceError> {
        self.state
            .read()
            .expect("publication bridge catalog lock poisoned")
            .committed_patches
            .values()
            .filter(|envelope| envelope.branch_identity() == branch_identity)
            .cloned()
            .last()
            .ok_or_else(|| {
                RelationalBridgeSourceError::new(format!(
                    "no publication bridge branch head registered for `{}`",
                    branch_identity.as_str()
                ))
            })
    }
}

#[derive(Debug, Clone)]
struct PublicationSnapshotReader {
    snapshot: PublicationBridgeSnapshot,
}

impl TruthSnapshotReader for PublicationSnapshotReader {
    fn snapshot_identity(&self) -> TruthSnapshotIdentity {
        self.snapshot.identity.clone()
    }

    fn read_packet(
        &self,
        request: &SnapshotReadPacket,
    ) -> Result<SnapshotReadPacketResult, forge_runtime_bridge::facade::BridgeSnapshotReadError>
    {
        let records_by_key = self
            .snapshot
            .records
            .iter()
            .map(|record| (record.request_key().to_string(), record.clone()))
            .collect::<BTreeMap<_, _>>();
        let records = request
            .reads()
            .iter()
            .filter_map(|read| records_by_key.get(read.request_key()).cloned())
            .collect::<Vec<_>>();
        Ok(SnapshotReadPacketResult::new(
            self.snapshot.read_result_identity.clone(),
            records,
        ))
    }
}
