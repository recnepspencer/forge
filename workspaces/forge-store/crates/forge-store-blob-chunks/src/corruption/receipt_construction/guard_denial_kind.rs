use crate::{
    BlobCorruptionCounterSnapshot, BlobCorruptionDetectionSource, BlobCorruptionGuardDenial,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GuardDenialKind {
    Dedupe,
    Export,
    ImportReadmission,
    CapsuleReadiness,
    VerifiedReadPublication,
    Reclaim,
    CompactionMovement,
}

pub(crate) const fn record_guard_denial_counters(
    base: BlobCorruptionCounterSnapshot,
    kind: GuardDenialKind,
) -> BlobCorruptionCounterSnapshot {
    let denial = match kind {
        GuardDenialKind::Dedupe => BlobCorruptionGuardDenial::DedupeDenied {
            source: BlobCorruptionDetectionSource::VerifiedRead,
            counters: base,
        },
        GuardDenialKind::Export => BlobCorruptionGuardDenial::ExportDenied {
            source: BlobCorruptionDetectionSource::VerifiedRead,
            counters: base,
        },
        GuardDenialKind::ImportReadmission => BlobCorruptionGuardDenial::ImportReadmissionDenied {
            source: BlobCorruptionDetectionSource::VerifiedRead,
            counters: base,
        },
        GuardDenialKind::CapsuleReadiness => BlobCorruptionGuardDenial::CapsuleReadinessDenied {
            source: BlobCorruptionDetectionSource::VerifiedRead,
            counters: base,
        },
        GuardDenialKind::VerifiedReadPublication => {
            BlobCorruptionGuardDenial::VerifiedReadPublicationDenied {
                source: BlobCorruptionDetectionSource::VerifiedRead,
                counters: base,
            }
        }
        GuardDenialKind::Reclaim => BlobCorruptionGuardDenial::ReclaimDenied {
            source: BlobCorruptionDetectionSource::VerifiedRead,
            counters: base,
        },
        GuardDenialKind::CompactionMovement => {
            BlobCorruptionGuardDenial::CompactionMovementDenied {
                source: BlobCorruptionDetectionSource::VerifiedRead,
                counters: base,
            }
        }
    };
    base.record_guard_denial(denial)
}

pub(crate) const fn assemble_guard_denial(
    source: BlobCorruptionDetectionSource,
    base: BlobCorruptionCounterSnapshot,
    kind: GuardDenialKind,
) -> BlobCorruptionGuardDenial {
    let counters = record_guard_denial_counters(base, kind);
    match kind {
        GuardDenialKind::Dedupe => BlobCorruptionGuardDenial::DedupeDenied { source, counters },
        GuardDenialKind::Export => BlobCorruptionGuardDenial::ExportDenied { source, counters },
        GuardDenialKind::ImportReadmission => {
            BlobCorruptionGuardDenial::ImportReadmissionDenied { source, counters }
        }
        GuardDenialKind::CapsuleReadiness => {
            BlobCorruptionGuardDenial::CapsuleReadinessDenied { source, counters }
        }
        GuardDenialKind::VerifiedReadPublication => {
            BlobCorruptionGuardDenial::VerifiedReadPublicationDenied { source, counters }
        }
        GuardDenialKind::Reclaim => BlobCorruptionGuardDenial::ReclaimDenied { source, counters },
        GuardDenialKind::CompactionMovement => {
            BlobCorruptionGuardDenial::CompactionMovementDenied { source, counters }
        }
    }
}
