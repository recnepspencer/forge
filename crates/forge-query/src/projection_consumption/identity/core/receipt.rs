use super::super::scope::{consumption_scope_encoder, seal};
use crate::ForgeQueryEvidenceTag;

use super::super::super::contracts::ProjectionContractSupportPosture;
use super::super::super::eligibility::ProjectionConsumptionWarningKind;
use super::super::super::receipt_transitions::ProjectionConsumptionDeferredNeighborFamily;
use super::super::super::source::ProjectionSourceFamily;

pub(crate) fn compose_receipt_integrity_digest(
    fact_set_digest: &str,
    counter_snapshot_digest: &str,
    source_identity: &str,
    materialized_fact_posture_digest: Option<&str>,
) -> String {
    let mut encoder = consumption_scope_encoder("projection_consumption_receipt_integrity_v1")
        .field_shape(ForgeQueryEvidenceTag::new("fact_set"), fact_set_digest)
        .field_shape(
            ForgeQueryEvidenceTag::new("counters"),
            counter_snapshot_digest,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("source_identity"),
            source_identity,
        );
    if let Some(posture_digest) = materialized_fact_posture_digest {
        encoder = encoder.field_shape(
            ForgeQueryEvidenceTag::new("materialized_fact_posture"),
            posture_digest,
        );
    }
    seal(encoder)
}

pub(crate) fn compose_receipt_digest(
    declaration_digest: &str,
    contract_digest: &str,
    fact_set_digest: &str,
    source_family: ProjectionSourceFamily,
    source_identity: &str,
    support_posture: &ProjectionContractSupportPosture,
    materialized_fact_posture_digest: Option<&str>,
    warning_kinds: &[ProjectionConsumptionWarningKind],
    deferred_neighbors: &[ProjectionConsumptionDeferredNeighborFamily],
) -> String {
    let mut encoder = consumption_scope_encoder("projection_consumption_receipt_v1")
        .field_shape(
            ForgeQueryEvidenceTag::new("declaration"),
            declaration_digest,
        )
        .field_shape(ForgeQueryEvidenceTag::new("contract"), contract_digest)
        .field_shape(ForgeQueryEvidenceTag::new("fact_set"), fact_set_digest)
        .field_shape(
            ForgeQueryEvidenceTag::new("source_family"),
            source_family.as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("source_identity"),
            source_identity,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("support_posture"),
            support_posture.as_str(),
        );
    if let Some(posture_digest) = materialized_fact_posture_digest {
        encoder = encoder.field_shape(
            ForgeQueryEvidenceTag::new("materialized_fact_posture"),
            posture_digest,
        );
    }
    let warnings = warning_kinds.iter().map(|warning| warning.as_str());
    let neighbors = deferred_neighbors.iter().map(|neighbor| neighbor.as_str());
    seal(
        encoder
            .field_value_sequence(ForgeQueryEvidenceTag::new("warning_kind"), warnings)
            .field_value_sequence(ForgeQueryEvidenceTag::new("deferred_neighbor"), neighbors),
    )
}
