use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::runtime::{ForgeQueryReadReceipt, ForgeQueryWriteReceiptInspection};

use super::inventory::CausalEvidenceFamily;
use super::observation_identity::{
    CausalEvidenceReferenceDigest, CausalObservationBasisIdentity, CausalObservationQueryIdentity,
    CausalObservationReceiptIdentity,
};
use super::receipt_types::{CausalObservationBasisPosture, QueryObservationReceiptFamily};

pub(super) fn causal_observation_receipt_evidence_identity(
    family: QueryObservationReceiptFamily,
    source_receipt: &ForgeQueryEvidenceIdentity,
) -> CausalObservationReceiptIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::CausalObservationReceipt)
        .field_shape(ForgeQueryEvidenceTag::new("family"), family.as_str())
        .field_evidence_identity(ForgeQueryEvidenceTag::new("source_receipt"), source_receipt)
        .seal()
        .into()
}

pub(super) fn causal_observation_query_evidence_identity(
    family: &str,
    source_query: &ForgeQueryEvidenceIdentity,
) -> CausalObservationQueryIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::CausalObservationQuery)
        .field_shape(ForgeQueryEvidenceTag::new("family"), family)
        .field_evidence_identity(ForgeQueryEvidenceTag::new("source_query"), source_query)
        .seal()
        .into()
}

pub(super) fn causal_observation_basis_evidence_identity(
    posture: &CausalObservationBasisPosture,
    basis_identity: &ForgeQueryEvidenceIdentity,
) -> CausalObservationBasisIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::CausalObservationBasis)
        .field_shape(ForgeQueryEvidenceTag::new("posture"), posture.as_str())
        .field_evidence_identity(ForgeQueryEvidenceTag::new("source_basis"), basis_identity)
        .seal()
        .into()
}

pub(super) fn causal_evidence_reference_identity_digest(
    family: CausalEvidenceFamily,
    source_reference: &ForgeQueryEvidenceIdentity,
) -> CausalEvidenceReferenceDigest {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::CausalEvidenceReference)
        .field_shape(ForgeQueryEvidenceTag::new("family"), family.as_str())
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("source_reference"),
            source_reference,
        )
        .seal()
        .into()
}

pub(super) fn write_observation_query_identity(
    inspection: &ForgeQueryWriteReceiptInspection,
) -> CausalObservationQueryIdentity {
    let snapshot_identity = inspection.snapshot_identity().evidence_identity();
    let mut encoder =
        ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::CausalObservationQuery)
            .field_shape(ForgeQueryEvidenceTag::new("family"), "write_receipt")
            .field_shape(
                ForgeQueryEvidenceTag::new("mutation_family"),
                inspection.mutation_family(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("basis_lane"),
                inspection.basis_lane().as_str(),
            )
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("snapshot_identity"),
                &snapshot_identity,
            );
    if !inspection.mutation_metadata().is_empty() {
        encoder = encoder.field_value_sequence(
            ForgeQueryEvidenceTag::new("metadata_entries"),
            inspection
                .mutation_metadata()
                .entries()
                .flat_map(|(key, value)| {
                    [
                        key.as_str().to_string(),
                        value.native_digest_text().to_string(),
                    ]
                }),
        );
    }
    encoder.seal().into()
}

pub(super) fn read_observation_receipt_identity(
    receipt: &ForgeQueryReadReceipt,
) -> CausalObservationReceiptIdentity {
    let snapshot_identity = receipt.snapshot_evidence_identity();
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::CausalObservationReceipt)
        .field_shape(
            ForgeQueryEvidenceTag::new("family"),
            QueryObservationReceiptFamily::ReadReceipt.as_str(),
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("snapshot"), &snapshot_identity)
        .field_shape(
            ForgeQueryEvidenceTag::new("execution_engine"),
            receipt.execution_engine().as_str(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("result"),
            receipt.result_digest(),
        )
        .seal()
        .into()
}

pub(super) fn read_observation_query_identity(
    receipt: &ForgeQueryReadReceipt,
    snapshot_identity: &ForgeQueryEvidenceIdentity,
) -> CausalObservationQueryIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::CausalObservationQuery)
        .field_shape(ForgeQueryEvidenceTag::new("family"), "read_receipt")
        .field_evidence_identity(ForgeQueryEvidenceTag::new("snapshot"), snapshot_identity)
        .field_shape(
            ForgeQueryEvidenceTag::new("execution_engine"),
            receipt.execution_engine().as_str(),
        )
        .field_value(ForgeQueryEvidenceTag::new("query"), receipt.query_digest())
        .seal()
        .into()
}

pub(super) fn read_observation_result_reference_digest(
    receipt: &ForgeQueryReadReceipt,
    snapshot_identity: &ForgeQueryEvidenceIdentity,
) -> CausalEvidenceReferenceDigest {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::CausalEvidenceReference)
        .field_shape(
            ForgeQueryEvidenceTag::new("family"),
            CausalEvidenceFamily::QueryInspection.as_str(),
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("snapshot"), snapshot_identity)
        .field_shape(
            ForgeQueryEvidenceTag::new("execution_engine"),
            receipt.execution_engine().as_str(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("result"),
            receipt.result_digest(),
        )
        .seal()
        .into()
}
