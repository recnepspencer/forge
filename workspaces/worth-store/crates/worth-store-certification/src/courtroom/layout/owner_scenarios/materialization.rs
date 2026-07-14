use worth_store_contracts::DurableArtifactFamilyId;
use worth_store_layout_indexes::{access_planning, ObserveOwnerCase};
use worth_store_security::{
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreTenantScope,
};
use worth_store_test_support::{
    admitted_layout_bootstrap_catalog, deterministic_admitted_btree_replay_physical_source,
    deterministic_baseline_btree_read_source, deterministic_cross_store_btree_read_source,
    execute_baseline_lsm_persisted_fixture, execute_baseline_lsm_replay_source_fixture,
    execute_frontierless_lsm_replay_source_fixture, SecurityScopeFixtureAuthority,
};

use super::fixture_admission::{admit_family, security_scope};
use super::LayoutOwnerObservationLedger;

pub(super) fn execute(ledger: &mut LayoutOwnerObservationLedger) {
    execute_catalog_root(ledger);
    execute_btree_publication(ledger);
    execute_btree_lookup(ledger);
    execute_btree_replay(ledger);
    execute_lsm_lookup(ledger);
    execute_lsm_publication(ledger);
    execute_lsm_replay(ledger);
    execute_imported_blob(ledger);
    execute_restore(ledger);
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

fn execute_lsm_lookup(ledger: &mut LayoutOwnerObservationLedger) {
    let catalog = admitted_layout_bootstrap_catalog();
    let published = execute_baseline_lsm_persisted_fixture();
    let source = published.admit_lookup_source();
    let families = lsm_family_cases();
    for family in families {
        let outcome = access_planning().admit_lsm_lookup_materialization(family, &catalog, &source);
        ledger.record_lsm_lookup_materialization(outcome.owner_case_observation());
    }
}

fn execute_lsm_publication(ledger: &mut LayoutOwnerObservationLedger) {
    let catalog = admitted_layout_bootstrap_catalog();
    let published = execute_baseline_lsm_persisted_fixture();
    let execution = published.publication_execution();
    for family in lsm_family_cases() {
        let outcome =
            access_planning().admit_lsm_publication_materialization(family, &catalog, &execution);
        ledger.record_lsm_publication_materialization(outcome.owner_case_observation());
    }
}

fn execute_lsm_replay(ledger: &mut LayoutOwnerObservationLedger) {
    let catalog = admitted_layout_bootstrap_catalog();
    let source = execute_baseline_lsm_replay_source_fixture();
    for family in lsm_family_cases() {
        let outcome = access_planning().admit_lsm_replay_materialization(family, &catalog, &source);
        ledger.record_lsm_replay_materialization(outcome.owner_case_observation());
    }
    let frontierless = execute_frontierless_lsm_replay_source_fixture();
    let family = lsm_family_cases()[0];
    let outcome =
        access_planning().admit_lsm_replay_materialization(family, &catalog, &frontierless);
    ledger.record_lsm_replay_materialization(outcome.owner_case_observation());
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

fn execute_restore(ledger: &mut LayoutOwnerObservationLedger) {
    let catalog = admitted_layout_bootstrap_catalog();
    let reopened =
        worth_store_test_support::reopened_recovery_artifact_fixture("layout-owner-restore");
    let observations =
        worth_store_operations::certification_test_authority::execute_restore_owner_scenarios(
            &catalog, &reopened,
        );
    for observed in observations.materialization() {
        ledger.record_restored_artifact_materialization(*observed);
    }
    for observed in observations.integration() {
        ledger.record_restored_layout_materialization(*observed);
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

fn lsm_family_cases() -> [worth_store_layout_indexes::AdmittedPhysicalArtifactFamily; 3] {
    let admitted = wal_security(SecurityScopeFixtureAuthority::Current);
    let wrong_security = tenant_page_security();
    let wrong_store = wal_security(SecurityScopeFixtureAuthority::Foreign);
    [
        admit_family(DurableArtifactFamilyId::PublicationWalIntent, &admitted),
        admit_family(
            DurableArtifactFamilyId::PublicationWalIntent,
            &wrong_security,
        ),
        admit_family(DurableArtifactFamilyId::PublicationWalIntent, &wrong_store),
    ]
}

fn wal_security(
    authority: SecurityScopeFixtureAuthority,
) -> worth_store_security::StoreAdmittedSecurityScope {
    security_scope(
        authority,
        StoreKeyScope::WalCheckpointEnvelope,
        StoreTenantScope::StoreInternal,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedWalRecord,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    )
}
