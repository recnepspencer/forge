use super::admission::{
    CompatibilityAdmissionCounters, CompatibilityEdgeRegistry, CompatibilityRejection,
    CompatibilityRejectionKind, CompatibilityRelation,
};
use super::manifests::{
    ArtifactCompatibilityWindow, ArtifactFamilyId, ArtifactSemanticVersion,
    CompatibilityManifestDigest,
};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BackupCompatibilityManifest {
    family_id: ArtifactFamilyId,
    window: ArtifactCompatibilityWindow,
    manifest_digest: CompatibilityManifestDigest,
}

impl BackupCompatibilityManifest {
    pub fn new(
        family_id: ArtifactFamilyId,
        window: ArtifactCompatibilityWindow,
        manifest_digest: CompatibilityManifestDigest,
    ) -> Self {
        Self {
            family_id,
            window,
            manifest_digest,
        }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn window(&self) -> &ArtifactCompatibilityWindow {
        &self.window
    }

    pub fn manifest_digest(&self) -> &CompatibilityManifestDigest {
        &self.manifest_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RestoreBackupScope {
    family_ids: Vec<ArtifactFamilyId>,
}

impl RestoreBackupScope {
    pub fn new(mut family_ids: Vec<ArtifactFamilyId>) -> Self {
        family_ids.sort();
        family_ids.dedup();
        Self { family_ids }
    }

    pub fn contains_family(&self, family_id: &ArtifactFamilyId) -> bool {
        self.family_ids.binary_search(family_id).is_ok()
    }

    pub fn family_count(&self) -> usize {
        self.family_ids.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RestoreCompatibilityTarget {
    family_id: ArtifactFamilyId,
    target_semantic_version: ArtifactSemanticVersion,
}

impl RestoreCompatibilityTarget {
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
pub struct DisasterRecoveryCompatibilityWindow {
    family_id: ArtifactFamilyId,
    window: ArtifactCompatibilityWindow,
    class: DisasterRecoveryCompatibilityClass,
}

impl DisasterRecoveryCompatibilityWindow {
    pub fn new(
        family_id: ArtifactFamilyId,
        window: ArtifactCompatibilityWindow,
        class: DisasterRecoveryCompatibilityClass,
    ) -> Self {
        Self {
            family_id,
            window,
            class,
        }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn class(&self) -> DisasterRecoveryCompatibilityClass {
        self.class
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DisasterRecoveryCompatibilityClass {
    AuthoritativeTruth,
    DerivedAcceleration,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DisasterRecoveryCompatibilityPlan {
    family_id: ArtifactFamilyId,
    class: DisasterRecoveryCompatibilityClass,
}

impl DisasterRecoveryCompatibilityPlan {
    pub(crate) fn new(window: &DisasterRecoveryCompatibilityWindow) -> Self {
        Self {
            family_id: window.family_id().clone(),
            class: window.class(),
        }
    }

    pub fn class(&self) -> DisasterRecoveryCompatibilityClass {
        self.class
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RestoreCompatibilityPlan {
    family_id: ArtifactFamilyId,
    admitted_manifest_digest: CompatibilityManifestDigest,
    target_semantic_version: ArtifactSemanticVersion,
    relation: CompatibilityRelation,
    publication_conflict_count: usize,
    witness: RestorePublicationWitness,
}

impl RestoreCompatibilityPlan {
    pub(crate) fn new(
        family_id: ArtifactFamilyId,
        admitted_manifest_digest: CompatibilityManifestDigest,
        target_semantic_version: ArtifactSemanticVersion,
        relation: CompatibilityRelation,
        publication_conflict_count: usize,
        witness: RestorePublicationWitness,
    ) -> Self {
        Self {
            family_id,
            admitted_manifest_digest,
            target_semantic_version,
            relation,
            publication_conflict_count,
            witness,
        }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn relation(&self) -> CompatibilityRelation {
        self.relation
    }

    pub fn publication_conflict_count(&self) -> usize {
        self.publication_conflict_count
    }

    pub(crate) fn witness(&self) -> &RestorePublicationWitness {
        &self.witness
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RestorePublicationWitness {
    family_id: ArtifactFamilyId,
}

impl RestorePublicationWitness {
    pub(crate) fn new(family_id: ArtifactFamilyId) -> Self {
        Self { family_id }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum RestorePublicationConflictKind {
    BranchHead,
    CursorCheckpoint,
    SchemaSupport,
    LineageSupport,
    TierManifest,
    Snapshot,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RestorePublicationConflictUnit {
    family_id: ArtifactFamilyId,
    kind: RestorePublicationConflictKind,
}

impl RestorePublicationConflictUnit {
    pub fn new(family_id: ArtifactFamilyId, kind: RestorePublicationConflictKind) -> Self {
        Self { family_id, kind }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RestorePublicationConflictSet {
    units: Vec<RestorePublicationConflictUnit>,
}

impl RestorePublicationConflictSet {
    pub fn new(units: Vec<RestorePublicationConflictUnit>) -> Self {
        Self { units }
    }

    pub fn is_empty(&self) -> bool {
        self.units.is_empty()
    }

    pub fn len(&self) -> usize {
        self.units.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RestoreVersionRejection {
    family_id: ArtifactFamilyId,
    reason: String,
}

impl RestoreVersionRejection {
    pub fn new(family_id: ArtifactFamilyId, reason: impl Into<String>) -> Self {
        Self {
            family_id,
            reason: reason.into(),
        }
    }
}

pub(crate) fn plan_restore_compatibility(
    counters: &mut CompatibilityAdmissionCounters,
    edge_registry: &CompatibilityEdgeRegistry,
    backup_scope: &RestoreBackupScope,
    backup_manifest: &BackupCompatibilityManifest,
    target: &RestoreCompatibilityTarget,
    publication_conflicts: &RestorePublicationConflictSet,
) -> Result<RestoreCompatibilityPlan, CompatibilityRejection> {
    if !backup_scope.contains_family(target.family_id()) {
        counters.record_restore_out_of_scope_scan_rejection();
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::RestoreOutOfScopeScanRejected,
            target.family_id().clone(),
            "restore compatibility may inspect only backup-scope families",
        ));
    }
    if backup_manifest.family_id() != target.family_id() {
        counters.record_restore_rejection();
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::RestoreCompatibilityRejected,
            target.family_id().clone(),
            "restore target family does not match backup manifest family",
        ));
    }
    if !publication_conflicts.is_empty() {
        counters.record_restore_publication_conflict_rejection();
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::RestorePublicationConflictRejected,
            target.family_id().clone(),
            "restore publication conflicts must be cleared before visibility publication",
        ));
    }

    let restored_semantic_version = backup_manifest.window().maximum_semantic();
    counters.record_relation_recheck();
    let Some(edge) = edge_registry.get(
        target.family_id(),
        restored_semantic_version,
        target.target_semantic_version(),
    ) else {
        counters.record_edge_missing_rejection();
        counters.record_restore_rejection();
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::MissingCompatibilityEdge,
            target.family_id().clone(),
            "declared restore compatibility edge is missing",
        ));
    };
    let relation = edge.relation();
    match relation {
        CompatibilityRelation::Native
        | CompatibilityRelation::ForwardRead
        | CompatibilityRelation::BackwardRead => {}
        CompatibilityRelation::AdapterRequired
        | CompatibilityRelation::DerivedRebuildRequired
        | CompatibilityRelation::Incompatible => {
            counters.record_restore_rejection();
            return Err(CompatibilityRejection::new(
                CompatibilityRejectionKind::RestoreCompatibilityRejected,
                target.family_id().clone(),
                "restore rejects adapter, rebuild, and incompatible compatibility edges",
            ));
        }
    }

    counters.record_restore_accept();
    Ok(RestoreCompatibilityPlan::new(
        target.family_id().clone(),
        backup_manifest.manifest_digest().clone(),
        target.target_semantic_version(),
        relation,
        publication_conflicts.len(),
        RestorePublicationWitness::new(target.family_id().clone()),
    ))
}

pub(crate) fn plan_disaster_recovery_compatibility(
    counters: &mut CompatibilityAdmissionCounters,
    window: &DisasterRecoveryCompatibilityWindow,
) -> DisasterRecoveryCompatibilityPlan {
    match window.class() {
        DisasterRecoveryCompatibilityClass::AuthoritativeTruth => {
            counters.record_disaster_recovery_truth_window();
        }
        DisasterRecoveryCompatibilityClass::DerivedAcceleration => {
            counters.record_disaster_recovery_derived_window();
        }
    }
    DisasterRecoveryCompatibilityPlan::new(window)
}
