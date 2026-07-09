use super::admission::{CompatibilityAdapterCostClass, CompatibilityRelation};
use super::manifests::{
    ArtifactFamilyId, ArtifactFormatVersion, ArtifactSemanticVersion, CompatibilityManifestDigest,
};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ArtifactFamilyVersionSummary {
    family_id: ArtifactFamilyId,
    format_version: ArtifactFormatVersion,
    semantic_version: ArtifactSemanticVersion,
}

impl ArtifactFamilyVersionSummary {
    pub fn new(
        family_id: ArtifactFamilyId,
        format_version: ArtifactFormatVersion,
        semantic_version: ArtifactSemanticVersion,
    ) -> Self {
        Self {
            family_id,
            format_version,
            semantic_version,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ArtifactFamilyCompatibilityIndex {
    family_count: u64,
    declared_edge_count: u64,
}

impl ArtifactFamilyCompatibilityIndex {
    pub fn new(family_count: u64, declared_edge_count: u64) -> Self {
        Self {
            family_count,
            declared_edge_count,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityManifestSummary {
    family_id: ArtifactFamilyId,
    manifest_digest: CompatibilityManifestDigest,
}

impl CompatibilityManifestSummary {
    pub fn new(family_id: ArtifactFamilyId, manifest_digest: CompatibilityManifestDigest) -> Self {
        Self {
            family_id,
            manifest_digest,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityAuditUnit {
    family_id: ArtifactFamilyId,
}

impl CompatibilityAuditUnit {
    pub fn new(family_id: ArtifactFamilyId) -> Self {
        Self { family_id }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityAuditPlan {
    audit_units: Vec<CompatibilityAuditUnit>,
}

impl CompatibilityAuditPlan {
    pub fn new(audit_units: Vec<CompatibilityAuditUnit>) -> Self {
        Self { audit_units }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityAuditSummary {
    audited_family_count: u64,
    rejected_family_count: u64,
}

impl CompatibilityAuditSummary {
    pub fn new(audited_family_count: u64, rejected_family_count: u64) -> Self {
        Self {
            audited_family_count,
            rejected_family_count,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReaderWriterSkewSummary {
    reader_version_count: u64,
    writer_version_count: u64,
    missing_edge_count: u64,
}

impl ReaderWriterSkewSummary {
    pub fn new(
        reader_version_count: u64,
        writer_version_count: u64,
        missing_edge_count: u64,
    ) -> Self {
        Self {
            reader_version_count,
            writer_version_count,
            missing_edge_count,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityAdmissionReceiptSummary {
    receipt_count: u64,
    relation: CompatibilityRelation,
}

impl CompatibilityAdmissionReceiptSummary {
    pub fn new(receipt_count: u64, relation: CompatibilityRelation) -> Self {
        Self {
            receipt_count,
            relation,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityAdapterCostClassReport {
    cost_class: CompatibilityAdapterCostClass,
    adapter_count: u64,
}

impl CompatibilityAdapterCostClassReport {
    pub fn new(cost_class: CompatibilityAdapterCostClass, adapter_count: u64) -> Self {
        Self {
            cost_class,
            adapter_count,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityAdapterCostSummary {
    reports: Vec<CompatibilityAdapterCostClassReport>,
}

impl CompatibilityAdapterCostSummary {
    pub fn new(reports: Vec<CompatibilityAdapterCostClassReport>) -> Self {
        Self { reports }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityBatchScopeReport {
    family_id: ArtifactFamilyId,
    batch_count: u64,
}

impl CompatibilityBatchScopeReport {
    pub fn new(family_id: ArtifactFamilyId, batch_count: u64) -> Self {
        Self {
            family_id,
            batch_count,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationSummary {
    invalidated_family_count: u64,
}

impl DerivedInvalidationSummary {
    pub fn new(invalidated_family_count: u64) -> Self {
        Self {
            invalidated_family_count,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityRebuildSummary {
    rebuild_family_count: u64,
    rebuild_debt_count: u64,
}

impl CompatibilityRebuildSummary {
    pub fn new(rebuild_family_count: u64, rebuild_debt_count: u64) -> Self {
        Self {
            rebuild_family_count,
            rebuild_debt_count,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RestoreCompatibilityBreadthBudget {
    family_id: ArtifactFamilyId,
    max_scanned_artifacts: u64,
}

impl RestoreCompatibilityBreadthBudget {
    pub fn new(family_id: ArtifactFamilyId, max_scanned_artifacts: u64) -> Self {
        Self {
            family_id,
            max_scanned_artifacts,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RestoreVersionSummary {
    restored_family_count: u64,
    rejected_family_count: u64,
}

impl RestoreVersionSummary {
    pub fn new(restored_family_count: u64, rejected_family_count: u64) -> Self {
        Self {
            restored_family_count,
            rejected_family_count,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12Phase1Evidence {
    registry_family_count: u64,
    declared_counter_count: u64,
    declared_matrix_row_count: u64,
}

impl Milestone12Phase1Evidence {
    pub fn new(
        registry_family_count: u64,
        declared_counter_count: u64,
        declared_matrix_row_count: u64,
    ) -> Self {
        Self {
            registry_family_count,
            declared_counter_count,
            declared_matrix_row_count,
        }
    }
}
