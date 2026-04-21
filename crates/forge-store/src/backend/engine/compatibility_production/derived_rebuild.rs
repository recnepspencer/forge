use crate::compatibility::{
    admit_derived_rebuild_maintenance, plan_derived_lane_compatibility, plan_read_compatibility,
    prove_compatibility_maintenance_lane_admission, prove_retained_authority_for_derived_rebuild,
    require_matching_maintenance_lane, ArtifactCompatibilityWindow, ArtifactSemanticVersion,
    CompatibilityAdmissionBatch, CompatibilityDerivedRebuildOutcome,
    CompatibilityDerivedRebuildRequest, CompatibilityMaintenanceLaneRequirement,
    CompatibilityReadIntent, CompatibilityRegistry, DerivedBasisCompatibilityInput,
    DerivedCompatibilityLaneRegistry, DerivedFamilyDeclaration, ReaderCapabilitySet,
};
use crate::failure::{StoreError, StoreErrorKind};
use crate::maintenance::{
    DerivedFamilyRebuildMaintenanceDeclaration, MaintenanceBatch, MaintenanceBatchClass,
    MaintenanceDeclaration, MaintenanceDeclarationId,
};

use super::super::compatibility_runtime::first_ship_native_edge_registry;
use super::super::{
    compatibility_runtime::compatibility_rejection_error, StateBackedStoreBackend, StatePersistence,
};

impl<P: StatePersistence> StateBackedStoreBackend<P> {
    pub(crate) fn execute_compatibility_derived_rebuild(
        &mut self,
        request: CompatibilityDerivedRebuildRequest,
    ) -> Result<CompatibilityDerivedRebuildOutcome, StoreError> {
        let family_kind = request.family_kind();
        let snapshot = CompatibilityRegistry::first_ship();
        let lane_snapshot =
            DerivedCompatibilityLaneRegistry::from_compatibility_snapshot(&snapshot).snapshot();
        let lane_declaration = lane_snapshot
            .get_by_family_kind(family_kind)
            .cloned()
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::CompatibilityDerivedRebuildIncompatible,
                    format!(
                        "compatibility-triggered rebuild is not declared for family `{}`",
                        family_kind.label()
                    ),
                )
            })?;
        let derived_family = DerivedFamilyDeclaration::new(
            snapshot
                .get(family_kind)
                .ok_or_else(|| {
                    StoreError::new(
                        StoreErrorKind::CompatibilityArtifactFamilyUndeclared,
                        format!(
                            "compatibility-triggered rebuild encountered undeclared family `{}`",
                            family_kind.label()
                        ),
                    )
                })?
                .clone(),
        );
        let artifact = self
            .runtime_compatibility_artifact(family_kind, "execute_compatibility_derived_rebuild")?;
        let family_id = artifact.family_id().clone();
        let semantic_version = artifact.semantic_version();
        let manifest_index = self.runtime_compatibility_manifest_index();
        let edge_registry = first_ship_native_edge_registry();
        let reader = ReaderCapabilitySet::new(family_id.clone(), vec![semantic_version]);
        let intent = CompatibilityReadIntent::new(family_id.clone(), semantic_version);
        let mut batch = CompatibilityAdmissionBatch::new();
        let read_receipt = plan_read_compatibility(
            &mut batch,
            &manifest_index,
            &edge_registry,
            &reader,
            &intent,
            &artifact,
        )
        .map_err(|rejection| {
            compatibility_rejection_error("execute_compatibility_derived_rebuild.read", rejection)
        })?;
        let target_semantic_version = ArtifactSemanticVersion::new(semantic_version.value() + 1);
        let plan = plan_derived_lane_compatibility(
            batch.counters_mut(),
            &DerivedBasisCompatibilityInput::new(
                lane_declaration.clone(),
                derived_family,
                ArtifactCompatibilityWindow::native(target_semantic_version.value()),
            ),
            &artifact,
            &read_receipt,
        )
        .map_err(|rejection| {
            compatibility_rejection_error("execute_compatibility_derived_rebuild.plan", rejection)
        })?;
        let rebuild_requirement = plan
            .rebuild_requirement()
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::CompatibilityDerivedRebuildIncompatible,
                    format!(
                        "compatibility-triggered rebuild for family `{}` did not produce a rebuild requirement",
                        family_kind.label()
                    ),
                )
            })?
            .requirement();
        let retained_authority = prove_retained_authority_for_derived_rebuild(family_id.clone());
        let maintenance_lane_requirement = CompatibilityMaintenanceLaneRequirement::new(
            family_id.clone(),
            lane_declaration.certification_lane_id().to_string(),
            lane_declaration
                .lane_kind()
                .maintenance_work_class_label()
                .unwrap_or("DerivedFamilyRebuild"),
        );
        let declaration_id = MaintenanceDeclarationId::new(format!(
            "compatibility-derived-rebuild:{}:{}:{}",
            family_kind.label(),
            semantic_version.value(),
            target_semantic_version.value()
        ));
        let declaration = MaintenanceDeclaration::derived_family_rebuild(
            declaration_id.clone(),
            DerivedFamilyRebuildMaintenanceDeclaration::new(
                "compatibility-retained-authority",
                family_kind.label(),
                format!(
                    "{}:{}->{}",
                    family_kind.label(),
                    semantic_version.value(),
                    target_semantic_version.value()
                ),
            ),
        );
        let receipt = self.admit_maintenance_batch(MaintenanceBatch::new(
            format!(
                "compatibility-derived-rebuild-batch:{}",
                family_kind.label()
            ),
            MaintenanceBatchClass::Retention,
            vec![declaration],
        ))?;
        let admitted = receipt
            .admitted_declarations()
            .first()
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::CompatibilityDerivedRebuildIncompatible,
                    format!(
                        "compatibility-triggered rebuild for family `{}` was not admitted into maintenance",
                        family_kind.label()
                    ),
                )
            })?;
        let maintenance_lane_id = admitted.descriptor().lane_key().artifact_id();
        let maintenance_lane_admission = prove_compatibility_maintenance_lane_admission(
            batch.counters_mut(),
            &maintenance_lane_requirement,
            maintenance_lane_id.clone(),
        );
        require_matching_maintenance_lane(
            batch.counters_mut(),
            &maintenance_lane_requirement,
            &maintenance_lane_admission,
        )
        .map_err(|rejection| {
            compatibility_rejection_error(
                "execute_compatibility_derived_rebuild.maintenance_lane",
                rejection,
            )
        })?;
        let _rebuild_plan = admit_derived_rebuild_maintenance(
            batch.counters_mut(),
            rebuild_requirement,
            Some(&retained_authority),
            Some(maintenance_lane_admission.witness()),
        )
        .map_err(|rejection| {
            compatibility_rejection_error("execute_compatibility_derived_rebuild.admit", rejection)
        })?;
        let completed = self
            .start_maintenance_declaration(admitted)
            .map_err(|failed| {
                StoreError::new(
                    StoreErrorKind::CompatibilityDerivedRebuildIncompatible,
                    format!(
                        "compatibility-triggered rebuild execution failed for family `{}`: {} ({})",
                        family_kind.label(),
                        failed.message(),
                        failed.error_kind()
                    ),
                )
            })?;
        Ok(CompatibilityDerivedRebuildOutcome::new(
            family_kind,
            declaration_id.as_str(),
            maintenance_lane_id,
            completed.last_completed_phase(),
            crate::Milestone12AdmissionReport::from_admission_counters(batch.counters()),
        ))
    }
}
