use forge_proof::{Artifact, TransitionOutcome};

use crate::aspects::{
    AuthoritativeRecordAspectPatch, ContractValidatedAspectValue, ContractValidatedAspectValueView,
};

use super::{
    CanonicalDigestPreparationEntry, DigestPreparationReadyAspectPatch,
    DigestPreparationReadyAspectPatchArtifact,
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
