use worth_store_physical_format::physical_work_obligation::decode_physical_work_obligation_v6;
use worth_store_physical_format::wal_frame::decode_bounded_wal_frame_v1;
use worth_store_physical_format::{
    decode_extent_chunk, inspect_inline_page, DurableExtentManifest,
    DurablePhysicalRootManifest, DurableRootSelector, ExtentChunkCoordinate,
};
use worth_store_physical_integrity::{
    IntegrityValidatedCurrentRootSelector, IntegrityValidatedPreviousRootSelector,
    IntegrityValidatedExtentChunkFrame, IntegrityValidatedExtentManifest,
    IntegrityValidatedPageFrame, IntegrityValidatedPhysicalWorkObligation,
    IntegrityValidatedRootManifest, IntegrityValidatedWalFrame,
};

fn decode_selector(validated: IntegrityValidatedCurrentRootSelector<'_>) {
    let _ = DurableRootSelector::decode(validated);
}

fn decode_previous_selector(validated: IntegrityValidatedPreviousRootSelector<'_>) {
    let _ = DurableRootSelector::decode(validated);
}

fn decode_manifest(validated: IntegrityValidatedRootManifest<'_>) {
    let _ = DurablePhysicalRootManifest::decode(validated, 2);
}

fn decode_physical_work(validated: IntegrityValidatedPhysicalWorkObligation<'_>) {
    let _ = decode_physical_work_obligation_v6(validated);
}

fn decode_page(validated: IntegrityValidatedPageFrame<'_>) {
    let _ = inspect_inline_page(validated.record_format(), validated);
}

fn decode_wal(validated: IntegrityValidatedWalFrame<'_>) {
    let _ = decode_bounded_wal_frame_v1(validated);
}

fn decode_extent_manifest(validated: IntegrityValidatedExtentManifest<'_>) {
    let _ = DurableExtentManifest::decode(validated);
}

fn decode_extent(
    validated: IntegrityValidatedExtentChunkFrame<'_>,
    expected: ExtentChunkCoordinate,
) {
    let _ = decode_extent_chunk(validated, expected);
}

fn extract_extent_payload(validated: IntegrityValidatedExtentChunkFrame<'_>) {
    let _ = validated.chunk_bytes();
}

fn main() {}
