use forge_store_budgets::CounterEvidenceStrength;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobCorruptionCounterSnapshot {
    strength: CounterEvidenceStrength,
    localizations: u64,
    read_detections: u64,
    scrub_detections: u64,
    cold_fetch_detections: u64,
    import_detections: u64,
    capsule_detections: u64,
    affected_reference_edges: u64,
    quarantine_holds: u64,
    derived_rebuild_admissions: u64,
    authoritative_repair_postures: u64,
    authoritative_restore_postures: u64,
    authoritative_degraded_truth_postures: u64,
    dedupe_denials: u64,
    export_denials: u64,
    import_readmission_denials: u64,
    capsule_denials: u64,
    verified_read_denials: u64,
    reclaim_denials: u64,
    compaction_denials: u64,
    denials: u64,
}

impl BlobCorruptionCounterSnapshot {
    pub(crate) const fn start() -> Self {
        Self {
            strength: CounterEvidenceStrength::Exact,
            localizations: 0,
            read_detections: 0,
            scrub_detections: 0,
            cold_fetch_detections: 0,
            import_detections: 0,
            capsule_detections: 0,
            affected_reference_edges: 0,
            quarantine_holds: 0,
            derived_rebuild_admissions: 0,
            authoritative_repair_postures: 0,
            authoritative_restore_postures: 0,
            authoritative_degraded_truth_postures: 0,
            dedupe_denials: 0,
            export_denials: 0,
            import_readmission_denials: 0,
            capsule_denials: 0,
            verified_read_denials: 0,
            reclaim_denials: 0,
            compaction_denials: 0,
            denials: 0,
        }
    }

    pub(crate) const fn record_localization(
        self,
        source: super::BlobCorruptionDetectionSource,
        affected_edges: u64,
    ) -> Self {
        let counters = match source {
            super::BlobCorruptionDetectionSource::VerifiedRead => Self {
                read_detections: self.read_detections + 1,
                ..self
            },
            super::BlobCorruptionDetectionSource::Scrub => Self {
                scrub_detections: self.scrub_detections + 1,
                ..self
            },
            super::BlobCorruptionDetectionSource::ColdFetch => Self {
                cold_fetch_detections: self.cold_fetch_detections + 1,
                ..self
            },
            super::BlobCorruptionDetectionSource::ImportReadmission => Self {
                import_detections: self.import_detections + 1,
                ..self
            },
            super::BlobCorruptionDetectionSource::CapsuleMaterialization => Self {
                capsule_detections: self.capsule_detections + 1,
                ..self
            },
        };
        Self {
            localizations: counters.localizations + 1,
            affected_reference_edges: counters.affected_reference_edges + affected_edges,
            ..counters
        }
    }

    pub(crate) const fn record_quarantine_hold(self) -> Self {
        Self {
            quarantine_holds: self.quarantine_holds + 1,
            ..self
        }
    }

    pub(crate) const fn record_derived_rebuild_admission(self) -> Self {
        Self {
            derived_rebuild_admissions: self.derived_rebuild_admissions + 1,
            ..self
        }
    }

    pub(crate) const fn record_authoritative_repair_posture(self) -> Self {
        Self {
            authoritative_repair_postures: self.authoritative_repair_postures + 1,
            ..self
        }
    }

    pub(crate) const fn record_authoritative_restore_posture(self) -> Self {
        Self {
            authoritative_restore_postures: self.authoritative_restore_postures + 1,
            ..self
        }
    }

    pub(crate) const fn record_authoritative_degraded_truth_posture(self) -> Self {
        Self {
            authoritative_degraded_truth_postures: self.authoritative_degraded_truth_postures + 1,
            ..self
        }
    }

    pub(crate) const fn record_denial(self) -> Self {
        Self {
            denials: self.denials + 1,
            ..self
        }
    }

    pub(crate) const fn record_guard_denial(self, guard: super::BlobCorruptionGuardDenial) -> Self {
        match guard {
            super::BlobCorruptionGuardDenial::DedupeDenied { .. } => Self {
                dedupe_denials: self.dedupe_denials + 1,
                denials: self.denials + 1,
                ..self
            },
            super::BlobCorruptionGuardDenial::ExportDenied { .. } => Self {
                export_denials: self.export_denials + 1,
                denials: self.denials + 1,
                ..self
            },
            super::BlobCorruptionGuardDenial::ImportReadmissionDenied { .. } => Self {
                import_readmission_denials: self.import_readmission_denials + 1,
                denials: self.denials + 1,
                ..self
            },
            super::BlobCorruptionGuardDenial::CapsuleReadinessDenied { .. } => Self {
                capsule_denials: self.capsule_denials + 1,
                denials: self.denials + 1,
                ..self
            },
            super::BlobCorruptionGuardDenial::VerifiedReadPublicationDenied { .. } => Self {
                verified_read_denials: self.verified_read_denials + 1,
                denials: self.denials + 1,
                ..self
            },
            super::BlobCorruptionGuardDenial::ReclaimDenied { .. } => Self {
                reclaim_denials: self.reclaim_denials + 1,
                denials: self.denials + 1,
                ..self
            },
            super::BlobCorruptionGuardDenial::CompactionMovementDenied { .. } => Self {
                compaction_denials: self.compaction_denials + 1,
                denials: self.denials + 1,
                ..self
            },
        }
    }

    pub const fn strength(self) -> CounterEvidenceStrength {
        self.strength
    }

    pub const fn localizations(self) -> u64 {
        self.localizations
    }

    pub const fn read_detections(self) -> u64 {
        self.read_detections
    }

    pub const fn scrub_detections(self) -> u64 {
        self.scrub_detections
    }

    pub const fn cold_fetch_detections(self) -> u64 {
        self.cold_fetch_detections
    }

    pub const fn import_detections(self) -> u64 {
        self.import_detections
    }

    pub const fn capsule_detections(self) -> u64 {
        self.capsule_detections
    }

    pub const fn affected_reference_edges(self) -> u64 {
        self.affected_reference_edges
    }

    pub const fn quarantine_holds(self) -> u64 {
        self.quarantine_holds
    }

    pub const fn derived_rebuild_admissions(self) -> u64 {
        self.derived_rebuild_admissions
    }

    pub const fn authoritative_repair_postures(self) -> u64 {
        self.authoritative_repair_postures
    }

    pub const fn authoritative_restore_postures(self) -> u64 {
        self.authoritative_restore_postures
    }

    pub const fn authoritative_degraded_truth_postures(self) -> u64 {
        self.authoritative_degraded_truth_postures
    }

    pub const fn dedupe_denials(self) -> u64 {
        self.dedupe_denials
    }

    pub const fn export_denials(self) -> u64 {
        self.export_denials
    }

    pub const fn import_readmission_denials(self) -> u64 {
        self.import_readmission_denials
    }

    pub const fn capsule_denials(self) -> u64 {
        self.capsule_denials
    }

    pub const fn verified_read_denials(self) -> u64 {
        self.verified_read_denials
    }

    pub const fn denials(self) -> u64 {
        self.denials
    }
}
