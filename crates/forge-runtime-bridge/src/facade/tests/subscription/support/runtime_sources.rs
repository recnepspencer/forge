use super::*;

#[derive(Clone)]
pub(crate) struct MisbindingSource;

#[derive(Clone)]
pub(crate) struct WrongBranchHeadSource;

pub(crate) struct MisbindingSnapshotReader;

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
        BridgeCommittedPatchEnvelope::new(
            BridgeCommittedPatchEnvelopeIdentity::new(
                request.commit_identity().clone(),
                TruthPatchIdentity::new(format!("patch-for-{}", request.commit_identity())),
                TruthSnapshotIdentity::new("snapshot-a"),
                TruthBranchIdentity::new("analysis"),
            ),
            vec![BridgeCommittedPatchItem::with_target(
                "entity-1",
                crate::facade::BridgeCommittedPatchTarget::entity_field_path(
                    forge_foundational::facade::AspectLocator::new(
                        forge_foundational::facade::LocatorAuthority::Authoritative,
                        forge_foundational::facade::AspectKey::new("profile")
                            .expect("valid bridge patch aspect key"),
                    ),
                    forge_foundational::facade::CanonicalFieldPath::single(
                        forge_foundational::facade::FieldKey::new("name".to_owned())
                            .expect("valid foundational field key"),
                    ),
                ),
            )],
        )
        .map_err(|error| crate::adapter::RelationalBridgeSourceError::new(error.to_string()))
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
        BridgeCommittedPatchEnvelope::new(
            BridgeCommittedPatchEnvelopeIdentity::new(
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
                crate::facade::BridgeCommittedPatchTarget::entity_field_path(
                    forge_foundational::facade::AspectLocator::new(
                        forge_foundational::facade::LocatorAuthority::Authoritative,
                        forge_foundational::facade::AspectKey::new("profile")
                            .expect("valid bridge patch aspect key"),
                    ),
                    forge_foundational::facade::CanonicalFieldPath::single(
                        forge_foundational::facade::FieldKey::new("name".to_owned())
                            .expect("valid foundational field key"),
                    ),
                ),
            )],
        )
        .map_err(|error| crate::adapter::RelationalBridgeSourceError::new(error.to_string()))
    }
}

impl crate::adapter::CommittedPatchSource for WrongBranchHeadSource {
    fn load_committed_patch(
        &self,
        request: crate::adapter::RelationalCommittedPatchRequest,
    ) -> Result<
        crate::input::envelope::BridgeCommittedPatchEnvelope,
        crate::adapter::RelationalBridgeSourceError,
    > {
        BridgeCommittedPatchEnvelope::new(
            BridgeCommittedPatchEnvelopeIdentity::new(
                request.commit_identity().clone(),
                TruthPatchIdentity::new(format!("patch-for-{}", request.commit_identity())),
                TruthSnapshotIdentity::new("snapshot-a"),
                TruthBranchIdentity::new("analysis"),
            ),
            vec![BridgeCommittedPatchItem::with_target(
                "entity-1",
                crate::facade::BridgeCommittedPatchTarget::entity_field_path(
                    forge_foundational::facade::AspectLocator::new(
                        forge_foundational::facade::LocatorAuthority::Authoritative,
                        forge_foundational::facade::AspectKey::new("profile")
                            .expect("valid bridge patch aspect key"),
                    ),
                    forge_foundational::facade::CanonicalFieldPath::single(
                        forge_foundational::facade::FieldKey::new("name".to_owned())
                            .expect("valid foundational field key"),
                    ),
                ),
            )],
        )
        .map_err(|error| crate::adapter::RelationalBridgeSourceError::new(error.to_string()))
    }
}

impl crate::adapter::SnapshotReadSource for WrongBranchHeadSource {
    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, crate::adapter::RelationalBridgeSourceError> {
        if identity.as_str() == "snapshot-a" {
            Ok(Box::new(super::super::super::StaticSnapshotReader))
        } else {
            Err(crate::adapter::RelationalBridgeSourceError::new(format!(
                "unknown snapshot `{}`",
                identity.as_str()
            )))
        }
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
            BridgeCommittedPatchEnvelopeIdentity::new(
                crate::facade::TruthCommitIdentity::new("head-wrong"),
                TruthPatchIdentity::new("patch-wrong"),
                TruthSnapshotIdentity::new("snapshot-a"),
                TruthBranchIdentity::new("wrong-branch"),
            ),
            vec![BridgeCommittedPatchItem::with_target(
                "entity-1",
                crate::facade::BridgeCommittedPatchTarget::entity_field_path(
                    forge_foundational::facade::AspectLocator::new(
                        forge_foundational::facade::LocatorAuthority::Authoritative,
                        forge_foundational::facade::AspectKey::new("profile")
                            .expect("valid bridge patch aspect key"),
                    ),
                    forge_foundational::facade::CanonicalFieldPath::single(
                        forge_foundational::facade::FieldKey::new("name".to_owned())
                            .expect("valid foundational field key"),
                    ),
                ),
            )],
        )
        .map_err(|error| crate::adapter::RelationalBridgeSourceError::new(error.to_string()))
    }
}
