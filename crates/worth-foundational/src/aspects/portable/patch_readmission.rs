use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use worth_proof::TransitionOutcome;

use super::{
    contract_for_readmission, PortableAspectContractLookup, PortableAspectFieldSet,
    PortableAspectPatchOperation, PortableAspectReadmissionDenial, PortableRecordAspectPatch,
};
use crate::aspects::{
    validate_aspect_value, AbsenceLaw, AspectContract, AspectMask, AuthoritativeRecordAspectPatch,
    CanonicalFieldPath, MutationMask,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PortablePatchReadmissionPurpose {
    RecordCreation,
    RecordMutation,
    RecordDeletion,
}

pub fn readmit_portable_record_aspect_patch(
    candidate: PortableRecordAspectPatch,
    purpose: PortablePatchReadmissionPurpose,
    contracts: &impl PortableAspectContractLookup,
) -> TransitionOutcome<AuthoritativeRecordAspectPatch, PortableAspectReadmissionDenial> {
    match readmit_patch(candidate, purpose, contracts) {
        Ok(patch) => TransitionOutcome::success(patch),
        Err(denial) => TransitionOutcome::denied(denial),
    }
}

fn readmit_patch(
    candidate: PortableRecordAspectPatch,
    purpose: PortablePatchReadmissionPurpose,
    contracts: &impl PortableAspectContractLookup,
) -> Result<AuthoritativeRecordAspectPatch, PortableAspectReadmissionDenial> {
    let mut admitted = AuthoritativeRecordAspectPatch::empty();
    let mut touched = BTreeSet::new();
    for operation in candidate.into_operations() {
        let key = operation.basis().key().clone();
        if !touched.insert(key.clone()) {
            return Err(PortableAspectReadmissionDenial::DuplicateAspectOperation(
                key,
            ));
        }
        let contract = contract_for_readmission(operation.basis(), contracts)?;
        let next = admit_operation(operation, &contract, purpose)?;
        admitted = transition_result(AuthoritativeRecordAspectPatch::combine(admitted, next))?;
    }
    Ok(admitted)
}

fn admit_operation(
    operation: PortableAspectPatchOperation,
    contract: &AspectContract,
    purpose: PortablePatchReadmissionPurpose,
) -> Result<AuthoritativeRecordAspectPatch, PortableAspectReadmissionDenial> {
    match operation {
        PortableAspectPatchOperation::SetWhole { value, .. } => {
            let validated = transition_value(validate_aspect_value(contract, value), contract)?;
            transition_result(AuthoritativeRecordAspectPatch::whole_aspect(
                [validated],
                [],
            ))
        }
        PortableAspectPatchOperation::ClearWhole { .. } => {
            admit_whole_clear(contract, purpose)?;
            transition_result(AuthoritativeRecordAspectPatch::whole_aspect(
                [],
                [contract.clone()],
            ))
        }
        PortableAspectPatchOperation::PatchFields {
            selected_fields,
            field_sets,
            field_clears,
            ..
        } => {
            if purpose == PortablePatchReadmissionPurpose::RecordCreation {
                return Err(
                    PortableAspectReadmissionDenial::FieldPatchDeniedForCreation(
                        contract.key().clone(),
                    ),
                );
            }
            let mask = AspectMask::<MutationMask>::new(
                selected_fields.into_iter().map(CanonicalFieldPath::single),
            );
            transition_result(AuthoritativeRecordAspectPatch::field_level(
                contract,
                &mask,
                field_sets
                    .into_iter()
                    .map(PortableAspectFieldSet::into_parts),
                field_clears,
            ))
        }
    }
}

fn admit_whole_clear(
    contract: &AspectContract,
    purpose: PortablePatchReadmissionPurpose,
) -> Result<(), PortableAspectReadmissionDenial> {
    let admitted = match purpose {
        PortablePatchReadmissionPurpose::RecordCreation => false,
        PortablePatchReadmissionPurpose::RecordMutation => {
            contract.absence() != AbsenceLaw::Required
        }
        PortablePatchReadmissionPurpose::RecordDeletion => true,
    };
    if admitted {
        Ok(())
    } else {
        Err(PortableAspectReadmissionDenial::WholeClearDenied {
            key: contract.key().clone(),
            purpose,
            absence: contract.absence(),
        })
    }
}

fn transition_value(
    outcome: worth_proof::TransitionOutcome<
        crate::aspects::ContractValidatedAspectArtifact,
        crate::aspects::ContractValidationDenial,
    >,
    contract: &AspectContract,
) -> Result<crate::aspects::ContractValidatedAspectArtifact, PortableAspectReadmissionDenial> {
    match outcome {
        TransitionOutcome::Success(value) => Ok(value),
        TransitionOutcome::Denied(denial) => {
            Err(PortableAspectReadmissionDenial::ValueValidation {
                key: contract.key().clone(),
                denial,
            })
        }
    }
}

fn transition_result<T>(
    outcome: TransitionOutcome<T, crate::aspects::AuthoritativePatchConstructionDenial>,
) -> Result<T, PortableAspectReadmissionDenial> {
    match outcome {
        TransitionOutcome::Success(value) => Ok(value),
        TransitionOutcome::Denied(denial) => {
            Err(PortableAspectReadmissionDenial::PatchConstruction(denial))
        }
    }
}
