use crate::corruption::{
    classify_streaming_damage_before_decode, construct_quarantine_diagnostics,
    from_streaming_read_request, seal_quarantine_from_localization,
};
use crate::{
    BlobChunkProofLeaf, BlobCorruptedChunkLocalization, BlobCorruptionGuard,
    BlobCorruptionPlacementClass, BlobDamageCase, BlobQuarantineAuthority,
    BlobQuarantineDiagnostics, BlobStreamingReadCounterSnapshot, BlobStreamingReadDenial,
    BlobStreamingReadObservedChunk, BlobStreamingReadRequest,
};

pub(crate) fn collect_streaming_read_damage_evidence(
    expected: &BlobChunkProofLeaf,
    actual: &BlobStreamingReadObservedChunk,
) -> bool {
    expected.checksum_digest() == actual.payload().checksum().checksum().digest()
}

pub(crate) fn classify_streaming_read_damage(checksums_match: bool) -> Option<BlobDamageCase> {
    classify_streaming_damage_before_decode(checksums_match)
}

pub(crate) fn verify_chunk_checksum_pre_decode(checksums_match: bool) -> bool {
    checksums_match
}

pub(crate) fn localize_verified_read_damage(
    request: &BlobStreamingReadRequest,
    ordinal: crate::BlobChunkOrdinal,
    damage_case: BlobDamageCase,
) -> Result<BlobCorruptedChunkLocalization, BlobStreamingReadDenial> {
    from_streaming_read_request(
        request,
        ordinal,
        BlobCorruptionPlacementClass::LocalPhysical,
        damage_case,
    )
    .map_err(|denial| BlobStreamingReadDenial::CorruptionReferenceEdgeMismatch(Box::new(denial)))
}

pub(crate) fn assemble_streaming_corruption_denial(
    ordinal: crate::BlobChunkOrdinal,
    damage_case: BlobDamageCase,
    diagnostics: BlobQuarantineDiagnostics,
    counters: BlobStreamingReadCounterSnapshot,
) -> BlobStreamingReadDenial {
    BlobStreamingReadDenial::CorruptedChunk {
        ordinal,
        damage_case,
        diagnostics: Box::new(diagnostics),
        counters,
    }
}

pub(crate) fn observe_and_deny_streaming_corruption(
    request: &BlobStreamingReadRequest,
    quarantine_authority: &mut Option<BlobQuarantineAuthority>,
    expected: &BlobChunkProofLeaf,
    actual: &BlobStreamingReadObservedChunk,
    counters: &mut BlobStreamingReadCounterSnapshot,
) -> Result<(), BlobStreamingReadDenial> {
    let checksums_match = collect_streaming_read_damage_evidence(expected, actual);
    let Some(damage_case) = classify_streaming_read_damage(checksums_match) else {
        debug_assert!(verify_chunk_checksum_pre_decode(checksums_match));
        return Ok(());
    };
    *counters = counters.record_corrupt_chunk_denial();
    let localization = localize_verified_read_damage(request, expected.ordinal(), damage_case)?;
    let quarantine = seal_quarantine_from_localization(
        localization,
        quarantine_authority
            .take()
            .expect("corruption verifier has a single quarantine authority"),
    );
    let guard = BlobCorruptionGuard::from_quarantine(quarantine);
    let diagnostics = construct_quarantine_diagnostics(guard.quarantine().clone(), damage_case);
    let _denial = guard.deny_verified_read_publication();
    Err(assemble_streaming_corruption_denial(
        expected.ordinal(),
        damage_case,
        diagnostics,
        *counters,
    ))
}
