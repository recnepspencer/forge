use crate::{BlobChunkQuarantine, BlobCorruptionCounterSnapshot, BlobCorruptionGuardDenial};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobCorruptionGuard {
    quarantine: BlobChunkQuarantine,
}

impl BlobCorruptionGuard {
    pub const fn from_quarantine(quarantine: BlobChunkQuarantine) -> Self {
        Self { quarantine }
    }

    pub const fn deny_dedupe(&self) -> BlobCorruptionGuardDenial {
        assemble_guard_denial(
            self,
            BlobCorruptionGuardDenial::DedupeDenied {
                source: self.quarantine.source(),
                counters: self.quarantine.counters(),
            },
        )
    }

    pub const fn deny_export(&self) -> BlobCorruptionGuardDenial {
        assemble_guard_denial(
            self,
            BlobCorruptionGuardDenial::ExportDenied {
                source: self.quarantine.source(),
                counters: self.quarantine.counters(),
            },
        )
    }

    pub const fn deny_import_readmission(&self) -> BlobCorruptionGuardDenial {
        assemble_guard_denial(
            self,
            BlobCorruptionGuardDenial::ImportReadmissionDenied {
                source: self.quarantine.source(),
                counters: self.quarantine.counters(),
            },
        )
    }

    pub const fn deny_capsule_readiness(&self) -> BlobCorruptionGuardDenial {
        assemble_guard_denial(
            self,
            BlobCorruptionGuardDenial::CapsuleReadinessDenied {
                source: self.quarantine.source(),
                counters: self.quarantine.counters(),
            },
        )
    }

    pub const fn deny_verified_read_publication(&self) -> BlobCorruptionGuardDenial {
        assemble_guard_denial(
            self,
            BlobCorruptionGuardDenial::VerifiedReadPublicationDenied {
                source: self.quarantine.source(),
                counters: self.quarantine.counters(),
            },
        )
    }

    pub const fn deny_reclaim(&self) -> BlobCorruptionGuardDenial {
        assemble_guard_denial(
            self,
            BlobCorruptionGuardDenial::ReclaimDenied {
                source: self.quarantine.source(),
                counters: self.quarantine.counters(),
            },
        )
    }

    pub const fn deny_compaction_movement(&self) -> BlobCorruptionGuardDenial {
        assemble_guard_denial(
            self,
            BlobCorruptionGuardDenial::CompactionMovementDenied {
                source: self.quarantine.source(),
                counters: self.quarantine.counters(),
            },
        )
    }

    pub const fn quarantine(&self) -> &BlobChunkQuarantine {
        &self.quarantine
    }
}

const fn record_guard_denial_counters(
    quarantine: &BlobChunkQuarantine,
    denial: BlobCorruptionGuardDenial,
) -> BlobCorruptionCounterSnapshot {
    quarantine.counters().record_guard_denial(denial)
}

const fn assemble_guard_denial(
    guard: &BlobCorruptionGuard,
    denial: BlobCorruptionGuardDenial,
) -> BlobCorruptionGuardDenial {
    match denial {
        BlobCorruptionGuardDenial::DedupeDenied { source, .. } => {
            BlobCorruptionGuardDenial::DedupeDenied {
                source,
                counters: record_guard_denial_counters(&guard.quarantine, denial),
            }
        }
        BlobCorruptionGuardDenial::ExportDenied { source, .. } => {
            BlobCorruptionGuardDenial::ExportDenied {
                source,
                counters: record_guard_denial_counters(&guard.quarantine, denial),
            }
        }
        BlobCorruptionGuardDenial::ImportReadmissionDenied { source, .. } => {
            BlobCorruptionGuardDenial::ImportReadmissionDenied {
                source,
                counters: record_guard_denial_counters(&guard.quarantine, denial),
            }
        }
        BlobCorruptionGuardDenial::CapsuleReadinessDenied { source, .. } => {
            BlobCorruptionGuardDenial::CapsuleReadinessDenied {
                source,
                counters: record_guard_denial_counters(&guard.quarantine, denial),
            }
        }
        BlobCorruptionGuardDenial::VerifiedReadPublicationDenied { source, .. } => {
            BlobCorruptionGuardDenial::VerifiedReadPublicationDenied {
                source,
                counters: record_guard_denial_counters(&guard.quarantine, denial),
            }
        }
        BlobCorruptionGuardDenial::ReclaimDenied { source, .. } => {
            BlobCorruptionGuardDenial::ReclaimDenied {
                source,
                counters: record_guard_denial_counters(&guard.quarantine, denial),
            }
        }
        BlobCorruptionGuardDenial::CompactionMovementDenied { source, .. } => {
            BlobCorruptionGuardDenial::CompactionMovementDenied {
                source,
                counters: record_guard_denial_counters(&guard.quarantine, denial),
            }
        }
    }
}