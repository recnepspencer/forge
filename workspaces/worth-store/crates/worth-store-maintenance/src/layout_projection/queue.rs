use worth_store_buffer_pool::{PhysicalOperationAllocationScope, PhysicalResidencyCounters};
use worth_store_contracts::{DurableArtifactFamilyId, DurableArtifactRebuildPosture};
use worth_store_layout_indexes::observation::AccessShape;

use crate::{CompactionPlanningMemoryEnvelope, ImportExportMemoryEnvelope};

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
    allocation_bytes: u64,
}

#[derive(Debug)]
enum MaintenanceQueueLayoutEvidence {
    Compaction(CompactionPlanningMemoryEnvelope),
    ImportExport(ImportExportMemoryEnvelope),
}

#[derive(Debug)]
pub struct MaintenanceQueueLayoutReport {
    family_id: DurableArtifactFamilyId,
    access_shape: AccessShape,
    rebuild_posture: DurableArtifactRebuildPosture,
    queue_class: MaintenanceQueueClass,
    interference_posture: MaintenanceQueueInterferencePosture,
    evidence: MaintenanceQueueLayoutEvidence,
}

impl MaintenanceQueueLayoutReport {
    fn from_compaction(envelope: CompactionPlanningMemoryEnvelope) -> Self {
        Self {
            family_id: DurableArtifactFamilyId::MaintenanceQueueDeclaration,
            access_shape: AccessShape::BoundedScan,
            rebuild_posture: DurableArtifactRebuildPosture::PartialRebuildOnly,
            queue_class: MaintenanceQueueClass::CompactionPlanning,
            interference_posture: MaintenanceQueueInterferencePosture::ForegroundMemoryBounded,
            evidence: MaintenanceQueueLayoutEvidence::Compaction(envelope),
        }
    }

    fn from_import_export(envelope: ImportExportMemoryEnvelope) -> Self {
        Self {
            family_id: DurableArtifactFamilyId::MaintenanceQueueDeclaration,
            access_shape: AccessShape::BoundedScan,
            rebuild_posture: DurableArtifactRebuildPosture::PartialRebuildOnly,
            queue_class: MaintenanceQueueClass::ImportExport,
            interference_posture: MaintenanceQueueInterferencePosture::ImportExportMemoryBounded,
            evidence: MaintenanceQueueLayoutEvidence::ImportExport(envelope),
        }
    }

    pub const fn family_id(&self) -> DurableArtifactFamilyId {
        self.family_id
    }

    pub const fn access_shape(&self) -> AccessShape {
        self.access_shape
    }

    pub const fn rebuild_posture(&self) -> DurableArtifactRebuildPosture {
        self.rebuild_posture
    }

    pub const fn queue_class(&self) -> MaintenanceQueueClass {
        self.queue_class
    }

    pub const fn allocation_scope(&self) -> PhysicalOperationAllocationScope {
        match &self.evidence {
            MaintenanceQueueLayoutEvidence::Compaction(envelope) => envelope.allocation_scope(),
            MaintenanceQueueLayoutEvidence::ImportExport(envelope) => envelope.allocation_scope(),
        }
    }

    pub const fn interference_posture(&self) -> MaintenanceQueueInterferencePosture {
        self.interference_posture
    }

    pub const fn declared_budget(&self) -> MaintenanceQueueAccessBudget {
        match &self.evidence {
            MaintenanceQueueLayoutEvidence::Compaction(envelope) => {
                MaintenanceQueueAccessBudget::from_compaction(envelope)
            }
            MaintenanceQueueLayoutEvidence::ImportExport(envelope) => {
                MaintenanceQueueAccessBudget::from_import_export(envelope)
            }
        }
    }

    pub fn exact_counters(&self) -> PhysicalResidencyCounters {
        match &self.evidence {
            MaintenanceQueueLayoutEvidence::Compaction(envelope) => envelope.counters(),
            MaintenanceQueueLayoutEvidence::ImportExport(envelope) => envelope.counters(),
        }
    }
}

impl MaintenanceQueueAccessBudget {
    const fn from_compaction(envelope: &CompactionPlanningMemoryEnvelope) -> Self {
        Self {
            allocation_bytes: envelope.allocation_bytes(),
        }
    }

    const fn from_import_export(envelope: &ImportExportMemoryEnvelope) -> Self {
        Self {
            allocation_bytes: envelope.allocation_bytes(),
        }
    }

    pub const fn allocation_bytes(&self) -> u64 {
        self.allocation_bytes
    }
}

impl CompactionPlanningMemoryEnvelope {
    pub fn project_maintenance_queue_layout(self) -> MaintenanceQueueLayoutReport {
        MaintenanceQueueLayoutReport::from_compaction(self)
    }
}

impl ImportExportMemoryEnvelope {
    pub fn project_maintenance_queue_layout(self) -> MaintenanceQueueLayoutReport {
        MaintenanceQueueLayoutReport::from_import_export(self)
    }
}
