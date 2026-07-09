use crate::authority::AuthoritativeExportBundle;
use crate::compatibility::{
    check_artifact_with_read_receipt, execute_restore_publication, plan_read_compatibility,
    plan_restore_compatibility, plan_write_compatibility, ArtifactSemanticVersion,
    BackupCompatibilityManifest, CompatibilityAdmissionBatch, CompatibilityAdmissionCounters,
    CompatibilityEdgeRegistry, CompatibilityFamilyKind, CompatibilityManifestDeclaration,
    CompatibilityManifestIndex, CompatibilityReadIntent, CompatibilityRegistry,
    CompatibilityRelation, CompatibilityWriteIntent, DeclaredCompatibilityEdge,
    QuarantinedDecodedArtifact, ReaderCapabilitySet, RestoreBackupScope,
    RestoreCompatibilityReceipt, RestoreCompatibilityTarget, RestorePublicationConflictSet,
    WriterCapabilitySet, FIRST_SHIP_COMPATIBILITY_FAMILIES,
};
use crate::failure::{StoreError, StoreErrorKind};
use std::collections::BTreeMap;

use super::{StateBackedStoreBackend, StatePersistence};

fn first_ship_native_semantic_version() -> ArtifactSemanticVersion {
    ArtifactSemanticVersion::new(1)
}

pub(crate) fn first_ship_native_edge_registry() -> CompatibilityEdgeRegistry {
    CompatibilityEdgeRegistry::new(
        FIRST_SHIP_COMPATIBILITY_FAMILIES
            .into_iter()
            .map(|family_kind| {
                DeclaredCompatibilityEdge::new(
                    family_kind.family_id(),
                    first_ship_native_semantic_version(),
                    first_ship_native_semantic_version(),
                    CompatibilityRelation::Native,
                )
            })
            .collect(),
    )
}

pub(crate) fn compatibility_rejection_error(
    operation_name: &str,
    rejection: crate::compatibility::CompatibilityRejection,
) -> StoreError {
    StoreError::new(
        rejection.store_error_kind(),
        format!(
            "{operation_name} compatibility rejected family `{}`: {}",
            rejection.family_id().as_str(),
            rejection.reason()
        ),
    )
}

fn authoritative_export_families(
    bundle: &AuthoritativeExportBundle,
) -> Vec<CompatibilityFamilyKind> {
    let mut families = Vec::new();
    if !bundle.commit_envelopes.is_empty() || !bundle.commit_parent_records.is_empty() {
        families.push(CompatibilityFamilyKind::CommitEnvelope);
    }
    if !bundle.branch_records.is_empty() || !bundle.branch_head_records.is_empty() {
        families.push(CompatibilityFamilyKind::BranchVersionDagRecord);
    }
    if !bundle.commit_support_summaries.is_empty()
        || !bundle.schema_support_records.is_empty()
        || !bundle.lineage_support_records.is_empty()
        || !bundle.durable_cursor_identity_records.is_empty()
        || !bundle.subscriber_checkpoint_records.is_empty()
    {
        families.push(CompatibilityFamilyKind::SchemaLineageCursorCheckpointSupport);
    }
    families
}

#[cfg_attr(not(test), allow(dead_code))]
#[derive(Debug)]
pub(crate) struct AuthoritativeExportRestoreExecution {
    receipts: Vec<RestoreCompatibilityReceipt>,
    counters: CompatibilityAdmissionCounters,
}

#[cfg_attr(not(test), allow(dead_code))]
impl AuthoritativeExportRestoreExecution {
    pub(crate) fn receipts(&self) -> &[RestoreCompatibilityReceipt] {
        &self.receipts
    }

    pub(crate) fn counters(&self) -> &CompatibilityAdmissionCounters {
        &self.counters
    }
}

pub(crate) fn execute_authoritative_export_restore_with_conflicts(
    bundle: &AuthoritativeExportBundle,
    conflicts_by_family: &BTreeMap<CompatibilityFamilyKind, RestorePublicationConflictSet>,
) -> Result<AuthoritativeExportRestoreExecution, StoreError> {
    let snapshot = CompatibilityRegistry::first_ship();
    let edge_registry = first_ship_native_edge_registry();
    let family_kinds = authoritative_export_families(bundle);
    let backup_scope = RestoreBackupScope::new(
        family_kinds
            .iter()
            .map(|family_kind| family_kind.family_id())
            .collect(),
    );
    let empty_conflicts = RestorePublicationConflictSet::new(Vec::new());
    let target_semantic_version = first_ship_native_semantic_version();
    let mut counters = CompatibilityAdmissionCounters::default();
    let mut receipts = Vec::with_capacity(family_kinds.len());

    for family_kind in family_kinds {
        let declaration = snapshot.get(family_kind).ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::CompatibilityArtifactFamilyUndeclared,
                format!(
                    "restore_from_authoritative_export encountered undeclared family `{}`",
                    family_kind.label()
                ),
            )
        })?;
        let CompatibilityManifestDeclaration::Authoritative(manifest) = declaration.manifest()
        else {
            return Err(StoreError::new(
                StoreErrorKind::CompatibilityRestoreRejected,
                format!(
                    "restore_from_authoritative_export cannot treat derived family `{}` as authoritative restore truth",
                    family_kind.label()
                ),
            ));
        };
        let backup_manifest = BackupCompatibilityManifest::new(
            manifest.family_id().clone(),
            manifest.window().clone(),
            manifest.digest().clone(),
        );
        let target =
            RestoreCompatibilityTarget::new(manifest.family_id().clone(), target_semantic_version);
        let plan = plan_restore_compatibility(
            &mut counters,
            &edge_registry,
            &backup_scope,
            &backup_manifest,
            &target,
            conflicts_by_family
                .get(&family_kind)
                .unwrap_or(&empty_conflicts),
        )
        .map_err(|rejection| {
            compatibility_rejection_error("restore_from_authoritative_export", rejection)
        })?;
        receipts.push(execute_restore_publication(plan));
    }

    Ok(AuthoritativeExportRestoreExecution { receipts, counters })
}

pub(crate) fn execute_authoritative_export_restore(
    bundle: &AuthoritativeExportBundle,
) -> Result<AuthoritativeExportRestoreExecution, StoreError> {
    execute_authoritative_export_restore_with_conflicts(bundle, &BTreeMap::new())
}

impl<P: StatePersistence> StateBackedStoreBackend<P> {
    pub(crate) fn runtime_compatibility_manifest_index(&self) -> CompatibilityManifestIndex {
        let snapshot = CompatibilityRegistry::first_ship();
        let recovered = self.state.recovered_compatibility_manifest_index();
        CompatibilityManifestIndex::rebuild_from_recovered_manifests(&snapshot, &recovered)
    }

    pub(crate) fn runtime_compatibility_artifact(
        &self,
        family_kind: CompatibilityFamilyKind,
        operation_name: &str,
    ) -> Result<QuarantinedDecodedArtifact, StoreError> {
        let recovered = self.state.recovered_compatibility_manifest_index();
        let family_id = family_kind.family_id();
        let record = recovered.get(&family_id).ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::CompatibilityManifestPublicationGap,
                format!(
                    "{operation_name} requires a persisted compatibility manifest publication for family `{}`",
                    family_id.as_str()
                ),
            )
        })?;
        Ok(QuarantinedDecodedArtifact::new(
            record.family_id().clone(),
            record.window().maximum_format(),
            record.window().maximum_semantic(),
            record.manifest_digest().clone(),
            format!("runtime:{}", record.family_id().as_str()),
            format!("{operation_name} compatibility runtime gate"),
        ))
    }

    pub(super) fn admit_runtime_read_compatibility(
        &self,
        family_kind: CompatibilityFamilyKind,
        operation_name: &str,
    ) -> Result<(), StoreError> {
        let artifact = self.runtime_compatibility_artifact(family_kind, operation_name)?;
        let family_id = artifact.family_id().clone();
        let semantic_version = artifact.semantic_version();
        let manifest_index = self.runtime_compatibility_manifest_index();
        let edge_registry = first_ship_native_edge_registry();
        let reader = ReaderCapabilitySet::new(family_id.clone(), vec![semantic_version]);
        let intent = CompatibilityReadIntent::new(family_id, semantic_version);
        let mut batch = CompatibilityAdmissionBatch::new();
        let receipt = plan_read_compatibility(
            &mut batch,
            &manifest_index,
            &edge_registry,
            &reader,
            &intent,
            &artifact,
        )
        .map_err(|rejection| compatibility_rejection_error(operation_name, rejection))?;
        check_artifact_with_read_receipt(artifact, &receipt)
            .map_err(|rejection| compatibility_rejection_error(operation_name, rejection))?;
        Ok(())
    }

    pub(super) fn admit_runtime_write_compatibility(
        &self,
        family_kind: CompatibilityFamilyKind,
        operation_name: &str,
    ) -> Result<(), StoreError> {
        let artifact = self.runtime_compatibility_artifact(family_kind, operation_name)?;
        let family_id = artifact.family_id().clone();
        let semantic_version = artifact.semantic_version();
        let manifest_index = self.runtime_compatibility_manifest_index();
        let edge_registry = first_ship_native_edge_registry();
        let writer = WriterCapabilitySet::new(family_id.clone(), vec![semantic_version]);
        let intent = CompatibilityWriteIntent::new(family_id, semantic_version);
        let mut batch = CompatibilityAdmissionBatch::new();
        let _receipt = plan_write_compatibility(
            &mut batch,
            &manifest_index,
            &edge_registry,
            &writer,
            &intent,
            &artifact,
        )
        .map_err(|rejection| compatibility_rejection_error(operation_name, rejection))?;
        Ok(())
    }

    #[cfg(test)]
    pub fn remove_compatibility_manifest_record_for_test(
        &mut self,
        family_kind: CompatibilityFamilyKind,
    ) {
        let artifact_id =
            crate::backend::records::compatibility_manifest_artifact_id(&family_kind.family_id());
        self.state
            .compatibility_manifest_records
            .remove(&artifact_id);
    }
}
