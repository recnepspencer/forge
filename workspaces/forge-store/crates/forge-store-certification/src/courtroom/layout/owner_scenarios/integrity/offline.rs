use forge_store_layout_indexes::integrity::{layout_corruption, offline_readmission};
use forge_store_layout_indexes::ObserveOwnerCase;

use super::fixture_authority::{authority, import_witness};
use super::LayoutOwnerObservationLedger;

pub(super) fn record(ledger: &mut LayoutOwnerObservationLedger) {
    let (fixture, family) = authority("integrity-offline-matrix");
    let reopened =
        forge_store_test_support::reopened_recovery_artifact_fixture("integrity-offline-matrix");
    let requirement = || {
        layout_corruption()
            .require_offline_readmission(family, &reopened)
            .into_offline_readmission_requirement()
            .unwrap()
    };
    let admitted = forge_store_recovery_physics::layout_readmission()
        .admit_offline(family.family_id(), &reopened)
        .expect("reopened artifact must issue offline evidence");
    let other_reopened =
        forge_store_test_support::reopened_recovery_artifact_fixture("integrity-offline-other");
    let other = forge_store_recovery_physics::layout_readmission()
        .admit_offline(family.family_id(), &other_reopened)
        .expect("other reopened artifact remains valid lower evidence");
    let imported = import_witness(family, &fixture, "integrity-offline-import");

    for witness in [admitted, other, imported] {
        let outcome = offline_readmission().admit(requirement(), witness);
        ledger.record_offline_readmission(outcome.owner_case_observation());
    }
}
