use std::collections::BTreeSet;

use worth_foundational::facade::{AbsenceLaw, AspectShape};

use super::{
    WorthQueryArtifactAccessPathContract, WorthQueryArtifactFieldSlicePosture,
    WorthQueryArtifactRowBatchPosture, WorthQueryArtifactScalarFallbackPosture,
};

pub(crate) fn validate_artifact_access_path(
    contract: &WorthQueryArtifactAccessPathContract,
) -> bool {
    let WorthQueryArtifactAccessPathContract::Native(contract) = contract else {
        return true;
    };
    let layout = contract.layout();
    if !portable_identity(layout.identity().as_str())
        || layout.version().get() == 0
        || !valid_alignment(layout.alignment().bytes())
        || layout.fields().is_empty()
    {
        return false;
    }
    let mut fields = BTreeSet::new();
    for field in layout.fields() {
        if !fields.insert(field.aspect().key().clone()) || !valid_field_slice(field) {
            return false;
        }
    }
    if contract
        .chunks()
        .is_some_and(|chunks| chunks.max_rows() == 0)
    {
        return false;
    }
    if !valid_scalar_fallback(contract.scalar_fallback()) {
        return false;
    }
    let mut projections = BTreeSet::new();
    for projection in contract.bulk_projections() {
        if !portable_identity(projection.identity())
            || !projections.insert(projection.identity())
            || projection.source_fields().is_empty()
            || projection
                .source_fields()
                .iter()
                .any(|field| !fields.contains(field))
            || projection.destination_fields().is_empty()
            || projection
                .destination_fields()
                .iter()
                .any(|field| !matches!(field.shape(), AspectShape::Scalar(_)))
            || !valid_alignment(projection.destination_alignment().bytes())
        {
            return false;
        }
    }
    contract.row_batch() == WorthQueryArtifactRowBatchPosture::Borrowed
        || contract.chunks().is_some()
        || !contract.bulk_projections().is_empty()
        || matches!(
            contract.scalar_fallback(),
            WorthQueryArtifactScalarFallbackPosture::Admitted { .. }
        )
        || layout
            .fields()
            .iter()
            .any(|field| field.field_slice() == WorthQueryArtifactFieldSlicePosture::Borrowed)
}

fn valid_field_slice(field: &super::WorthQueryArtifactNativeFieldContract) -> bool {
    if field.field_slice() != WorthQueryArtifactFieldSlicePosture::Borrowed {
        return true;
    }
    field.aspect().absence() == AbsenceLaw::Required
        && matches!(
            field.aspect().shape(),
            AspectShape::Scalar(_) | AspectShape::Struct(_)
        )
}

fn valid_scalar_fallback(posture: WorthQueryArtifactScalarFallbackPosture) -> bool {
    match posture {
        WorthQueryArtifactScalarFallbackPosture::Denied => true,
        WorthQueryArtifactScalarFallbackPosture::Admitted {
            max_calls_per_admission,
            max_call_amplification,
        } => max_calls_per_admission > 0 && max_call_amplification > 0,
    }
}

fn valid_alignment(alignment: usize) -> bool {
    alignment != 0 && alignment.is_power_of_two()
}

fn portable_identity(value: &str) -> bool {
    !value.trim().is_empty() && value.trim() == value && !value.chars().any(char::is_whitespace)
}
