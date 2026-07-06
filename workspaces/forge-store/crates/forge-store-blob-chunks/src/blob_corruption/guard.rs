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
        self.guard_denial(BlobCorruptionGuardDenial::DedupeDenied {
            source: self.quarantine.source(),
            counters: self.quarantine.counters(),
        })
    }

    pub const fn deny_export(&self) -> BlobCorruptionGuardDenial {
        self.guard_denial(BlobCorruptionGuardDenial::ExportDenied {
            source: self.quarantine.source(),
            counters: self.quarantine.counters(),
        })
    }

    pub const fn deny_import_readmission(&self) -> BlobCorruptionGuardDenial {
        self.guard_denial(BlobCorruptionGuardDenial::ImportReadmissionDenied {
            source: self.quarantine.source(),
            counters: self.quarantine.counters(),
        })
    }

    pub const fn deny_capsule_readiness(&self) -> BlobCorruptionGuardDenial {
        self.guard_denial(BlobCorruptionGuardDenial::CapsuleReadinessDenied {
            source: self.quarantine.source(),
            counters: self.quarantine.counters(),
        })
    }

    pub const fn deny_verified_read_publication(&self) -> BlobCorruptionGuardDenial {
        self.guard_denial(BlobCorruptionGuardDenial::VerifiedReadPublicationDenied {
            source: self.quarantine.source(),
            counters: self.quarantine.counters(),
        })
    }

    pub const fn deny_reclaim(&self) -> BlobCorruptionGuardDenial {
        self.guard_denial(BlobCorruptionGuardDenial::ReclaimDenied {
            source: self.quarantine.source(),
            counters: self.quarantine.counters(),
        })
    }

    pub const fn deny_compaction_movement(&self) -> BlobCorruptionGuardDenial {
        self.guard_denial(BlobCorruptionGuardDenial::CompactionMovementDenied {
            source: self.quarantine.source(),
            counters: self.quarantine.counters(),
        })
    }

    pub const fn quarantine(&self) -> &BlobChunkQuarantine {
        &self.quarantine
    }

    const fn guard_denial(&self, denial: BlobCorruptionGuardDenial) -> BlobCorruptionGuardDenial {
        match denial {
            BlobCorruptionGuardDenial::DedupeDenied { source, .. } => {
                BlobCorruptionGuardDenial::DedupeDenied {
                    source,
                    counters: self.record_guard_denial(denial),
                }
            }
            BlobCorruptionGuardDenial::ExportDenied { source, .. } => {
                BlobCorruptionGuardDenial::ExportDenied {
                    source,
                    counters: self.record_guard_denial(denial),
                }
            }
            BlobCorruptionGuardDenial::ImportReadmissionDenied { source, .. } => {
                BlobCorruptionGuardDenial::ImportReadmissionDenied {
                    source,
                    counters: self.record_guard_denial(denial),
                }
            }
            BlobCorruptionGuardDenial::CapsuleReadinessDenied { source, .. } => {
                BlobCorruptionGuardDenial::CapsuleReadinessDenied {
                    source,
                    counters: self.record_guard_denial(denial),
                }
            }
            BlobCorruptionGuardDenial::VerifiedReadPublicationDenied { source, .. } => {
                BlobCorruptionGuardDenial::VerifiedReadPublicationDenied {
                    source,
                    counters: self.record_guard_denial(denial),
                }
            }
            BlobCorruptionGuardDenial::ReclaimDenied { source, .. } => {
                BlobCorruptionGuardDenial::ReclaimDenied {
                    source,
                    counters: self.record_guard_denial(denial),
                }
            }
            BlobCorruptionGuardDenial::CompactionMovementDenied { source, .. } => {
                BlobCorruptionGuardDenial::CompactionMovementDenied {
                    source,
                    counters: self.record_guard_denial(denial),
                }
            }
        }
    }

    const fn record_guard_denial(
        &self,
        denial: BlobCorruptionGuardDenial,
    ) -> BlobCorruptionCounterSnapshot {
        self.quarantine.counters().record_guard_denial(denial)
    }
}
