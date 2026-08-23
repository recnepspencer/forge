use worth_store_physical_integrity::{AdmittedRecoveryIntegrityInput, IntegrityHandoffAdmission};

use super::s4_recovery_handoff_fixture::intact_payload;
use super::s4_recovery_readiness_fixture::physical_integrity_model_payload;

pub fn with_admitted_recovery_integrity_input<R>(
    label: &str,
    run: impl FnOnce(AdmittedRecoveryIntegrityInput) -> R,
) -> R {
    let input = IntegrityHandoffAdmission::admit_model_payload(
        physical_integrity_model_payload(),
        intact_payload(label),
    )
    .expect("test support recovery model admits the integrity input");
    run(input)
}
