use super::super::scope::{consumption_scope_encoder, seal};
use crate::ForgeQueryEvidenceTag;

use super::super::super::consumed::ProjectionFactExtractionCounters;
use super::super::super::facts::ProjectionMaterializedFactPostureKind;

pub(crate) fn compose_extraction_counters_digest(
    counters: &ProjectionFactExtractionCounters,
) -> String {
    seal(
        consumption_scope_encoder("projection_fact_extraction_counters_v1")
            .field_usize(
                ForgeQueryEvidenceTag::new("declared"),
                counters.declared_fact_family_count(),
            )
            .field_usize(
                ForgeQueryEvidenceTag::new("admitted"),
                counters.admitted_fact_family_count(),
            )
            .field_usize(
                ForgeQueryEvidenceTag::new("extracted"),
                counters.extracted_fact_count(),
            )
            .field_usize(
                ForgeQueryEvidenceTag::new("row_width"),
                counters.source_row_width_consumed(),
            )
            .field_usize(
                ForgeQueryEvidenceTag::new("evidence_width"),
                counters.source_evidence_lookup_width(),
            )
            .field_usize(
                ForgeQueryEvidenceTag::new("authority_reopen"),
                counters.authority_reopen_count(),
            ),
    )
}

pub(crate) fn compose_materialized_fact_posture_digest(
    kind: ProjectionMaterializedFactPostureKind,
    lower_declaration_digest: &str,
    basis_digest: &str,
    support_evidence_digest: &str,
    runtime_origin_digest: Option<&str>,
) -> String {
    let mut encoder = consumption_scope_encoder("projection_materialized_fact_posture_v1")
        .field_shape(ForgeQueryEvidenceTag::new("kind"), kind.as_str())
        .field_shape(
            ForgeQueryEvidenceTag::new("lower_declaration"),
            lower_declaration_digest,
        )
        .field_shape(ForgeQueryEvidenceTag::new("basis"), basis_digest)
        .field_shape(
            ForgeQueryEvidenceTag::new("support"),
            support_evidence_digest,
        );
    if let Some(runtime_origin) = runtime_origin_digest {
        encoder = encoder.field_shape(ForgeQueryEvidenceTag::new("runtime_origin"), runtime_origin);
    }
    seal(encoder)
}
