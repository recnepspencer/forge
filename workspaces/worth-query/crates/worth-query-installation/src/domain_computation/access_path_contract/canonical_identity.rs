use crate::canonical_hash_encoding::CanonicalHashSink;
use worth_foundational::facade::{
    canonical_basis_sequence_material, prepare_aspect_contract_for_canonical_basis,
    CanonicalizationRuleVersion,
};

use crate::canonical_hash_encoding::hash_text_field;

use super::{
    WorthQueryArtifactAccessPathContract, WorthQueryArtifactFieldSlicePosture,
    WorthQueryArtifactRowBatchPosture, WorthQueryArtifactScalarFallbackPosture,
};

pub(crate) fn hash_artifact_access_path(
    hash: &mut impl CanonicalHashSink,
    access_path: &WorthQueryArtifactAccessPathContract,
) {
    let WorthQueryArtifactAccessPathContract::Native(contract) = access_path else {
        hash_text_field(hash, "artifact-access-path", "denied");
        return;
    };
    hash_text_field(hash, "artifact-access-path", "native");
    let layout = contract.layout();
    hash_text_field(hash, "native-layout", layout.identity().as_str());
    hash_text_field(
        hash,
        "native-layout-version",
        &layout.version().get().to_string(),
    );
    hash_text_field(
        hash,
        "native-layout-alignment",
        &layout.alignment().bytes().to_string(),
    );
    hash_text_field(
        hash,
        "native-row-batch",
        row_batch_name(contract.row_batch()),
    );
    for field in layout.fields() {
        hash_text_field(
            hash,
            "native-field-contract",
            &canonical_aspect_contract(field.aspect()),
        );
        hash_text_field(
            hash,
            "native-field-slice",
            field_slice_name(field.field_slice()),
        );
    }
    if let Some(chunks) = contract.chunks() {
        hash_text_field(
            hash,
            "native-chunk-max-rows",
            &chunks.max_rows().to_string(),
        );
    } else {
        hash_text_field(hash, "native-chunks", "denied");
    }
    for projection in contract.bulk_projections() {
        hash_text_field(hash, "native-projection", projection.identity());
        for source in projection.source_fields() {
            hash_text_field(hash, "native-projection-source", source.as_str());
        }
        hash_text_field(
            hash,
            "native-projection-alignment",
            &projection.destination_alignment().bytes().to_string(),
        );
        for destination in projection.destination_fields() {
            hash_text_field(
                hash,
                "native-projection-destination",
                &canonical_aspect_contract(destination),
            );
        }
    }
    match contract.scalar_fallback() {
        WorthQueryArtifactScalarFallbackPosture::Denied => {
            hash_text_field(hash, "native-scalar-fallback", "denied");
        }
        WorthQueryArtifactScalarFallbackPosture::Admitted {
            max_calls_per_admission,
            max_call_amplification,
        } => {
            hash_text_field(hash, "native-scalar-fallback", "admitted");
            hash_text_field(
                hash,
                "native-scalar-max-calls",
                &max_calls_per_admission.to_string(),
            );
            hash_text_field(
                hash,
                "native-scalar-amplification",
                &max_call_amplification.to_string(),
            );
        }
    }
}

fn canonical_aspect_contract(contract: &worth_foundational::facade::AspectContract) -> String {
    let version = CanonicalizationRuleVersion::new("worth-query-artifact-native-layout-v1")
        .expect("fixed artifact native layout canonicalization version is valid");
    let basis = prepare_aspect_contract_for_canonical_basis(version, contract.clone())
        .into_result()
        .expect("a constructed Foundational aspect contract has canonical material");
    canonical_basis_sequence_material(basis.payload())
}

const fn row_batch_name(posture: WorthQueryArtifactRowBatchPosture) -> &'static str {
    match posture {
        WorthQueryArtifactRowBatchPosture::Denied => "denied",
        WorthQueryArtifactRowBatchPosture::Borrowed => "borrowed",
    }
}

const fn field_slice_name(posture: WorthQueryArtifactFieldSlicePosture) -> &'static str {
    match posture {
        WorthQueryArtifactFieldSlicePosture::Denied => "denied",
        WorthQueryArtifactFieldSlicePosture::Borrowed => "borrowed",
        WorthQueryArtifactFieldSlicePosture::ProviderNativeProjectionOnly => {
            "provider-native-projection-only"
        }
    }
}
