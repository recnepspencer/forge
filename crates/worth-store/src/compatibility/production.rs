use super::catalog::CompatibilityFamilyKind;
use super::rolling::{MixedVersionStorePosture, ReplicaCompatibilityPosture, RollingUpgradeWindow};
use super::{
    CompatibilityAdapterDigest, CompatibilityAdapterId, CompatibilityAdapterParityWitness,
    CompatibilityRelation, ReaderCapabilitySet, WriterCapabilitySet,
};
use crate::authority::PersistedAuthoritativeCommit;
use crate::Milestone12AdmissionReport;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct CompatibilityDerivedRebuildRequest {
    family_kind: CompatibilityFamilyKind,
}

impl CompatibilityDerivedRebuildRequest {
    pub fn new(family_kind: CompatibilityFamilyKind) -> Self {
        Self { family_kind }
    }

    pub fn family_kind(&self) -> CompatibilityFamilyKind {
        self.family_kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityAuthoritativeAdapterRequest {
    family_kind: CompatibilityFamilyKind,
    observed_semantic_version: crate::ArtifactSemanticVersion,
    target_semantic_version: crate::ArtifactSemanticVersion,
    adapter_id: CompatibilityAdapterId,
    adapter_digest: CompatibilityAdapterDigest,
}

impl CompatibilityAuthoritativeAdapterRequest {
    pub fn new(
        family_kind: CompatibilityFamilyKind,
        observed_semantic_version: crate::ArtifactSemanticVersion,
        target_semantic_version: crate::ArtifactSemanticVersion,
        adapter_id: CompatibilityAdapterId,
        adapter_digest: CompatibilityAdapterDigest,
    ) -> Self {
        Self {
            family_kind,
            observed_semantic_version,
            target_semantic_version,
            adapter_id,
            adapter_digest,
        }
    }

    pub fn family_kind(&self) -> CompatibilityFamilyKind {
        self.family_kind
    }

    pub fn observed_semantic_version(&self) -> crate::ArtifactSemanticVersion {
        self.observed_semantic_version
    }

    pub fn target_semantic_version(&self) -> crate::ArtifactSemanticVersion {
        self.target_semantic_version
    }

    pub fn adapter_id(&self) -> &CompatibilityAdapterId {
        &self.adapter_id
    }

    pub fn adapter_digest(&self) -> &CompatibilityAdapterDigest {
        &self.adapter_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityAuthoritativeAdapterOutcome {
    family_kind: CompatibilityFamilyKind,
    relation: CompatibilityRelation,
    control_lane_digest: String,
    adapted_lane_digest: String,
    parity_witness: CompatibilityAdapterParityWitness,
    admission_report: Milestone12AdmissionReport,
}

impl CompatibilityAuthoritativeAdapterOutcome {
    pub(crate) fn new(
        family_kind: CompatibilityFamilyKind,
        relation: CompatibilityRelation,
        control_lane_digest: impl Into<String>,
        adapted_lane_digest: impl Into<String>,
        parity_witness: CompatibilityAdapterParityWitness,
        admission_report: Milestone12AdmissionReport,
    ) -> Self {
        Self {
            family_kind,
            relation,
            control_lane_digest: control_lane_digest.into(),
            adapted_lane_digest: adapted_lane_digest.into(),
            parity_witness,
            admission_report,
        }
    }

    pub fn family_kind(&self) -> CompatibilityFamilyKind {
        self.family_kind
    }

    pub fn relation(&self) -> CompatibilityRelation {
        self.relation
    }

    pub fn control_lane_digest(&self) -> &str {
        &self.control_lane_digest
    }

    pub fn adapted_lane_digest(&self) -> &str {
        &self.adapted_lane_digest
    }

    pub fn parity_witness(&self) -> &CompatibilityAdapterParityWitness {
        &self.parity_witness
    }

    pub fn admission_report(&self) -> &Milestone12AdmissionReport {
        &self.admission_report
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityDerivedRebuildOutcome {
    family_kind: CompatibilityFamilyKind,
    maintenance_declaration_id: String,
    maintenance_lane_id: String,
    completed_phase: String,
    admission_report: Milestone12AdmissionReport,
}

impl CompatibilityDerivedRebuildOutcome {
    pub(crate) fn new(
        family_kind: CompatibilityFamilyKind,
        maintenance_declaration_id: impl Into<String>,
        maintenance_lane_id: impl Into<String>,
        completed_phase: impl Into<String>,
        admission_report: Milestone12AdmissionReport,
    ) -> Self {
        Self {
            family_kind,
            maintenance_declaration_id: maintenance_declaration_id.into(),
            maintenance_lane_id: maintenance_lane_id.into(),
            completed_phase: completed_phase.into(),
            admission_report,
        }
    }

    pub fn family_kind(&self) -> CompatibilityFamilyKind {
        self.family_kind
    }

    pub fn maintenance_declaration_id(&self) -> &str {
        &self.maintenance_declaration_id
    }

    pub fn maintenance_lane_id(&self) -> &str {
        &self.maintenance_lane_id
    }

    pub fn completed_phase(&self) -> &str {
        &self.completed_phase
    }

    pub fn admission_report(&self) -> &Milestone12AdmissionReport {
        &self.admission_report
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityRollingPublicationRequest {
    rolling_window: RollingUpgradeWindow,
    reader_capabilities: Vec<ReaderCapabilitySet>,
    writer_capabilities: Vec<WriterCapabilitySet>,
}

impl CompatibilityRollingPublicationRequest {
    pub fn new(
        rolling_window: RollingUpgradeWindow,
        reader_capabilities: Vec<ReaderCapabilitySet>,
        writer_capabilities: Vec<WriterCapabilitySet>,
    ) -> Self {
        Self {
            rolling_window,
            reader_capabilities,
            writer_capabilities,
        }
    }

    pub fn rolling_window(&self) -> &RollingUpgradeWindow {
        &self.rolling_window
    }

    pub fn reader_capabilities(&self) -> &[ReaderCapabilitySet] {
        &self.reader_capabilities
    }

    pub fn writer_capabilities(&self) -> &[WriterCapabilitySet] {
        &self.writer_capabilities
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityRestoreExecutionOutcome {
    visible_family_count: usize,
    receipt_count: usize,
    admission_report: Milestone12AdmissionReport,
}

impl CompatibilityRestoreExecutionOutcome {
    pub(crate) fn new(
        visible_family_count: usize,
        receipt_count: usize,
        admission_report: Milestone12AdmissionReport,
    ) -> Self {
        Self {
            visible_family_count,
            receipt_count,
            admission_report,
        }
    }

    pub fn visible_family_count(&self) -> usize {
        self.visible_family_count
    }

    pub fn receipt_count(&self) -> usize {
        self.receipt_count
    }

    pub fn admission_report(&self) -> &Milestone12AdmissionReport {
        &self.admission_report
    }
}

#[derive(Debug, Clone)]
pub struct CompatibilityRollingPublicationOutcome {
    relation: CompatibilityRelation,
    store_posture: MixedVersionStorePosture,
    replica_posture: ReplicaCompatibilityPosture,
    persisted_commit: PersistedAuthoritativeCommit,
    admission_report: Milestone12AdmissionReport,
}

impl CompatibilityRollingPublicationOutcome {
    pub(crate) fn new(
        relation: CompatibilityRelation,
        store_posture: MixedVersionStorePosture,
        replica_posture: ReplicaCompatibilityPosture,
        persisted_commit: PersistedAuthoritativeCommit,
        admission_report: Milestone12AdmissionReport,
    ) -> Self {
        Self {
            relation,
            store_posture,
            replica_posture,
            persisted_commit,
            admission_report,
        }
    }

    pub fn relation(&self) -> CompatibilityRelation {
        self.relation
    }

    pub fn store_posture(&self) -> &MixedVersionStorePosture {
        &self.store_posture
    }

    pub fn replica_posture(&self) -> &ReplicaCompatibilityPosture {
        &self.replica_posture
    }

    pub fn persisted_commit(&self) -> &PersistedAuthoritativeCommit {
        &self.persisted_commit
    }

    pub fn admission_report(&self) -> &Milestone12AdmissionReport {
        &self.admission_report
    }

    pub(crate) fn with_persisted_commit(
        mut self,
        persisted_commit: PersistedAuthoritativeCommit,
    ) -> Self {
        self.persisted_commit = persisted_commit;
        self
    }
}
