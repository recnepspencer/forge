use super::*;

define_materialization_admission_outcome!(
    RestoredArtifactMaterializationAdmissionCase,
    RestoredArtifactMaterializationAdmissionOutcome,
    RestoredArtifactMaterializationAdmissionView,
    RestoredArtifactMaterializationAdmissionCaseId,
    restored_artifact_materialization_admission_cases,
    [
        MaterializationDenialKind::RestoreOfflineReadmissionRequired,
        MaterializationDenialKind::RestoreCustodyReadmissionRequired,
        MaterializationDenialKind::RestoreCurrentStoreAuthorityRequired,
    ]
);

impl crate::planning::AccessPlanningFacade {
    pub fn admit_restored_artifact_materialization(
        &self,
        family: crate::AdmittedPhysicalArtifactFamily,
        catalog: &crate::BootstrapCatalogReadAdmission,
        readmission: crate::integrity::LayoutReadmissionWitness,
        custody: &worth_store_security::StoreReadmittedSecurityScope,
    ) -> RestoredArtifactMaterializationAdmissionOutcome {
        RestoredArtifactMaterializationAdmissionOutcome::issue(
            AdmittedLayoutMaterialization::admit_restored_artifact_exact(
                family,
                catalog,
                readmission,
                custody,
            ),
        )
    }
}
