use forge_store_buffer_pool::{AllocationScope, BackgroundEnvelopeCounterSnapshot};
use forge_store_contracts::{DurableArtifactFamilyId, DurableArtifactRebuildPosture};
use forge_store_layout_indexes::access_planning::S8AccessShape;
use forge_store_layout_indexes::layout_strategy_admission::{
    phase26_maintenance_queue_rule, AdmittedMaintenanceQueueLayoutRule,
};

use crate::{CompactionPlanningMemoryEnvelope, ImportExportMemoryEnvelope};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MaintenanceQueueLayoutFamilyHome;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MaintenanceQueueLayoutAdmission {
    _rule: AdmittedMaintenanceQueueLayoutRule,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AdmittedMaintenanceQueueLayoutFamily {
    _admission: MaintenanceQueueLayoutAdmission,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaintenanceQueueClass {
    CompactionPlanning,
    ImportExport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaintenanceQueueInterferencePosture {
    ForegroundMemoryBounded,
    ImportExportMemoryBounded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaintenanceQueueAccessBudget {
    resident_frames: u32,
    resident_bytes: u64,
    pinned_pages: u32,
    allocation_bytes: u64,
    copied_bytes: u64,
    streaming_window_bytes: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MaintenanceQueueLayoutEvidence {
    Compaction(CompactionPlanningMemoryEnvelope),
    ImportExport(ImportExportMemoryEnvelope),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaintenanceQueueLayoutReport {
    family_id: DurableArtifactFamilyId,
    access_shape: S8AccessShape,
    rebuild_posture: DurableArtifactRebuildPosture,
    queue_class: MaintenanceQueueClass,
    interference_posture: MaintenanceQueueInterferencePosture,
    evidence: MaintenanceQueueLayoutEvidence,
}

impl MaintenanceQueueLayoutFamilyHome {
    const fn s8() -> Self {
        Self
    }

    fn admit(self, rule: AdmittedMaintenanceQueueLayoutRule) -> MaintenanceQueueLayoutAdmission {
        let _ = self;
        MaintenanceQueueLayoutAdmission { _rule: rule }
    }
}

fn maintenance_queue_layout() -> AdmittedMaintenanceQueueLayoutFamily {
    AdmittedMaintenanceQueueLayoutFamily {
        _admission: MaintenanceQueueLayoutFamilyHome::s8().admit(
            phase26_maintenance_queue_rule()
                .expect("phase 26 maintenance queue rule must stay admitted"),
        ),
    }
}

impl MaintenanceQueueLayoutReport {
    const fn from_compaction(envelope: CompactionPlanningMemoryEnvelope) -> Self {
        Self {
            family_id: DurableArtifactFamilyId::MaintenanceQueueDeclaration,
            access_shape: S8AccessShape::BoundedScan,
            rebuild_posture: DurableArtifactRebuildPosture::PartialRebuildOnly,
            queue_class: MaintenanceQueueClass::CompactionPlanning,
            interference_posture: MaintenanceQueueInterferencePosture::ForegroundMemoryBounded,
            evidence: MaintenanceQueueLayoutEvidence::Compaction(envelope),
        }
    }

    const fn from_import_export(envelope: ImportExportMemoryEnvelope) -> Self {
        Self {
            family_id: DurableArtifactFamilyId::MaintenanceQueueDeclaration,
            access_shape: S8AccessShape::BoundedScan,
            rebuild_posture: DurableArtifactRebuildPosture::PartialRebuildOnly,
            queue_class: MaintenanceQueueClass::ImportExport,
            interference_posture: MaintenanceQueueInterferencePosture::ImportExportMemoryBounded,
            evidence: MaintenanceQueueLayoutEvidence::ImportExport(envelope),
        }
    }

    pub const fn family_id(&self) -> DurableArtifactFamilyId {
        self.family_id
    }

    pub const fn access_shape(&self) -> S8AccessShape {
        self.access_shape
    }

    pub const fn rebuild_posture(&self) -> DurableArtifactRebuildPosture {
        self.rebuild_posture
    }

    pub const fn queue_class(&self) -> MaintenanceQueueClass {
        self.queue_class
    }

    pub const fn allocation_scope(&self) -> AllocationScope {
        match self.evidence {
            MaintenanceQueueLayoutEvidence::Compaction(envelope) => envelope.allocation_scope(),
            MaintenanceQueueLayoutEvidence::ImportExport(envelope) => envelope.allocation_scope(),
        }
    }

    pub const fn interference_posture(&self) -> MaintenanceQueueInterferencePosture {
        self.interference_posture
    }

    pub const fn declared_budget(&self) -> MaintenanceQueueAccessBudget {
        match self.evidence {
            MaintenanceQueueLayoutEvidence::Compaction(envelope) => {
                MaintenanceQueueAccessBudget::from_compaction(envelope)
            }
            MaintenanceQueueLayoutEvidence::ImportExport(envelope) => {
                MaintenanceQueueAccessBudget::from_import_export(envelope)
            }
        }
    }

    pub const fn exact_counters(&self) -> BackgroundEnvelopeCounterSnapshot {
        match self.evidence {
            MaintenanceQueueLayoutEvidence::Compaction(envelope) => envelope.counters(),
            MaintenanceQueueLayoutEvidence::ImportExport(envelope) => envelope.counters(),
        }
    }
}

impl MaintenanceQueueAccessBudget {
    const fn from_compaction(envelope: CompactionPlanningMemoryEnvelope) -> Self {
        Self {
            resident_frames: envelope.resident_frames(),
            resident_bytes: envelope.resident_bytes(),
            pinned_pages: envelope.pinned_pages(),
            allocation_bytes: envelope.allocation_bytes(),
            copied_bytes: envelope.copied_bytes(),
            streaming_window_bytes: envelope.streaming_window_bytes(),
        }
    }

    const fn from_import_export(envelope: ImportExportMemoryEnvelope) -> Self {
        Self {
            resident_frames: envelope.resident_frames(),
            resident_bytes: envelope.resident_bytes(),
            pinned_pages: envelope.pinned_pages(),
            allocation_bytes: envelope.allocation_bytes(),
            copied_bytes: envelope.copied_bytes(),
            streaming_window_bytes: envelope.streaming_window_bytes(),
        }
    }

    pub const fn resident_frames(&self) -> u32 {
        self.resident_frames
    }

    pub const fn resident_bytes(&self) -> u64 {
        self.resident_bytes
    }

    pub const fn pinned_pages(&self) -> u32 {
        self.pinned_pages
    }

    pub const fn allocation_bytes(&self) -> u64 {
        self.allocation_bytes
    }

    pub const fn copied_bytes(&self) -> u64 {
        self.copied_bytes
    }

    pub const fn streaming_window_bytes(&self) -> u64 {
        self.streaming_window_bytes
    }
}

impl AdmittedMaintenanceQueueLayoutFamily {
    const fn admit_compaction(
        &self,
        envelope: CompactionPlanningMemoryEnvelope,
    ) -> MaintenanceQueueLayoutReport {
        let _ = self;
        MaintenanceQueueLayoutReport::from_compaction(envelope)
    }

    const fn admit_import_export(
        &self,
        envelope: ImportExportMemoryEnvelope,
    ) -> MaintenanceQueueLayoutReport {
        let _ = self;
        MaintenanceQueueLayoutReport::from_import_export(envelope)
    }
}

impl CompactionPlanningMemoryEnvelope {
    pub fn admit_maintenance_queue_layout(&self) -> MaintenanceQueueLayoutReport {
        maintenance_queue_layout().admit_compaction(*self)
    }
}

impl ImportExportMemoryEnvelope {
    pub fn admit_maintenance_queue_layout(&self) -> MaintenanceQueueLayoutReport {
        maintenance_queue_layout().admit_import_export(*self)
    }
}
