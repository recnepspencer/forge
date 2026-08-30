use worth_query_declaration::facade::diagnostics::{
    CanonicalizationCounters, CanonicalizationReport, CanonicalizationWarning,
    CompatibilityEvidence, IdentityFreezeEvidence, NormalizationEvent,
};

use crate::binary_encoding::BinaryEncodingSink;
use crate::binary_input::BinaryInput;
use crate::denial::{
    WorthQueryPackageArchiveDenial as Denial, WorthQueryPackageArchiveDenialKind as Kind,
};
use crate::record::decode_budget::RecordDecodeAttempt;
use crate::record::sequence::{decode_sequence, write_sequence};

pub(super) fn write_report(
    output: &mut dyn BinaryEncodingSink,
    report: &CanonicalizationReport,
) -> Result<(), Denial> {
    write_sequence(output, report.warnings(), write_warning)?;
    write_sequence(output, report.events(), write_event)?;
    match report.compatibility() {
        CompatibilityEvidence::Compatible => output.u16(1)?,
    }
    write_usize(output, report.normalized_projection_entries())?;
    write_usize(output, report.normalized_traversal_entries())?;
    write_usize(output, report.normalized_result_fields())?;
    output.text(&report.identity_freeze().query_digest)?;
    output.text(&report.identity_freeze().result_shape_digest)
}

pub(super) fn decode_report(
    input: &mut BinaryInput<'_>,
    budget: &mut RecordDecodeAttempt,
) -> Result<CanonicalizationReport, Denial> {
    let warnings = decode_sequence(input, budget, 2, |input, _| decode_warning(input))?;
    let events = decode_sequence(input, budget, 2, |input, _| decode_event(input))?;
    let compatibility = match input.u16()? {
        1 => CompatibilityEvidence::Compatible,
        _ => return Err(Denial::new(Kind::UnsupportedRecordVariant)),
    };
    Ok(CanonicalizationReport::new(
        warnings,
        events,
        compatibility,
        decode_usize(input)?,
        decode_usize(input)?,
        decode_usize(input)?,
        IdentityFreezeEvidence {
            query_digest: input.text()?.to_owned(),
            result_shape_digest: input.text()?.to_owned(),
        },
    ))
}

pub(super) fn write_counters(
    output: &mut dyn BinaryEncodingSink,
    counters: &CanonicalizationCounters,
) -> Result<(), Denial> {
    for value in [
        counters.raw_clause_count,
        counters.normalized_clause_count,
        counters.projection_entry_count,
        counters.traversal_clause_count,
        counters.result_shape_field_count,
        counters.binding_descriptor_count,
        counters.query_deduplication_count,
        counters.result_shape_deduplication_count,
        counters.canonicalization_warning_count,
        counters.canonicalization_fallback_count,
    ] {
        write_usize(output, value)?;
    }
    Ok(())
}

pub(super) fn decode_counters(
    input: &mut BinaryInput<'_>,
) -> Result<CanonicalizationCounters, Denial> {
    Ok(CanonicalizationCounters {
        raw_clause_count: decode_usize(input)?,
        normalized_clause_count: decode_usize(input)?,
        projection_entry_count: decode_usize(input)?,
        traversal_clause_count: decode_usize(input)?,
        result_shape_field_count: decode_usize(input)?,
        binding_descriptor_count: decode_usize(input)?,
        query_deduplication_count: decode_usize(input)?,
        result_shape_deduplication_count: decode_usize(input)?,
        canonicalization_warning_count: decode_usize(input)?,
        canonicalization_fallback_count: decode_usize(input)?,
    })
}

fn write_warning(
    output: &mut dyn BinaryEncodingSink,
    warning: &CanonicalizationWarning,
) -> Result<(), Denial> {
    match warning {
        CanonicalizationWarning::DuplicateProjectionCollapsed { aspect, field } => {
            output.u16(1)?;
            output.text(aspect)?;
            output.text(field)
        }
        CanonicalizationWarning::DuplicateTraversalCollapsed { relation, depth } => {
            output.u16(2)?;
            output.text(relation)?;
            output.u8(*depth)
        }
        CanonicalizationWarning::DuplicateResultFieldCollapsed { delivered_name } => {
            output.u16(3)?;
            output.text(delivered_name)
        }
        CanonicalizationWarning::NonIdentityBindingMetadataIgnored { key } => {
            output.u16(4)?;
            output.text(key)
        }
    }
}

fn decode_warning(input: &mut BinaryInput<'_>) -> Result<CanonicalizationWarning, Denial> {
    match input.u16()? {
        1 => Ok(CanonicalizationWarning::DuplicateProjectionCollapsed {
            aspect: input.text()?.to_owned(),
            field: input.text()?.to_owned(),
        }),
        2 => Ok(CanonicalizationWarning::DuplicateTraversalCollapsed {
            relation: input.text()?.to_owned(),
            depth: input.u8()?,
        }),
        3 => Ok(CanonicalizationWarning::DuplicateResultFieldCollapsed {
            delivered_name: input.text()?.to_owned(),
        }),
        4 => Ok(CanonicalizationWarning::NonIdentityBindingMetadataIgnored {
            key: input.text()?.to_owned(),
        }),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

fn write_event(
    output: &mut dyn BinaryEncodingSink,
    event: &NormalizationEvent,
) -> Result<(), Denial> {
    match event {
        NormalizationEvent::ProjectionRetained { aspect, field } => {
            output.u16(1)?;
            output.text(aspect)?;
            output.text(field)
        }
        NormalizationEvent::ProjectionCollapsedDuplicate { aspect, field } => {
            output.u16(2)?;
            output.text(aspect)?;
            output.text(field)
        }
        NormalizationEvent::TraversalRetained { relation, depth } => {
            output.u16(3)?;
            output.text(relation)?;
            output.u8(*depth)
        }
        NormalizationEvent::TraversalCollapsedDuplicate { relation, depth } => {
            output.u16(4)?;
            output.text(relation)?;
            output.u8(*depth)
        }
        NormalizationEvent::ResultFieldRetained {
            source_aspect,
            source_field,
            delivered_name,
        } => {
            output.u16(5)?;
            output.text(source_aspect)?;
            output.text(source_field)?;
            output.text(delivered_name)
        }
        NormalizationEvent::ResultFieldCollapsedDuplicate { delivered_name } => {
            output.u16(6)?;
            output.text(delivered_name)
        }
        NormalizationEvent::IdentityBindingRetained { slot } => {
            output.u16(7)?;
            output.text(slot)
        }
        NormalizationEvent::IdentityBindingCollapsedDuplicate { slot } => {
            output.u16(8)?;
            output.text(slot)
        }
        NormalizationEvent::NonIdentityBindingIgnored { key } => {
            output.u16(9)?;
            output.text(key)
        }
        NormalizationEvent::CompatibilityEstablished => output.u16(10),
        NormalizationEvent::IdentityFrozen {
            query_digest,
            result_shape_digest,
        } => {
            output.u16(11)?;
            output.text(query_digest)?;
            output.text(result_shape_digest)
        }
    }
}

fn decode_event(input: &mut BinaryInput<'_>) -> Result<NormalizationEvent, Denial> {
    match input.u16()? {
        1 => Ok(NormalizationEvent::ProjectionRetained {
            aspect: input.text()?.to_owned(),
            field: input.text()?.to_owned(),
        }),
        2 => Ok(NormalizationEvent::ProjectionCollapsedDuplicate {
            aspect: input.text()?.to_owned(),
            field: input.text()?.to_owned(),
        }),
        3 => Ok(NormalizationEvent::TraversalRetained {
            relation: input.text()?.to_owned(),
            depth: input.u8()?,
        }),
        4 => Ok(NormalizationEvent::TraversalCollapsedDuplicate {
            relation: input.text()?.to_owned(),
            depth: input.u8()?,
        }),
        5 => Ok(NormalizationEvent::ResultFieldRetained {
            source_aspect: input.text()?.to_owned(),
            source_field: input.text()?.to_owned(),
            delivered_name: input.text()?.to_owned(),
        }),
        6 => Ok(NormalizationEvent::ResultFieldCollapsedDuplicate {
            delivered_name: input.text()?.to_owned(),
        }),
        7 => Ok(NormalizationEvent::IdentityBindingRetained {
            slot: input.text()?.to_owned(),
        }),
        8 => Ok(NormalizationEvent::IdentityBindingCollapsedDuplicate {
            slot: input.text()?.to_owned(),
        }),
        9 => Ok(NormalizationEvent::NonIdentityBindingIgnored {
            key: input.text()?.to_owned(),
        }),
        10 => Ok(NormalizationEvent::CompatibilityEstablished),
        11 => Ok(NormalizationEvent::IdentityFrozen {
            query_digest: input.text()?.to_owned(),
            result_shape_digest: input.text()?.to_owned(),
        }),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

fn write_usize(output: &mut dyn BinaryEncodingSink, value: usize) -> Result<(), Denial> {
    output.u64(u64::try_from(value).map_err(|_| Denial::new(Kind::NumericWidthExceeded))?)
}

fn decode_usize(input: &mut BinaryInput<'_>) -> Result<usize, Denial> {
    usize::try_from(input.u64()?).map_err(|_| Denial::new(Kind::NumericWidthExceeded))
}
