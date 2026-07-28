use worth_store_recovery_physics::{
    IntegrityHandoffAdmission, RecoveryEntryAdmission, RecoveryEntryAdmissionDecision,
};

use super::s4_recovery_handoff_fixture::intact_payload;
use super::s4_recovery_readiness_fixture::{physical_authority, physical_integrity_model_payload};

pub fn with_admitted_recovery_entry<R>(
    label: &str,
    run: impl FnOnce(RecoveryEntryAdmission<'_>) -> R,
) -> R {
    with_admitted_recovery_entry_from_handoff(label, false, run)
}

pub fn with_admitted_recovery_partial_publication_entry<R>(
    label: &str,
    run: impl FnOnce(RecoveryEntryAdmission<'_>) -> R,
) -> R {
    with_admitted_recovery_entry_from_handoff(label, true, run)
}

fn with_admitted_recovery_entry_from_handoff<R>(
    label: &str,
    include_partial_publication_replay_read: bool,
    run: impl FnOnce(RecoveryEntryAdmission<'_>) -> R,
) -> R {
    let integrity_input = IntegrityHandoffAdmission::admit_model_payload(
        physical_integrity_model_payload(),
        intact_payload(label, include_partial_publication_replay_read),
    )
    .expect("test support recovery model admits the S4 algorithm input");
    super::memory_budget::with_recovery_memory_allocation(|memory_allocation| {
        let decision =
            RecoveryEntryAdmission::admit(integrity_input, memory_allocation, physical_authority());
        let RecoveryEntryAdmissionDecision::Admitted(admission) = decision else {
            panic!("intact test support recovery handoff admits recovery entry");
        };
        run(*admission)
    })
}
