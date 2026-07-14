use super::*;

define_materialization_admission_outcome!(
    CatalogRootMaterializationAdmissionCase,
    CatalogRootMaterializationAdmissionOutcome,
    CatalogRootMaterializationAdmissionView,
    CatalogRootMaterializationAdmissionCaseId,
    catalog_root_materialization_admission_cases,
    []
);

impl crate::planning::AccessPlanningFacade {
    pub fn admit_current_catalog_root_materialization(
        &self,
        family: crate::AdmittedPhysicalArtifactFamily,
        catalog: &crate::BootstrapCatalogReadAdmission,
    ) -> CatalogRootMaterializationAdmissionOutcome {
        CatalogRootMaterializationAdmissionOutcome::issue(
            AdmittedLayoutMaterialization::admit_current_catalog_root(family, catalog),
        )
    }
}
