use forge_relational::facade::bridge::{
    bridge_snapshot_identity_for_handle, publication_bundle_to_bridge_envelope,
};
use forge_relational::facade::identity::VersionId;
use forge_relational::facade::runtime::{RelationalReadView, RelationalRuntime};
use forge_relational::facade::transactions::RecordRef;
use forge_runtime_bridge::facade::{
    BridgeSnapshotReadError, CommittedPatchSource, RawCommittedPatchEnvelope,
    RelationalBridgeSourceError, RelationalCommittedPatchRequest, SnapshotReadPacket,
    SnapshotReadPacketResult, SnapshotReadRecord, SnapshotReadSource, TruthBranchHeadSource,
    TruthBranchIdentity, TruthSnapshotIdentity, TruthSnapshotReader,
};

use super::binding::WorthTopologyRuntimeBinding;
use super::bridge_source_support::{
    missing_aspect_error, missing_record_error, parse_bridge_commit_identity,
    parse_bridge_record_identity, parse_bridge_snapshot_identity, payload_bytes_for_entity_aspect,
    payload_bytes_for_relation_aspect,
};

#[derive(Clone)]
pub(super) struct WorthTopologyRuntimeBridgeSource {
    binding: WorthTopologyRuntimeBinding,
}

impl WorthTopologyRuntimeBridgeSource {
    pub(super) fn new(binding: WorthTopologyRuntimeBinding) -> Self {
        Self { binding }
    }
}

impl CommittedPatchSource for WorthTopologyRuntimeBridgeSource {
    fn load_committed_patch(
        &self,
        request: RelationalCommittedPatchRequest,
    ) -> Result<RawCommittedPatchEnvelope, RelationalBridgeSourceError> {
        let commit_id = parse_bridge_commit_identity(request.commit_identity())?;
        let Some(runtime) = self.binding.runtime() else {
            return Err(RelationalBridgeSourceError::new(format!(
                "worth topology snapshot certification runtime does not expose committed patch loading for `{}`",
                request.commit_identity()
            )));
        };
        let runtime = runtime
            .read()
            .expect("worth topology bridge source lock poisoned");
        let publication = runtime.publication();
        let bundle = publication.latest_bundle().ok_or_else(|| {
            RelationalBridgeSourceError::new(format!(
                "worth topology runtime has no published bundle for bridge commit `{}`",
                request.commit_identity()
            ))
        })?;
        if bundle.commit.commit_id != commit_id {
            return Err(RelationalBridgeSourceError::new(format!(
                "worth topology runtime could not resolve authoritative commit `{}`",
                request.commit_identity()
            )));
        }
        Ok(publication_bundle_to_bridge_envelope(bundle))
    }
}

impl SnapshotReadSource for WorthTopologyRuntimeBridgeSource {
    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError> {
        match &self.binding {
            WorthTopologyRuntimeBinding::CurrentHead(runtime) => {
                let version_id = {
                    let runtime = runtime
                        .read()
                        .expect("worth topology bridge source lock poisoned");
                    resolve_bridge_snapshot_version(&runtime, identity)?
                };
                Ok(Box::new(WorthTopologySnapshotReader::current_head(
                    runtime.clone(),
                    identity.clone(),
                    version_id,
                )))
            }
            WorthTopologyRuntimeBinding::SnapshotReadOnly {
                read_view,
                snapshot,
            } => {
                let expected = bridge_snapshot_identity_for_handle(snapshot);
                if expected != *identity {
                    return Err(RelationalBridgeSourceError::new(format!(
                        "worth topology snapshot certification runtime only exposes authoritative snapshot `{}`; requested `{}`",
                        expected.as_str(),
                        identity.as_str()
                    )));
                }
                Ok(Box::new(WorthTopologySnapshotReader::snapshot_read_only(
                    read_view.clone(),
                    identity.clone(),
                )))
            }
        }
    }
}

impl TruthBranchHeadSource for WorthTopologyRuntimeBridgeSource {
    fn load_branch_head_patch(
        &self,
        branch_identity: &TruthBranchIdentity,
    ) -> Result<RawCommittedPatchEnvelope, RelationalBridgeSourceError> {
        let Some(runtime) = self.binding.runtime() else {
            return Err(RelationalBridgeSourceError::new(format!(
                "worth topology snapshot certification runtime does not expose branch-head patch loading for `{}`",
                branch_identity.as_str()
            )));
        };
        let runtime = runtime
            .read()
            .expect("worth topology bridge source lock poisoned");
        let publication = runtime.publication();
        let bundle = publication.latest_bundle().ok_or_else(|| {
            RelationalBridgeSourceError::new(format!(
                "worth topology runtime has no published bundle for branch `{}`",
                branch_identity.as_str()
            ))
        })?;
        if bundle.commit.branch_id.0 != branch_identity.as_str() {
            return Err(RelationalBridgeSourceError::new(format!(
                "worth topology current-head bridge source only exposes latest branch `{}`; requested `{}`",
                bundle.commit.branch_id.0,
                branch_identity.as_str()
            )));
        }
        Ok(publication_bundle_to_bridge_envelope(bundle))
    }
}

enum WorthTopologySnapshotReadMode {
    CurrentHead {
        runtime: std::sync::Arc<std::sync::RwLock<RelationalRuntime>>,
        version_id: VersionId,
    },
    SnapshotReadOnly {
        read_view: std::sync::Arc<RelationalReadView>,
    },
}

struct WorthTopologySnapshotReader {
    mode: WorthTopologySnapshotReadMode,
    snapshot_identity: TruthSnapshotIdentity,
}

impl WorthTopologySnapshotReader {
    fn current_head(
        runtime: std::sync::Arc<std::sync::RwLock<RelationalRuntime>>,
        snapshot_identity: TruthSnapshotIdentity,
        version_id: VersionId,
    ) -> Self {
        Self {
            mode: WorthTopologySnapshotReadMode::CurrentHead {
                runtime,
                version_id,
            },
            snapshot_identity,
        }
    }

    fn snapshot_read_only(
        read_view: std::sync::Arc<RelationalReadView>,
        snapshot_identity: TruthSnapshotIdentity,
    ) -> Self {
        Self {
            mode: WorthTopologySnapshotReadMode::SnapshotReadOnly { read_view },
            snapshot_identity,
        }
    }
}

impl TruthSnapshotReader for WorthTopologySnapshotReader {
    fn snapshot_identity(&self) -> TruthSnapshotIdentity {
        self.snapshot_identity.clone()
    }

    fn read_packet(
        &self,
        request: &SnapshotReadPacket,
    ) -> Result<SnapshotReadPacketResult, BridgeSnapshotReadError> {
        let mut records = Vec::with_capacity(request.reads().len());
        for read in request.reads() {
            let record_ref = parse_bridge_record_identity(read.entity_identity())
                .map_err(|error| BridgeSnapshotReadError::new(error.to_string()))?;
            let payload = match &self.mode {
                WorthTopologySnapshotReadMode::CurrentHead {
                    runtime,
                    version_id,
                } => {
                    let runtime = runtime
                        .read()
                        .expect("worth topology bridge source lock poisoned");
                    let projection = runtime.read_truth().project_version(*version_id);
                    match record_ref {
                        RecordRef::Entity(entity_id) => {
                            let record = projection.entity_record(entity_id).ok_or_else(|| {
                                missing_record_error(
                                    "entity",
                                    read.entity_identity(),
                                    &self.snapshot_identity,
                                )
                            })?;
                            payload_bytes_for_entity_aspect(&record, read.aspect_label())
                                .ok_or_else(|| {
                                    missing_aspect_error(
                                        "entity",
                                        read.aspect_label(),
                                        read.entity_identity(),
                                        &self.snapshot_identity,
                                    )
                                })?
                        }
                        RecordRef::Relation(relation_id) => {
                            let record =
                                projection.relation_record(relation_id).ok_or_else(|| {
                                    missing_record_error(
                                        "relation",
                                        read.entity_identity(),
                                        &self.snapshot_identity,
                                    )
                                })?;
                            payload_bytes_for_relation_aspect(&record, read.aspect_label())
                                .ok_or_else(|| {
                                    missing_aspect_error(
                                        "relation",
                                        read.aspect_label(),
                                        read.entity_identity(),
                                        &self.snapshot_identity,
                                    )
                                })?
                        }
                    }
                }
                WorthTopologySnapshotReadMode::SnapshotReadOnly { read_view } => {
                    payload_from_read_view(read_view, &self.snapshot_identity, read, record_ref)?
                }
            };
            records.push(SnapshotReadRecord::new(read.request_key(), payload));
        }

        Ok(SnapshotReadPacketResult::new(
            self.snapshot_identity.clone(),
            records,
        ))
    }
}

fn resolve_bridge_snapshot_version(
    runtime: &RelationalRuntime,
    identity: &TruthSnapshotIdentity,
) -> Result<VersionId, RelationalBridgeSourceError> {
    let (snapshot_id, expected_version_id) = parse_bridge_snapshot_identity(identity)?;
    let observed_version_id = runtime
        .publication()
        .latest_bundle()
        .and_then(|bundle| {
            (bundle.commit.commit_id.0 == snapshot_id.0).then_some(bundle.commit.version_id)
        })
        .ok_or_else(|| {
            RelationalBridgeSourceError::new(format!(
                "worth topology bridge snapshot identity `{}` does not resolve to the current-head published bundle",
                identity.as_str()
            ))
        })?;
    if observed_version_id != expected_version_id {
        return Err(RelationalBridgeSourceError::new(format!(
            "worth topology bridge snapshot identity `{}` expected version `{}` but authoritative binding resolved to version `{}`",
            identity.as_str(),
            expected_version_id.0,
            observed_version_id.0
        )));
    }
    Ok(observed_version_id)
}

fn payload_from_read_view(
    read_view: &RelationalReadView,
    snapshot_identity: &TruthSnapshotIdentity,
    read: &forge_runtime_bridge::facade::SnapshotReadRequest,
    record_ref: RecordRef,
) -> Result<Vec<u8>, BridgeSnapshotReadError> {
    match record_ref {
        RecordRef::Entity(entity_id) => {
            let record = read_view.get_entity(entity_id).ok_or_else(|| {
                missing_record_error("entity", read.entity_identity(), snapshot_identity)
            })?;
            payload_bytes_for_entity_aspect(record, read.aspect_label()).ok_or_else(|| {
                missing_aspect_error(
                    "entity",
                    read.aspect_label(),
                    read.entity_identity(),
                    snapshot_identity,
                )
            })
        }
        RecordRef::Relation(relation_id) => {
            let record = read_view.get_relation(relation_id).ok_or_else(|| {
                missing_record_error("relation", read.entity_identity(), snapshot_identity)
            })?;
            payload_bytes_for_relation_aspect(record, read.aspect_label()).ok_or_else(|| {
                missing_aspect_error(
                    "relation",
                    read.aspect_label(),
                    read.entity_identity(),
                    snapshot_identity,
                )
            })
        }
    }
}
