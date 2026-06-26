use forge_store_buffer_pool::{
    AdmittedBackgroundEnvelope, AllocationScope, BackgroundEnvelopeCounterSnapshot,
    BackgroundWorkClass,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompactionPlanningMemoryEnvelope {
    envelope: AdmittedBackgroundEnvelope,
}

impl CompactionPlanningMemoryEnvelope {
    pub fn from_admitted(
        envelope: AdmittedBackgroundEnvelope,
    ) -> Result<Self, MaintenanceMemoryEnvelopeDenial> {
        require_class(envelope, BackgroundWorkClass::CompactionPlanning)
            .map(|envelope| Self { envelope })
    }

    pub const fn allocation_scope(self) -> AllocationScope {
        self.envelope.allocation_scope()
    }

    pub const fn counters(self) -> BackgroundEnvelopeCounterSnapshot {
        self.envelope.counters()
    }

    pub const fn proves_compaction_validity(self) -> bool {
        false
    }

    pub const fn proves_retained_truth_preservation(self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImportExportMemoryEnvelope {
    envelope: AdmittedBackgroundEnvelope,
}

impl ImportExportMemoryEnvelope {
    pub fn from_admitted(
        envelope: AdmittedBackgroundEnvelope,
    ) -> Result<Self, MaintenanceMemoryEnvelopeDenial> {
        require_class(envelope, BackgroundWorkClass::ImportExport).map(|envelope| Self { envelope })
    }

    pub const fn allocation_scope(self) -> AllocationScope {
        self.envelope.allocation_scope()
    }

    pub const fn counters(self) -> BackgroundEnvelopeCounterSnapshot {
        self.envelope.counters()
    }

    pub const fn proves_import_export_semantic_correctness(self) -> bool {
        false
    }

    pub const fn proves_replication_correctness(self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaintenanceMemoryEnvelopeDenial {
    WrongBackgroundEnvelopeClass {
        expected: BackgroundWorkClass,
        actual: BackgroundWorkClass,
    },
}

fn require_class(
    envelope: AdmittedBackgroundEnvelope,
    expected: BackgroundWorkClass,
) -> Result<AdmittedBackgroundEnvelope, MaintenanceMemoryEnvelopeDenial> {
    if envelope.work_class() == expected {
        Ok(envelope)
    } else {
        Err(
            MaintenanceMemoryEnvelopeDenial::WrongBackgroundEnvelopeClass {
                expected,
                actual: envelope.work_class(),
            },
        )
    }
}
