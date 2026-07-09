use crate::{PhysicalScenarioActor, PhysicalScenarioActorRole};

use super::ScheduleReplayDenial;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalActorId(String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalActorStep {
    step_index: u32,
    actor_id: PhysicalActorId,
    actor_role: PhysicalScenarioActorRole,
    yieldpoint: String,
}

impl PhysicalActorId {
    pub(crate) fn from_actor(actor: &PhysicalScenarioActor) -> Result<Self, ScheduleReplayDenial> {
        if actor.id().trim().is_empty() {
            return Err(ScheduleReplayDenial::MissingActorId);
        }
        Ok(Self(actor.id().to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl PhysicalActorStep {
    pub(crate) fn from_actor(
        step_index: u32,
        actor: &PhysicalScenarioActor,
        yieldpoint: &str,
    ) -> Result<Self, ScheduleReplayDenial> {
        let actor_id = PhysicalActorId::from_actor(actor)?;
        Ok(Self {
            step_index,
            actor_id,
            actor_role: actor.role(),
            yieldpoint: yieldpoint.to_owned(),
        })
    }

    pub const fn step_index(&self) -> u32 {
        self.step_index
    }

    pub fn actor_id(&self) -> &str {
        self.actor_id.as_str()
    }

    pub const fn actor_id_proof(&self) -> &PhysicalActorId {
        &self.actor_id
    }

    pub const fn actor_role(&self) -> PhysicalScenarioActorRole {
        self.actor_role
    }

    pub fn yieldpoint(&self) -> &str {
        &self.yieldpoint
    }
}

pub(crate) fn actor_role_token(role: PhysicalScenarioActorRole) -> &'static str {
    match role {
        PhysicalScenarioActorRole::ForegroundReader => "foreground-reader",
        PhysicalScenarioActorRole::ForegroundWriter => "foreground-writer",
        PhysicalScenarioActorRole::CheckpointDriver => "checkpoint-driver",
        PhysicalScenarioActorRole::CompactionDriver => "compaction-driver",
        PhysicalScenarioActorRole::MaintenanceReclaimer => "maintenance-reclaimer",
        PhysicalScenarioActorRole::RecoveryDriver => "recovery-driver",
        PhysicalScenarioActorRole::ScrubDriver => "scrub-driver",
        PhysicalScenarioActorRole::OfflineVerifier => "offline-verifier",
        PhysicalScenarioActorRole::ShortcutRejectionProbe => "shortcut-rejection-probe",
        PhysicalScenarioActorRole::BlobIngestActor => "blob-ingest-actor",
        PhysicalScenarioActorRole::BlobReadActor => "blob-read-actor",
        PhysicalScenarioActorRole::BlobVerifyActor => "blob-verify-actor",
        PhysicalScenarioActorRole::BlobResumeActor => "blob-resume-actor",
        PhysicalScenarioActorRole::BlobDedupeActor => "blob-dedupe-actor",
        PhysicalScenarioActorRole::BlobExportActor => "blob-export-actor",
        PhysicalScenarioActorRole::BlobImportActor => "blob-import-actor",
        PhysicalScenarioActorRole::BlobPlacementMoveActor => "blob-placement-move-actor",
        PhysicalScenarioActorRole::BlobPartialReplicationActor => "blob-partial-replication-actor",
        PhysicalScenarioActorRole::BlobReclaimActor => "blob-reclaim-actor",
        PhysicalScenarioActorRole::FutureExtensionSlot => "future-extension-slot",
    }
}
