use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};
use worth_store_physical_backend::ClosedNonCurrentStagingMedia;

use crate::{BackupStructuralVerificationDenial, BackupVerificationBudget};

mod authority_posture;
mod owner_verification;

pub use authority_posture::{StagedRecoveryAuthorityPosture, StagedRecoveryRegionPosture};
pub use owner_verification::StagedRecoveryOwnerVerificationSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StagedRecoveryExpectedFrontier {
    checkpoint_durability: u64,
    wal_structural: u64,
    acknowledged: u64,
}

impl StagedRecoveryExpectedFrontier {
    pub const fn exact(
        checkpoint_durability: u64,
        wal_structural: u64,
        acknowledged: u64,
    ) -> Option<Self> {
        if checkpoint_durability <= wal_structural && wal_structural <= acknowledged {
            Some(Self {
                checkpoint_durability,
                wal_structural,
                acknowledged,
            })
        } else {
            None
        }
    }
    pub const fn checkpoint_durability(self) -> u64 {
        self.checkpoint_durability
    }
    pub const fn wal_structural(self) -> u64 {
        self.wal_structural
    }
    pub const fn acknowledged(self) -> u64 {
        self.acknowledged
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClosedStagedRecoveryVerificationRequest {
    root: PathBuf,
    staging_plan_fingerprint: [u8; 32],
    staged_content_fingerprint: [u8; 32],
    expected_frontier: StagedRecoveryExpectedFrontier,
}

impl ClosedStagedRecoveryVerificationRequest {
    pub fn from_closed_media(
        media: &ClosedNonCurrentStagingMedia,
        expected_frontier: StagedRecoveryExpectedFrontier,
    ) -> Self {
        Self {
            root: media.root().to_path_buf(),
            staging_plan_fingerprint: media.plan_fingerprint(),
            staged_content_fingerprint: media.content_fingerprint(),
            expected_frontier,
        }
    }

    pub fn from_reopened_published_media(
        root: impl Into<PathBuf>,
        staging_plan_fingerprint: [u8; 32],
        staged_content_fingerprint: [u8; 32],
        expected_frontier: StagedRecoveryExpectedFrontier,
    ) -> Option<Self> {
        if staging_plan_fingerprint == [0; 32] || staged_content_fingerprint == [0; 32] {
            return None;
        }
        Some(Self {
            root: root.into(),
            staging_plan_fingerprint,
            staged_content_fingerprint,
            expected_frontier,
        })
    }
}

#[derive(Debug)]
pub enum StagedRecoveryPostVerificationDenial {
    MediaNotClosed,
    PendingExecutionResidue {
        path: PathBuf,
    },
    StagingIdentityMismatch,
    StagedContentFingerprintMismatch,
    AllocationFailed,
    Format(worth_store_physical_format::BackupBundleFormatDenial),
    Structural(BackupStructuralVerificationDenial),
    FrontierMismatch {
        expected: StagedRecoveryExpectedFrontier,
        observed: StagedRecoveryExpectedFrontier,
    },
    Io,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PostVerifiedStagedRecovery {
    root: PathBuf,
    staging_plan_fingerprint: [u8; 32],
    staged_content_fingerprint: [u8; 32],
    verification_identity: [u8; 32],
    manifest_digest: [u8; 32],
    observed_frontier: StagedRecoveryExpectedFrontier,
    root_generation: u64,
    authority_posture: StagedRecoveryAuthorityPosture,
    owner_verification: StagedRecoveryOwnerVerificationSet,
}

impl PostVerifiedStagedRecovery {
    pub fn root(&self) -> &Path {
        &self.root
    }
    pub const fn staging_plan_fingerprint(&self) -> [u8; 32] {
        self.staging_plan_fingerprint
    }
    pub const fn staged_content_fingerprint(&self) -> [u8; 32] {
        self.staged_content_fingerprint
    }
    pub const fn verification_identity(&self) -> [u8; 32] {
        self.verification_identity
    }
    pub const fn manifest_digest(&self) -> [u8; 32] {
        self.manifest_digest
    }
    pub const fn observed_frontier(&self) -> StagedRecoveryExpectedFrontier {
        self.observed_frontier
    }
    pub const fn root_generation(&self) -> u64 {
        self.root_generation
    }
    pub const fn authority_posture(&self) -> StagedRecoveryAuthorityPosture {
        self.authority_posture
    }
    pub const fn owner_verification(&self) -> StagedRecoveryOwnerVerificationSet {
        self.owner_verification
    }
}

pub fn post_verify_closed_staged_recovery(
    request: ClosedStagedRecoveryVerificationRequest,
    budget: BackupVerificationBudget,
) -> Result<PostVerifiedStagedRecovery, StagedRecoveryPostVerificationDenial> {
    let owner_effect_fingerprint = verify_closed_markers(&request)?;
    let materialized = worth_store_physical_format::BackupBundleFormatAuthority::canonical()
        .admit_materialized_with_limits(&request.root, budget.manifest())
        .map_err(StagedRecoveryPostVerificationDenial::Format)?;
    let manifest_digest = materialized.manifest_digest();
    let root_generation = materialized.manifest().root_generation();
    let observed = StagedRecoveryExpectedFrontier::exact(
        materialized.manifest().durable_checkpoint_lsn(),
        materialized.manifest().wal_half_open_interval().1,
        materialized.manifest().acknowledged_frontier(),
    )
    .ok_or(StagedRecoveryPostVerificationDenial::Io)?;
    if observed != request.expected_frontier {
        return Err(StagedRecoveryPostVerificationDenial::FrontierMismatch {
            expected: request.expected_frontier,
            observed,
        });
    }
    let verified = crate::backup_verification::verify_staged_materialized_backup(
        materialized,
        budget,
        request.staging_plan_fingerprint,
        owner_effect_fingerprint,
    )
    .map_err(StagedRecoveryPostVerificationDenial::Structural)?;
    let authority_posture =
        StagedRecoveryAuthorityPosture::from_truth(verified.operational_truth())?;
    let owner_verification = StagedRecoveryOwnerVerificationSet::for_manifest(
        verified.materialized().manifest(),
        manifest_digest,
    )
    .ok_or(StagedRecoveryPostVerificationDenial::Io)?;
    let observed_content_fingerprint =
        verified_content_fingerprint(&verified, manifest_digest, owner_effect_fingerprint)?;
    if observed_content_fingerprint != request.staged_content_fingerprint {
        return Err(StagedRecoveryPostVerificationDenial::StagedContentFingerprintMismatch);
    }
    let mut identity = Sha256::new();
    identity.update(b"worth-store-post-verified-staged-recovery-v1");
    identity.update(request.staging_plan_fingerprint);
    identity.update(request.staged_content_fingerprint);
    identity.update(verified.verification_identity());
    identity.update(manifest_digest);
    Ok(PostVerifiedStagedRecovery {
        root: request.root,
        staging_plan_fingerprint: request.staging_plan_fingerprint,
        staged_content_fingerprint: request.staged_content_fingerprint,
        verification_identity: identity.finalize().into(),
        manifest_digest,
        observed_frontier: observed,
        root_generation,
        authority_posture,
        owner_verification,
    })
}

fn verified_content_fingerprint(
    verified: &crate::StructurallyVerifiedBackupBundle,
    manifest_digest: [u8; 32],
    owner_effect_fingerprint: [u8; 32],
) -> Result<[u8; 32], StagedRecoveryPostVerificationDenial> {
    let rows = verified.materialized().manifest().artifacts();
    let mut artifacts = Vec::new();
    artifacts
        .try_reserve_exact(rows.len().saturating_add(1))
        .map_err(|_| StagedRecoveryPostVerificationDenial::AllocationFailed)?;
    artifacts.push(("backup.manifest", manifest_digest));
    artifacts.extend(
        rows.iter()
            .map(|row| (row.output_name(), row.content_digest())),
    );
    artifacts.sort_by(|left, right| left.0.cmp(right.0));
    let mut content = Sha256::new();
    for (_, digest) in artifacts {
        content.update(digest);
    }
    content.update(b"worth-store-staged-owner-effect-v1");
    content.update(owner_effect_fingerprint);
    Ok(content.finalize().into())
}

fn verify_closed_markers(
    request: &ClosedStagedRecoveryVerificationRequest,
) -> Result<[u8; 32], StagedRecoveryPostVerificationDenial> {
    let identity = std::fs::read(request.root.join(".staging-identity"))
        .map_err(|_| StagedRecoveryPostVerificationDenial::MediaNotClosed)?;
    let closed = std::fs::read(request.root.join(".closed-staging"))
        .map_err(|_| StagedRecoveryPostVerificationDenial::MediaNotClosed)?;
    if identity.as_slice() != request.staging_plan_fingerprint
        || closed.len() != 64
        || closed[..32] != request.staging_plan_fingerprint
    {
        return Err(StagedRecoveryPostVerificationDenial::StagingIdentityMismatch);
    }
    let entries =
        std::fs::read_dir(&request.root).map_err(|_| StagedRecoveryPostVerificationDenial::Io)?;
    for entry in entries {
        let path = entry
            .map_err(|_| StagedRecoveryPostVerificationDenial::Io)?
            .path();
        if path
            .extension()
            .is_some_and(|extension| extension == "pending")
        {
            return Err(StagedRecoveryPostVerificationDenial::PendingExecutionResidue { path });
        }
    }
    let mut owner_effect_fingerprint = [0_u8; 32];
    owner_effect_fingerprint.copy_from_slice(&closed[32..]);
    Ok(owner_effect_fingerprint)
}
