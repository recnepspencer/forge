use crate::access::shape::{
    access_shapes, AccessLaneClassification, AccessShapeContract, AccessShapeUnsupportedDenial,
};
use crate::materialization::{AdmittedLayoutMaterialization, MaterializationDenial};
use crate::planning::AccessPlanSelector;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccessPlanningFacade;

impl AccessPlanningFacade {
    pub fn point_access(&self) -> AccessShapeContract {
        access_shapes().point_lookup_declaration()
    }

    pub fn range_access(&self) -> AccessShapeContract {
        access_shapes().range_lookup_declaration()
    }

    pub fn prefix_access(&self) -> AccessShapeContract {
        access_shapes().prefix_lookup_declaration()
    }

    pub fn rebuild_access(
        &self,
        lane: AccessLaneClassification,
    ) -> Result<AccessShapeContract, AccessShapeUnsupportedDenial> {
        access_shapes().rebuild_read_declaration(lane)
    }

    pub fn current_materialization_frontier(
        &self,
        catalog: &crate::BootstrapCatalogReadAdmission,
    ) -> crate::CurrentMaterializationFrontier {
        crate::CurrentMaterializationFrontier::from_catalog(catalog)
    }
    pub fn current_btree_materialization_frontier(
        &self,
        catalog: &crate::BootstrapCatalogReadAdmission,
        source: &crate::BaselineBTreeReadSource,
    ) -> crate::CurrentMaterializationFrontier {
        crate::CurrentMaterializationFrontier::from_btree_source(catalog, source)
    }
    pub fn current_lsm_materialization_frontier(
        &self,
        catalog: &crate::BootstrapCatalogReadAdmission,
        source: &crate::BaselineLsmLookupSource,
    ) -> crate::CurrentMaterializationFrontier {
        crate::CurrentMaterializationFrontier::from_lsm_lookup_source(catalog, source)
    }

    pub fn current_lsm_replay_materialization_frontier(
        self,
        catalog: &crate::BootstrapCatalogReadAdmission,
        source: &forge_store_lsm_authority::AdmittedLsmReplaySource,
    ) -> Result<crate::CurrentMaterializationFrontier, MaterializationDenial> {
        crate::CurrentMaterializationFrontier::from_lsm_replay_source(catalog, source)
    }
    pub fn admit_current_catalog_root_materialization(
        &self,
        family: crate::AdmittedPhysicalArtifactFamily,
        catalog: &crate::BootstrapCatalogReadAdmission,
    ) -> Result<AdmittedLayoutMaterialization, MaterializationDenial> {
        AdmittedLayoutMaterialization::admit_current_catalog_root(family, catalog)
    }

    pub fn admit_btree_publication_materialization(
        &self,
        family: crate::AdmittedPhysicalArtifactFamily,
        catalog: &crate::BootstrapCatalogReadAdmission,
        publication: forge_store_physical_format::RootPublicationValidationWitness,
    ) -> Result<AdmittedLayoutMaterialization, MaterializationDenial> {
        AdmittedLayoutMaterialization::admit_btree_publication_exact(family, catalog, publication)
    }

    pub fn admit_btree_lookup_materialization(
        &self,
        family: crate::AdmittedPhysicalArtifactFamily,
        catalog: &crate::BootstrapCatalogReadAdmission,
        source: &crate::BaselineBTreeReadSource,
    ) -> Result<AdmittedLayoutMaterialization, MaterializationDenial> {
        AdmittedLayoutMaterialization::admit_btree_lookup_exact(family, catalog, source)
    }

    pub fn admit_btree_replay_materialization(
        &self,
        family: crate::AdmittedPhysicalArtifactFamily,
        catalog: &crate::BootstrapCatalogReadAdmission,
        source: &forge_store_recovery_physics::AdmittedBTreeReplayPhysicalSource,
    ) -> Result<AdmittedLayoutMaterialization, MaterializationDenial> {
        AdmittedLayoutMaterialization::admit_btree_replay_exact(family, catalog, source)
    }

    pub fn admit_lsm_lookup_materialization(
        &self,
        family: crate::AdmittedPhysicalArtifactFamily,
        catalog: &crate::BootstrapCatalogReadAdmission,
        source: &crate::BaselineLsmLookupSource,
    ) -> Result<AdmittedLayoutMaterialization, MaterializationDenial> {
        AdmittedLayoutMaterialization::admit_lsm_lookup_exact(family, catalog, source)
    }

    pub fn admit_lsm_publication_materialization(
        &self,
        family: crate::AdmittedPhysicalArtifactFamily,
        catalog: &crate::BootstrapCatalogReadAdmission,
        execution: &crate::BaselineLsmManifestPublicationExecution,
    ) -> Result<AdmittedLayoutMaterialization, MaterializationDenial> {
        AdmittedLayoutMaterialization::admit_lsm_publication_exact(family, catalog, execution)
    }

    pub fn admit_lsm_replay_materialization(
        &self,
        family: crate::AdmittedPhysicalArtifactFamily,
        catalog: &crate::BootstrapCatalogReadAdmission,
        source: &forge_store_lsm_authority::AdmittedLsmReplaySource,
    ) -> Result<crate::AdmittedLayoutMaterialization, crate::MaterializationDenial> {
        crate::AdmittedLayoutMaterialization::admit_lsm_replay_exact(family, catalog, source)
    }

    pub fn admit_imported_blob_materialization(
        &self,
        family: crate::AdmittedPhysicalArtifactFamily,
        catalog: &crate::BootstrapCatalogReadAdmission,
        witness: &forge_store_blob_chunks::ImportedBlobWitness,
    ) -> Result<crate::AdmittedLayoutMaterialization, crate::MaterializationDenial> {
        crate::AdmittedLayoutMaterialization::admit_imported_blob_exact(family, catalog, witness)
    }

    pub fn admit_restored_artifact_materialization(
        &self,
        family: crate::AdmittedPhysicalArtifactFamily,
        catalog: &crate::BootstrapCatalogReadAdmission,
        readmission: crate::integrity::LayoutReadmissionWitness,
        custody: forge_store_security::StoreReadmittedSecurityScope,
    ) -> Result<crate::AdmittedLayoutMaterialization, crate::MaterializationDenial> {
        crate::AdmittedLayoutMaterialization::admit_restored_artifact_exact(
            family,
            catalog,
            readmission,
            custody,
        )
    }

    pub fn admit_imported_blob_read_request(
        &self,
        family: crate::AdmittedPhysicalArtifactFamily,
        key_domain: crate::AdmittedPhysicalKeyDomain,
        catalog: &crate::BootstrapCatalogReadAdmission,
        witness: &forge_store_blob_chunks::ImportedBlobWitness,
    ) -> super::ImportedBlobReadAdmissionOutcome {
        super::imported_blob::admit_imported_blob_read_request(family, key_domain, catalog, witness)
    }

    pub const fn selection(&self) -> AccessPlanSelector {
        AccessPlanSelector
    }
}

pub const fn access_planning() -> AccessPlanningFacade {
    AccessPlanningFacade
}
