use worth_store_blob_chunks::LargeRecordStreamingEnvelope;
use worth_store_buffer_pool::{
    AllocationScope, BackgroundEnvelopeCounterSnapshot, BackgroundEnvelopeDenialKind,
    BackgroundMemoryInterferenceReport, BackgroundWorkClass, OperationAllocationScope,
};
use worth_store_maintenance::{CompactionPlanningMemoryEnvelope, ImportExportMemoryEnvelope};
use worth_store_physical_integrity::ScrubPlanningMemoryEnvelope;
use worth_store_recovery_physics::RecoveryMemoryEnvelope;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackgroundEnvelopeEvidenceBundle {
    admitted_classes: [BackgroundWorkClass; 5],
    envelopes: [BackgroundClassEnvelopeEvidence; 5],
}

impl BackgroundEnvelopeEvidenceBundle {
    pub fn from_envelopes(
        recovery: RecoveryMemoryEnvelope,
        compaction: CompactionPlanningMemoryEnvelope,
        scrub: ScrubPlanningMemoryEnvelope,
        import_export: ImportExportMemoryEnvelope,
        streaming: LargeRecordStreamingEnvelope,
        interference_reports: &[BackgroundMemoryInterferenceReport],
    ) -> Result<Self, BackgroundEnvelopeEvidenceDenial> {
        reject_recovery_scope(
            recovery.allocation_scope(),
            BackgroundWorkClass::RecoveryPlanning,
        )?;
        reject_foreground_scope(
            compaction.allocation_scope(),
            BackgroundWorkClass::CompactionPlanning,
        )?;
        reject_foreground_scope(scrub.allocation_scope(), BackgroundWorkClass::ScrubPlanning)?;
        reject_foreground_scope(
            import_export.allocation_scope(),
            BackgroundWorkClass::ImportExport,
        )?;
        reject_foreground_scope(
            streaming.allocation_scope(),
            BackgroundWorkClass::LargeRecordStreaming,
        )?;
        reject_later_semantic_claims(recovery, compaction, scrub, import_export, streaming)?;
        require_interference_report(
            interference_reports,
            RequiredInterferenceKind::ForegroundResidency,
        )?;
        require_interference_report(
            interference_reports,
            RequiredInterferenceKind::IndefinitePin,
        )?;
        require_interference_report(
            interference_reports,
            RequiredInterferenceKind::PinBudgetPressure,
        )?;
        require_interference_report(interference_reports, RequiredInterferenceKind::WholeObject)?;
        require_interference_report(
            interference_reports,
            RequiredInterferenceKind::StreamingWindowExceedsEnvelope,
        )?;
        require_interference_report(
            interference_reports,
            RequiredInterferenceKind::StreamingEnvelopeExceedsWindow,
        )?;
        Ok(Self {
            admitted_classes: BackgroundWorkClass::ALL,
            envelopes: [
                BackgroundClassEnvelopeEvidence::from_recovery_counters(
                    BackgroundWorkClass::RecoveryPlanning,
                    recovery.counters(),
                )?,
                BackgroundClassEnvelopeEvidence::from_counters(
                    BackgroundWorkClass::CompactionPlanning,
                    compaction.counters(),
                )?,
                BackgroundClassEnvelopeEvidence::from_counters(
                    BackgroundWorkClass::ScrubPlanning,
                    scrub.counters(),
                )?,
                BackgroundClassEnvelopeEvidence::from_counters(
                    BackgroundWorkClass::ImportExport,
                    import_export.counters(),
                )?,
                BackgroundClassEnvelopeEvidence::from_counters(
                    BackgroundWorkClass::LargeRecordStreaming,
                    streaming.counters(),
                )?,
            ],
        })
    }

    pub const fn admitted_classes(&self) -> [BackgroundWorkClass; 5] {
        self.admitted_classes
    }

    pub fn envelope_for(
        &self,
        work_class: BackgroundWorkClass,
    ) -> Option<BackgroundClassEnvelopeEvidence> {
        self.envelopes
            .iter()
            .copied()
            .find(|evidence| evidence.work_class() == work_class)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackgroundClassEnvelopeEvidence {
    work_class: BackgroundWorkClass,
    counters: BackgroundClassEnvelopeCounters,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackgroundClassEnvelopeCounters {
    admitted: u32,
    allocation_bytes: u64,
    resident_frames: u32,
    resident_bytes: u64,
    pinned_pages: u32,
    copied_bytes: u64,
}

impl BackgroundClassEnvelopeEvidence {
    pub fn from_counters(
        work_class: BackgroundWorkClass,
        counters: BackgroundEnvelopeCounterSnapshot,
    ) -> Result<Self, BackgroundEnvelopeEvidenceDenial> {
        Self::from_normalized(
            work_class,
            BackgroundClassEnvelopeCounters {
                admitted: counters.admitted(),
                allocation_bytes: counters.allocation_bytes_admitted(),
                resident_frames: counters.resident_frames_admitted(),
                resident_bytes: counters.resident_bytes_admitted(),
                pinned_pages: counters.pinned_pages_admitted(),
                copied_bytes: counters.copied_bytes(),
            },
        )
    }

    pub fn from_recovery_counters(
        work_class: BackgroundWorkClass,
        counters: worth_store_recovery_physics::RecoveryMemoryCounterSnapshot,
    ) -> Result<Self, BackgroundEnvelopeEvidenceDenial> {
        Self::from_normalized(
            work_class,
            BackgroundClassEnvelopeCounters {
                admitted: counters.admitted(),
                allocation_bytes: counters.allocation_bytes_allocated(),
                resident_frames: counters.resident_frames_admitted(),
                resident_bytes: counters.resident_bytes_admitted(),
                pinned_pages: counters.pinned_pages_admitted(),
                copied_bytes: counters.copied_bytes(),
            },
        )
    }

    pub const fn work_class(self) -> BackgroundWorkClass {
        self.work_class
    }

    pub const fn counters(self) -> BackgroundClassEnvelopeCounters {
        self.counters
    }

    fn from_normalized(
        work_class: BackgroundWorkClass,
        counters: BackgroundClassEnvelopeCounters,
    ) -> Result<Self, BackgroundEnvelopeEvidenceDenial> {
        if counters.admitted == 0
            || counters.allocation_bytes == 0
            || counters.resident_frames == 0
            || counters.resident_bytes == 0
        {
            return Err(BackgroundEnvelopeEvidenceDenial::MissingEnvelopeCounters { work_class });
        }
        Ok(Self {
            work_class,
            counters,
        })
    }
}

impl BackgroundClassEnvelopeCounters {
    pub const fn admitted(self) -> u32 {
        self.admitted
    }

    pub const fn allocation_bytes_admitted(self) -> u64 {
        self.allocation_bytes
    }

    pub const fn resident_frames_admitted(self) -> u32 {
        self.resident_frames
    }

    pub const fn resident_bytes_admitted(self) -> u64 {
        self.resident_bytes
    }

    pub const fn pinned_pages_admitted(self) -> u32 {
        self.pinned_pages
    }

    pub const fn allocation_bytes_allocated(self) -> u64 {
        self.allocation_bytes
    }

    pub const fn copied_bytes(self) -> u64 {
        self.copied_bytes
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackgroundEnvelopeEvidenceDenial {
    ForegroundScopeUsed { work_class: BackgroundWorkClass },
    LaterSemanticClaimed { work_class: BackgroundWorkClass },
    MissingEnvelopeCounters { work_class: BackgroundWorkClass },
    MissingInterferenceReport(RequiredInterferenceKind),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequiredInterferenceKind {
    ForegroundResidency,
    IndefinitePin,
    PinBudgetPressure,
    WholeObject,
    StreamingWindowExceedsEnvelope,
    StreamingEnvelopeExceedsWindow,
}

fn reject_foreground_scope(
    scope: AllocationScope,
    work_class: BackgroundWorkClass,
) -> Result<(), BackgroundEnvelopeEvidenceDenial> {
    if scope == AllocationScope::Foreground {
        Err(BackgroundEnvelopeEvidenceDenial::ForegroundScopeUsed { work_class })
    } else {
        Ok(())
    }
}

fn reject_recovery_scope(
    scope: OperationAllocationScope,
    work_class: BackgroundWorkClass,
) -> Result<(), BackgroundEnvelopeEvidenceDenial> {
    if matches!(
        scope,
        OperationAllocationScope::ForegroundRead | OperationAllocationScope::ForegroundWrite
    ) {
        Err(BackgroundEnvelopeEvidenceDenial::ForegroundScopeUsed { work_class })
    } else {
        Ok(())
    }
}

fn reject_later_semantic_claims(
    recovery: RecoveryMemoryEnvelope,
    compaction: CompactionPlanningMemoryEnvelope,
    scrub: ScrubPlanningMemoryEnvelope,
    import_export: ImportExportMemoryEnvelope,
    streaming: LargeRecordStreamingEnvelope,
) -> Result<(), BackgroundEnvelopeEvidenceDenial> {
    if recovery.proves_wal_recovery() || recovery.proves_checkpoint_safety() {
        return Err(BackgroundEnvelopeEvidenceDenial::LaterSemanticClaimed {
            work_class: BackgroundWorkClass::RecoveryPlanning,
        });
    }
    if compaction.proves_compaction_validity() || compaction.proves_retained_truth_preservation() {
        return Err(BackgroundEnvelopeEvidenceDenial::LaterSemanticClaimed {
            work_class: BackgroundWorkClass::CompactionPlanning,
        });
    }
    if scrub.proves_scrub_correctness()
        || scrub.proves_corruption_localization()
        || scrub.proves_repair_behavior()
    {
        return Err(BackgroundEnvelopeEvidenceDenial::LaterSemanticClaimed {
            work_class: BackgroundWorkClass::ScrubPlanning,
        });
    }
    if import_export.proves_import_export_semantic_correctness()
        || import_export.proves_replication_correctness()
    {
        return Err(BackgroundEnvelopeEvidenceDenial::LaterSemanticClaimed {
            work_class: BackgroundWorkClass::ImportExport,
        });
    }
    if streaming.proves_blob_lifecycle_completion()
        || streaming.proves_blob_reachability()
        || streaming.proves_blob_checksum_correctness()
    {
        return Err(BackgroundEnvelopeEvidenceDenial::LaterSemanticClaimed {
            work_class: BackgroundWorkClass::LargeRecordStreaming,
        });
    }
    Ok(())
}

fn require_interference_report(
    reports: &[BackgroundMemoryInterferenceReport],
    required: RequiredInterferenceKind,
) -> Result<(), BackgroundEnvelopeEvidenceDenial> {
    if reports
        .iter()
        .any(|report| report_matches(*report, required))
    {
        Ok(())
    } else {
        Err(BackgroundEnvelopeEvidenceDenial::MissingInterferenceReport(
            required,
        ))
    }
}

fn report_matches(
    report: BackgroundMemoryInterferenceReport,
    required: RequiredInterferenceKind,
) -> bool {
    matches!(
        (required, report.kind()),
        (
            RequiredInterferenceKind::ForegroundResidency,
            BackgroundEnvelopeDenialKind::ForegroundResidencyInterference { .. }
        ) | (
            RequiredInterferenceKind::IndefinitePin,
            BackgroundEnvelopeDenialKind::IndefinitePinRequested { .. }
        ) | (
            RequiredInterferenceKind::PinBudgetPressure,
            BackgroundEnvelopeDenialKind::PinBudgetWouldBeExceeded { .. }
        ) | (
            RequiredInterferenceKind::WholeObject,
            BackgroundEnvelopeDenialKind::WholeObjectMemoryRequired { .. }
        ) | (
            RequiredInterferenceKind::StreamingWindowExceedsEnvelope,
            BackgroundEnvelopeDenialKind::StreamingWindowExceedsEnvelope { .. }
        ) | (
            RequiredInterferenceKind::StreamingEnvelopeExceedsWindow,
            BackgroundEnvelopeDenialKind::StreamingEnvelopeExceedsWindow { .. }
        )
    )
}
