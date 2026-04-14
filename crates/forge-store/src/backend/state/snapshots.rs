use std::collections::{BTreeMap, BTreeSet, VecDeque};

use forge_relational::facade::history::{BranchId, CommitId};

use crate::{
    authority::AuthoritativeExportBundle,
    backend::{
        integrity::{branch_key, stable_structural_digest},
        records::{
            AuthoritativeArtifactDigestRecord, AuthoritativeArtifactFamily, BranchHeadRecord,
            CommitParentRecord, SnapshotBasisRecord, SnapshotImageRecord, StoreState,
        },
    },
    failure::{StoreError, StoreErrorKind},
    snapshot::{
        stable_snapshot_digest, PublishedSnapshotHandle, SnapshotCaptureRequest, SnapshotId,
        SnapshotImageBundle, SnapshotReadMode, SnapshotReadRequest, SnapshotReadResult,
        SnapshotRestoreOutcome,
    },
};

impl StoreState {
    pub fn allocate_snapshot_id(&mut self) -> SnapshotId {
        let snapshot_id = SnapshotId(self.next_snapshot_id);
        self.next_snapshot_id += 1;
        snapshot_id
    }

    pub fn stage_snapshot_capture(
        &self,
        request: SnapshotCaptureRequest,
    ) -> Result<(Self, PublishedSnapshotHandle, usize), StoreError> {
        let image = self.build_snapshot_image(
            &request.snapshot_branch_id,
            request.snapshot_frontier_commit_id,
        )?;
        let snapshot_id = {
            let mut next = self.clone();
            next.allocate_snapshot_id()
        };
        let history_range = self.snapshot_history_range(request.snapshot_frontier_commit_id)?;
        let basis = SnapshotBasisRecord {
            snapshot_id,
            snapshot_branch_id: request.snapshot_branch_id.clone(),
            snapshot_frontier_commit_id: request.snapshot_frontier_commit_id,
            snapshot_history_range: history_range.clone(),
            snapshot_canonicalization_version: self.canonicalization_version,
            snapshot_authority_digest: stable_snapshot_digest(&(
                &request.snapshot_branch_id,
                request.snapshot_frontier_commit_id,
                &history_range,
                self.canonicalization_version,
            )),
            snapshot_image_digest: stable_snapshot_digest(&image),
        };
        let image_record = SnapshotImageRecord {
            snapshot_id,
            image: image.clone(),
        };

        let mut next = self.clone();
        next.next_snapshot_id = snapshot_id.0 + 1;
        next.snapshot_basis_records
            .insert(snapshot_id.0, basis.clone());
        next.snapshot_image_records
            .insert(snapshot_id.0, image_record);

        let handle = PublishedSnapshotHandle {
            snapshot_id,
            snapshot_branch_id: basis.snapshot_branch_id,
            snapshot_frontier_commit_id: basis.snapshot_frontier_commit_id,
            snapshot_authority_digest: basis.snapshot_authority_digest,
            snapshot_image_digest: basis.snapshot_image_digest,
        };
        let record_count = image.authoritative_export().commit_envelopes.len()
            + image.authoritative_export().commit_parent_records.len()
            + image.authoritative_export().branch_records.len()
            + image.authoritative_export().branch_head_records.len()
            + image
                .authoritative_export()
                .authoritative_artifact_digests
                .len();
        Ok((next, handle, record_count))
    }

    pub fn read_snapshot(
        &self,
        request: SnapshotReadRequest,
    ) -> Result<(SnapshotReadResult, usize), StoreError> {
        let basis = self.snapshot_basis(request.snapshot_id)?;
        let image = match request.mode {
            SnapshotReadMode::PureSnapshot => {
                if request.target_commit_id != basis.snapshot_frontier_commit_id {
                    return Err(StoreError::new(
                        StoreErrorKind::SnapshotReadBasisMismatch,
                        format!(
                            "pure snapshot read requires target frontier {}; got {}",
                            basis.snapshot_frontier_commit_id.0, request.target_commit_id.0
                        ),
                    ));
                }
                self.snapshot_image(request.snapshot_id)?
            }
            SnapshotReadMode::SnapshotPlusTail => {
                self.build_snapshot_image(&basis.snapshot_branch_id, request.target_commit_id)?
            }
        };

        if matches!(request.mode, SnapshotReadMode::SnapshotPlusTail) {
            self.require_snapshot_restore_target(&basis, request.target_commit_id)?;
        }

        let record_count = image.authoritative_export().commit_envelopes.len()
            + image.authoritative_export().commit_parent_records.len()
            + image.authoritative_export().branch_records.len()
            + image.authoritative_export().branch_head_records.len()
            + image
                .authoritative_export()
                .authoritative_artifact_digests
                .len();

        Ok((
            SnapshotReadResult {
                snapshot_id: request.snapshot_id,
                target_commit_id: request.target_commit_id,
                mode: request.mode,
                image,
            },
            record_count,
        ))
    }

    pub fn restore_snapshot(
        &self,
        snapshot_id: SnapshotId,
        target_commit_id: CommitId,
    ) -> Result<(SnapshotRestoreOutcome, usize), StoreError> {
        let basis = self.snapshot_basis(snapshot_id)?;
        self.require_snapshot_restore_target(&basis, target_commit_id)?;
        let image = self.build_snapshot_image(&basis.snapshot_branch_id, target_commit_id)?;
        let tail_commit_count = self
            .snapshot_history_range(target_commit_id)?
            .into_iter()
            .filter(|commit_id| !basis.snapshot_history_range.contains(commit_id))
            .count();
        Ok((
            SnapshotRestoreOutcome {
                snapshot_id,
                restored_branch_id: basis.snapshot_branch_id,
                restored_frontier_commit_id: target_commit_id,
                restored_image: image,
            },
            tail_commit_count,
        ))
    }

    pub fn rebuild_snapshot(
        &self,
        snapshot_id: SnapshotId,
    ) -> Result<(SnapshotImageBundle, usize), StoreError> {
        let basis = self.snapshot_basis(snapshot_id)?;
        let image = self
            .build_snapshot_image(&basis.snapshot_branch_id, basis.snapshot_frontier_commit_id)?;
        let digest = stable_snapshot_digest(&image);
        if digest != basis.snapshot_image_digest {
            return Err(StoreError::new(
                StoreErrorKind::SnapshotRebuildParityViolation,
                format!(
                    "rebuilt snapshot {} image digest {} did not match basis {}",
                    snapshot_id.0, digest, basis.snapshot_image_digest
                ),
            ));
        }
        let record_count = image.authoritative_export().commit_envelopes.len()
            + image.authoritative_export().commit_parent_records.len()
            + image.authoritative_export().branch_records.len()
            + image.authoritative_export().branch_head_records.len()
            + image
                .authoritative_export()
                .authoritative_artifact_digests
                .len();
        Ok((image, record_count))
    }

    pub fn snapshot_basis(
        &self,
        snapshot_id: SnapshotId,
    ) -> Result<SnapshotBasisRecord, StoreError> {
        self.snapshot_basis_records
            .get(&snapshot_id.0)
            .cloned()
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::SnapshotBasisUnsupported,
                    format!("snapshot basis {} not found", snapshot_id.0),
                )
            })
    }

    pub fn snapshot_image(
        &self,
        snapshot_id: SnapshotId,
    ) -> Result<SnapshotImageBundle, StoreError> {
        let basis = self.snapshot_basis(snapshot_id)?;
        let record = self
            .snapshot_image_records
            .get(&snapshot_id.0)
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::SnapshotPublicationStateGap,
                    format!(
                        "snapshot {} image missing while basis exists",
                        snapshot_id.0
                    ),
                )
            })?;
        let digest = stable_snapshot_digest(&record.image);
        if digest != basis.snapshot_image_digest {
            return Err(StoreError::new(
                StoreErrorKind::SnapshotDigestMismatch,
                format!(
                    "snapshot {} image digest {} did not match basis {}",
                    snapshot_id.0, digest, basis.snapshot_image_digest
                ),
            ));
        }
        Ok(record.image.clone())
    }

    #[cfg(test)]
    pub fn remove_snapshot_image(&mut self, snapshot_id: SnapshotId) {
        self.snapshot_image_records.remove(&snapshot_id.0);
    }

    fn require_snapshot_restore_target(
        &self,
        basis: &SnapshotBasisRecord,
        target_commit_id: CommitId,
    ) -> Result<(), StoreError> {
        let target = self.commit_record(target_commit_id).ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::SnapshotRestoreTargetIllegal,
                format!("target commit {} does not exist", target_commit_id.0),
            )
        })?;
        if target.envelope.branch_context != basis.snapshot_branch_id {
            return Err(StoreError::new(
                StoreErrorKind::SnapshotRestoreTargetIllegal,
                format!(
                    "target commit {} is on branch `{}` not snapshot branch `{}`",
                    target_commit_id.0,
                    target.envelope.branch_context.0,
                    basis.snapshot_branch_id.0
                ),
            ));
        }
        if target_commit_id == basis.snapshot_frontier_commit_id {
            return Ok(());
        }
        if basis.snapshot_history_range.contains(&target_commit_id) {
            return Err(StoreError::new(
                StoreErrorKind::SnapshotRestoreTargetIllegal,
                format!(
                    "target commit {} predates snapshot frontier {}",
                    target_commit_id.0, basis.snapshot_frontier_commit_id.0
                ),
            ));
        }
        if !self.is_descendant_of(target_commit_id, basis.snapshot_frontier_commit_id)? {
            return Err(StoreError::new(
                StoreErrorKind::SnapshotTailRangeGap,
                format!(
                    "target commit {} is not a descendant of snapshot frontier {}",
                    target_commit_id.0, basis.snapshot_frontier_commit_id.0
                ),
            ));
        }
        Ok(())
    }

    fn build_snapshot_image(
        &self,
        branch_id: &BranchId,
        frontier_commit_id: CommitId,
    ) -> Result<SnapshotImageBundle, StoreError> {
        let frontier_record = self.commit_record(frontier_commit_id).ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::SnapshotBasisUnsupported,
                format!("frontier commit {} not found", frontier_commit_id.0),
            )
        })?;
        if &frontier_record.envelope.branch_context != branch_id {
            return Err(StoreError::new(
                StoreErrorKind::SnapshotBasisAmbiguous,
                format!(
                    "frontier commit {} belongs to branch `{}` not requested branch `{}`",
                    frontier_commit_id.0, frontier_record.envelope.branch_context.0, branch_id.0
                ),
            ));
        }
        let branch_record = self
            .branch_records
            .get(&branch_key(branch_id))
            .cloned()
            .ok_or_else(|| StoreError::unknown_branch(branch_id))?;
        let history_range = self.snapshot_history_range(frontier_commit_id)?;
        let commit_set: BTreeSet<_> = history_range.iter().copied().collect();

        let commit_envelopes = history_range
            .iter()
            .map(|commit_id| {
                self.commit_record(*commit_id).cloned().ok_or_else(|| {
                    StoreError::backend_integrity("snapshot history range commit missing")
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        let commit_parent_records = history_range
            .iter()
            .flat_map(|commit_id| {
                self.commit_record(*commit_id)
                    .into_iter()
                    .flat_map(|record| record.envelope.commit.parents.iter().copied().enumerate())
                    .map(
                        move |(parent_position, parent_commit_id)| CommitParentRecord {
                            commit_id: *commit_id,
                            parent_position,
                            parent_commit_id,
                        },
                    )
            })
            .filter(|record| commit_set.contains(&record.parent_commit_id))
            .collect::<Vec<_>>();

        let branch_head_record = BranchHeadRecord {
            branch_id: branch_id.clone(),
            head_commit_id: Some(frontier_commit_id),
            head_commit_digest: Some(frontier_record.envelope_digest.clone()),
            head_update_sequence: frontier_record.commit_sequence,
        };
        let mut authoritative_artifact_digests = BTreeMap::new();
        let branch_digest = stable_structural_digest(&branch_record)?;
        authoritative_artifact_digests.insert(
            format!(
                "{:?}:{}:v{}",
                AuthoritativeArtifactFamily::BranchRecord,
                branch_record.branch_id.0,
                self.canonicalization_version
            ),
            AuthoritativeArtifactDigestRecord {
                artifact_family: AuthoritativeArtifactFamily::BranchRecord,
                artifact_id: branch_record.branch_id.0.clone(),
                canonicalization_version: self.canonicalization_version,
                digest_algorithm: "sha256".to_string(),
                artifact_digest: branch_digest,
            },
        );
        let head_digest = stable_structural_digest(&branch_head_record)?;
        authoritative_artifact_digests.insert(
            format!(
                "{:?}:{}:v{}",
                AuthoritativeArtifactFamily::BranchHeadRecord,
                branch_head_record.branch_id.0,
                self.canonicalization_version
            ),
            AuthoritativeArtifactDigestRecord {
                artifact_family: AuthoritativeArtifactFamily::BranchHeadRecord,
                artifact_id: branch_head_record.branch_id.0.clone(),
                canonicalization_version: self.canonicalization_version,
                digest_algorithm: "sha256".to_string(),
                artifact_digest: head_digest,
            },
        );
        for commit in &commit_envelopes {
            authoritative_artifact_digests.insert(
                format!(
                    "{:?}:{}:v{}",
                    AuthoritativeArtifactFamily::CommitEnvelope,
                    commit.envelope.commit.commit_id.0,
                    self.canonicalization_version
                ),
                AuthoritativeArtifactDigestRecord {
                    artifact_family: AuthoritativeArtifactFamily::CommitEnvelope,
                    artifact_id: commit.envelope.commit.commit_id.0.to_string(),
                    canonicalization_version: self.canonicalization_version,
                    digest_algorithm: "sha256".to_string(),
                    artifact_digest: commit.envelope_digest.clone(),
                },
            );
        }
        for parent in &commit_parent_records {
            let digest = stable_structural_digest(parent)?;
            authoritative_artifact_digests.insert(
                format!(
                    "{:?}:{}:{}:v{}",
                    AuthoritativeArtifactFamily::CommitParentRecord,
                    parent.commit_id.0,
                    parent.parent_position,
                    self.canonicalization_version
                ),
                AuthoritativeArtifactDigestRecord {
                    artifact_family: AuthoritativeArtifactFamily::CommitParentRecord,
                    artifact_id: format!("{}:{}", parent.commit_id.0, parent.parent_position),
                    canonicalization_version: self.canonicalization_version,
                    digest_algorithm: "sha256".to_string(),
                    artifact_digest: digest,
                },
            );
        }
        let mut bundle = AuthoritativeExportBundle {
            canonicalization_version: self.canonicalization_version,
            branch_records: vec![branch_record],
            branch_head_records: vec![branch_head_record],
            commit_envelopes,
            commit_parent_records,
            authoritative_artifact_digests: authoritative_artifact_digests.into_values().collect(),
        };
        bundle.canonicalize_order();
        Ok(SnapshotImageBundle::new(bundle))
    }

    fn snapshot_history_range(
        &self,
        frontier_commit_id: CommitId,
    ) -> Result<Vec<CommitId>, StoreError> {
        let mut visited = BTreeSet::new();
        let mut queue = VecDeque::from([frontier_commit_id]);
        while let Some(commit_id) = queue.pop_front() {
            if !visited.insert(commit_id) {
                continue;
            }
            let record = self.commit_record(commit_id).ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::SnapshotBasisUnsupported,
                    format!("snapshot frontier closure missing commit {}", commit_id.0),
                )
            })?;
            for parent_id in &record.envelope.commit.parents {
                queue.push_back(*parent_id);
            }
        }

        let mut ordered = visited
            .into_iter()
            .map(|commit_id| {
                self.commit_record(commit_id)
                    .map(|record| (record.commit_sequence, commit_id))
                    .ok_or_else(|| {
                        StoreError::backend_integrity("snapshot history range record missing")
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;
        ordered.sort_by_key(|(sequence, _)| *sequence);
        Ok(ordered
            .into_iter()
            .map(|(_, commit_id)| commit_id)
            .collect())
    }

    fn is_descendant_of(&self, target: CommitId, ancestor: CommitId) -> Result<bool, StoreError> {
        if target == ancestor {
            return Ok(true);
        }
        let mut queue = VecDeque::from([target]);
        let mut visited = BTreeSet::new();
        while let Some(commit_id) = queue.pop_front() {
            if !visited.insert(commit_id) {
                continue;
            }
            let record = self.commit_record(commit_id).ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::SnapshotTailRangeGap,
                    format!("tail traversal missing commit {}", commit_id.0),
                )
            })?;
            for parent_id in &record.envelope.commit.parents {
                if *parent_id == ancestor {
                    return Ok(true);
                }
                queue.push_back(*parent_id);
            }
        }
        Ok(false)
    }
}
