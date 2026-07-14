use super::*;

define_materialization_admission_outcome!(
    LsmLookupMaterializationAdmissionCase,
    LsmLookupMaterializationAdmissionOutcome,
    LsmLookupMaterializationAdmissionView,
    LsmLookupMaterializationAdmissionCaseId,
    lsm_lookup_materialization_admission_cases,
    [
        MaterializationDenialKind::LsmSourceSecurityScopeMismatch,
        MaterializationDenialKind::LsmSourceStoreAuthorityMismatch,
    ]
);
define_materialization_admission_outcome!(
    LsmPublicationMaterializationAdmissionCase,
    LsmPublicationMaterializationAdmissionOutcome,
    LsmPublicationMaterializationAdmissionView,
    LsmPublicationMaterializationAdmissionCaseId,
    lsm_publication_materialization_admission_cases,
    [
        MaterializationDenialKind::LsmSourceSecurityScopeMismatch,
        MaterializationDenialKind::LsmSourceStoreAuthorityMismatch,
    ]
);
define_materialization_admission_outcome!(
    LsmReplayMaterializationAdmissionCase,
    LsmReplayMaterializationAdmissionOutcome,
    LsmReplayMaterializationAdmissionView,
    LsmReplayMaterializationAdmissionCaseId,
    lsm_replay_materialization_admission_cases,
    [
        MaterializationDenialKind::LsmSourceSecurityScopeMismatch,
        MaterializationDenialKind::LsmSourceStoreAuthorityMismatch,
        MaterializationDenialKind::MaterializationFrontierMismatch,
    ]
);

impl crate::planning::AccessPlanningFacade {
    pub fn admit_lsm_lookup_materialization(
        &self,
        family: crate::AdmittedPhysicalArtifactFamily,
        catalog: &crate::BootstrapCatalogReadAdmission,
        source: &crate::BaselineLsmLookupSource,
    ) -> LsmLookupMaterializationAdmissionOutcome {
        LsmLookupMaterializationAdmissionOutcome::issue(
            AdmittedLayoutMaterialization::admit_lsm_lookup_exact(family, catalog, source),
        )
    }

    pub fn admit_lsm_publication_materialization(
        &self,
        family: crate::AdmittedPhysicalArtifactFamily,
        catalog: &crate::BootstrapCatalogReadAdmission,
        execution: &crate::BaselineLsmManifestPublicationExecution,
    ) -> LsmPublicationMaterializationAdmissionOutcome {
        LsmPublicationMaterializationAdmissionOutcome::issue(
            AdmittedLayoutMaterialization::admit_lsm_publication_exact(family, catalog, execution),
        )
    }

    pub fn admit_lsm_replay_materialization(
        &self,
        family: crate::AdmittedPhysicalArtifactFamily,
        catalog: &crate::BootstrapCatalogReadAdmission,
        source: &worth_store_lsm_authority::AdmittedLsmReplaySource,
    ) -> LsmReplayMaterializationAdmissionOutcome {
        LsmReplayMaterializationAdmissionOutcome::issue(
            AdmittedLayoutMaterialization::admit_lsm_replay_exact(family, catalog, source),
        )
    }
}
