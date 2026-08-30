use worth_foundational::facade::AspectKey;
use worth_query_installation::facade::{
    WorthQueryArtifactAccessPathContract as AccessPath,
    WorthQueryArtifactBulkProjectionContract as BulkProjection,
    WorthQueryArtifactChunkContract as Chunk, WorthQueryArtifactFieldSlicePosture as FieldSlice,
    WorthQueryArtifactNativeAccessContract as NativeAccess,
    WorthQueryArtifactNativeAlignment as Alignment,
    WorthQueryArtifactNativeFieldContract as NativeField,
    WorthQueryArtifactNativeLayoutContract as Layout,
    WorthQueryArtifactNativeLayoutIdentity as LayoutIdentity,
    WorthQueryArtifactNativeLayoutVersion as LayoutVersion,
    WorthQueryArtifactRowBatchPosture as RowBatch,
    WorthQueryArtifactScalarFallbackPosture as ScalarFallback,
};

use crate::binary_encoding::BinaryEncodingSink;
use crate::binary_input::BinaryInput;
use crate::denial::{
    WorthQueryPackageArchiveDenial as Denial, WorthQueryPackageArchiveDenialKind as Kind,
};
use crate::record::decode_budget::RecordDecodeAttempt;
use crate::record::foundational_aspect::{decode_aspect_contract, write_aspect_contract};
use crate::record::sequence::{decode_sequence, require_canonical_sequence_by, write_sequence};

pub(super) fn write_access_path(
    output: &mut dyn BinaryEncodingSink,
    contract: &AccessPath,
) -> Result<(), Denial> {
    match contract {
        AccessPath::Denied => output.u16(1),
        AccessPath::Native(contract) => {
            output.u16(2)?;
            write_native_access(output, contract)
        }
    }
}

pub(super) fn decode_access_path(
    input: &mut BinaryInput<'_>,
    budget: &mut RecordDecodeAttempt,
) -> Result<AccessPath, Denial> {
    match input.u16()? {
        1 => Ok(AccessPath::denied()),
        2 => Ok(AccessPath::native(decode_native_access(input, budget)?)),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

fn write_native_access(
    output: &mut dyn BinaryEncodingSink,
    contract: &NativeAccess,
) -> Result<(), Denial> {
    write_layout(output, contract.layout())?;
    output.u16(row_batch_tag(contract.row_batch()))?;
    match contract.chunks() {
        None => output.u16(1)?,
        Some(chunk) => {
            output.u16(2)?;
            write_usize(output, chunk.max_rows())?;
        }
    }
    write_sequence(output, contract.bulk_projections(), |output, projection| {
        write_bulk_projection(output, projection)
    })?;
    write_scalar_fallback(output, contract.scalar_fallback())
}

fn decode_native_access(
    input: &mut BinaryInput<'_>,
    budget: &mut RecordDecodeAttempt,
) -> Result<NativeAccess, Denial> {
    let layout = decode_layout(input, budget)?;
    let row_batch = row_batch_from_tag(input.u16()?)?;
    let chunks = match input.u16()? {
        1 => None,
        2 => Some(Chunk::bounded(decode_usize(input)?)),
        _ => return Err(Denial::new(Kind::UnsupportedRecordVariant)),
    };
    let bulk_projections = decode_sequence(input, budget, 20, decode_bulk_projection)?;
    require_canonical_sequence_by(&bulk_projections, |projection| projection.identity())?;
    let scalar_fallback = decode_scalar_fallback(input)?;
    Ok(NativeAccess::new(
        layout,
        row_batch,
        chunks,
        bulk_projections,
        scalar_fallback,
    ))
}

fn write_layout(output: &mut dyn BinaryEncodingSink, contract: &Layout) -> Result<(), Denial> {
    output.text(contract.identity().as_str())?;
    output.u32(contract.version().get())?;
    write_usize(output, contract.alignment().bytes())?;
    write_sequence(output, contract.fields(), |output, field| {
        write_aspect_contract(output, field.aspect())?;
        output.u16(field_slice_tag(field.field_slice()))
    })
}

fn decode_layout(
    input: &mut BinaryInput<'_>,
    budget: &mut RecordDecodeAttempt,
) -> Result<Layout, Denial> {
    let identity = LayoutIdentity::new(input.text()?.to_owned());
    let version = LayoutVersion::new(input.u32()?);
    let alignment = Alignment::new(decode_usize(input)?);
    let fields = decode_sequence(input, budget, 24, |input, budget| {
        Ok(NativeField::new(
            decode_aspect_contract(input, budget)?,
            field_slice_from_tag(input.u16()?)?,
        ))
    })?;
    Ok(Layout::new(identity, version, alignment, fields))
}

fn write_bulk_projection(
    output: &mut dyn BinaryEncodingSink,
    contract: &BulkProjection,
) -> Result<(), Denial> {
    output.text(contract.identity())?;
    write_sequence(output, contract.source_fields(), |output, field| {
        output.text(field.as_str())
    })?;
    write_usize(output, contract.destination_alignment().bytes())?;
    write_sequence(output, contract.destination_fields(), |output, field| {
        write_aspect_contract(output, field)
    })
}

fn decode_bulk_projection(
    input: &mut BinaryInput<'_>,
    budget: &mut RecordDecodeAttempt,
) -> Result<BulkProjection, Denial> {
    let identity = input.text()?.to_owned();
    let source_fields = decode_sequence(input, budget, 4, |input, _| {
        AspectKey::new(input.text()?.to_owned())
            .ok_or_else(|| Denial::new(Kind::InvalidRecordShape))
    })?;
    let destination_alignment = Alignment::new(decode_usize(input)?);
    let destination_fields = decode_sequence(input, budget, 22, |input, budget| {
        decode_aspect_contract(input, budget)
    })?;
    Ok(BulkProjection::new(
        identity,
        source_fields,
        destination_alignment,
        destination_fields,
    ))
}

fn write_scalar_fallback(
    output: &mut dyn BinaryEncodingSink,
    posture: ScalarFallback,
) -> Result<(), Denial> {
    match posture {
        ScalarFallback::Denied => output.u16(1),
        ScalarFallback::Admitted {
            max_calls_per_admission,
            max_call_amplification,
        } => {
            output.u16(2)?;
            write_usize(output, max_calls_per_admission)?;
            write_usize(output, max_call_amplification)
        }
    }
}

fn decode_scalar_fallback(input: &mut BinaryInput<'_>) -> Result<ScalarFallback, Denial> {
    match input.u16()? {
        1 => Ok(ScalarFallback::Denied),
        2 => Ok(ScalarFallback::admitted(
            decode_usize(input)?,
            decode_usize(input)?,
        )),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

const fn row_batch_tag(value: RowBatch) -> u16 {
    match value {
        RowBatch::Denied => 1,
        RowBatch::Borrowed => 2,
    }
}

fn row_batch_from_tag(tag: u16) -> Result<RowBatch, Denial> {
    match tag {
        1 => Ok(RowBatch::Denied),
        2 => Ok(RowBatch::Borrowed),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

const fn field_slice_tag(value: FieldSlice) -> u16 {
    match value {
        FieldSlice::Denied => 1,
        FieldSlice::Borrowed => 2,
        FieldSlice::ProviderNativeProjectionOnly => 3,
    }
}

fn field_slice_from_tag(tag: u16) -> Result<FieldSlice, Denial> {
    match tag {
        1 => Ok(FieldSlice::Denied),
        2 => Ok(FieldSlice::Borrowed),
        3 => Ok(FieldSlice::ProviderNativeProjectionOnly),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

fn write_usize(output: &mut dyn BinaryEncodingSink, value: usize) -> Result<(), Denial> {
    output.u64(u64::try_from(value).map_err(|_| Denial::new(Kind::NumericWidthExceeded))?)
}

fn decode_usize(input: &mut BinaryInput<'_>) -> Result<usize, Denial> {
    usize::try_from(input.u64()?).map_err(|_| Denial::new(Kind::NumericWidthExceeded))
}
