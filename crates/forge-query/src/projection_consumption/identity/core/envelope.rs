use super::super::scope::{consumption_scope_encoder, seal};
use crate::ForgeQueryEvidenceTag;

use super::super::super::contracts::ProjectionContractSupportPosture;
use super::super::super::eligibility::ProjectionConsumptionWarningKind;
use super::super::super::source::ProjectionSourceFamily;

pub(crate) fn compose_envelope_source_refs_digest(
    receipt_digest: &str,
    fact_set_digest: &str,
    contract_digest: &str,
) -> String {
    seal(
        consumption_scope_encoder("projection_consumption_envelope_source_refs_v1")
            .field_shape(ForgeQueryEvidenceTag::new("receipt"), receipt_digest)
            .field_shape(ForgeQueryEvidenceTag::new("fact_set"), fact_set_digest)
            .field_shape(ForgeQueryEvidenceTag::new("contract"), contract_digest),
    )
}

pub(crate) fn compose_envelope_performance_digest(
    receipt_digest: &str,
    counter_snapshot_digest: &str,
) -> String {
    seal(
        consumption_scope_encoder("projection_consumption_envelope_performance_v1")
            .field_shape(ForgeQueryEvidenceTag::new("receipt"), receipt_digest)
            .field_shape(
                ForgeQueryEvidenceTag::new("counters"),
                counter_snapshot_digest,
            ),
    )
}

pub(crate) fn compose_envelope_boundary_digest(
    source_family: ProjectionSourceFamily,
    source_identity: &str,
    support_posture: &ProjectionContractSupportPosture,
    warning_kinds: &[ProjectionConsumptionWarningKind],
) -> String {
    seal(
        consumption_scope_encoder("projection_consumption_envelope_boundary_v1")
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
            )
            .field_value_sequence(
                ForgeQueryEvidenceTag::new("warning_kind"),
                warning_kinds.iter().map(|warning| warning.as_str()),
            ),
    )
}

pub(crate) fn compose_envelope_digest(
    receipt_digest: &str,
    integrity_digest: &str,
    performance_digest: &str,
    boundary_digest: &str,
    transition_rules_digest: &str,
    source_refs_digest: &str,
) -> String {
    seal(
        consumption_scope_encoder("self_describing_projection_consumption_envelope_v1")
            .field_shape(ForgeQueryEvidenceTag::new("receipt"), receipt_digest)
            .field_shape(ForgeQueryEvidenceTag::new("integrity"), integrity_digest)
            .field_shape(
                ForgeQueryEvidenceTag::new("performance"),
                performance_digest,
            )
            .field_shape(ForgeQueryEvidenceTag::new("boundary"), boundary_digest)
            .field_shape(
                ForgeQueryEvidenceTag::new("transitions"),
                transition_rules_digest,
            )
            .field_shape(ForgeQueryEvidenceTag::new("sources"), source_refs_digest),
    )
}
