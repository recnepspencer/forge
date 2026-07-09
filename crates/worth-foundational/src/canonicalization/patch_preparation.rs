use worth_proof::{Artifact, TransitionOutcome};

use crate::aspects::{
    AuthoritativeRecordAspectPatch, ContractValidatedAspectValue, ContractValidatedAspectValueView,
};

use super::value_lowering::canonical_basis_value_for_aspect_value;
use super::{
    prepare_canonical_basis_sequence, CanonicalBasisDomain, CanonicalBasisEntry,
    CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisReadyArtifact, CanonicalBasisValue,
    CanonicalDigestPreparationEntry, CanonicalizationRuleVersion,
    DigestPreparationReadyAspectPatch, DigestPreparationReadyAspectPatchArtifact,
};

pub fn prepare_aspect_patch_for_digest(
    patch: &AuthoritativeRecordAspectPatch,
) -> TransitionOutcome<DigestPreparationReadyAspectPatchArtifact> {
    let mut basis = Vec::new();

    basis.extend(
        patch
            .whole_aspect_clears()
            .cloned()
            .map(|key| CanonicalDigestPreparationEntry::PatchWholeAspectClear { key }),
    );

    for (_key, value) in patch.whole_aspect_sets() {
        basis.extend(digest_basis_for_whole_aspect_set(value));
    }

    for (key, field_patch) in patch.field_patches() {
        basis.extend(field_patch.field_clears().cloned().map(|field| {
            CanonicalDigestPreparationEntry::PatchFieldClear {
                key: key.clone(),
                field,
            }
        }));
        basis.extend(field_patch.field_sets().map(|(field, value)| {
            CanonicalDigestPreparationEntry::PatchFieldSet {
                key: key.clone(),
                field: field.clone(),
                value: value.clone(),
            }
        }));
    }

    basis.sort();

    TransitionOutcome::success(Artifact::new(DigestPreparationReadyAspectPatch::new(
        patch.clone(),
        basis,
    )))
}

pub fn aspect_patch_digest_preparation_basis(
    ready: &DigestPreparationReadyAspectPatchArtifact,
) -> &[CanonicalDigestPreparationEntry] {
    ready.payload().basis()
}

pub fn prepare_aspect_patch_for_canonical_basis(
    version: CanonicalizationRuleVersion,
    patch: &AuthoritativeRecordAspectPatch,
) -> TransitionOutcome<CanonicalBasisReadyArtifact, super::CanonicalBasisConstructionDenial> {
    let mut basis = Vec::new();

    basis.extend(patch.whole_aspect_clears().map(|key| {
        patch_entry(
            key.as_str(),
            "whole.clear",
            CanonicalBasisEntryKind::PatchOperation,
            CanonicalBasisValue::ExactText("clear".into()),
        )
    }));

    for (_key, value) in patch.whole_aspect_sets() {
        basis.extend(canonical_basis_for_whole_aspect_set(value));
    }

    for (key, field_patch) in patch.field_patches() {
        basis.extend(field_patch.field_clears().map(|field| {
            patch_entry(
                key.as_str(),
                format!("field.{}.clear", field.as_str()),
                CanonicalBasisEntryKind::PatchOperation,
                CanonicalBasisValue::ExactText("clear".into()),
            )
        }));
        basis.extend(field_patch.field_sets().map(|(field, value)| {
            patch_entry(
                key.as_str(),
                format!("field.{}.set", field.as_str()),
                CanonicalBasisEntryKind::PatchOperation,
                canonical_basis_value_for_aspect_value(value),
            )
        }));
    }

    if basis.is_empty() {
        basis.push(patch_entry(
            "patch",
            "noop",
            CanonicalBasisEntryKind::PatchOperation,
            CanonicalBasisValue::Null,
        ));
    }

    prepare_canonical_basis_sequence(version, CanonicalBasisDomain::AuthoritativePatch, basis)
}

fn digest_basis_for_whole_aspect_set(
    value: &ContractValidatedAspectValue,
) -> Vec<CanonicalDigestPreparationEntry> {
    let mut entries = vec![CanonicalDigestPreparationEntry::PatchWholeAspectSet {
        key: value.key().clone(),
        revision: value.contract_revision(),
    }];

    match value.view() {
        ContractValidatedAspectValueView::Scalar(scalar) => {
            entries.push(
                CanonicalDigestPreparationEntry::PatchWholeAspectScalarValue {
                    key: value.key().clone(),
                    value: scalar.clone(),
                },
            );
        }
        ContractValidatedAspectValueView::Struct(struct_value) => {
            entries.extend(struct_value.fields().map(|(field, field_value)| {
                CanonicalDigestPreparationEntry::PatchWholeAspectStructFieldValue {
                    key: value.key().clone(),
                    field: field.clone(),
                    value: field_value.clone(),
                }
            }));
        }
    }

    entries
}

fn canonical_basis_for_whole_aspect_set(
    value: &ContractValidatedAspectValue,
) -> Vec<CanonicalBasisEntry> {
    let mut entries = vec![patch_entry(
        value.key().as_str(),
        "whole.set.revision",
        CanonicalBasisEntryKind::PatchOperation,
        CanonicalBasisValue::UnsignedInteger {
            width: super::CanonicalIntegerWidth::Bits64,
            value: u128::from(value.contract_revision().0),
        },
    )];

    match value.view() {
        ContractValidatedAspectValueView::Scalar(scalar) => {
            entries.push(patch_entry(
                value.key().as_str(),
                "whole.set.value",
                CanonicalBasisEntryKind::Value,
                canonical_basis_value_for_aspect_value(scalar),
            ));
        }
        ContractValidatedAspectValueView::Struct(struct_value) => {
            entries.extend(struct_value.fields().map(|(field, field_value)| {
                patch_entry(
                    value.key().as_str(),
                    format!("whole.set.field.{}", field.as_str()),
                    CanonicalBasisEntryKind::Field,
                    canonical_basis_value_for_aspect_value(field_value),
                )
            }));
        }
    }

    entries
}

fn patch_entry(
    aspect_key: &str,
    locus: impl Into<String>,
    kind: CanonicalBasisEntryKind,
    value: CanonicalBasisValue,
) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::AuthoritativePatch,
        CanonicalBasisLocus::Named(format!("{aspect_key}.{}", locus.into()).into()),
        kind,
        value,
    )
}
