use worth_proof::{Artifact, TransitionOutcome};

use crate::aspects::{
    AuthoritativeRecordAspectStateArtifact, ContractValidatedAspectValue,
    ContractValidatedAspectValueView,
};

use super::value_lowering::canonical_basis_value_for_aspect_value;
use super::{
    prepare_canonical_basis_sequence, CanonicalBasisDomain, CanonicalBasisEntry,
    CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisReadyArtifact, CanonicalBasisValue,
    CanonicalDigestPreparationEntry, CanonicalizationRuleVersion,
    DigestPreparationReadyAspectState, DigestPreparationReadyAspectStateArtifact,
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

pub fn prepare_aspect_state_for_canonical_basis(
    version: CanonicalizationRuleVersion,
    state: AuthoritativeRecordAspectStateArtifact,
) -> TransitionOutcome<CanonicalBasisReadyArtifact, super::CanonicalBasisConstructionDenial> {
    let (state, _proofs, _basis) = state.into_parts().into_parts();
    let basis: Vec<_> = state
        .aspects()
        .entries()
        .flat_map(|(_key, value)| canonical_basis_for_validated_value(value))
        .collect();

    prepare_canonical_basis_sequence(version, CanonicalBasisDomain::AuthoritativeState, basis)
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

fn canonical_basis_for_validated_value(
    value: &ContractValidatedAspectValue,
) -> Vec<CanonicalBasisEntry> {
    let mut entries = vec![state_entry(
        value.key().as_str(),
        "revision",
        CanonicalBasisEntryKind::StateAspect,
        CanonicalBasisValue::UnsignedInteger {
            width: super::CanonicalIntegerWidth::Bits64,
            value: u128::from(value.contract_revision().0),
        },
    )];

    match value.view() {
        ContractValidatedAspectValueView::Scalar(scalar) => {
            entries.push(state_entry(
                value.key().as_str(),
                "value",
                CanonicalBasisEntryKind::Value,
                canonical_basis_value_for_aspect_value(scalar),
            ));
        }
        ContractValidatedAspectValueView::Struct(struct_value) => {
            entries.extend(struct_value.fields().map(|(field, field_value)| {
                state_entry(
                    value.key().as_str(),
                    format!("field.{}", field.as_str()),
                    CanonicalBasisEntryKind::Field,
                    canonical_basis_value_for_aspect_value(field_value),
                )
            }));
        }
    }

    entries
}

fn state_entry(
    aspect_key: &str,
    locus: impl Into<String>,
    kind: CanonicalBasisEntryKind,
    value: CanonicalBasisValue,
) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::AuthoritativeState,
        CanonicalBasisLocus::Named(format!("{aspect_key}.{}", locus.into()).into()),
        kind,
        value,
    )
}
