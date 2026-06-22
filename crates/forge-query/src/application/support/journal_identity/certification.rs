use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::runtime::ForgeQueryJournalReplayCounterSnapshot;

use super::evidence::{
    ForgeQueryJournalIdentityInventoryEvidence, ForgeQueryJournalIdentityScheduleEvidence,
    ForgeQueryJournalReplaySurfaceEvidence,
};

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryJournalIdentityBoundaryPosture {
    Open,
    Partial,
    Closed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryJournalIdentityCertification {
    inventory: ForgeQueryJournalIdentityInventoryEvidence,
    schedule: ForgeQueryJournalIdentityScheduleEvidence,
    replay: ForgeQueryJournalReplaySurfaceEvidence,
    posture: ForgeQueryJournalIdentityBoundaryPosture,
    replay_boundary: ForgeQueryJournalReplayBoundaryCertification,
    artifact_digest: String,
    failure_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryJournalReplayBoundaryCertification {
    journal_segment_identity_digest: String,
    journal_replay_outcome_digest: String,
    journal_replay_truth_digest: String,
    published_artifact_replay_digest: String,
    journal_identity_inventory_digest: String,
    journal_boundary_posture: ForgeQueryJournalIdentityBoundaryPosture,
    failure_digest: String,
    counter_snapshot: ForgeQueryJournalReplayCounterSnapshot,
}

#[allow(dead_code)]
impl ForgeQueryJournalIdentityCertification {
    pub fn from_evidence(
        inventory: ForgeQueryJournalIdentityInventoryEvidence,
        schedule: ForgeQueryJournalIdentityScheduleEvidence,
        replay: ForgeQueryJournalReplaySurfaceEvidence,
    ) -> Self {
        let posture = derive_journal_identity_posture(&inventory, &schedule, &replay);
        let failure_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::ApplicationSupportReport)
                .field_usize(
                    ForgeQueryEvidenceTag::new("journal_identity_failure_count"),
                    inventory.total_failure_count(),
                )
                .field_bool(
                    ForgeQueryEvidenceTag::new("journal_identity_schedule_certified"),
                    schedule.certified(),
                )
                .field_bool(
                    ForgeQueryEvidenceTag::new("journal_replay_surface_certified"),
                    replay.certified(),
                )
                .seal()
                .as_str()
                .to_string();
        let artifact_digest = forge_query_evidence_identity(
            ForgeQueryEvidenceScope::RuntimeHostileCertificationArtifact,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("journal_identity_inventory_digest"),
            inventory.inventory_digest(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("journal_identity_schedule_digest"),
            schedule.schedule_digest(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("journal_replay_outcome_digest"),
            replay.replay_outcome_digest(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("journal_replay_truth_digest"),
            replay.replay_truth_digest(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("published_artifact_replay_digest"),
            replay.published_artifact_digest(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("journal_replay_retained_entry_count"),
            replay.counter_snapshot().retained_entry_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("journal_replay_residue_count"),
            replay.counter_snapshot().replay_residue_count(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("journal_identity_failure_count"),
            inventory.total_failure_count(),
        )
        .field_bool(
            ForgeQueryEvidenceTag::new("journal_identity_schedule_certified"),
            schedule.certified(),
        )
        .field_bool(
            ForgeQueryEvidenceTag::new("journal_replay_surface_certified"),
            replay.certified(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("journal_identity_failure_digest"),
            &failure_digest,
        )
        .seal()
        .as_str()
        .to_string();
        let replay_boundary = ForgeQueryJournalReplayBoundaryCertification {
            journal_segment_identity_digest: replay.journal_segment_identity_digest().to_string(),
            journal_replay_outcome_digest: replay.replay_outcome_digest().to_string(),
            journal_replay_truth_digest: replay.replay_truth_digest().to_string(),
            published_artifact_replay_digest: replay.published_artifact_digest().to_string(),
            journal_identity_inventory_digest: inventory.inventory_digest().to_string(),
            journal_boundary_posture: posture,
            failure_digest: failure_digest.clone(),
            counter_snapshot: replay.counter_snapshot().clone(),
        };
        Self {
            inventory,
            schedule,
            replay,
            posture,
            replay_boundary,
            artifact_digest,
            failure_digest,
        }
    }

    pub fn closed(&self) -> bool {
        self.posture == ForgeQueryJournalIdentityBoundaryPosture::Closed
    }

    #[allow(dead_code)]
    pub fn posture(&self) -> ForgeQueryJournalIdentityBoundaryPosture {
        self.posture
    }

    pub fn artifact_digest(&self) -> &str {
        &self.artifact_digest
    }

    pub fn failure_digest(&self) -> &str {
        &self.failure_digest
    }

    pub fn replay_boundary_certification(&self) -> &ForgeQueryJournalReplayBoundaryCertification {
        &self.replay_boundary
    }

    #[allow(dead_code)]
    pub fn inventory(&self) -> &ForgeQueryJournalIdentityInventoryEvidence {
        &self.inventory
    }

    #[allow(dead_code)]
    pub fn schedule(&self) -> &ForgeQueryJournalIdentityScheduleEvidence {
        &self.schedule
    }

    #[allow(dead_code)]
    pub fn replay(&self) -> &ForgeQueryJournalReplaySurfaceEvidence {
        &self.replay
    }
}

#[allow(dead_code)]
impl ForgeQueryJournalReplayBoundaryCertification {
    pub fn journal_segment_identity_digest(&self) -> &str {
        &self.journal_segment_identity_digest
    }

    pub fn journal_replay_outcome_digest(&self) -> &str {
        &self.journal_replay_outcome_digest
    }

    pub fn journal_replay_truth_digest(&self) -> &str {
        &self.journal_replay_truth_digest
    }

    pub fn published_artifact_replay_digest(&self) -> &str {
        &self.published_artifact_replay_digest
    }

    pub fn journal_identity_inventory_digest(&self) -> &str {
        &self.journal_identity_inventory_digest
    }

    pub fn journal_boundary_posture(&self) -> ForgeQueryJournalIdentityBoundaryPosture {
        self.journal_boundary_posture
    }

    pub fn failure_digest(&self) -> &str {
        &self.failure_digest
    }

    pub fn counter_snapshot(&self) -> &ForgeQueryJournalReplayCounterSnapshot {
        &self.counter_snapshot
    }
}

fn derive_journal_identity_posture(
    inventory: &ForgeQueryJournalIdentityInventoryEvidence,
    schedule: &ForgeQueryJournalIdentityScheduleEvidence,
    replay: &ForgeQueryJournalReplaySurfaceEvidence,
) -> ForgeQueryJournalIdentityBoundaryPosture {
    let inventory_clean = inventory.total_failure_count() == 0;
    let schedule_certified = schedule.certified();
    let replay_certified = replay.certified();
    if inventory_clean && schedule_certified && replay_certified {
        return ForgeQueryJournalIdentityBoundaryPosture::Closed;
    }
    if inventory_clean || schedule_certified || replay_certified {
        return ForgeQueryJournalIdentityBoundaryPosture::Partial;
    }
    ForgeQueryJournalIdentityBoundaryPosture::Open
}
