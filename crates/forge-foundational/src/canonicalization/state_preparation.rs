use forge_proof::{Artifact, TransitionOutcome};

use crate::aspects::{
    AuthoritativeRecordAspectStateArtifact, ContractValidatedAspectValue,
    ContractValidatedAspectValueView,
};

use super::{
    CanonicalDigestPreparationEntry, DigestPreparationReadyAspectState,
    DigestPreparationReadyAspectStateArtifact,
};

pub fn prepare_aspect_state_for_digest(
    state: AuthoritativeRecordAspectStateArtifact,
) -> TransitionOutcome<DigestPreparationReadyAspectStateArtifact> {
    let (state, _proofs, _basis) = state.into_parts().into_parts();
    let basis = state
        .aspects()
        .entries()
        .flat_map(|(_key, value)| digest_basis_for_validated_value(value))
        .collect();

    TransitionOutcome::success(Artifact::new(DigestPreparationReadyAspectState::new(
        state, basis,
    )))
}

pub fn aspect_state_digest_preparation_basis(
    ready: &DigestPreparationReadyAspectStateArtifact,
) -> &[CanonicalDigestPreparationEntry] {
    ready.payload().basis()
}

pub(crate) fn digest_basis_for_validated_value(
    value: &ContractValidatedAspectValue,
) -> Vec<CanonicalDigestPreparationEntry> {
    let mut entries = vec![CanonicalDigestPreparationEntry::StateAspect {
        key: value.key().clone(),
        revision: value.contract_revision(),
    }];

    match value.view() {
        ContractValidatedAspectValueView::Scalar(scalar) => {
            entries.push(CanonicalDigestPreparationEntry::StateScalarValue {
                key: value.key().clone(),
                value: scalar.clone(),
            });
        }
        ContractValidatedAspectValueView::Struct(struct_value) => {
            entries.extend(struct_value.fields().map(|(field, field_value)| {
                CanonicalDigestPreparationEntry::StateStructFieldValue {
                    key: value.key().clone(),
                    field: field.clone(),
                    value: field_value.clone(),
                }
            }));
        }
    }

    entries
}
