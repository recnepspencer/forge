use crate::facade::BridgeCommittedPatchEnvelope;
use crate::input::envelope::{BridgeCommittedPatchItem, TruthBranchIdentity, TruthPatchIdentity};
use crate::snapshot::{SnapshotReadPacket, TruthSnapshotIdentity, TruthSnapshotReader};

#[derive(Clone)]
pub(super) struct StaticSource;

struct StaticSnapshotReader;

impl TruthSnapshotReader for StaticSnapshotReader {
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
                    crate::snapshot::SnapshotReadRecord::for_request(
                        read,
                        forge_foundational::facade::AspectValue::String("fixture-value".into()),
                    )
                })
                .collect(),
        ))
    }
}

impl crate::adapter::CommittedPatchSource for StaticSource {
    fn load_committed_patch(
        &self,
        request: crate::adapter::RelationalCommittedPatchRequest,
    ) -> Result<
        crate::input::envelope::BridgeCommittedPatchEnvelope,
        crate::adapter::RelationalBridgeSourceError,
    > {
        BridgeCommittedPatchEnvelope::new(
            crate::input::envelope::BridgeCommittedPatchEnvelopeIdentity::new(
                request.commit_identity().clone(),
                TruthPatchIdentity::new(format!("patch-for-{}", request.commit_identity())),
                TruthSnapshotIdentity::new("snapshot-a"),
                TruthBranchIdentity::new("analysis"),
            ),
            vec![BridgeCommittedPatchItem::with_target(
                "entity-1",
                profile_name_patch_target(),
            )],
        )
        .map_err(|error| crate::adapter::RelationalBridgeSourceError::new(error.to_string()))
    }
}

impl crate::adapter::SnapshotReadSource for StaticSource {
    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, crate::adapter::RelationalBridgeSourceError> {
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
        crate::input::envelope::BridgeCommittedPatchEnvelope,
        crate::adapter::RelationalBridgeSourceError,
    > {
        BridgeCommittedPatchEnvelope::new(
            crate::input::envelope::BridgeCommittedPatchEnvelopeIdentity::new(
                crate::facade::TruthCommitIdentity::new(format!(
                    "head-{}",
                    branch_identity.as_str()
                )),
                TruthPatchIdentity::new(format!("patch-{}", branch_identity.as_str())),
                TruthSnapshotIdentity::new("snapshot-a"),
                branch_identity.clone(),
            ),
            vec![BridgeCommittedPatchItem::with_target(
                "entity-1",
                profile_name_patch_target(),
            )],
        )
        .map_err(|error| crate::adapter::RelationalBridgeSourceError::new(error.to_string()))
    }
}

pub(super) struct StaticSink;

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
pub(crate) struct MisbindingSource;

struct MisbindingSnapshotReader;

impl TruthSnapshotReader for MisbindingSnapshotReader {
    fn snapshot_identity(&self) -> TruthSnapshotIdentity {
        TruthSnapshotIdentity::new("snapshot-bad")
    }

    fn read_packet(
        &self,
        request: &SnapshotReadPacket,
    ) -> Result<crate::snapshot::SnapshotReadPacketResult, crate::snapshot::BridgeSnapshotReadError>
    {
        Ok(crate::snapshot::SnapshotReadPacketResult::new(
            TruthSnapshotIdentity::new("snapshot-bad"),
            request
                .reads()
                .iter()
                .map(|read| {
                    crate::snapshot::SnapshotReadRecord::for_request(
                        read,
                        forge_foundational::facade::AspectValue::String("fixture-value".into()),
                    )
                })
                .collect(),
        ))
    }
}

impl crate::adapter::CommittedPatchSource for MisbindingSource {
    fn load_committed_patch(
        &self,
        request: crate::adapter::RelationalCommittedPatchRequest,
    ) -> Result<
        crate::input::envelope::BridgeCommittedPatchEnvelope,
        crate::adapter::RelationalBridgeSourceError,
    > {
        StaticSource.load_committed_patch(request)
    }
}

impl crate::adapter::SnapshotReadSource for MisbindingSource {
    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, crate::adapter::RelationalBridgeSourceError> {
        if identity.as_str() == "snapshot-a" {
            Ok(Box::new(MisbindingSnapshotReader))
        } else {
            Err(crate::adapter::RelationalBridgeSourceError::new(format!(
                "unknown snapshot `{}`",
                identity.as_str()
            )))
        }
    }
}

impl crate::adapter::TruthBranchHeadSource for MisbindingSource {
    fn load_branch_head_patch(
        &self,
        branch_identity: &TruthBranchIdentity,
    ) -> Result<
        crate::input::envelope::BridgeCommittedPatchEnvelope,
        crate::adapter::RelationalBridgeSourceError,
    > {
        StaticSource.load_branch_head_patch(branch_identity)
    }
}

#[derive(Clone)]
pub(crate) struct WrongBranchHeadSource;

impl crate::adapter::CommittedPatchSource for WrongBranchHeadSource {
    fn load_committed_patch(
        &self,
        request: crate::adapter::RelationalCommittedPatchRequest,
    ) -> Result<
        crate::input::envelope::BridgeCommittedPatchEnvelope,
        crate::adapter::RelationalBridgeSourceError,
    > {
        StaticSource.load_committed_patch(request)
    }
}

impl crate::adapter::SnapshotReadSource for WrongBranchHeadSource {
    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, crate::adapter::RelationalBridgeSourceError> {
        StaticSource.open_snapshot(identity)
    }
}

impl crate::adapter::TruthBranchHeadSource for WrongBranchHeadSource {
    fn load_branch_head_patch(
        &self,
        _branch_identity: &TruthBranchIdentity,
    ) -> Result<
        crate::input::envelope::BridgeCommittedPatchEnvelope,
        crate::adapter::RelationalBridgeSourceError,
    > {
        BridgeCommittedPatchEnvelope::new(
            crate::input::envelope::BridgeCommittedPatchEnvelopeIdentity::new(
                crate::facade::TruthCommitIdentity::new("head-wrong"),
                TruthPatchIdentity::new("patch-wrong"),
                TruthSnapshotIdentity::new("snapshot-a"),
                TruthBranchIdentity::new("wrong-branch"),
            ),
            vec![BridgeCommittedPatchItem::with_target(
                "entity-1",
                profile_name_patch_target(),
            )],
        )
        .map_err(|error| crate::adapter::RelationalBridgeSourceError::new(error.to_string()))
    }
}

fn profile_name_patch_target() -> crate::facade::BridgeCommittedPatchTarget {
    crate::facade::BridgeCommittedPatchTarget::entity_field_path(
        forge_foundational::facade::AspectLocator::new(
            forge_foundational::facade::LocatorAuthority::Authoritative,
            profile_aspect_key(),
        ),
        forge_foundational::facade::CanonicalFieldPath::single(profile_name_field_key()),
    )
}

pub(super) fn profile_aspect_key() -> forge_foundational::facade::AspectKey {
    forge_foundational::facade::AspectKey::new("profile").expect("valid native aspect key")
}

pub(super) fn profile_name_field_key() -> forge_foundational::facade::FieldKey {
    forge_foundational::facade::FieldKey::new("name".to_owned()).expect("valid native field key")
}
