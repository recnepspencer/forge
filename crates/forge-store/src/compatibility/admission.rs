use super::catalog::{CompatibilityFamilyDeclaration, CompatibilityRegistrySnapshot};
use super::decoding::{CompatibilityCheckedArtifact, QuarantinedDecodedArtifact};
use super::manifests::{
    ArtifactFamilyId, ArtifactFormatVersion, ArtifactSemanticVersion, CompatibilityManifestDigest,
    CompatibilityRecoveredManifestIndex,
};
use crate::failure::StoreErrorKind;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReaderCapabilitySet {
    family_id: ArtifactFamilyId,
    semantic_versions: Vec<ArtifactSemanticVersion>,
}

impl ReaderCapabilitySet {
    pub fn new(
        family_id: ArtifactFamilyId,
        mut semantic_versions: Vec<ArtifactSemanticVersion>,
    ) -> Self {
        semantic_versions.sort();
        semantic_versions.dedup();
        Self {
            family_id,
            semantic_versions,
        }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn semantic_versions(&self) -> &[ArtifactSemanticVersion] {
        &self.semantic_versions
    }

    pub fn admits_semantic_version(&self, version: ArtifactSemanticVersion) -> bool {
        self.semantic_versions.binary_search(&version).is_ok()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WriterCapabilitySet {
    family_id: ArtifactFamilyId,
    semantic_versions: Vec<ArtifactSemanticVersion>,
}

impl WriterCapabilitySet {
    pub fn new(
        family_id: ArtifactFamilyId,
        mut semantic_versions: Vec<ArtifactSemanticVersion>,
    ) -> Self {
        semantic_versions.sort();
        semantic_versions.dedup();
        Self {
            family_id,
            semantic_versions,
        }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn semantic_versions(&self) -> &[ArtifactSemanticVersion] {
        &self.semantic_versions
    }

    pub fn admits_semantic_version(&self, version: ArtifactSemanticVersion) -> bool {
        self.semantic_versions.binary_search(&version).is_ok()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum CompatibilityRelation {
    Native,
    BackwardRead,
    ForwardRead,
    AdapterRequired,
    DerivedRebuildRequired,
    Incompatible,
}

impl CompatibilityRelation {
    pub fn from_declared_edge(edge: Option<&DeclaredCompatibilityEdge>) -> Self {
        edge.map_or(Self::Incompatible, |edge| edge.relation())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum CompatibilityAdapterCostClass {
    ZeroCopy,
    BoundedRecordLocal,
    BoundedBatchLocal,
    MaintenanceOnly,
    OutOfScope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum CompatibilityAdmissionPath {
    HotRead,
    BatchRead,
    MaintenanceScheduled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityAdapterId(String);

impl CompatibilityAdapterId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityAdapterDigest(String);

impl CompatibilityAdapterDigest {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeclaredCompatibilityAdapter {
    adapter_id: CompatibilityAdapterId,
    adapter_digest: CompatibilityAdapterDigest,
    cost_class: CompatibilityAdapterCostClass,
}

impl DeclaredCompatibilityAdapter {
    pub fn new(
        adapter_id: CompatibilityAdapterId,
        adapter_digest: CompatibilityAdapterDigest,
        cost_class: CompatibilityAdapterCostClass,
    ) -> Self {
        Self {
            adapter_id,
            adapter_digest,
            cost_class,
        }
    }

    pub fn cost_class(&self) -> CompatibilityAdapterCostClass {
        self.cost_class
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeclaredCompatibilityEdge {
    family_id: ArtifactFamilyId,
    from_semantic_version: ArtifactSemanticVersion,
    to_semantic_version: ArtifactSemanticVersion,
    relation: CompatibilityRelation,
    adapter: Option<DeclaredCompatibilityAdapter>,
}

impl DeclaredCompatibilityEdge {
    pub fn new(
        family_id: ArtifactFamilyId,
        from_semantic_version: ArtifactSemanticVersion,
        to_semantic_version: ArtifactSemanticVersion,
        relation: CompatibilityRelation,
    ) -> Self {
        Self {
            family_id,
            from_semantic_version,
            to_semantic_version,
            relation,
            adapter: None,
        }
    }

    pub fn with_adapter(mut self, adapter: DeclaredCompatibilityAdapter) -> Self {
        self.adapter = Some(adapter);
        self
    }

    pub fn relation(&self) -> CompatibilityRelation {
        self.relation
    }

    pub fn adapter(&self) -> Option<&DeclaredCompatibilityAdapter> {
        self.adapter.as_ref()
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn from_semantic_version(&self) -> ArtifactSemanticVersion {
        self.from_semantic_version
    }

    pub fn to_semantic_version(&self) -> ArtifactSemanticVersion {
        self.to_semantic_version
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct CompatibilityAdmissionCounters {
    accepted_count: u64,
    rejected_count: u64,
    manifest_index_rebuild_count: u64,
    manifest_entries_visited: u64,
    manifest_index_lookup_count: u64,
    manifest_digest_check_count: u64,
    manifest_publication_count: u64,
    manifest_recovery_record_count: u64,
    manifest_publication_gap_count: u64,
    manifest_digest_mismatch_count: u64,
    manifest_window_mismatch_count: u64,
    relation_recheck_count: u64,
    edge_missing_rejection_count: u64,
    receipt_reuse_hit_count: u64,
    receipt_reuse_rejection_count: u64,
    receipt_basis_mismatch_count: u64,
    artifact_row_scan_count: u64,
    malformed_frame_count: u64,
    adapter_hot_path_rejection_count: u64,
    adapter_maintenance_required_rejection_count: u64,
    adapter_out_of_scope_rejection_count: u64,
    admitted_native_count: u64,
    admitted_forward_backward_count: u64,
    admitted_adapter_count: u64,
    authoritative_partial_truth_rejection_count: u64,
    derived_reuse_incompatibility_count: u64,
    derived_rebuild_incompatibility_count: u64,
    derived_rebuild_required_count: u64,
    derived_invalidation_count: u64,
    derived_stale_version_rejection_count: u64,
    derived_rebuild_debt_count: u64,
    maintenance_compatibility_rebuild_admission_count: u64,
    maintenance_compatibility_rebuild_rejection_count: u64,
    derived_lane_plan_count: u64,
    derived_lane_reuse_count: u64,
    derived_lane_invalidation_count: u64,
    derived_lane_rejection_count: u64,
    derived_snapshot_reuse_count: u64,
    derived_delta_reuse_count: u64,
    derived_layout_basis_rejection_count: u64,
    derived_bulk_resume_rejection_count: u64,
    derived_maintenance_summary_rebuild_count: u64,
    tier_non_authority_preserved_count: u64,
    tier_manifest_rejection_count: u64,
    maintenance_lane_mismatch_rejection_count: u64,
    rolling_window_admission_count: u64,
    rolling_window_rejection_count: u64,
    rolling_multi_writer_rejection_count: u64,
    mixed_version_skew_count: u64,
    restore_accept_count: u64,
    restore_rejection_count: u64,
    restore_out_of_scope_scan_count: u64,
    restore_publication_conflict_rejection_count: u64,
    disaster_recovery_truth_window_count: u64,
    disaster_recovery_derived_window_count: u64,
}

impl CompatibilityAdmissionCounters {
    pub fn accepted_count(&self) -> u64 {
        self.accepted_count
    }

    pub fn rejected_count(&self) -> u64 {
        self.rejected_count
    }

    pub fn manifest_index_rebuild_count(&self) -> u64 {
        self.manifest_index_rebuild_count
    }

    pub fn manifest_entries_visited(&self) -> u64 {
        self.manifest_entries_visited
    }

    pub fn manifest_index_lookup_count(&self) -> u64 {
        self.manifest_index_lookup_count
    }

    pub fn manifest_digest_check_count(&self) -> u64 {
        self.manifest_digest_check_count
    }

    pub fn manifest_publication_count(&self) -> u64 {
        self.manifest_publication_count
    }

    pub fn manifest_recovery_record_count(&self) -> u64 {
        self.manifest_recovery_record_count
    }

    pub fn manifest_publication_gap_count(&self) -> u64 {
        self.manifest_publication_gap_count
    }

    pub fn manifest_digest_mismatch_count(&self) -> u64 {
        self.manifest_digest_mismatch_count
    }

    pub fn manifest_window_mismatch_count(&self) -> u64 {
        self.manifest_window_mismatch_count
    }

    pub fn relation_recheck_count(&self) -> u64 {
        self.relation_recheck_count
    }

    pub fn edge_missing_rejection_count(&self) -> u64 {
        self.edge_missing_rejection_count
    }

    pub fn receipt_reuse_hit_count(&self) -> u64 {
        self.receipt_reuse_hit_count
    }

    pub fn receipt_reuse_rejection_count(&self) -> u64 {
        self.receipt_reuse_rejection_count
    }

    pub fn receipt_basis_mismatch_count(&self) -> u64 {
        self.receipt_basis_mismatch_count
    }

    pub fn artifact_row_scan_count(&self) -> u64 {
        self.artifact_row_scan_count
    }

    pub fn malformed_frame_count(&self) -> u64 {
        self.malformed_frame_count
    }

    pub fn adapter_hot_path_rejection_count(&self) -> u64 {
        self.adapter_hot_path_rejection_count
    }

    pub fn adapter_maintenance_required_rejection_count(&self) -> u64 {
        self.adapter_maintenance_required_rejection_count
    }

    pub fn adapter_out_of_scope_rejection_count(&self) -> u64 {
        self.adapter_out_of_scope_rejection_count
    }

    pub fn admitted_native_count(&self) -> u64 {
        self.admitted_native_count
    }

    pub fn admitted_forward_backward_count(&self) -> u64 {
        self.admitted_forward_backward_count
    }

    pub fn admitted_adapter_count(&self) -> u64 {
        self.admitted_adapter_count
    }

    pub fn authoritative_partial_truth_rejection_count(&self) -> u64 {
        self.authoritative_partial_truth_rejection_count
    }

    pub fn derived_reuse_incompatibility_count(&self) -> u64 {
        self.derived_reuse_incompatibility_count
    }

    pub fn derived_rebuild_required_count(&self) -> u64 {
        self.derived_rebuild_required_count
    }

    pub fn derived_rebuild_incompatibility_count(&self) -> u64 {
        self.derived_rebuild_incompatibility_count
    }

    pub fn derived_invalidation_count(&self) -> u64 {
        self.derived_invalidation_count
    }

    pub fn derived_stale_version_rejection_count(&self) -> u64 {
        self.derived_stale_version_rejection_count
    }

    pub fn derived_rebuild_debt_count(&self) -> u64 {
        self.derived_rebuild_debt_count
    }

    pub fn maintenance_compatibility_rebuild_admission_count(&self) -> u64 {
        self.maintenance_compatibility_rebuild_admission_count
    }

    pub fn maintenance_compatibility_rebuild_rejection_count(&self) -> u64 {
        self.maintenance_compatibility_rebuild_rejection_count
    }

    pub fn derived_lane_plan_count(&self) -> u64 {
        self.derived_lane_plan_count
    }

    pub fn derived_lane_reuse_count(&self) -> u64 {
        self.derived_lane_reuse_count
    }

    pub fn derived_lane_invalidation_count(&self) -> u64 {
        self.derived_lane_invalidation_count
    }

    pub fn derived_lane_rejection_count(&self) -> u64 {
        self.derived_lane_rejection_count
    }

    pub fn derived_snapshot_reuse_count(&self) -> u64 {
        self.derived_snapshot_reuse_count
    }

    pub fn derived_delta_reuse_count(&self) -> u64 {
        self.derived_delta_reuse_count
    }

    pub fn derived_layout_basis_rejection_count(&self) -> u64 {
        self.derived_layout_basis_rejection_count
    }

    pub fn derived_bulk_resume_rejection_count(&self) -> u64 {
        self.derived_bulk_resume_rejection_count
    }

    pub fn derived_maintenance_summary_rebuild_count(&self) -> u64 {
        self.derived_maintenance_summary_rebuild_count
    }

    pub fn tier_non_authority_preserved_count(&self) -> u64 {
        self.tier_non_authority_preserved_count
    }

    pub fn tier_manifest_rejection_count(&self) -> u64 {
        self.tier_manifest_rejection_count
    }

    pub fn maintenance_lane_mismatch_rejection_count(&self) -> u64 {
        self.maintenance_lane_mismatch_rejection_count
    }

    pub fn rolling_window_admission_count(&self) -> u64 {
        self.rolling_window_admission_count
    }

    pub fn rolling_window_rejection_count(&self) -> u64 {
        self.rolling_window_rejection_count
    }

    pub fn rolling_multi_writer_rejection_count(&self) -> u64 {
        self.rolling_multi_writer_rejection_count
    }

    pub fn mixed_version_skew_count(&self) -> u64 {
        self.mixed_version_skew_count
    }

    pub fn restore_accept_count(&self) -> u64 {
        self.restore_accept_count
    }

    pub fn restore_rejection_count(&self) -> u64 {
        self.restore_rejection_count
    }

    pub fn restore_out_of_scope_scan_count(&self) -> u64 {
        self.restore_out_of_scope_scan_count
    }

    pub fn restore_publication_conflict_rejection_count(&self) -> u64 {
        self.restore_publication_conflict_rejection_count
    }

    pub fn disaster_recovery_truth_window_count(&self) -> u64 {
        self.disaster_recovery_truth_window_count
    }

    pub fn disaster_recovery_derived_window_count(&self) -> u64 {
        self.disaster_recovery_derived_window_count
    }

    pub(crate) fn record_malformed_frame(&mut self) {
        self.malformed_frame_count += 1;
        self.rejected_count += 1;
    }

    pub(crate) fn record_derived_reuse_incompatible(&mut self) {
        self.derived_reuse_incompatibility_count += 1;
        self.rejected_count += 1;
    }

    pub(crate) fn record_derived_rebuild_required(&mut self) {
        self.derived_rebuild_required_count += 1;
    }

    pub(crate) fn record_derived_rebuild_incompatible(&mut self) {
        self.derived_rebuild_incompatibility_count += 1;
        self.rejected_count += 1;
    }

    pub(crate) fn record_derived_invalidation(&mut self) {
        self.derived_invalidation_count += 1;
    }

    pub(crate) fn record_derived_stale_version_rejection(&mut self) {
        self.derived_stale_version_rejection_count += 1;
        self.rejected_count += 1;
    }

    pub(crate) fn record_derived_rebuild_debt(&mut self, debt_record_count: u64) {
        self.derived_rebuild_debt_count += debt_record_count;
    }

    pub(crate) fn record_maintenance_compatibility_rebuild_admission(&mut self) {
        self.maintenance_compatibility_rebuild_admission_count += 1;
    }

    pub(crate) fn record_maintenance_compatibility_rebuild_rejection(&mut self) {
        self.maintenance_compatibility_rebuild_rejection_count += 1;
        self.rejected_count += 1;
    }

    pub(crate) fn record_derived_lane_plan(&mut self) {
        self.derived_lane_plan_count += 1;
    }

    pub(crate) fn record_derived_lane_reuse(&mut self) {
        self.derived_lane_reuse_count += 1;
    }

    pub(crate) fn record_derived_lane_invalidation(&mut self) {
        self.derived_lane_invalidation_count += 1;
    }

    pub(crate) fn record_derived_lane_rejection(&mut self) {
        self.derived_lane_rejection_count += 1;
        self.rejected_count += 1;
    }

    pub(crate) fn record_derived_snapshot_reuse(&mut self) {
        self.derived_snapshot_reuse_count += 1;
    }

    pub(crate) fn record_derived_delta_reuse(&mut self) {
        self.derived_delta_reuse_count += 1;
    }

    pub(crate) fn record_derived_layout_basis_rejection(&mut self) {
        self.derived_layout_basis_rejection_count += 1;
        self.record_derived_lane_rejection();
    }

    pub(crate) fn record_derived_bulk_resume_rejection(&mut self) {
        self.derived_bulk_resume_rejection_count += 1;
        self.record_derived_lane_rejection();
    }

    pub(crate) fn record_derived_maintenance_summary_rebuild(&mut self) {
        self.derived_maintenance_summary_rebuild_count += 1;
    }

    pub(crate) fn record_tier_non_authority_preserved(&mut self) {
        self.tier_non_authority_preserved_count += 1;
    }

    pub(crate) fn record_tier_manifest_rejection(&mut self) {
        self.tier_manifest_rejection_count += 1;
        self.record_derived_lane_rejection();
    }

    pub(crate) fn record_maintenance_lane_mismatch_rejection(&mut self) {
        self.maintenance_lane_mismatch_rejection_count += 1;
        self.rejected_count += 1;
    }

    pub(crate) fn record_rolling_window_admission(&mut self) {
        self.rolling_window_admission_count += 1;
        self.accepted_count += 1;
    }

    pub(crate) fn record_rolling_window_rejection(&mut self) {
        self.rolling_window_rejection_count += 1;
        self.rejected_count += 1;
    }

    pub(crate) fn record_rolling_multi_writer_rejection(&mut self) {
        self.rolling_multi_writer_rejection_count += 1;
        self.record_rolling_window_rejection();
    }

    pub(crate) fn record_mixed_version_skew(&mut self) {
        self.mixed_version_skew_count += 1;
    }

    pub(crate) fn record_restore_accept(&mut self) {
        self.restore_accept_count += 1;
        self.accepted_count += 1;
    }

    pub(crate) fn record_restore_rejection(&mut self) {
        self.restore_rejection_count += 1;
        self.rejected_count += 1;
    }

    pub(crate) fn record_restore_out_of_scope_scan_rejection(&mut self) {
        self.restore_out_of_scope_scan_count += 1;
        self.record_restore_rejection();
    }

    pub(crate) fn record_restore_publication_conflict_rejection(&mut self) {
        self.restore_publication_conflict_rejection_count += 1;
        self.record_restore_rejection();
    }

    pub(crate) fn record_disaster_recovery_truth_window(&mut self) {
        self.disaster_recovery_truth_window_count += 1;
    }

    pub(crate) fn record_disaster_recovery_derived_window(&mut self) {
        self.disaster_recovery_derived_window_count += 1;
    }

    pub(crate) fn record_authoritative_partial_truth_rejection(&mut self) {
        self.authoritative_partial_truth_rejection_count += 1;
        self.rejected_count += 1;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityManifestIndexEntry {
    family_id: ArtifactFamilyId,
    minimum_format: ArtifactFormatVersion,
    maximum_format: ArtifactFormatVersion,
    minimum_semantic: ArtifactSemanticVersion,
    maximum_semantic: ArtifactSemanticVersion,
    manifest_digest: CompatibilityManifestDigest,
}

impl CompatibilityManifestIndexEntry {
    fn from_declaration(declaration: &CompatibilityFamilyDeclaration) -> Self {
        let manifest = declaration.manifest();
        let window = manifest.window();
        Self {
            family_id: manifest.family_id().clone(),
            minimum_format: window.minimum_format(),
            maximum_format: window.maximum_format(),
            minimum_semantic: window.minimum_semantic(),
            maximum_semantic: window.maximum_semantic(),
            manifest_digest: manifest.digest().clone(),
        }
    }

    fn from_publication_record(
        record: &super::manifests::CompatibilityManifestPublicationRecord,
    ) -> Self {
        let window = record.window();
        Self {
            family_id: record.family_id().clone(),
            minimum_format: window.minimum_format(),
            maximum_format: window.maximum_format(),
            minimum_semantic: window.minimum_semantic(),
            maximum_semantic: window.maximum_semantic(),
            manifest_digest: record.manifest_digest().clone(),
        }
    }

    fn rejection_kind(
        &self,
        format_version: ArtifactFormatVersion,
        semantic_version: ArtifactSemanticVersion,
        manifest_digest: &CompatibilityManifestDigest,
        recovered: bool,
    ) -> Option<CompatibilityRejectionKind> {
        if format_version < self.minimum_format || self.maximum_format < format_version {
            return Some(if recovered {
                CompatibilityRejectionKind::RecoveredManifestWindowMismatch
            } else {
                CompatibilityRejectionKind::UnsupportedFormatVersion
            });
        }
        if semantic_version < self.minimum_semantic || self.maximum_semantic < semantic_version {
            return Some(if recovered {
                CompatibilityRejectionKind::RecoveredManifestWindowMismatch
            } else {
                CompatibilityRejectionKind::UnsupportedSemanticVersion
            });
        }
        if &self.manifest_digest != manifest_digest {
            return Some(if recovered {
                CompatibilityRejectionKind::RecoveredManifestDigestMismatch
            } else {
                CompatibilityRejectionKind::ManifestDigestMismatch
            });
        }
        None
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn manifest_digest(&self) -> &CompatibilityManifestDigest {
        &self.manifest_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityManifestIndex {
    entries_by_family: BTreeMap<ArtifactFamilyId, CompatibilityManifestIndexEntry>,
    rebuild_counters: CompatibilityAdmissionCounters,
    registry_snapshot_identity: String,
    manifest_frontier_identity: String,
    recovered: bool,
}

impl CompatibilityManifestIndex {
    pub fn rebuild_from_registry(snapshot: &CompatibilityRegistrySnapshot) -> Self {
        let mut entries_by_family = BTreeMap::new();
        let mut counters = CompatibilityAdmissionCounters::default();
        counters.manifest_index_rebuild_count = 1;
        for declaration in snapshot.declarations() {
            counters.manifest_entries_visited += 1;
            let entry = CompatibilityManifestIndexEntry::from_declaration(declaration);
            entries_by_family.insert(entry.family_id.clone(), entry);
        }
        Self {
            entries_by_family,
            rebuild_counters: counters,
            registry_snapshot_identity: registry_snapshot_identity(snapshot),
            manifest_frontier_identity: "registry-declaration-frontier".to_string(),
            recovered: false,
        }
    }

    pub fn rebuild_from_recovered_manifests(
        snapshot: &CompatibilityRegistrySnapshot,
        recovered: &CompatibilityRecoveredManifestIndex,
    ) -> Self {
        let mut entries_by_family = BTreeMap::new();
        let mut counters = CompatibilityAdmissionCounters::default();
        counters.manifest_index_rebuild_count = 1;
        counters.manifest_publication_count = recovered.frontier().publication_count();
        for declaration in snapshot.declarations() {
            counters.manifest_entries_visited += 1;
            if let Some(record) = recovered.get(declaration.family_id()) {
                counters.manifest_recovery_record_count += 1;
                let entry = CompatibilityManifestIndexEntry::from_publication_record(record);
                entries_by_family.insert(entry.family_id.clone(), entry);
            } else {
                counters.manifest_publication_gap_count += 1;
            }
        }
        Self {
            entries_by_family,
            rebuild_counters: counters,
            registry_snapshot_identity: registry_snapshot_identity(snapshot),
            manifest_frontier_identity: recovered.frontier().identity().to_string(),
            recovered: true,
        }
    }

    pub fn entries(&self) -> impl Iterator<Item = &CompatibilityManifestIndexEntry> {
        self.entries_by_family.values()
    }

    pub fn rebuild_counters(&self) -> &CompatibilityAdmissionCounters {
        &self.rebuild_counters
    }

    pub fn registry_snapshot_identity(&self) -> &str {
        &self.registry_snapshot_identity
    }

    pub fn manifest_frontier_identity(&self) -> &str {
        &self.manifest_frontier_identity
    }

    fn lookup(
        &self,
        artifact: &QuarantinedDecodedArtifact,
        counters: &mut CompatibilityAdmissionCounters,
    ) -> Result<&CompatibilityManifestIndexEntry, CompatibilityRejection> {
        counters.manifest_index_lookup_count += 1;
        counters.manifest_digest_check_count += 1;
        let Some(entry) = self.entries_by_family.get(artifact.family_id()) else {
            if self.recovered {
                counters.manifest_publication_gap_count += 1;
            }
            return Err(CompatibilityRejection::new(
                if self.recovered {
                    CompatibilityRejectionKind::MissingManifestPublication
                } else {
                    CompatibilityRejectionKind::UndeclaredFamily
                },
                artifact.family_id().clone(),
                "compatibility manifest publication is missing or family is undeclared",
            ));
        };
        if let Some(kind) = entry.rejection_kind(
            artifact.format_version(),
            artifact.semantic_version(),
            artifact.manifest_digest(),
            self.recovered,
        ) {
            match kind {
                CompatibilityRejectionKind::RecoveredManifestDigestMismatch
                | CompatibilityRejectionKind::ManifestDigestMismatch => {
                    counters.manifest_digest_mismatch_count += 1;
                }
                CompatibilityRejectionKind::RecoveredManifestWindowMismatch
                | CompatibilityRejectionKind::UnsupportedFormatVersion
                | CompatibilityRejectionKind::UnsupportedSemanticVersion => {
                    counters.manifest_window_mismatch_count += 1;
                }
                _ => {}
            }
            return Err(CompatibilityRejection::new(
                kind,
                artifact.family_id().clone(),
                "compatibility manifest window or digest rejected artifact",
            ));
        }
        Ok(entry)
    }
}

fn registry_snapshot_identity(snapshot: &CompatibilityRegistrySnapshot) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    for declaration in snapshot.declarations() {
        hasher.update(declaration.family_id().as_str().as_bytes());
        hasher.update(declaration.manifest().digest().as_str().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct CompatibilityEdgeRegistry {
    edges: BTreeMap<EdgeKey, DeclaredCompatibilityEdge>,
}

impl CompatibilityEdgeRegistry {
    pub fn new(edges: Vec<DeclaredCompatibilityEdge>) -> Self {
        let mut registry = Self::default();
        for edge in edges {
            registry.declare(edge);
        }
        registry
    }

    pub fn declare(&mut self, edge: DeclaredCompatibilityEdge) {
        self.edges.insert(EdgeKey::from_edge(&edge), edge);
    }

    pub fn get(
        &self,
        family_id: &ArtifactFamilyId,
        from_semantic_version: ArtifactSemanticVersion,
        to_semantic_version: ArtifactSemanticVersion,
    ) -> Option<&DeclaredCompatibilityEdge> {
        self.edges.get(&EdgeKey::new(
            family_id.clone(),
            from_semantic_version,
            to_semantic_version,
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
struct EdgeKey {
    family_id: ArtifactFamilyId,
    from_semantic_version: ArtifactSemanticVersion,
    to_semantic_version: ArtifactSemanticVersion,
}

impl EdgeKey {
    fn new(
        family_id: ArtifactFamilyId,
        from_semantic_version: ArtifactSemanticVersion,
        to_semantic_version: ArtifactSemanticVersion,
    ) -> Self {
        Self {
            family_id,
            from_semantic_version,
            to_semantic_version,
        }
    }

    fn from_edge(edge: &DeclaredCompatibilityEdge) -> Self {
        Self::new(
            edge.family_id().clone(),
            edge.from_semantic_version(),
            edge.to_semantic_version(),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityEdgeProof {
    edge: DeclaredCompatibilityEdge,
}

impl CompatibilityEdgeProof {
    pub(crate) fn new(edge: DeclaredCompatibilityEdge) -> Self {
        Self { edge }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityBatchScope {
    family_id: ArtifactFamilyId,
    record_count: u64,
}

impl CompatibilityBatchScope {
    pub fn new(family_id: ArtifactFamilyId, record_count: u64) -> Self {
        Self {
            family_id,
            record_count,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityAdmissionPlan {
    family_id: ArtifactFamilyId,
    relation: CompatibilityRelation,
}

impl CompatibilityAdmissionPlan {
    pub fn new(family_id: ArtifactFamilyId, relation: CompatibilityRelation) -> Self {
        Self {
            family_id,
            relation,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum CompatibilityDecision {
    Admit(CompatibilityRelation),
    Reject(CompatibilityRejection),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum CompatibilityRejectionKind {
    FamilyMismatch,
    MalformedFrame,
    TruncatedFrame,
    UndeclaredFamily,
    UnsupportedFormatVersion,
    UnsupportedSemanticVersion,
    ManifestDigestMismatch,
    MissingManifestPublication,
    RecoveredManifestDigestMismatch,
    RecoveredManifestWindowMismatch,
    MissingCompatibilityEdge,
    DeclaredIncompatibleRelation,
    AdapterHotPathRejected,
    AdapterMaintenanceRequired,
    AdapterOutOfScope,
    ReaderCapabilityUnsupported,
    WriterCapabilityUnsupported,
    ReceiptArtifactMismatch,
    ReceiptBasisMismatch,
    AuthoritativePartialTruthRejected,
    DerivedReuseIncompatible,
    DerivedRebuildIncompatible,
    DerivedBasisIncompatible,
    DerivedStaleVersion,
    DerivedRebuildAdmissionRejected,
    DerivedLaneRejected,
    BulkResumeCompatibilityRejected,
    TierManifestCompatibilityRejected,
    MaintenanceLaneMismatch,
    RollingWindowRejected,
    RollingMultiWriterRejected,
    MixedVersionSkewRejected,
    RestoreCompatibilityRejected,
    RestoreOutOfScopeScanRejected,
    RestorePublicationConflictRejected,
}

impl CompatibilityRejectionKind {
    pub fn store_error_kind(self) -> StoreErrorKind {
        match self {
            Self::FamilyMismatch => StoreErrorKind::CompatibilityArtifactFamilyUndeclared,
            Self::MalformedFrame => StoreErrorKind::CompatibilityArtifactFrameMalformed,
            Self::TruncatedFrame => StoreErrorKind::CompatibilityArtifactFrameMalformed,
            Self::UndeclaredFamily => StoreErrorKind::CompatibilityArtifactFamilyUndeclared,
            Self::UnsupportedFormatVersion => {
                StoreErrorKind::CompatibilityArtifactFormatUnsupported
            }
            Self::UnsupportedSemanticVersion => {
                StoreErrorKind::CompatibilityArtifactSemanticVersionUnsupported
            }
            Self::ManifestDigestMismatch => StoreErrorKind::CompatibilityArtifactManifestMalformed,
            Self::MissingManifestPublication => StoreErrorKind::CompatibilityManifestPublicationGap,
            Self::RecoveredManifestDigestMismatch => {
                StoreErrorKind::CompatibilityArtifactManifestMalformed
            }
            Self::RecoveredManifestWindowMismatch => {
                StoreErrorKind::CompatibilityArtifactManifestMalformed
            }
            Self::MissingCompatibilityEdge => StoreErrorKind::CompatibilityEdgeMissing,
            Self::DeclaredIncompatibleRelation => StoreErrorKind::CompatibilityEdgeMissing,
            Self::AdapterHotPathRejected => StoreErrorKind::CompatibilityAdapterParityFailure,
            Self::AdapterMaintenanceRequired => {
                StoreErrorKind::CompatibilityAuthoritativePartialTruthRejected
            }
            Self::AdapterOutOfScope => StoreErrorKind::CompatibilityAdapterParityFailure,
            Self::ReaderCapabilityUnsupported => {
                StoreErrorKind::CompatibilityArtifactSemanticVersionUnsupported
            }
            Self::WriterCapabilityUnsupported => {
                StoreErrorKind::CompatibilityArtifactSemanticVersionUnsupported
            }
            Self::ReceiptArtifactMismatch => {
                StoreErrorKind::CompatibilityAuthoritativePartialTruthRejected
            }
            Self::ReceiptBasisMismatch => {
                StoreErrorKind::CompatibilityAuthoritativePartialTruthRejected
            }
            Self::AuthoritativePartialTruthRejected => {
                StoreErrorKind::CompatibilityAuthoritativePartialTruthRejected
            }
            Self::DerivedReuseIncompatible => StoreErrorKind::CompatibilityDerivedReuseIncompatible,
            Self::DerivedRebuildIncompatible => {
                StoreErrorKind::CompatibilityDerivedRebuildIncompatible
            }
            Self::DerivedBasisIncompatible => {
                StoreErrorKind::CompatibilityDerivedRebuildIncompatible
            }
            Self::DerivedStaleVersion => StoreErrorKind::CompatibilityDerivedReuseIncompatible,
            Self::DerivedRebuildAdmissionRejected => {
                StoreErrorKind::CompatibilityDerivedRebuildIncompatible
            }
            Self::DerivedLaneRejected => StoreErrorKind::CompatibilityDerivedReuseIncompatible,
            Self::BulkResumeCompatibilityRejected => {
                StoreErrorKind::CompatibilityDerivedReuseIncompatible
            }
            Self::TierManifestCompatibilityRejected => {
                StoreErrorKind::CompatibilityDerivedReuseIncompatible
            }
            Self::MaintenanceLaneMismatch => {
                StoreErrorKind::CompatibilityDerivedRebuildIncompatible
            }
            Self::RollingWindowRejected
            | Self::RollingMultiWriterRejected
            | Self::MixedVersionSkewRejected => StoreErrorKind::CompatibilityRollingUpgradeRejected,
            Self::RestoreCompatibilityRejected | Self::RestorePublicationConflictRejected => {
                StoreErrorKind::CompatibilityRestoreRejected
            }
            Self::RestoreOutOfScopeScanRejected => {
                StoreErrorKind::CompatibilityRestoreOutOfScopeScanRejected
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityRejection {
    kind: CompatibilityRejectionKind,
    family_id: ArtifactFamilyId,
    reason: String,
}

impl CompatibilityRejection {
    pub fn new(
        kind: CompatibilityRejectionKind,
        family_id: ArtifactFamilyId,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            family_id,
            reason: reason.into(),
        }
    }

    pub fn kind(&self) -> CompatibilityRejectionKind {
        self.kind
    }

    pub fn store_error_kind(&self) -> StoreErrorKind {
        self.kind.store_error_kind()
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityReadIntent {
    family_id: ArtifactFamilyId,
    target_semantic_version: ArtifactSemanticVersion,
}

impl CompatibilityReadIntent {
    pub fn new(
        family_id: ArtifactFamilyId,
        target_semantic_version: ArtifactSemanticVersion,
    ) -> Self {
        Self {
            family_id,
            target_semantic_version,
        }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn target_semantic_version(&self) -> ArtifactSemanticVersion {
        self.target_semantic_version
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityWriteIntent {
    family_id: ArtifactFamilyId,
    target_semantic_version: ArtifactSemanticVersion,
}

impl CompatibilityWriteIntent {
    pub fn new(
        family_id: ArtifactFamilyId,
        target_semantic_version: ArtifactSemanticVersion,
    ) -> Self {
        Self {
            family_id,
            target_semantic_version,
        }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn target_semantic_version(&self) -> ArtifactSemanticVersion {
        self.target_semantic_version
    }
}

macro_rules! proof_wrapper {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
        pub struct $name {
            family_id: ArtifactFamilyId,
        }

        impl $name {
            pub(crate) fn new(family_id: ArtifactFamilyId) -> Self {
                Self { family_id }
            }

            pub fn family_id(&self) -> &ArtifactFamilyId {
                &self.family_id
            }
        }
    };
}

proof_wrapper!(SemanticMeaningPreservationWitness);
proof_wrapper!(ForwardReadCompatibilityWitness);
proof_wrapper!(BackwardReadCompatibilityWitness);
proof_wrapper!(UpgradeAdmissionWitness);

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityAdapterParityWitness {
    adapter_id: CompatibilityAdapterId,
    adapter_digest: CompatibilityAdapterDigest,
    cost_class: CompatibilityAdapterCostClass,
}

impl CompatibilityAdapterParityWitness {
    pub(crate) fn new(
        adapter_id: CompatibilityAdapterId,
        adapter_digest: CompatibilityAdapterDigest,
        cost_class: CompatibilityAdapterCostClass,
    ) -> Self {
        Self {
            adapter_id,
            adapter_digest,
            cost_class,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityAdmissionReceipt {
    family_id: ArtifactFamilyId,
    manifest_digest: CompatibilityManifestDigest,
    registry_snapshot_identity: String,
    manifest_frontier_identity: String,
    observed_semantic_version: ArtifactSemanticVersion,
    target_semantic_version: ArtifactSemanticVersion,
    admission_path: CompatibilityAdmissionPath,
    relation: CompatibilityRelation,
}

impl CompatibilityAdmissionReceipt {
    pub(crate) fn new(
        family_id: ArtifactFamilyId,
        manifest_digest: CompatibilityManifestDigest,
        registry_snapshot_identity: impl Into<String>,
        manifest_frontier_identity: impl Into<String>,
        observed_semantic_version: ArtifactSemanticVersion,
        target_semantic_version: ArtifactSemanticVersion,
        admission_path: CompatibilityAdmissionPath,
        relation: CompatibilityRelation,
    ) -> Self {
        Self {
            family_id,
            manifest_digest,
            registry_snapshot_identity: registry_snapshot_identity.into(),
            manifest_frontier_identity: manifest_frontier_identity.into(),
            observed_semantic_version,
            target_semantic_version,
            admission_path,
            relation,
        }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn manifest_digest(&self) -> &CompatibilityManifestDigest {
        &self.manifest_digest
    }

    pub fn registry_snapshot_identity(&self) -> &str {
        &self.registry_snapshot_identity
    }

    pub fn manifest_frontier_identity(&self) -> &str {
        &self.manifest_frontier_identity
    }

    pub fn observed_semantic_version(&self) -> ArtifactSemanticVersion {
        self.observed_semantic_version
    }

    pub fn target_semantic_version(&self) -> ArtifactSemanticVersion {
        self.target_semantic_version
    }

    pub fn admission_path(&self) -> CompatibilityAdmissionPath {
        self.admission_path
    }

    pub fn relation(&self) -> CompatibilityRelation {
        self.relation
    }
}

macro_rules! receipt_wrapper {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
        pub struct $name {
            receipt: CompatibilityAdmissionReceipt,
        }

        impl $name {
            pub(crate) fn new(receipt: CompatibilityAdmissionReceipt) -> Self {
                Self { receipt }
            }

            pub fn receipt(&self) -> &CompatibilityAdmissionReceipt {
                &self.receipt
            }
        }
    };
}

receipt_wrapper!(ReadCompatibilityReceipt);
receipt_wrapper!(WriteCompatibilityReceipt);
receipt_wrapper!(DerivedReuseCompatibilityReceipt);
receipt_wrapper!(RestoreCompatibilityReceipt);
receipt_wrapper!(RollingWindowCompatibilityReceipt);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
struct ReceiptKey {
    family_id: ArtifactFamilyId,
    manifest_digest: CompatibilityManifestDigest,
    registry_snapshot_identity: String,
    manifest_frontier_identity: String,
    observed_semantic_version: ArtifactSemanticVersion,
    capability_family_id: ArtifactFamilyId,
    target_semantic_version: ArtifactSemanticVersion,
    admission_path: CompatibilityAdmissionPath,
    direction: ReceiptDirection,
}

impl ReceiptKey {
    fn read(
        manifest_index: &CompatibilityManifestIndex,
        artifact: &QuarantinedDecodedArtifact,
        reader_capabilities: &ReaderCapabilitySet,
        intent: &CompatibilityReadIntent,
        path: CompatibilityAdmissionPath,
    ) -> Self {
        Self {
            family_id: artifact.family_id().clone(),
            manifest_digest: artifact.manifest_digest().clone(),
            registry_snapshot_identity: manifest_index.registry_snapshot_identity().to_string(),
            manifest_frontier_identity: manifest_index.manifest_frontier_identity().to_string(),
            observed_semantic_version: artifact.semantic_version(),
            capability_family_id: reader_capabilities.family_id().clone(),
            target_semantic_version: intent.target_semantic_version(),
            admission_path: path,
            direction: ReceiptDirection::Read,
        }
    }

    fn write(
        manifest_index: &CompatibilityManifestIndex,
        artifact: &QuarantinedDecodedArtifact,
        writer_capabilities: &WriterCapabilitySet,
        intent: &CompatibilityWriteIntent,
        path: CompatibilityAdmissionPath,
    ) -> Self {
        Self {
            family_id: artifact.family_id().clone(),
            manifest_digest: artifact.manifest_digest().clone(),
            registry_snapshot_identity: manifest_index.registry_snapshot_identity().to_string(),
            manifest_frontier_identity: manifest_index.manifest_frontier_identity().to_string(),
            observed_semantic_version: artifact.semantic_version(),
            capability_family_id: writer_capabilities.family_id().clone(),
            target_semantic_version: intent.target_semantic_version(),
            admission_path: path,
            direction: ReceiptDirection::Write,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
enum ReceiptDirection {
    Read,
    Write,
}

fn has_stale_receipt_basis<'a>(
    mut existing: impl Iterator<Item = &'a ReceiptKey>,
    candidate: &ReceiptKey,
) -> bool {
    existing.any(|key| {
        key.family_id == candidate.family_id
            && key.manifest_digest == candidate.manifest_digest
            && key.observed_semantic_version == candidate.observed_semantic_version
            && key.capability_family_id == candidate.capability_family_id
            && key.target_semantic_version == candidate.target_semantic_version
            && key.admission_path == candidate.admission_path
            && key.direction == candidate.direction
            && (key.registry_snapshot_identity != candidate.registry_snapshot_identity
                || key.manifest_frontier_identity != candidate.manifest_frontier_identity)
    })
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct CompatibilityAdmissionBatch {
    read_receipts: BTreeMap<ReceiptKey, ReadCompatibilityReceipt>,
    write_receipts: BTreeMap<ReceiptKey, WriteCompatibilityReceipt>,
    counters: CompatibilityAdmissionCounters,
}

impl CompatibilityAdmissionBatch {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn counters(&self) -> &CompatibilityAdmissionCounters {
        &self.counters
    }

    pub(crate) fn counters_mut(&mut self) -> &mut CompatibilityAdmissionCounters {
        &mut self.counters
    }
}

pub fn plan_read_compatibility(
    batch: &mut CompatibilityAdmissionBatch,
    manifest_index: &CompatibilityManifestIndex,
    edge_registry: &CompatibilityEdgeRegistry,
    reader_capabilities: &ReaderCapabilitySet,
    intent: &CompatibilityReadIntent,
    artifact: &QuarantinedDecodedArtifact,
) -> Result<ReadCompatibilityReceipt, CompatibilityRejection> {
    plan_read_compatibility_for_path(
        batch,
        manifest_index,
        edge_registry,
        reader_capabilities,
        intent,
        artifact,
        CompatibilityAdmissionPath::HotRead,
    )
}

pub(crate) fn plan_read_compatibility_for_path(
    batch: &mut CompatibilityAdmissionBatch,
    manifest_index: &CompatibilityManifestIndex,
    edge_registry: &CompatibilityEdgeRegistry,
    reader_capabilities: &ReaderCapabilitySet,
    intent: &CompatibilityReadIntent,
    artifact: &QuarantinedDecodedArtifact,
    path: CompatibilityAdmissionPath,
) -> Result<ReadCompatibilityReceipt, CompatibilityRejection> {
    if artifact.family_id() != intent.family_id()
        || artifact.family_id() != reader_capabilities.family_id()
    {
        batch.counters.receipt_reuse_rejection_count += 1;
        batch.counters.rejected_count += 1;
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::FamilyMismatch,
            artifact.family_id().clone(),
            "read compatibility family mismatch",
        ));
    }
    let key = ReceiptKey::read(manifest_index, artifact, reader_capabilities, intent, path);
    if let Some(receipt) = batch.read_receipts.get(&key) {
        batch.counters.receipt_reuse_hit_count += 1;
        batch.counters.accepted_count += 1;
        return Ok(receipt.clone());
    }
    if has_stale_receipt_basis(batch.read_receipts.keys(), &key) {
        batch.counters.receipt_basis_mismatch_count += 1;
        batch.counters.receipt_reuse_rejection_count += 1;
        batch.counters.rejected_count += 1;
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::ReceiptBasisMismatch,
            artifact.family_id().clone(),
            "read receipt reuse basis does not match registry or manifest frontier",
        ));
    }
    if let Err(rejection) = manifest_index.lookup(artifact, &mut batch.counters) {
        batch.counters.rejected_count += 1;
        return Err(rejection);
    }
    let relation = match resolve_relation(
        &mut batch.counters,
        edge_registry,
        artifact.family_id(),
        artifact.semantic_version(),
        intent.target_semantic_version(),
        path,
    ) {
        Ok(relation) => relation,
        Err(rejection) => {
            batch.counters.rejected_count += 1;
            return Err(rejection);
        }
    };
    if !reader_capabilities.admits_semantic_version(intent.target_semantic_version()) {
        batch.counters.rejected_count += 1;
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::ReaderCapabilityUnsupported,
            artifact.family_id().clone(),
            "reader capability does not admit target semantic version",
        ));
    }
    let receipt = ReadCompatibilityReceipt::new(CompatibilityAdmissionReceipt::new(
        artifact.family_id().clone(),
        artifact.manifest_digest().clone(),
        manifest_index.registry_snapshot_identity(),
        manifest_index.manifest_frontier_identity(),
        artifact.semantic_version(),
        intent.target_semantic_version(),
        path,
        relation,
    ));
    batch.read_receipts.insert(key, receipt.clone());
    batch.counters.accepted_count += 1;
    batch.counters.record_admitted_relation(relation);
    Ok(receipt)
}

pub fn plan_write_compatibility(
    batch: &mut CompatibilityAdmissionBatch,
    manifest_index: &CompatibilityManifestIndex,
    edge_registry: &CompatibilityEdgeRegistry,
    writer_capabilities: &WriterCapabilitySet,
    intent: &CompatibilityWriteIntent,
    artifact: &QuarantinedDecodedArtifact,
) -> Result<WriteCompatibilityReceipt, CompatibilityRejection> {
    plan_write_compatibility_for_path(
        batch,
        manifest_index,
        edge_registry,
        writer_capabilities,
        intent,
        artifact,
        CompatibilityAdmissionPath::HotRead,
    )
}

pub(crate) fn plan_write_compatibility_for_path(
    batch: &mut CompatibilityAdmissionBatch,
    manifest_index: &CompatibilityManifestIndex,
    edge_registry: &CompatibilityEdgeRegistry,
    writer_capabilities: &WriterCapabilitySet,
    intent: &CompatibilityWriteIntent,
    artifact: &QuarantinedDecodedArtifact,
    path: CompatibilityAdmissionPath,
) -> Result<WriteCompatibilityReceipt, CompatibilityRejection> {
    if artifact.family_id() != intent.family_id()
        || artifact.family_id() != writer_capabilities.family_id()
    {
        batch.counters.receipt_reuse_rejection_count += 1;
        batch.counters.rejected_count += 1;
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::FamilyMismatch,
            artifact.family_id().clone(),
            "write compatibility family mismatch",
        ));
    }
    let key = ReceiptKey::write(manifest_index, artifact, writer_capabilities, intent, path);
    if let Some(receipt) = batch.write_receipts.get(&key) {
        batch.counters.receipt_reuse_hit_count += 1;
        batch.counters.accepted_count += 1;
        return Ok(receipt.clone());
    }
    if has_stale_receipt_basis(batch.write_receipts.keys(), &key) {
        batch.counters.receipt_basis_mismatch_count += 1;
        batch.counters.receipt_reuse_rejection_count += 1;
        batch.counters.rejected_count += 1;
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::ReceiptBasisMismatch,
            artifact.family_id().clone(),
            "write receipt reuse basis does not match registry or manifest frontier",
        ));
    }
    if let Err(rejection) = manifest_index.lookup(artifact, &mut batch.counters) {
        batch.counters.rejected_count += 1;
        return Err(rejection);
    }
    let relation = match resolve_relation(
        &mut batch.counters,
        edge_registry,
        artifact.family_id(),
        artifact.semantic_version(),
        intent.target_semantic_version(),
        path,
    ) {
        Ok(relation) => relation,
        Err(rejection) => {
            batch.counters.rejected_count += 1;
            return Err(rejection);
        }
    };
    if !writer_capabilities.admits_semantic_version(intent.target_semantic_version()) {
        batch.counters.rejected_count += 1;
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::WriterCapabilityUnsupported,
            artifact.family_id().clone(),
            "writer capability does not admit target semantic version",
        ));
    }
    let receipt = WriteCompatibilityReceipt::new(CompatibilityAdmissionReceipt::new(
        artifact.family_id().clone(),
        artifact.manifest_digest().clone(),
        manifest_index.registry_snapshot_identity(),
        manifest_index.manifest_frontier_identity(),
        artifact.semantic_version(),
        intent.target_semantic_version(),
        path,
        relation,
    ));
    batch.write_receipts.insert(key, receipt.clone());
    batch.counters.accepted_count += 1;
    batch.counters.record_admitted_relation(relation);
    Ok(receipt)
}

pub(crate) fn check_artifact_with_read_receipt(
    artifact: QuarantinedDecodedArtifact,
    receipt: &ReadCompatibilityReceipt,
) -> Result<CompatibilityCheckedArtifact, CompatibilityRejection> {
    if artifact.family_id() != receipt.receipt().family_id()
        || artifact.manifest_digest() != receipt.receipt().manifest_digest()
        || artifact.semantic_version() != receipt.receipt().observed_semantic_version()
    {
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::ReceiptArtifactMismatch,
            artifact.family_id().clone(),
            "read receipt does not match quarantined artifact",
        ));
    }
    Ok(CompatibilityCheckedArtifact::new(
        artifact,
        CompatibilityDecision::Admit(receipt.receipt().relation()),
    ))
}

fn resolve_relation(
    counters: &mut CompatibilityAdmissionCounters,
    edge_registry: &CompatibilityEdgeRegistry,
    family_id: &ArtifactFamilyId,
    from_semantic_version: ArtifactSemanticVersion,
    to_semantic_version: ArtifactSemanticVersion,
    path: CompatibilityAdmissionPath,
) -> Result<CompatibilityRelation, CompatibilityRejection> {
    counters.record_relation_recheck();
    let Some(edge) = edge_registry.get(family_id, from_semantic_version, to_semantic_version)
    else {
        counters.record_edge_missing_rejection();
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::MissingCompatibilityEdge,
            family_id.clone(),
            "declared compatibility edge is missing",
        ));
    };
    let relation = edge.relation();
    if relation == CompatibilityRelation::Incompatible {
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::DeclaredIncompatibleRelation,
            family_id.clone(),
            "declared compatibility edge explicitly rejects this semantic relation",
        ));
    }
    admit_adapter_cost(counters, family_id, edge, path)?;
    Ok(relation)
}

fn admit_adapter_cost(
    counters: &mut CompatibilityAdmissionCounters,
    family_id: &ArtifactFamilyId,
    edge: &DeclaredCompatibilityEdge,
    path: CompatibilityAdmissionPath,
) -> Result<(), CompatibilityRejection> {
    let Some(adapter) = edge.adapter() else {
        return Ok(());
    };
    match (path, adapter.cost_class()) {
        (_, CompatibilityAdapterCostClass::ZeroCopy)
        | (_, CompatibilityAdapterCostClass::BoundedRecordLocal) => Ok(()),
        (CompatibilityAdmissionPath::HotRead, CompatibilityAdapterCostClass::BoundedBatchLocal) => {
            counters.adapter_hot_path_rejection_count += 1;
            Err(CompatibilityRejection::new(
                CompatibilityRejectionKind::AdapterHotPathRejected,
                family_id.clone(),
                "batch-local compatibility adapter rejected from hot read path",
            ))
        }
        (
            CompatibilityAdmissionPath::HotRead | CompatibilityAdmissionPath::BatchRead,
            CompatibilityAdapterCostClass::MaintenanceOnly,
        ) => {
            counters.adapter_maintenance_required_rejection_count += 1;
            Err(CompatibilityRejection::new(
                CompatibilityRejectionKind::AdapterMaintenanceRequired,
                family_id.clone(),
                "maintenance-only compatibility adapter rejected from read path",
            ))
        }
        (_, CompatibilityAdapterCostClass::OutOfScope) => {
            counters.adapter_out_of_scope_rejection_count += 1;
            Err(CompatibilityRejection::new(
                CompatibilityRejectionKind::AdapterOutOfScope,
                family_id.clone(),
                "out-of-scope compatibility adapter rejected",
            ))
        }
        (
            CompatibilityAdmissionPath::BatchRead
            | CompatibilityAdmissionPath::MaintenanceScheduled,
            CompatibilityAdapterCostClass::BoundedBatchLocal,
        )
        | (
            CompatibilityAdmissionPath::MaintenanceScheduled,
            CompatibilityAdapterCostClass::MaintenanceOnly,
        ) => Ok(()),
    }
}

impl CompatibilityAdmissionCounters {
    pub(crate) fn record_relation_recheck(&mut self) {
        self.relation_recheck_count += 1;
    }

    pub(crate) fn record_edge_missing_rejection(&mut self) {
        self.edge_missing_rejection_count += 1;
    }

    fn record_admitted_relation(&mut self, relation: CompatibilityRelation) {
        match relation {
            CompatibilityRelation::Native => self.admitted_native_count += 1,
            CompatibilityRelation::BackwardRead | CompatibilityRelation::ForwardRead => {
                self.admitted_forward_backward_count += 1;
            }
            CompatibilityRelation::AdapterRequired => self.admitted_adapter_count += 1,
            CompatibilityRelation::DerivedRebuildRequired | CompatibilityRelation::Incompatible => {
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityReadAdmissionOutcome {
    family_id: ArtifactFamilyId,
    manifest_digest: CompatibilityManifestDigest,
    relation: Option<CompatibilityRelation>,
    rejection_kind: Option<CompatibilityRejectionKind>,
    counters: CompatibilityAdmissionCounters,
}

impl CompatibilityReadAdmissionOutcome {
    pub(crate) fn accepted(
        receipt: &ReadCompatibilityReceipt,
        counters: &CompatibilityAdmissionCounters,
    ) -> Self {
        Self {
            family_id: receipt.receipt().family_id().clone(),
            manifest_digest: receipt.receipt().manifest_digest().clone(),
            relation: Some(receipt.receipt().relation()),
            rejection_kind: None,
            counters: counters.clone(),
        }
    }

    pub(crate) fn rejected(
        artifact: &QuarantinedDecodedArtifact,
        rejection: &CompatibilityRejection,
        counters: &CompatibilityAdmissionCounters,
    ) -> Self {
        Self {
            family_id: artifact.family_id().clone(),
            manifest_digest: artifact.manifest_digest().clone(),
            relation: None,
            rejection_kind: Some(rejection.kind()),
            counters: counters.clone(),
        }
    }

    pub fn is_accepted(&self) -> bool {
        self.rejection_kind.is_none()
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn manifest_digest(&self) -> &CompatibilityManifestDigest {
        &self.manifest_digest
    }

    pub fn relation(&self) -> Option<CompatibilityRelation> {
        self.relation
    }

    pub fn rejection_kind(&self) -> Option<CompatibilityRejectionKind> {
        self.rejection_kind
    }

    pub fn counters(&self) -> &CompatibilityAdmissionCounters {
        &self.counters
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityWriteAdmissionOutcome {
    family_id: ArtifactFamilyId,
    manifest_digest: CompatibilityManifestDigest,
    relation: Option<CompatibilityRelation>,
    rejection_kind: Option<CompatibilityRejectionKind>,
    counters: CompatibilityAdmissionCounters,
}

impl CompatibilityWriteAdmissionOutcome {
    pub(crate) fn accepted(
        receipt: &WriteCompatibilityReceipt,
        counters: &CompatibilityAdmissionCounters,
    ) -> Self {
        Self {
            family_id: receipt.receipt().family_id().clone(),
            manifest_digest: receipt.receipt().manifest_digest().clone(),
            relation: Some(receipt.receipt().relation()),
            rejection_kind: None,
            counters: counters.clone(),
        }
    }

    pub(crate) fn rejected(
        artifact: &QuarantinedDecodedArtifact,
        rejection: &CompatibilityRejection,
        counters: &CompatibilityAdmissionCounters,
    ) -> Self {
        Self {
            family_id: artifact.family_id().clone(),
            manifest_digest: artifact.manifest_digest().clone(),
            relation: None,
            rejection_kind: Some(rejection.kind()),
            counters: counters.clone(),
        }
    }

    pub fn is_accepted(&self) -> bool {
        self.rejection_kind.is_none()
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn manifest_digest(&self) -> &CompatibilityManifestDigest {
        &self.manifest_digest
    }

    pub fn relation(&self) -> Option<CompatibilityRelation> {
        self.relation
    }

    pub fn rejection_kind(&self) -> Option<CompatibilityRejectionKind> {
        self.rejection_kind
    }

    pub fn counters(&self) -> &CompatibilityAdmissionCounters {
        &self.counters
    }
}
