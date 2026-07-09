use crate::{PhysicalScenarioActor, PhysicalScenarioActorRole};

use super::admission::PhysicalSimulationActorAdmissionDenial;
use super::role_contract::admit_actor_role_contract;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalSimulationActor {
    id: String,
    role: PhysicalScenarioActorRole,
}

macro_rules! actor_wrapper {
    ($name:ident, $role:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name {
            actor: PhysicalSimulationActor,
        }

        impl $name {
            pub fn admit(
                id: impl Into<String>,
            ) -> Result<Self, PhysicalSimulationActorAdmissionDenial> {
                let actor = PhysicalScenarioActor::$role(id);
                Ok(Self {
                    actor: PhysicalSimulationActor::admit(actor)?,
                })
            }

            pub const fn actor(&self) -> &PhysicalSimulationActor {
                &self.actor
            }
        }
    };
}

actor_wrapper!(ForegroundReadActor, foreground_reader);
actor_wrapper!(ForegroundWriteActor, foreground_writer);
actor_wrapper!(CheckpointActor, checkpoint_driver);
actor_wrapper!(CompactionActor, compaction_driver);
actor_wrapper!(RecoveryActor, recovery_driver);
actor_wrapper!(ReclaimActor, maintenance_reclaimer);
actor_wrapper!(ScrubActor, scrub_driver);
actor_wrapper!(OfflineVerifierActor, offline_verifier);
actor_wrapper!(BlobIngestActor, blob_ingest_actor);
actor_wrapper!(BlobReadActor, blob_read_actor);
actor_wrapper!(BlobVerifyActor, blob_verify_actor);
actor_wrapper!(BlobResumeActor, blob_resume_actor);
actor_wrapper!(BlobDedupeActor, blob_dedupe_actor);
actor_wrapper!(BlobExportActor, blob_export_actor);
actor_wrapper!(BlobImportActor, blob_import_actor);
actor_wrapper!(BlobPlacementMoveActor, blob_placement_move_actor);
actor_wrapper!(BlobPartialReplicationActor, blob_partial_replication_actor);
actor_wrapper!(BlobReclaimActor, blob_reclaim_actor);

impl PhysicalSimulationActor {
    pub(crate) fn admit(
        actor: PhysicalScenarioActor,
    ) -> Result<Self, PhysicalSimulationActorAdmissionDenial> {
        admit_actor_role_contract(&actor)?;
        Ok(Self {
            id: actor.id().to_owned(),
            role: actor.role(),
        })
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub const fn role(&self) -> PhysicalScenarioActorRole {
        self.role
    }

    pub fn scenario_actor(&self) -> PhysicalScenarioActor {
        match self.role {
            PhysicalScenarioActorRole::ForegroundReader => {
                PhysicalScenarioActor::foreground_reader(self.id.clone())
            }
            PhysicalScenarioActorRole::ForegroundWriter => {
                PhysicalScenarioActor::foreground_writer(self.id.clone())
            }
            PhysicalScenarioActorRole::CheckpointDriver => {
                PhysicalScenarioActor::checkpoint_driver(self.id.clone())
            }
            PhysicalScenarioActorRole::CompactionDriver => {
                PhysicalScenarioActor::compaction_driver(self.id.clone())
            }
            PhysicalScenarioActorRole::MaintenanceReclaimer => {
                PhysicalScenarioActor::maintenance_reclaimer(self.id.clone())
            }
            PhysicalScenarioActorRole::RecoveryDriver => {
                PhysicalScenarioActor::recovery_driver(self.id.clone())
            }
            PhysicalScenarioActorRole::ScrubDriver => {
                PhysicalScenarioActor::scrub_driver(self.id.clone())
            }
            PhysicalScenarioActorRole::OfflineVerifier => {
                PhysicalScenarioActor::offline_verifier(self.id.clone())
            }
            PhysicalScenarioActorRole::ShortcutRejectionProbe => {
                PhysicalScenarioActor::shortcut_rejection_probe(self.id.clone())
            }
            PhysicalScenarioActorRole::BlobIngestActor => {
                PhysicalScenarioActor::blob_ingest_actor(self.id.clone())
            }
            PhysicalScenarioActorRole::BlobReadActor => {
                PhysicalScenarioActor::blob_read_actor(self.id.clone())
            }
            PhysicalScenarioActorRole::BlobVerifyActor => {
                PhysicalScenarioActor::blob_verify_actor(self.id.clone())
            }
            PhysicalScenarioActorRole::BlobResumeActor => {
                PhysicalScenarioActor::blob_resume_actor(self.id.clone())
            }
            PhysicalScenarioActorRole::BlobDedupeActor => {
                PhysicalScenarioActor::blob_dedupe_actor(self.id.clone())
            }
            PhysicalScenarioActorRole::BlobExportActor => {
                PhysicalScenarioActor::blob_export_actor(self.id.clone())
            }
            PhysicalScenarioActorRole::BlobImportActor => {
                PhysicalScenarioActor::blob_import_actor(self.id.clone())
            }
            PhysicalScenarioActorRole::BlobPlacementMoveActor => {
                PhysicalScenarioActor::blob_placement_move_actor(self.id.clone())
            }
            PhysicalScenarioActorRole::BlobPartialReplicationActor => {
                PhysicalScenarioActor::blob_partial_replication_actor(self.id.clone())
            }
            PhysicalScenarioActorRole::BlobReclaimActor => {
                PhysicalScenarioActor::blob_reclaim_actor(self.id.clone())
            }
            PhysicalScenarioActorRole::FutureExtensionSlot => {
                PhysicalScenarioActor::future_extension_slot(self.id.clone())
            }
        }
    }
}

impl PhysicalSimulationActor {
    pub fn future_extension_slot(
        id: impl Into<String>,
    ) -> Result<Self, PhysicalSimulationActorAdmissionDenial> {
        Self::admit(PhysicalScenarioActor::future_extension_slot(id))
    }
}
