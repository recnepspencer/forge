use worth_store_contracts::DurableArtifactFamilyId;
use worth_store_layout_indexes::{access_planning, ObserveOwnerCase};
use worth_store_security::{
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreTenantScope,
};
use worth_store_test_support::{
    admitted_layout_bootstrap_catalog, deterministic_admitted_btree_replay_physical_source,
    deterministic_baseline_btree_read_source, deterministic_cross_store_btree_read_source,
    SecurityScopeFixtureAuthority,
};

use super::fixture_admission::{admit_family, security_scope};
use super::LayoutOwnerObservationLedger;

pub(super) fn execute(ledger: &mut LayoutOwnerObservationLedger) {
    execute_catalog_root(ledger);
    execute_btree_publication(ledger);
    execute_btree_lookup(ledger);
    execute_btree_replay(ledger);
    execute_imported_blob(ledger);
}

fn execute_catalog_root(ledger: &mut LayoutOwnerObservationLedger) {
    let security = tenant_page_security();
    let family = admit_family(DurableArtifactFamilyId::PhysicalPage, &security);
    let catalog = admitted_layout_bootstrap_catalog();
    let outcome = access_planning().admit_current_catalog_root_materialization(family, &catalog);
    ledger.record_catalog_root_materialization(outcome.owner_case_observation());
}

fn execute_btree_publication(ledger: &mut LayoutOwnerObservationLedger) {
    let security = tenant_page_security();
    let family = admit_family(DurableArtifactFamilyId::PhysicalPage, &security);
    let catalog = admitted_layout_bootstrap_catalog();
    let publication = worth_store_test_support::harness::physical_isolation::publication::
        root_publication_validation(41, 2);
    let outcome =
        access_planning().admit_btree_publication_materialization(family, &catalog, publication);
    ledger.record_btree_publication_materialization(outcome.owner_case_observation());
}

fn execute_btree_lookup(ledger: &mut LayoutOwnerObservationLedger) {
    let security = tenant_page_security();
    let family = admit_family(DurableArtifactFamilyId::PhysicalPage, &security);
    let catalog = admitted_layout_bootstrap_catalog();
    for source in [
        deterministic_baseline_btree_read_source(),
        deterministic_cross_store_btree_read_source(),
    ] {
        let outcome =
            access_planning().admit_btree_lookup_materialization(family, &catalog, &source);
        ledger.record_btree_lookup_materialization(outcome.owner_case_observation());
    }
}

fn execute_btree_replay(ledger: &mut LayoutOwnerObservationLedger) {
    let security = tenant_page_security();
    let family = admit_family(DurableArtifactFamilyId::PhysicalPage, &security);
    let catalog = admitted_layout_bootstrap_catalog();
    let source = deterministic_admitted_btree_replay_physical_source();
    let outcome = access_planning().admit_btree_replay_materialization(family, &catalog, &source);
    ledger.record_btree_replay_materialization(outcome.owner_case_observation());
}

fn execute_imported_blob(ledger: &mut LayoutOwnerObservationLedger) {
    let catalog = admitted_layout_bootstrap_catalog();
    let blob_security = tenant_blob_security();
    let blob_family = admit_family(DurableArtifactFamilyId::BlobManifest, &blob_security);
    let page_security = tenant_page_security();
    let page_family = admit_family(DurableArtifactFamilyId::PhysicalPage, &page_security);
    let wrong_scope_family = admit_family(DurableArtifactFamilyId::BlobManifest, &page_security);
    let admitted = worth_store_blob_chunks::certification_test_authority::
        execute_readmitted_blob_import_for_store(
            "layout.owner-matrix.imported-blob",
            "store.physical.default_instance",
        );
    let foreign = worth_store_blob_chunks::certification_test_authority::
        execute_readmitted_blob_import_for_store(
            "layout.owner-matrix.imported-blob.foreign",
            "store.owner_matrix.foreign",
        );

    for (family, witness) in [
        (blob_family, &admitted),
        (page_family, &admitted),
        (wrong_scope_family, &admitted),
        (blob_family, &foreign),
    ] {
        let outcome =
            access_planning().admit_imported_blob_materialization(family, &catalog, witness);
        ledger.record_imported_blob_materialization(outcome.owner_case_observation());
    }
}

fn tenant_page_security() -> worth_store_security::StoreAdmittedSecurityScope {
    security_scope(
        SecurityScopeFixtureAuthority::Current,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    )
}

fn tenant_blob_security() -> worth_store_security::StoreAdmittedSecurityScope {
    security_scope(
        SecurityScopeFixtureAuthority::Current,
        StoreKeyScope::BlobChunkEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedBlobChunk,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    )
}
