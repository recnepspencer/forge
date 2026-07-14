use forge_store_recovery_physics::{
    IntegrityHandoffAdmission, RecoveryEntryAdmission, RecoveryEntryAdmissionDecision,
};

use super::s4_recovery_handoff_fixture::intact_payload;
use super::s4_recovery_readiness_fixture::{
    physical_authority, physical_integrity_readiness, recovery_memory_envelope,
};

pub fn admitted_recovery_entry(label: &str) -> RecoveryEntryAdmission {
    admit_recovery_entry_from_recovery_handoff(label, false)
}

pub fn admitted_recovery_partial_publication_recovery_entry(label: &str) -> RecoveryEntryAdmission {
    admit_recovery_entry_from_recovery_handoff(label, true)
}

fn admit_recovery_entry_from_recovery_handoff(
    label: &str,
    include_partial_publication_replay_read: bool,
) -> RecoveryEntryAdmission {
    let readiness = IntegrityHandoffAdmission::admit(
        physical_integrity_readiness().payload(),
        intact_payload(label, include_partial_publication_replay_read),
    )
    .expect("test support recovery handoff admits through public S4 admission");
    let decision =
        RecoveryEntryAdmission::admit(readiness, recovery_memory_envelope(), physical_authority());
    let RecoveryEntryAdmissionDecision::Admitted(admission) = decision else {
        panic!("intact test support recovery handoff admits recovery entry");
    };
    admission
}
