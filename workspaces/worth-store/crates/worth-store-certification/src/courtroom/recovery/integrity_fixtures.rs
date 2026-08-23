use worth_store_physical_integrity::{AdmittedRecoveryIntegrityInput, IntegrityHandoffPayload};

use crate::courtroom::harness::test_support::integrity_handoff_test_support::admit_recovery_handoff_payload;
pub(super) use crate::courtroom::harness::test_support::integrity_handoff_test_support::intact_integrity_model_input;
use crate::courtroom::harness::test_support::integrity_handoff_test_support::recovery_blocking_quarantine_binding;
use crate::courtroom::harness::test_support::recovery_blocking_damage_test_support::recovery_blocking_wal_damage_map;

pub(super) fn damaged_integrity_model_input() -> AdmittedRecoveryIntegrityInput {
    let intact = intact_integrity_model_input("blocked-entry");
    let (quarantine_record, quarantine_receipt, quarantine_damage) =
        recovery_blocking_quarantine_binding();
    let damage_map = recovery_blocking_wal_damage_map()
        .with_recovery_blocking_quarantine(
            &quarantine_record,
            quarantine_receipt,
            &quarantine_damage,
        )
        .unwrap();

    let payload = IntegrityHandoffPayload::declare()
        .root_manifest(intact.payload().root_manifest().clone())
        .segment_manifest(intact.payload().segment_manifest().clone())
        .page_frame(intact.payload().page_frames()[0].clone())
        .wal_frame(intact.payload().wal_frames()[0].clone())
        .checkpoint_record(intact.payload().checkpoint_records()[0].clone())
        .damage_map(damage_map)
        .inspection_envelope(intact.payload().inspection_envelope().clone())
        .seal()
        .unwrap();
    admit_recovery_handoff_payload(payload)
}
