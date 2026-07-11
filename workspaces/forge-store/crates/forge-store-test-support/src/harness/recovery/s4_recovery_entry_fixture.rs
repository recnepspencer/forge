use forge_store_recovery_physics::{
    IntegrityHandoffAdmission, RecoveryEntryAdmission, RecoveryEntryAdmissionDecision,
};

use super::s4_recovery_handoff_fixture::intact_payload;
use super::s4_recovery_readiness_fixture::{
    physical_authority, recovery_memory_envelope, s3_readiness,
};

pub fn admitted_s4_recovery_entry(label: &str) -> RecoveryEntryAdmission {
    admit_recovery_entry_from_s4_handoff(label, false)
}

pub fn admitted_s4_partial_publication_recovery_entry(label: &str) -> RecoveryEntryAdmission {
    admit_recovery_entry_from_s4_handoff(label, true)
}

fn admit_recovery_entry_from_s4_handoff(
    label: &str,
    include_partial_publication_replay_read: bool,
) -> RecoveryEntryAdmission {
    let readiness = IntegrityHandoffAdmission::admit(
        s3_readiness().payload(),
        intact_payload(label, include_partial_publication_replay_read),
    )
    .expect("test support S4 handoff admits through public S4 admission");
    let decision =
        RecoveryEntryAdmission::admit(readiness, recovery_memory_envelope(), physical_authority());
    let RecoveryEntryAdmissionDecision::Admitted(admission) = decision else {
        panic!("intact test support S4 handoff admits recovery entry");
    };
    admission
}
