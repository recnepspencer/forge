use std::io::Read;

use flate2::read::GzDecoder;

use crate::{
    ForgeServerMultipartUpload, ForgeServerQueryHandoffDenial, ForgeServerQueryHandoffDenialCode,
    ForgeServerUploadContentEncoding, ForgeServerUploadTransferMode,
};

use super::performance::ForgeServerIngressMetricSnapshot;

const MAX_STREAMED_WIRE_BYTES: u64 = 16 * 1024 * 1024;
const MAX_CHUNKS_PER_PART: usize = 32;
const MAX_SINGLE_CHUNK_WIRE_BYTES: u64 = 2 * 1024 * 1024;
const MAX_DECOMPRESSION_RATIO: u64 = 32;

pub(crate) fn admit_binary_ingress(
    upload: &ForgeServerMultipartUpload,
    diagnostics_profile: forge_foundational::facade::DiagnosticRichnessProfile,
) -> Result<ForgeServerIngressMetricSnapshot, ForgeServerQueryHandoffDenial> {
    let mut metrics = ForgeServerIngressMetricSnapshot {
        sessions_started: 1,
        parts_processed: upload.parts().len() as u64,
        ..ForgeServerIngressMetricSnapshot::default()
    };

    for part in upload.parts() {
        let wire_chunks = part.effective_wire_chunks();
        let wire_bytes = wire_chunks
            .iter()
            .map(crate::ForgeServerUploadChunk::wire_len)
            .sum::<u64>();
        let authoritative_bytes = part.effective_authoritative_bytes();

        metrics.wire_bytes += wire_bytes;
        metrics.authoritative_bytes += authoritative_bytes.len() as u64;
        metrics.chunks_observed += wire_chunks.len() as u64;
        if part.transfer_mode() == ForgeServerUploadTransferMode::UnknownLength {
            metrics.unknown_length_parts += 1;
        }
        if part.content_encoding() == ForgeServerUploadContentEncoding::Gzip {
            metrics.compressed_parts += 1;
        }

        ensure_declared_length_matches(
            part.name(),
            part.declared_length(),
            authoritative_bytes.len() as u64,
            diagnostics_profile,
        )?;
        ensure_chunk_budget_holds(part.name(), &wire_chunks, diagnostics_profile)?;
        ensure_unknown_length_budget_holds(
            part.name(),
            part.transfer_mode(),
            wire_bytes,
            diagnostics_profile,
        )?;
        ensure_transport_round_trips(
            part.name(),
            part.content_encoding(),
            &wire_chunks,
            &authoritative_bytes,
            diagnostics_profile,
        )?;
        ensure_decompression_ratio_holds(
            part.name(),
            part.content_encoding(),
            wire_bytes,
            authoritative_bytes.len() as u64,
            diagnostics_profile,
        )?;
    }

    Ok(metrics)
}

fn ensure_declared_length_matches(
    part_name: &str,
    declared_length: u64,
    authoritative_length: u64,
    diagnostics_profile: forge_foundational::facade::DiagnosticRichnessProfile,
) -> Result<(), ForgeServerQueryHandoffDenial> {
    if authoritative_length != declared_length {
        return Err(ForgeServerQueryHandoffDenial::new(
            ForgeServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
            diagnostics_profile,
            format!(
                "compatibility upload part `{part_name}` authoritative byte count `{authoritative_length}` did not match declared length `{declared_length}`"
            ),
        ));
    }
    Ok(())
}

fn ensure_chunk_budget_holds(
    part_name: &str,
    wire_chunks: &[crate::ForgeServerUploadChunk],
    diagnostics_profile: forge_foundational::facade::DiagnosticRichnessProfile,
) -> Result<(), ForgeServerQueryHandoffDenial> {
    if wire_chunks.len() > MAX_CHUNKS_PER_PART {
        return Err(ForgeServerQueryHandoffDenial::new(
            ForgeServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
            diagnostics_profile,
            format!(
                "compatibility upload part `{part_name}` exceeded the chunk pacing cap `{MAX_CHUNKS_PER_PART}` with `{}` chunks",
                wire_chunks.len(),
            ),
        ));
    }
    if let Some(observed) = wire_chunks
        .iter()
        .map(crate::ForgeServerUploadChunk::wire_len)
        .find(|value| *value > MAX_SINGLE_CHUNK_WIRE_BYTES)
    {
        return Err(ForgeServerQueryHandoffDenial::new(
            ForgeServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
            diagnostics_profile,
            format!(
                "compatibility upload part `{part_name}` chunk wire size `{observed}` exceeded the per-chunk cap `{MAX_SINGLE_CHUNK_WIRE_BYTES}`"
            ),
        ));
    }
    Ok(())
}

fn ensure_unknown_length_budget_holds(
    part_name: &str,
    transfer_mode: ForgeServerUploadTransferMode,
    wire_bytes: u64,
    diagnostics_profile: forge_foundational::facade::DiagnosticRichnessProfile,
) -> Result<(), ForgeServerQueryHandoffDenial> {
    if transfer_mode == ForgeServerUploadTransferMode::UnknownLength
        && wire_bytes > MAX_STREAMED_WIRE_BYTES
    {
        return Err(ForgeServerQueryHandoffDenial::new(
            ForgeServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
            diagnostics_profile,
            format!(
                "compatibility upload part `{part_name}` streamed `{wire_bytes}` wire bytes without a known content length, exceeding the cap `{MAX_STREAMED_WIRE_BYTES}`"
            ),
        ));
    }
    Ok(())
}

fn ensure_transport_round_trips(
    part_name: &str,
    content_encoding: ForgeServerUploadContentEncoding,
    wire_chunks: &[crate::ForgeServerUploadChunk],
    authoritative_bytes: &[u8],
    diagnostics_profile: forge_foundational::facade::DiagnosticRichnessProfile,
) -> Result<(), ForgeServerQueryHandoffDenial> {
    let wire_bytes = join_wire_chunks(wire_chunks);
    match content_encoding {
        ForgeServerUploadContentEncoding::Identity => {
            if wire_bytes != authoritative_bytes {
                return Err(ForgeServerQueryHandoffDenial::new(
                    ForgeServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
                    diagnostics_profile,
                    format!(
                        "compatibility upload part `{part_name}` wire bytes did not exactly match authoritative bytes under identity encoding"
                    ),
                ));
            }
        }
        ForgeServerUploadContentEncoding::Gzip => {
            let decoded_bytes = decode_gzip_payload(part_name, &wire_bytes, diagnostics_profile)?;
            if decoded_bytes != authoritative_bytes {
                return Err(ForgeServerQueryHandoffDenial::new(
                    ForgeServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
                    diagnostics_profile,
                    format!(
                        "compatibility upload part `{part_name}` decoded wire bytes did not exactly match authoritative bytes"
                    ),
                ));
            }
        }
    }
    Ok(())
}

fn ensure_decompression_ratio_holds(
    part_name: &str,
    content_encoding: ForgeServerUploadContentEncoding,
    wire_bytes: u64,
    authoritative_bytes: u64,
    diagnostics_profile: forge_foundational::facade::DiagnosticRichnessProfile,
) -> Result<(), ForgeServerQueryHandoffDenial> {
    if content_encoding == ForgeServerUploadContentEncoding::Gzip
        && wire_bytes > 0
        && authoritative_bytes > wire_bytes.saturating_mul(MAX_DECOMPRESSION_RATIO)
    {
        return Err(ForgeServerQueryHandoffDenial::new(
            ForgeServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
            diagnostics_profile,
            format!(
                "compatibility upload part `{part_name}` exceeded the decompression ratio cap `{MAX_DECOMPRESSION_RATIO}` with `{authoritative_bytes}` decoded bytes over `{wire_bytes}` wire bytes"
            ),
        ));
    }
    Ok(())
}

fn join_wire_chunks(wire_chunks: &[crate::ForgeServerUploadChunk]) -> Vec<u8> {
    let total_bytes = wire_chunks
        .iter()
        .map(|chunk| chunk.wire_bytes().len())
        .sum();
    let mut joined = Vec::with_capacity(total_bytes);
    for chunk in wire_chunks {
        joined.extend_from_slice(chunk.wire_bytes());
    }
    joined
}

fn decode_gzip_payload(
    part_name: &str,
    wire_bytes: &[u8],
    diagnostics_profile: forge_foundational::facade::DiagnosticRichnessProfile,
) -> Result<Vec<u8>, ForgeServerQueryHandoffDenial> {
    let mut decoder = GzDecoder::new(wire_bytes);
    let mut decoded = Vec::new();
    decoder.read_to_end(&mut decoded).map_err(|error| {
        ForgeServerQueryHandoffDenial::new(
            ForgeServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid,
            diagnostics_profile,
            format!(
                "compatibility upload part `{part_name}` declared gzip encoding but could not be decoded: {error}"
            ),
        )
    })?;
    Ok(decoded)
}
