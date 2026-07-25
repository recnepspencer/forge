use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};
use worth_store_authority::RecoveryWriteFenceReceipt;
use worth_store_physical_backend::ClosedNonCurrentStagingMedia;

use crate::{
    CopyOnWritePublicationPlan, CurrentPhysicalRoot, NewRootPublicationProof,
    OldReachabilityPreservation, PhysicalPublicationIntent, PhysicalPublicationReadiness,
    PhysicalRootPublicationRuntime, PublicationLatchReadiness, PublicationRootCandidate,
    PublicationRootSuccessorOwner, RootSwapOrderingContract,
};

mod durable_locator;
mod publication_start;
mod reopen_request;

use durable_locator::DurableRecoveryPublicationLocator;
pub use publication_start::RecoveryPublicationStartPosture;
pub use reopen_request::{
    ReopenRecoveryPublicationByIdentityRequest, ReopenRecoveryPublicationRequest,
};

#[derive(Debug, Clone)]
pub struct RecoveryPublicationPlanRequest {
    publication_directory: PathBuf,
    current_root: PublicationRootCandidate,
    old_reachability: OldReachabilityPreservation,
    staged_media: ClosedNonCurrentStagingMedia,
    staged_root_generation: u64,
    cutover_plan_fingerprint: [u8; 32],
}

impl RecoveryPublicationPlanRequest {
    pub fn new(
        publication_directory: impl Into<PathBuf>,
        current_root: PublicationRootCandidate,
        old_reachability: OldReachabilityPreservation,
        staged_media: ClosedNonCurrentStagingMedia,
        staged_root_generation: u64,
        cutover_plan_fingerprint: [u8; 32],
    ) -> Self {
        Self {
            publication_directory: publication_directory.into(),
            current_root,
            old_reachability,
            staged_media,
            staged_root_generation,
            cutover_plan_fingerprint,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryPublicationDenial {
    InvalidBinding,
    StagedMediaNotClosed,
    StagedMediaIdentityMismatch,
    Physical(crate::PhysicalPublicationDenial),
    WriteFenceMismatch,
    RootPublicationBindingMismatch,
    PublicationLocatorIo,
    PublicationLocatorConflict,
    PublicationLocatorPathTooLong,
}

#[derive(Debug, Clone)]
pub struct RecoveryPublicationLoweredPlan {
    fingerprint: [u8; 32],
    publication_identity: [u8; 32],
    publication_directory: PathBuf,
    expected_current_root: CurrentPhysicalRoot,
    candidate_root: CurrentPhysicalRoot,
    candidate_media_root: PathBuf,
    staging_plan_fingerprint: [u8; 32],
    candidate_media_identity: [u8; 32],
    cutover_plan_fingerprint: [u8; 32],
    physical: CopyOnWritePublicationPlan,
}

impl RecoveryPublicationLoweredPlan {
    pub const fn fingerprint(&self) -> [u8; 32] {
        self.fingerprint
    }
    pub const fn candidate_media_identity(&self) -> [u8; 32] {
        self.candidate_media_identity
    }
    pub const fn candidate_root(&self) -> CurrentPhysicalRoot {
        self.candidate_root
    }
    pub const fn publication_identity(&self) -> [u8; 32] {
        self.publication_identity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryPublicationPosture {
    PublishedNow,
    RecoveredAlreadyPublished,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtomicRecoveryPublicationReceipt {
    publication_identity: [u8; 32],
    plan_fingerprint: [u8; 32],
    candidate_media_identity: [u8; 32],
    candidate_media_root: PathBuf,
    current_root: CurrentPhysicalRoot,
    posture: RecoveryPublicationPosture,
}

impl AtomicRecoveryPublicationReceipt {
    pub const fn publication_identity(&self) -> [u8; 32] {
        self.publication_identity
    }
    pub const fn plan_fingerprint(&self) -> [u8; 32] {
        self.plan_fingerprint
    }
    pub const fn candidate_media_identity(&self) -> [u8; 32] {
        self.candidate_media_identity
    }
    pub fn candidate_media_root(&self) -> &Path {
        &self.candidate_media_root
    }
    pub const fn current_root(&self) -> CurrentPhysicalRoot {
        self.current_root
    }
    pub const fn posture(&self) -> RecoveryPublicationPosture {
        self.posture
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RecoveryPublicationOwner;

impl RecoveryPublicationOwner {
    pub fn lower(
        request: RecoveryPublicationPlanRequest,
    ) -> Result<RecoveryPublicationLoweredPlan, RecoveryPublicationDenial> {
        if request.cutover_plan_fingerprint == [0; 32] || request.staged_root_generation == 0 {
            return Err(RecoveryPublicationDenial::InvalidBinding);
        }
        validate_closed_media(&request.staged_media)?;
        let old_root = request.current_root.root();
        let generation = worth_store_physical_format::PhysicalGeneration::from_raw(
            request.staged_root_generation,
        )
        .map_err(|_| RecoveryPublicationDenial::InvalidBinding)?;
        let new_candidate = PublicationRootSuccessorOwner::plan(request.current_root, generation)
            .map_err(RecoveryPublicationDenial::Physical)?;
        let candidate_root = new_candidate.root();
        let validation = new_candidate.validation();
        let validated = PhysicalPublicationIntent::copy_on_write_root_manifest(
            request.current_root,
            new_candidate,
            request.old_reachability,
        )
        .validate_copy_on_write_inputs()
        .map_err(RecoveryPublicationDenial::Physical)?;
        let lowered = validated
            .clone()
            .lower_with_ordering(RootSwapOrderingContract::acquire_release_or_stronger())
            .map_err(RecoveryPublicationDenial::Physical)?;
        let readiness = PhysicalPublicationReadiness::from_validated_intent(
            &validated,
            NewRootPublicationProof::from_root_validation(validation),
            PublicationLatchReadiness::declared_publish_latches_released_before_blocking_io(),
        );
        let physical = lowered
            .join_readiness(readiness)
            .map_err(RecoveryPublicationDenial::Physical)?;
        let candidate_media_identity = request.staged_media.content_fingerprint();
        let fingerprint = plan_fingerprint(
            old_root,
            candidate_root,
            candidate_media_identity,
            request.cutover_plan_fingerprint,
        );
        let publication_identity = publication_identity_from_coordinates(
            fingerprint,
            candidate_media_identity,
            candidate_root.epoch().get(),
        );
        Ok(RecoveryPublicationLoweredPlan {
            fingerprint,
            publication_identity,
            publication_directory: request.publication_directory,
            expected_current_root: old_root,
            candidate_root,
            candidate_media_root: request.staged_media.root().to_path_buf(),
            staging_plan_fingerprint: request.staged_media.plan_fingerprint(),
            candidate_media_identity,
            cutover_plan_fingerprint: request.cutover_plan_fingerprint,
            physical,
        })
    }

    pub fn publish(
        plan: RecoveryPublicationLoweredPlan,
        fence: RecoveryWriteFenceReceipt,
    ) -> Result<AtomicRecoveryPublicationReceipt, RecoveryPublicationDenial> {
        if fence.cutover_plan_fingerprint() != plan.cutover_plan_fingerprint
            || fence.candidate_media_identity() != plan.candidate_media_identity
            || fence.fenced_authority() != plan.expected_current_root.store_authority_identity()
        {
            return Err(RecoveryPublicationDenial::WriteFenceMismatch);
        }
        validate_closed_path(&plan.candidate_media_root, plan.staging_plan_fingerprint)?;
        let publication_identity = plan.publication_identity;
        DurableRecoveryPublicationLocator::admit_or_persist(
            &plan.publication_directory,
            publication_identity,
            plan.fingerprint,
            plan.candidate_media_identity,
            plan.staging_plan_fingerprint,
            plan.candidate_root,
            &plan.candidate_media_root,
        )?;
        let posture = match PhysicalRootPublicationRuntime::open(
            &plan.publication_directory,
            plan.expected_current_root,
        ) {
            Ok(mut runtime) => {
                runtime
                    .publish_recovery(plan.physical.clone(), publication_identity)
                    .map_err(RecoveryPublicationDenial::Physical)?;
                RecoveryPublicationPosture::PublishedNow
            }
            Err(crate::PhysicalPublicationDenial::PersistedRootMismatch) => {
                let runtime = PhysicalRootPublicationRuntime::open(
                    &plan.publication_directory,
                    plan.candidate_root,
                )
                .map_err(RecoveryPublicationDenial::Physical)?;
                if runtime
                    .current_recovery_binding()
                    .map_err(RecoveryPublicationDenial::Physical)?
                    != Some(publication_identity)
                {
                    return Err(RecoveryPublicationDenial::RootPublicationBindingMismatch);
                }
                RecoveryPublicationPosture::RecoveredAlreadyPublished
            }
            Err(denial) => return Err(RecoveryPublicationDenial::Physical(denial)),
        };
        Ok(publication_receipt(&plan, posture))
    }

    pub fn reopen_published(
        request: ReopenRecoveryPublicationRequest,
    ) -> Result<AtomicRecoveryPublicationReceipt, RecoveryPublicationDenial> {
        let runtime = PhysicalRootPublicationRuntime::open(
            &request.publication_directory,
            request.current_root,
        )
        .map_err(RecoveryPublicationDenial::Physical)?;
        if runtime
            .current_recovery_binding()
            .map_err(RecoveryPublicationDenial::Physical)?
            != Some(request.publication_identity)
        {
            return Err(RecoveryPublicationDenial::RootPublicationBindingMismatch);
        }
        let locator = DurableRecoveryPublicationLocator::reopen(
            &request.publication_directory,
            request.publication_identity,
            request.current_root,
        )?;
        if locator.plan_fingerprint != request.publication_plan_fingerprint
            || locator.media_identity != request.candidate_media_identity
        {
            return Err(RecoveryPublicationDenial::PublicationLocatorConflict);
        }
        validate_closed_path(&locator.media_root, locator.staging_plan_fingerprint)?;
        Ok(AtomicRecoveryPublicationReceipt {
            publication_identity: request.publication_identity,
            plan_fingerprint: locator.plan_fingerprint,
            candidate_media_identity: locator.media_identity,
            candidate_media_root: locator.media_root,
            current_root: request.current_root,
            posture: RecoveryPublicationPosture::RecoveredAlreadyPublished,
        })
    }

    pub fn reopen_published_by_identity(
        request: ReopenRecoveryPublicationByIdentityRequest,
    ) -> Result<AtomicRecoveryPublicationReceipt, RecoveryPublicationDenial> {
        let locator = DurableRecoveryPublicationLocator::reopen_by_binding(
            &request.publication_directory,
            request.publication_identity,
        )?;
        if locator.plan_fingerprint != request.publication_plan_fingerprint
            || locator.media_identity != request.candidate_media_identity
        {
            return Err(RecoveryPublicationDenial::PublicationLocatorConflict);
        }
        let runtime = PhysicalRootPublicationRuntime::open(
            &request.publication_directory,
            locator.candidate_root,
        )
        .map_err(RecoveryPublicationDenial::Physical)?;
        if runtime
            .current_recovery_binding()
            .map_err(RecoveryPublicationDenial::Physical)?
            != Some(request.publication_identity)
        {
            return Err(RecoveryPublicationDenial::RootPublicationBindingMismatch);
        }
        validate_closed_path(&locator.media_root, locator.staging_plan_fingerprint)?;
        Ok(AtomicRecoveryPublicationReceipt {
            publication_identity: request.publication_identity,
            plan_fingerprint: locator.plan_fingerprint,
            candidate_media_identity: locator.media_identity,
            candidate_media_root: locator.media_root,
            current_root: locator.candidate_root,
            posture: RecoveryPublicationPosture::RecoveredAlreadyPublished,
        })
    }
}

fn validate_closed_media(
    media: &ClosedNonCurrentStagingMedia,
) -> Result<(), RecoveryPublicationDenial> {
    let marker = std::fs::read(media.root().join(".closed-staging"))
        .map_err(|_| RecoveryPublicationDenial::StagedMediaNotClosed)?;
    if marker.len() == 64 && marker[..32] == media.plan_fingerprint() {
        Ok(())
    } else {
        Err(RecoveryPublicationDenial::StagedMediaIdentityMismatch)
    }
}
fn validate_closed_path(
    path: &Path,
    expected_plan: [u8; 32],
) -> Result<(), RecoveryPublicationDenial> {
    let identity = std::fs::read(path.join(".staging-identity"))
        .map_err(|_| RecoveryPublicationDenial::StagedMediaNotClosed)?;
    let closed = std::fs::read(path.join(".closed-staging"))
        .map_err(|_| RecoveryPublicationDenial::StagedMediaNotClosed)?;
    if identity.as_slice() == expected_plan && closed.len() == 64 && closed[..32] == expected_plan {
        Ok(())
    } else {
        Err(RecoveryPublicationDenial::StagedMediaIdentityMismatch)
    }
}
fn plan_fingerprint(
    old: CurrentPhysicalRoot,
    new: CurrentPhysicalRoot,
    media: [u8; 32],
    cutover: [u8; 32],
) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"worth-store-recovery-publication-plan-v1");
    digest.update(old.epoch().get().to_be_bytes());
    digest.update(new.epoch().get().to_be_bytes());
    digest.update(media);
    digest.update(cutover);
    digest.finalize().into()
}
fn publication_receipt(
    plan: &RecoveryPublicationLoweredPlan,
    posture: RecoveryPublicationPosture,
) -> AtomicRecoveryPublicationReceipt {
    AtomicRecoveryPublicationReceipt {
        publication_identity: plan.publication_identity,
        plan_fingerprint: plan.fingerprint,
        candidate_media_identity: plan.candidate_media_identity,
        candidate_media_root: plan.candidate_media_root.clone(),
        current_root: plan.candidate_root,
        posture,
    }
}

fn publication_identity_from_coordinates(
    fingerprint: [u8; 32],
    media: [u8; 32],
    epoch: u64,
) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"worth-store-atomic-recovery-publication-v1");
    digest.update(fingerprint);
    digest.update(media);
    digest.update(epoch.to_be_bytes());
    digest.finalize().into()
}
