use worth_proof::TransitionOutcome;

use super::{
    contract_for_readmission, PortableAspectContractLookup, PortableAspectReadmissionDenial,
    PortableRecordAspectState,
};
use crate::aspects::{
    admit_authoritative_record_aspect_state, validate_aspect_value,
    AuthoritativeRecordAspectStateArtifact,
};

pub fn readmit_portable_record_aspect_state(
    candidate: PortableRecordAspectState,
    contracts: &impl PortableAspectContractLookup,
) -> TransitionOutcome<AuthoritativeRecordAspectStateArtifact, PortableAspectReadmissionDenial> {
    let mut validated_entries = Vec::with_capacity(candidate.entries().len());
    for entry in candidate.into_entries() {
        let (basis, value) = entry.into_parts();
        let contract = match contract_for_readmission(&basis, contracts) {
            Ok(contract) => contract,
            Err(denial) => return TransitionOutcome::denied(denial),
        };
        match validate_aspect_value(&contract, value) {
            TransitionOutcome::Success(validated) => validated_entries.push(validated),
            TransitionOutcome::Denied(denial) => {
                return TransitionOutcome::denied(
                    PortableAspectReadmissionDenial::ValueValidation {
                        key: contract.key().clone(),
                        denial,
                    },
                );
            }
        }
    }

    match admit_authoritative_record_aspect_state(validated_entries) {
        TransitionOutcome::Success(state) => TransitionOutcome::success(state),
        TransitionOutcome::Denied(denial) => {
            TransitionOutcome::denied(PortableAspectReadmissionDenial::StateAdmission(denial))
        }
    }
}
