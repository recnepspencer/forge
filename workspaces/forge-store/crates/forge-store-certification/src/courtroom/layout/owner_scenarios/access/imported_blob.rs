use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_layout_indexes::{access_planning, ObserveOwnerCase};
use forge_store_test_support::{admitted_layout_bootstrap_catalog, SecurityScopeFixtureAuthority};

use super::super::fixture_admission::{admit_family, admit_key_domain};
use super::super::LayoutOwnerObservationLedger;
use super::fixture_values::{blob_security, page_security};

pub(super) fn execute(ledger: &mut LayoutOwnerObservationLedger) {
    let catalog = admitted_layout_bootstrap_catalog();
    let current_blob_security = blob_security(SecurityScopeFixtureAuthority::Current);
    let foreign_blob_security = blob_security(SecurityScopeFixtureAuthority::Foreign);
    let page_security = page_security(SecurityScopeFixtureAuthority::Current);
    let blob_family = admit_family(
        DurableArtifactFamilyId::BlobManifest,
        &current_blob_security,
    );
    let blob_domain = admit_key_domain(blob_family, &current_blob_security);
    let foreign_blob_family = admit_family(
        DurableArtifactFamilyId::BlobManifest,
        &foreign_blob_security,
    );
    let foreign_blob_domain = admit_key_domain(foreign_blob_family, &foreign_blob_security);
    let page_family = admit_family(DurableArtifactFamilyId::PhysicalPage, &page_security);
    let page_domain = admit_key_domain(page_family, &page_security);
    let witness = forge_store_blob_chunks::certification_test_authority::
        execute_readmitted_blob_import_for_store(
            "layout.owner-matrix.imported-read",
            "store.physical.default_instance",
        );

    for (family, domain) in [
        (blob_family, blob_domain),
        (page_family, page_domain),
        (blob_family, page_domain),
        (blob_family, foreign_blob_domain),
    ] {
        let outcome =
            access_planning().admit_imported_blob_read_request(family, domain, &catalog, &witness);
        ledger.record_imported_blob_read_admission(outcome.owner_case_observation());
    }
}
