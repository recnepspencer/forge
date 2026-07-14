use super::*;

define_materialization_admission_outcome!(
    ImportedBlobMaterializationAdmissionCase,
    ImportedBlobMaterializationAdmissionOutcome,
    ImportedBlobMaterializationAdmissionView,
    ImportedBlobMaterializationAdmissionCaseId,
    imported_blob_materialization_admission_cases,
    [
        MaterializationDenialKind::ImportedBlobFamilyRequired,
        MaterializationDenialKind::ImportedBlobSecurityScopeMismatch,
        MaterializationDenialKind::ImportedBlobStoreAuthorityMismatch,
    ]
);

impl crate::planning::AccessPlanningFacade {
    pub fn admit_imported_blob_materialization(
        &self,
        family: crate::AdmittedPhysicalArtifactFamily,
        catalog: &crate::BootstrapCatalogReadAdmission,
        witness: &forge_store_blob_chunks::ImportedBlobWitness,
    ) -> ImportedBlobMaterializationAdmissionOutcome {
        ImportedBlobMaterializationAdmissionOutcome::issue(
            AdmittedLayoutMaterialization::admit_imported_blob_exact(family, catalog, witness),
        )
    }
}
