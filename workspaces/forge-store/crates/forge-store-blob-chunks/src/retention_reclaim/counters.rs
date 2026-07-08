use forge_store_budgets::CounterEvidenceStrength;

use super::holds::BlobRetentionHoldKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobRetentionReclaimCounterSnapshot {
    strength: CounterEvidenceStrength,
    orphan_candidates: u64,
    reclaim_permits: u64,
    reclaimed_chunks: u64,
    residue_localizations: u64,
    s6_posture_denials: u64,
    reachability_denials: u64,
    copied_or_weak_denials: u64,
    identity_mismatch_denials: u64,
    generation_hold_denials: u64,
    time_window_hold_denials: u64,
    export_hold_denials: u64,
    capsule_hold_denials: u64,
    quarantine_hold_denials: u64,
    read_plan_hold_denials: u64,
    checkpoint_hold_denials: u64,
    tenant_custody_hold_denials: u64,
    resume_session_hold_denials: u64,
    placement_move_hold_denials: u64,
    backup_hold_denials: u64,
    replay_convergence_checks: u64,
}

impl BlobRetentionReclaimCounterSnapshot {
    pub const fn start() -> Self {
        Self {
            strength: CounterEvidenceStrength::Exact,
            orphan_candidates: 0,
            reclaim_permits: 0,
            reclaimed_chunks: 0,
            residue_localizations: 0,
            s6_posture_denials: 0,
            reachability_denials: 0,
            copied_or_weak_denials: 0,
            identity_mismatch_denials: 0,
            generation_hold_denials: 0,
            time_window_hold_denials: 0,
            export_hold_denials: 0,
            capsule_hold_denials: 0,
            quarantine_hold_denials: 0,
            read_plan_hold_denials: 0,
            checkpoint_hold_denials: 0,
            tenant_custody_hold_denials: 0,
            resume_session_hold_denials: 0,
            placement_move_hold_denials: 0,
            backup_hold_denials: 0,
            replay_convergence_checks: 0,
        }
    }

    pub(crate) const fn with_orphan_candidate(self) -> Self {
        Self {
            orphan_candidates: self.orphan_candidates + 1,
            ..self
        }
    }

    pub(crate) const fn with_permit(self) -> Self {
        Self {
            reclaim_permits: self.reclaim_permits + 1,
            reclaimed_chunks: self.reclaimed_chunks + 1,
            ..self
        }
    }

    pub(crate) const fn with_residue_localization(self) -> Self {
        Self {
            residue_localizations: self.residue_localizations + 1,
            ..self
        }
    }

    pub(crate) const fn record_s6_posture_denial(self) -> Self {
        Self {
            s6_posture_denials: self.s6_posture_denials + 1,
            ..self
        }
    }

    pub(crate) const fn record_copied_or_weak_denial(self) -> Self {
        Self {
            copied_or_weak_denials: self.copied_or_weak_denials + 1,
            ..self
        }
    }

    pub(crate) const fn record_reachability_denial(self) -> Self {
        Self {
            reachability_denials: self.reachability_denials + 1,
            ..self
        }
    }

    pub(crate) const fn record_identity_mismatch_denial(self) -> Self {
        Self {
            identity_mismatch_denials: self.identity_mismatch_denials + 1,
            ..self
        }
    }

    pub(crate) const fn record_hold_denial(self, kind: BlobRetentionHoldKind) -> Self {
        match kind {
            BlobRetentionHoldKind::Generation => Self {
                generation_hold_denials: self.generation_hold_denials + 1,
                ..self
            },
            BlobRetentionHoldKind::TimeWindow => Self {
                time_window_hold_denials: self.time_window_hold_denials + 1,
                ..self
            },
            BlobRetentionHoldKind::Export => Self {
                export_hold_denials: self.export_hold_denials + 1,
                ..self
            },
            BlobRetentionHoldKind::Capsule => Self {
                capsule_hold_denials: self.capsule_hold_denials + 1,
                ..self
            },
            BlobRetentionHoldKind::Quarantine => Self {
                quarantine_hold_denials: self.quarantine_hold_denials + 1,
                ..self
            },
            BlobRetentionHoldKind::ReadPlan => Self {
                read_plan_hold_denials: self.read_plan_hold_denials + 1,
                ..self
            },
            BlobRetentionHoldKind::Checkpoint => Self {
                checkpoint_hold_denials: self.checkpoint_hold_denials + 1,
                ..self
            },
            BlobRetentionHoldKind::TenantCustody => Self {
                tenant_custody_hold_denials: self.tenant_custody_hold_denials + 1,
                ..self
            },
            BlobRetentionHoldKind::ResumeSession => Self {
                resume_session_hold_denials: self.resume_session_hold_denials + 1,
                ..self
            },
            BlobRetentionHoldKind::PlacementMove => Self {
                placement_move_hold_denials: self.placement_move_hold_denials + 1,
                ..self
            },
            BlobRetentionHoldKind::Backup => Self {
                backup_hold_denials: self.backup_hold_denials + 1,
                ..self
            },
        }
    }

    pub(crate) const fn record_replay_convergence_check(self) -> Self {
        Self {
            replay_convergence_checks: self.replay_convergence_checks + 1,
            ..self
        }
    }

    pub const fn strength(self) -> CounterEvidenceStrength {
        self.strength
    }

    pub const fn orphan_candidates(self) -> u64 {
        self.orphan_candidates
    }

    pub const fn reclaim_permits(self) -> u64 {
        self.reclaim_permits
    }

    pub const fn reclaimed_chunks(self) -> u64 {
        self.reclaimed_chunks
    }

    pub const fn residue_localizations(self) -> u64 {
        self.residue_localizations
    }

    pub const fn s6_posture_denials(self) -> u64 {
        self.s6_posture_denials
    }

    pub const fn copied_or_weak_denials(self) -> u64 {
        self.copied_or_weak_denials
    }

    pub const fn reachability_denials(self) -> u64 {
        self.reachability_denials
    }

    pub const fn identity_mismatch_denials(self) -> u64 {
        self.identity_mismatch_denials
    }

    pub const fn generation_hold_denials(self) -> u64 {
        self.generation_hold_denials
    }

    pub const fn time_window_hold_denials(self) -> u64 {
        self.time_window_hold_denials
    }

    pub const fn export_hold_denials(self) -> u64 {
        self.export_hold_denials
    }

    pub const fn capsule_hold_denials(self) -> u64 {
        self.capsule_hold_denials
    }

    pub const fn quarantine_hold_denials(self) -> u64 {
        self.quarantine_hold_denials
    }

    pub const fn read_plan_hold_denials(self) -> u64 {
        self.read_plan_hold_denials
    }

    pub const fn checkpoint_hold_denials(self) -> u64 {
        self.checkpoint_hold_denials
    }

    pub const fn tenant_custody_hold_denials(self) -> u64 {
        self.tenant_custody_hold_denials
    }

    pub const fn resume_session_hold_denials(self) -> u64 {
        self.resume_session_hold_denials
    }

    pub const fn placement_move_hold_denials(self) -> u64 {
        self.placement_move_hold_denials
    }

    pub const fn backup_hold_denials(self) -> u64 {
        self.backup_hold_denials
    }

    pub const fn replay_convergence_checks(self) -> u64 {
        self.replay_convergence_checks
    }
}

impl Default for BlobRetentionReclaimCounterSnapshot {
    fn default() -> Self {
        Self::start()
    }
}
