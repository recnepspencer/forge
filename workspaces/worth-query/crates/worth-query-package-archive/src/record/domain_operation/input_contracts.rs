use worth_foundational::facade::{AspectIdentity, AspectKey};
use worth_query_installation::facade::{
    WorthQueryOperationCapabilityRequirement as Capability,
    WorthQueryOperationCollectionContract as Collection,
    WorthQueryOperationCollectionField as CollectionField,
    WorthQueryOperationContinuationPosture as Continuation,
    WorthQueryOperationGroupingContract as Grouping,
    WorthQueryOperationNativeProjectionContract as NativeProjection,
    WorthQueryOperationParameterContract as Parameters,
    WorthQueryOperationParameterField as ParameterField,
    WorthQueryOperationRequiredDomainRole as RequiredDomain,
    WorthQueryOperationValueFamily as ValueFamily, WorthQueryOperationWindowPolicy as Window,
};

use crate::binary_encoding::BinaryEncodingSink;
use crate::binary_input::BinaryInput;
use crate::denial::{
    WorthQueryPackageArchiveDenial as Denial, WorthQueryPackageArchiveDenialKind as Kind,
};
use crate::record::decode_budget::RecordDecodeAttempt;
use crate::record::foundational_aspect::{
    decode_aspect_contract, decode_field_path, decode_projection_mask, write_aspect_contract,
    write_field_path, write_projection_mask,
};
use crate::record::foundational_value::decode_bool;
use crate::record::sequence::{decode_sequence, write_sequence};

pub(super) fn write_parameters(
    output: &mut dyn BinaryEncodingSink,
    contract: &Parameters,
) -> Result<(), Denial> {
    match contract {
        Parameters::NotRequired => output.u16(1),
        Parameters::Declared { fields } => {
            output.u16(2)?;
            write_sequence(output, fields, |output, field| {
                output.text(&field.name)?;
                write_value_family(output, &field.value_family)?;
                write_bool_dyn(output, field.required)
            })
        }
    }
}

pub(super) fn decode_parameters(
    input: &mut BinaryInput<'_>,
    budget: &mut RecordDecodeAttempt,
) -> Result<Parameters, Denial> {
    match input.u16()? {
        1 => Ok(Parameters::NotRequired),
        2 => Ok(Parameters::Declared {
            fields: decode_sequence(input, budget, 9, |input, _| {
                Ok(ParameterField {
                    name: input.text()?.to_owned(),
                    value_family: decode_value_family(input)?,
                    required: decode_bool(input)?,
                })
            })?,
        }),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

pub(super) fn write_native_projection(
    output: &mut dyn BinaryEncodingSink,
    contract: &NativeProjection,
) -> Result<(), Denial> {
    write_aspect_contract(output, contract.contract())?;
    write_projection_mask(output, contract.mask())
}

pub(super) fn decode_native_projection(
    input: &mut BinaryInput<'_>,
    budget: &mut RecordDecodeAttempt,
) -> Result<NativeProjection, Denial> {
    let contract = decode_aspect_contract(input, budget)?;
    let mask = decode_projection_mask(input, budget)?;
    NativeProjection::new(contract, mask).map_err(|_| Denial::new(Kind::InvalidRecordShape))
}

pub(super) fn write_collection(
    output: &mut dyn BinaryEncodingSink,
    contract: &Collection,
) -> Result<(), Denial> {
    match contract {
        Collection::NotCollection => output.u16(1),
        Collection::Collection {
            row_identity_field,
            ordering_fields,
            grouping,
            window,
            continuation,
        } => {
            output.u16(2)?;
            write_collection_field(output, row_identity_field)?;
            write_sequence(output, ordering_fields, |output, field| {
                write_collection_field(output, field)
            })?;
            match grouping {
                Grouping::Ungrouped => output.u16(1)?,
                Grouping::Grouped { grouping_fields } => {
                    output.u16(2)?;
                    write_sequence(output, grouping_fields, |output, field| {
                        write_collection_field(output, field)
                    })?;
                }
            }
            output.u16(match window {
                Window::CompleteCollection => 1,
                Window::ContinuationBounded => 2,
            })?;
            output.u16(match continuation {
                Continuation::NotRequired => 1,
                Continuation::SnapshotCursor => 2,
                Continuation::LiveCursor => 3,
            })
        }
    }
}

pub(super) fn decode_collection(
    input: &mut BinaryInput<'_>,
    budget: &mut RecordDecodeAttempt,
) -> Result<Collection, Denial> {
    match input.u16()? {
        1 => Ok(Collection::NotCollection),
        2 => {
            let row_identity_field = decode_collection_field(input, budget)?;
            let ordering_fields = decode_sequence(input, budget, 9, decode_collection_field)?;
            let grouping = match input.u16()? {
                1 => Grouping::Ungrouped,
                2 => Grouping::Grouped {
                    grouping_fields: decode_sequence(input, budget, 9, decode_collection_field)?,
                },
                _ => return Err(Denial::new(Kind::UnsupportedRecordVariant)),
            };
            let window = match input.u16()? {
                1 => Window::CompleteCollection,
                2 => Window::ContinuationBounded,
                _ => return Err(Denial::new(Kind::UnsupportedRecordVariant)),
            };
            let continuation = match input.u16()? {
                1 => Continuation::NotRequired,
                2 => Continuation::SnapshotCursor,
                3 => Continuation::LiveCursor,
                _ => return Err(Denial::new(Kind::UnsupportedRecordVariant)),
            };
            Ok(Collection::Collection {
                row_identity_field,
                ordering_fields,
                grouping,
                window,
                continuation,
            })
        }
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

pub(super) fn write_capabilities(
    output: &mut dyn BinaryEncodingSink,
    values: &[Capability],
) -> Result<(), Denial> {
    write_sequence(output, values, |output, value| {
        output.u16(capability_tag(*value))
    })
}

pub(super) fn decode_capabilities(
    input: &mut BinaryInput<'_>,
    budget: &mut RecordDecodeAttempt,
) -> Result<Vec<Capability>, Denial> {
    decode_sequence(input, budget, 2, |input, _| {
        capability_from_tag(input.u16()?)
    })
}

pub(super) fn write_required_domains(
    output: &mut dyn BinaryEncodingSink,
    values: &[RequiredDomain],
) -> Result<(), Denial> {
    write_sequence(output, values, |output, value| output.text(value.as_str()))
}

pub(super) fn decode_required_domains(
    input: &mut BinaryInput<'_>,
    budget: &mut RecordDecodeAttempt,
) -> Result<Vec<RequiredDomain>, Denial> {
    decode_sequence(input, budget, 4, |input, _| {
        RequiredDomain::new(input.text()?.to_owned())
            .map_err(|_| Denial::new(Kind::InvalidRecordShape))
    })
}

fn write_value_family(
    output: &mut dyn BinaryEncodingSink,
    family: &ValueFamily,
) -> Result<(), Denial> {
    match family {
        ValueFamily::Bool => output.u16(1),
        ValueFamily::I64 => output.u16(2),
        ValueFamily::U64 => output.u16(3),
        ValueFamily::Text => output.u16(4),
        ValueFamily::EntityIdentity => output.u16(5),
        ValueFamily::NativeAspect { key, identity } => {
            output.u16(6)?;
            output.text(key.as_str())?;
            output.u64(identity.0)
        }
    }
}

fn decode_value_family(input: &mut BinaryInput<'_>) -> Result<ValueFamily, Denial> {
    match input.u16()? {
        1 => Ok(ValueFamily::Bool),
        2 => Ok(ValueFamily::I64),
        3 => Ok(ValueFamily::U64),
        4 => Ok(ValueFamily::Text),
        5 => Ok(ValueFamily::EntityIdentity),
        6 => Ok(ValueFamily::NativeAspect {
            key: AspectKey::new(input.text()?.to_owned())
                .ok_or_else(|| Denial::new(Kind::InvalidRecordShape))?,
            identity: AspectIdentity(input.u64()?),
        }),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

fn write_collection_field(
    output: &mut dyn BinaryEncodingSink,
    field: &CollectionField,
) -> Result<(), Denial> {
    output.text(field.aspect_key().as_str())?;
    write_field_path(output, field.field_path())
}

fn decode_collection_field(
    input: &mut BinaryInput<'_>,
    budget: &mut RecordDecodeAttempt,
) -> Result<CollectionField, Denial> {
    let aspect_key = AspectKey::new(input.text()?.to_owned())
        .ok_or_else(|| Denial::new(Kind::InvalidRecordShape))?;
    let field_path = decode_field_path(input, budget)?;
    Ok(CollectionField::new(aspect_key, field_path))
}

fn write_bool_dyn(output: &mut dyn BinaryEncodingSink, value: bool) -> Result<(), Denial> {
    output.u16(if value { 1 } else { 0 })
}

const fn capability_tag(value: Capability) -> u16 {
    match value {
        Capability::QueryRead => 1,
        Capability::QueryComposition => 2,
        Capability::QueryContext => 3,
        Capability::IdentityEvolution => 4,
        Capability::LiveQuery => 5,
        Capability::PreviewSession => 6,
        Capability::WorkflowOrchestration => 7,
        Capability::HistoricalEvaluation => 8,
        Capability::DurableArtifacts => 9,
    }
}

fn capability_from_tag(tag: u16) -> Result<Capability, Denial> {
    match tag {
        1 => Ok(Capability::QueryRead),
        2 => Ok(Capability::QueryComposition),
        3 => Ok(Capability::QueryContext),
        4 => Ok(Capability::IdentityEvolution),
        5 => Ok(Capability::LiveQuery),
        6 => Ok(Capability::PreviewSession),
        7 => Ok(Capability::WorkflowOrchestration),
        8 => Ok(Capability::HistoricalEvaluation),
        9 => Ok(Capability::DurableArtifacts),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}
