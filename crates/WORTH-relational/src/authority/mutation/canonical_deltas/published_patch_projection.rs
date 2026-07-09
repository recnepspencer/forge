use crate::publication::patch::data::{
    PublishedAuthoritativeFieldSet, PublishedAuthoritativePatch,
    PublishedAuthoritativePatchOperation, PublishedAuthoritativePatchValue,
};
use worth_foundational::facade::{
    AuthoritativeRecordAspectPatch, ContractValidatedAspectValue, ContractValidatedAspectValueView,
    FieldLevelAspectPatch,
};

use super::data::AuthoritativePatchDeltaOperation;

pub(super) fn published_patch_from_authoritative_delta_operation(
    operation: &AuthoritativePatchDeltaOperation,
) -> PublishedAuthoritativePatch {
    PublishedAuthoritativePatch::new(vec![
        published_operation_from_authoritative_delta_operation(operation),
    ])
}

pub(super) fn published_patch_from_foundational_patch(
    patch: &AuthoritativeRecordAspectPatch,
) -> PublishedAuthoritativePatch {
    let operations = patch
        .whole_aspect_sets()
        .map(
            |(aspect_key, value)| PublishedAuthoritativePatchOperation::WholeAspectSet {
                aspect_key: aspect_key.clone(),
                value: published_patch_value(value),
            },
        )
        .chain(patch.whole_aspect_clears().map(|aspect_key| {
            PublishedAuthoritativePatchOperation::WholeAspectClear {
                aspect_key: aspect_key.clone(),
            }
        }))
        .chain(
            patch
                .field_patches()
                .map(|(_, field_patch)| published_field_patch_operation(field_patch)),
        )
        .collect();
    PublishedAuthoritativePatch::new(operations)
}

fn published_operation_from_authoritative_delta_operation(
    operation: &AuthoritativePatchDeltaOperation,
) -> PublishedAuthoritativePatchOperation {
    match operation {
        AuthoritativePatchDeltaOperation::WholeAspectSet { value } => {
            PublishedAuthoritativePatchOperation::WholeAspectSet {
                aspect_key: value.key().clone(),
                value: published_patch_value(value),
            }
        }
        AuthoritativePatchDeltaOperation::WholeAspectClear { aspect_key } => {
            PublishedAuthoritativePatchOperation::WholeAspectClear {
                aspect_key: aspect_key.clone(),
            }
        }
        AuthoritativePatchDeltaOperation::FieldLevelPatch { patch } => {
            published_field_patch_operation(patch)
        }
    }
}

fn published_field_patch_operation(
    patch: &FieldLevelAspectPatch,
) -> PublishedAuthoritativePatchOperation {
    PublishedAuthoritativePatchOperation::FieldLevelPatch {
        aspect_key: patch.key().clone(),
        field_sets: patch
            .field_sets()
            .map(|(field, value)| PublishedAuthoritativeFieldSet {
                field: field.clone(),
                value: value.clone(),
            })
            .collect(),
        field_clears: patch.field_clears().cloned().collect(),
    }
}

fn published_patch_value(value: &ContractValidatedAspectValue) -> PublishedAuthoritativePatchValue {
    match value.view() {
        ContractValidatedAspectValueView::Scalar(value) => {
            PublishedAuthoritativePatchValue::Scalar(value.clone())
        }
        ContractValidatedAspectValueView::Struct(value) => {
            PublishedAuthoritativePatchValue::Struct(value.clone())
        }
    }
}
