use crate::{BlobChunkQuarantine, BlobCorruptionGuardDenial};

use super::guard_denial_kind::{assemble_guard_denial, GuardDenialKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobCorruptionGuard {
    quarantine: BlobChunkQuarantine,
}

impl BlobCorruptionGuard {
    pub const fn from_quarantine(quarantine: BlobChunkQuarantine) -> Self {
        Self { quarantine }
    }

    pub const fn deny_dedupe(&self) -> BlobCorruptionGuardDenial {
        self.deny(GuardDenialKind::Dedupe)
    }

    pub const fn deny_export(&self) -> BlobCorruptionGuardDenial {
        self.deny(GuardDenialKind::Export)
    }

    pub const fn deny_import_readmission(&self) -> BlobCorruptionGuardDenial {
        self.deny(GuardDenialKind::ImportReadmission)
    }

    pub const fn deny_capsule_readiness(&self) -> BlobCorruptionGuardDenial {
        self.deny(GuardDenialKind::CapsuleReadiness)
    }

    pub const fn deny_verified_read_publication(&self) -> BlobCorruptionGuardDenial {
        self.deny(GuardDenialKind::VerifiedReadPublication)
    }

    pub const fn deny_reclaim(&self) -> BlobCorruptionGuardDenial {
        self.deny(GuardDenialKind::Reclaim)
    }

    pub const fn deny_compaction_movement(&self) -> BlobCorruptionGuardDenial {
        self.deny(GuardDenialKind::CompactionMovement)
    }

    pub const fn quarantine(&self) -> &BlobChunkQuarantine {
        &self.quarantine
    }

    const fn deny(&self, kind: GuardDenialKind) -> BlobCorruptionGuardDenial {
        assemble_guard_denial(self.quarantine.source(), self.quarantine.counters(), kind)
    }
}
