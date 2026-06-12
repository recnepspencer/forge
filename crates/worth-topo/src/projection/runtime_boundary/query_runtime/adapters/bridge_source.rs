use forge_relational::facade::bridge::{
    bridge_snapshot_identity_for_handle, publication_bundle_to_bridge_envelope,
};
use forge_relational::facade::identity::VersionId;
use forge_relational::facade::runtime::{RelationalReadView, RelationalRuntime};
use forge_relational::facade::transactions::RecordRef;
use forge_runtime_bridge::facade::{
    BridgeCommittedPatchEnvelope, BridgeSnapshotReadError, CommittedPatchSource,
    RelationalBridgeSourceError, RelationalCommittedPatchRequest, SnapshotReadPacket,
    SnapshotReadPacketResult, SnapshotReadRecord, SnapshotReadSource, TruthBranchHeadSource,
    TruthBranchIdentity, TruthSnapshotIdentity, TruthSnapshotReader,
};

use super::binding::TopologyRuntimeBinding;
use super::bridge_source_support::{
    bridge_commit_id, bridge_record_ref, missing_aspect_error, missing_record_error,
    parse_bridge_snapshot_identity, snapshot_aspect_value_for_entity_aspect,
    snapshot_aspect_value_for_relation_aspect,
};

#[derive(Clone)]
pub(super) struct TopologyRuntimeBridgeSource {
    binding: TopologyRuntimeBinding,
}

impl TopologyRuntimeBridgeSource {
    pub(super) fn new(binding: TopologyRuntimeBinding) -> Self {
        Self { binding }
    }
}

impl CommittedPatchSource for TopologyRuntimeBridgeSource {
    fn load_committed_patch(
        &self,
        request: RelationalCommittedPatchRequest,
    ) -> Result<BridgeCommittedPatchEnvelope, RelationalBridgeSourceError> {
        let commit_id = bridge_commit_id(request.commit_identity())?;
        let Some(runtime) = self.binding.runtime() else {
            return Err(RelationalBridgeSourceError::new(format!(
                "topology snapshot certification runtime does not expose committed patch loading for requested commit"
            )));
        };
        let runtime = runtime
            .read()
            .expect("topology bridge source lock poisoned");
        let publication = runtime.publication();
        let bundle = publication.latest_bundle().ok_or_else(|| {
            RelationalBridgeSourceError::new(format!(
                "topology runtime has no published bundle for requested bridge commit"
            ))
        })?;
        if bundle.commit.commit_id != commit_id {
            return Err(RelationalBridgeSourceError::new(format!(
                "topology runtime could not resolve requested authoritative commit"
            )));
        }
        Ok(publication_bundle_to_bridge_envelope(bundle))
    }
}

impl SnapshotReadSource for TopologyRuntimeBridgeSource {
    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError> {
        match &self.binding {
            TopologyRuntimeBinding::CurrentHead(runtime) => {
                let version_id = {
                    let runtime = runtime
                        .read()
                        .expect("topology bridge source lock poisoned");
                    resolve_bridge_snapshot_version(&runtime, identity)?
                };
                Ok(Box::new(TopologySnapshotReader::current_head(
                    runtime.clone(),
                    identity.clone(),
                    version_id,
                )))
            }
            TopologyRuntimeBinding::SnapshotReadOnly {
                read_view,
                snapshot,
            } => {
                let expected = bridge_snapshot_identity_for_handle(snapshot);
                if expected != *identity {
                    return Err(RelationalBridgeSourceError::new(format!(
                        "topology snapshot certification runtime only exposes authoritative snapshot `{}`; requested `{}`",
                        expected.evidence_identity().as_str(),
                        identity.evidence_identity().as_str()
                    )));
                }
                Ok(Box::new(TopologySnapshotReader::snapshot_read_only(
                    read_view.clone(),
                    identity.clone(),
                )))
            }
        }
    }
}

impl TruthBranchHeadSource for TopologyRuntimeBridgeSource {
    fn load_branch_head_patch(
        &self,
        branch_identity: &TruthBranchIdentity,
    ) -> Result<BridgeCommittedPatchEnvelope, RelationalBridgeSourceError> {
        let Some(runtime) = self.binding.runtime() else {
            return Err(RelationalBridgeSourceError::new(format!(
                "topology snapshot certification runtime does not expose branch-head patch loading for requested branch"
            )));
        };
        let runtime = runtime
            .read()
            .expect("topology bridge source lock poisoned");
        let publication = runtime.publication();
        let bundle = publication.latest_bundle().ok_or_else(|| {
            RelationalBridgeSourceError::new(format!(
                "topology runtime has no published bundle for requested branch"
            ))
        })?;
        let Some(requested_branch) = branch_identity.relational_branch_id() else {
            return Err(RelationalBridgeSourceError::new(
                "topology bridge source requires typed relational branch identity",
            ));
        };
        if bundle.commit.branch_id.0 != requested_branch {
            return Err(RelationalBridgeSourceError::new(format!(
                "topology current-head bridge source only exposes latest branch `{}`; requested `{}`",
                bundle.commit.branch_id.0,
                requested_branch
            )));
        }
        Ok(publication_bundle_to_bridge_envelope(bundle))
    }
}

enum TopologySnapshotReadMode {
    CurrentHead {
        runtime: std::sync::Arc<std::sync::RwLock<RelationalRuntime>>,
        version_id: VersionId,
    },
    SnapshotReadOnly {
        read_view: std::sync::Arc<RelationalReadView>,
    },
}

struct TopologySnapshotReader {
    mode: TopologySnapshotReadMode,
    snapshot_identity: TruthSnapshotIdentity,
}

impl TopologySnapshotReader {
    fn current_head(
        runtime: std::sync::Arc<std::sync::RwLock<RelationalRuntime>>,
        snapshot_identity: TruthSnapshotIdentity,
        version_id: VersionId,
    ) -> Self {
        Self {
            mode: TopologySnapshotReadMode::CurrentHead {
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
            mode: TopologySnapshotReadMode::SnapshotReadOnly { read_view },
            snapshot_identity,
        }
    }
}

impl TruthSnapshotReader for TopologySnapshotReader {
    fn snapshot_identity(&self) -> TruthSnapshotIdentity {
        self.snapshot_identity.clone()
    }

    fn read_packet(
        &self,
        request: &SnapshotReadPacket,
    ) -> Result<SnapshotReadPacketResult, BridgeSnapshotReadError> {
        let mut records = Vec::with_capacity(request.reads().len());
        for read in request.reads() {
            let record_identity = read.relational_record_identity_parts().ok_or_else(|| {
                BridgeSnapshotReadError::new(
                    "topology snapshot reader requires typed relational record identity",
                )
            })?;
            let record_ref = bridge_record_ref(record_identity)
                .map_err(|error| BridgeSnapshotReadError::new(error.to_string()))?;
            let payload = match &self.mode {
                TopologySnapshotReadMode::CurrentHead {
                    runtime,
                    version_id,
                } => {
                    let runtime = runtime
                        .read()
                        .expect("topology bridge source lock poisoned");
                    let read_view = runtime.read_truth().read_version(*version_id);
                    match record_ref {
                        RecordRef::Entity(entity_id) => {
                            let record = read_view.get_entity(entity_id).ok_or_else(|| {
                                missing_record_error(
                                    "entity",
                                    read.entity_identity(),
                                    &self.snapshot_identity,
                                )
                            })?;
                            snapshot_aspect_value_for_entity_aspect(
                                &record,
                                read.aspect_key().as_str(),
                            )
                            .ok_or_else(|| {
                                missing_aspect_error(
                                    "entity",
                                    read.aspect_key().as_str(),
                                    read.entity_identity(),
                                    &self.snapshot_identity,
                                )
                            })?
                        }
                        RecordRef::Relation(relation_id) => {
                            let record = read_view.get_relation(relation_id).ok_or_else(|| {
                                missing_record_error(
                                    "relation",
                                    read.entity_identity(),
                                    &self.snapshot_identity,
                                )
                            })?;
                            snapshot_aspect_value_for_relation_aspect(
                                &record,
                                read.aspect_key().as_str(),
                            )
                            .ok_or_else(|| {
                                missing_aspect_error(
                                    "relation",
                                    read.aspect_key().as_str(),
                                    read.entity_identity(),
                                    &self.snapshot_identity,
                                )
                            })?
                        }
                    }
                }
                TopologySnapshotReadMode::SnapshotReadOnly { read_view } => {
                    payload_from_read_view(read_view, &self.snapshot_identity, read, record_ref)?
                }
            };
            records.push(SnapshotReadRecord::for_request(read, payload));
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
    let observed_snapshot = runtime
        .publication()
        .latest_bundle()
        .and_then(|bundle| {
            (bundle.snapshot.snapshot_id == snapshot_id).then_some(bundle.snapshot.clone())
        })
        .ok_or_else(|| {
            RelationalBridgeSourceError::new(format!(
                "topology bridge snapshot identity `{}` does not resolve to the current-head published bundle",
                identity.evidence_identity().as_str()
            ))
        })?;
    if observed_snapshot.version_id != expected_version_id {
        return Err(RelationalBridgeSourceError::new(format!(
            "topology bridge snapshot identity `{}` expected version `{}` but authoritative binding resolved to version `{}`",
            identity.evidence_identity().as_str(),
            expected_version_id.0,
            observed_snapshot.version_id.0
        )));
    }
    Ok(observed_snapshot.version_id)
}

fn payload_from_read_view(
    read_view: &RelationalReadView,
    snapshot_identity: &TruthSnapshotIdentity,
    read: &forge_runtime_bridge::facade::SnapshotReadRequest,
    record_ref: RecordRef,
) -> Result<forge_foundational::facade::AspectValue, BridgeSnapshotReadError> {
    match record_ref {
        RecordRef::Entity(entity_id) => {
            let record = read_view.get_entity(entity_id).ok_or_else(|| {
                missing_record_error("entity", read.entity_identity(), snapshot_identity)
            })?;
            snapshot_aspect_value_for_entity_aspect(record, read.aspect_key().as_str()).ok_or_else(
                || {
                    missing_aspect_error(
                        "entity",
                        read.aspect_key().as_str(),
                        read.entity_identity(),
                        snapshot_identity,
                    )
                },
            )
        }
        RecordRef::Relation(relation_id) => {
            let record = read_view.get_relation(relation_id).ok_or_else(|| {
                missing_record_error("relation", read.entity_identity(), snapshot_identity)
            })?;
            snapshot_aspect_value_for_relation_aspect(record, read.aspect_key().as_str())
                .ok_or_else(|| {
                    missing_aspect_error(
                        "relation",
                        read.aspect_key().as_str(),
                        read.entity_identity(),
                        snapshot_identity,
                    )
                })
        }
    }
}
