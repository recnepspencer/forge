use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::counters::ReplayUndoConsumerCutoverCounters;
use super::residue_ledger::ReplayUndoConsumerCutoverResidueLedger;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReplayUndoMilestoneThirteenSeedPosture {
    ReplayUndoOnlyNoConflictOrCacheClaim,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayUndoMilestoneThirteenSeed {
    seed_identity: String,
    transaction_packet_identity: String,
    replay_scope_identity: String,
    undo_scope_identity: String,
    residue_row_count: usize,
    migrated_source_count: usize,
    source_firewall_clean: bool,
    hard_deletion_ledger_digest: Option<String>,
    residue_cap_audit_digest: Option<String>,
    hard_deletion_source_firewall_digest: Option<String>,
    posture: ReplayUndoMilestoneThirteenSeedPosture,
}

impl ReplayUndoMilestoneThirteenSeed {
    pub(crate) fn lower(
        transaction_packet_identity: &str,
        replay_scope_identity: &str,
        undo_scope_identity: &str,
        residue_ledger: &ReplayUndoConsumerCutoverResidueLedger,
        counters: &ReplayUndoConsumerCutoverCounters,
        source_firewall_clean: bool,
    ) -> Self {
        let seed_identity = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-kernel:replay-undo-milestone-thirteen-seed:v1".to_string(),
                format!("transaction-packet:{transaction_packet_identity}"),
                format!("replay-scope:{replay_scope_identity}"),
                format!("undo-scope:{undo_scope_identity}"),
                format!("residue-rows:{}", residue_ledger.row_count()),
                format!("migrated-sources:{}", counters.migrated_sources()),
                format!("firewall-clean:{source_firewall_clean}"),
                "posture:replay-undo-only-no-conflict-or-cache-claim".to_string(),
            ],
        );
        Self {
            seed_identity,
            transaction_packet_identity: transaction_packet_identity.to_string(),
            replay_scope_identity: replay_scope_identity.to_string(),
            undo_scope_identity: undo_scope_identity.to_string(),
            residue_row_count: residue_ledger.row_count(),
            migrated_source_count: counters.migrated_sources(),
            source_firewall_clean,
            hard_deletion_ledger_digest: None,
            residue_cap_audit_digest: None,
            hard_deletion_source_firewall_digest: None,
            posture: ReplayUndoMilestoneThirteenSeedPosture::ReplayUndoOnlyNoConflictOrCacheClaim,
        }
    }

    pub(crate) fn lower_after_hard_deletion(
        prior_seed: &Self,
        hard_deletion_ledger_digest: &str,
        residue_cap_audit_digest: &str,
        hard_deletion_source_firewall_digest: &str,
    ) -> Self {
        let seed_identity = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-kernel:replay-undo-milestone-thirteen-seed:v2".to_string(),
                format!("prior-replay-undo-seed:{}", prior_seed.seed_identity()),
                format!("hard-deletion-ledger:{hard_deletion_ledger_digest}"),
                format!("residue-cap-audit:{residue_cap_audit_digest}"),
                format!("hard-source-firewall:{hard_deletion_source_firewall_digest}"),
                "posture:replay-undo-only-no-conflict-or-cache-claim".to_string(),
            ],
        );
        Self {
            seed_identity,
            transaction_packet_identity: prior_seed.transaction_packet_identity.clone(),
            replay_scope_identity: prior_seed.replay_scope_identity.clone(),
            undo_scope_identity: prior_seed.undo_scope_identity.clone(),
            residue_row_count: prior_seed.residue_row_count,
            migrated_source_count: prior_seed.migrated_source_count,
            source_firewall_clean: prior_seed.source_firewall_clean,
            hard_deletion_ledger_digest: Some(hard_deletion_ledger_digest.to_string()),
            residue_cap_audit_digest: Some(residue_cap_audit_digest.to_string()),
            hard_deletion_source_firewall_digest: Some(
                hard_deletion_source_firewall_digest.to_string(),
            ),
            posture: ReplayUndoMilestoneThirteenSeedPosture::ReplayUndoOnlyNoConflictOrCacheClaim,
        }
    }

    pub fn seed_identity(&self) -> &str {
        &self.seed_identity
    }

    pub fn transaction_packet_identity(&self) -> &str {
        &self.transaction_packet_identity
    }

    pub fn replay_scope_identity(&self) -> &str {
        &self.replay_scope_identity
    }

    pub fn undo_scope_identity(&self) -> &str {
        &self.undo_scope_identity
    }

    pub const fn residue_row_count(&self) -> usize {
        self.residue_row_count
    }

    pub const fn migrated_source_count(&self) -> usize {
        self.migrated_source_count
    }

    pub const fn source_firewall_clean(&self) -> bool {
        self.source_firewall_clean
    }

    pub fn hard_deletion_ledger_digest(&self) -> Option<&str> {
        self.hard_deletion_ledger_digest.as_deref()
    }

    pub fn residue_cap_audit_digest(&self) -> Option<&str> {
        self.residue_cap_audit_digest.as_deref()
    }

    pub fn hard_deletion_source_firewall_digest(&self) -> Option<&str> {
        self.hard_deletion_source_firewall_digest.as_deref()
    }

    pub const fn posture(&self) -> ReplayUndoMilestoneThirteenSeedPosture {
        self.posture
    }
}
