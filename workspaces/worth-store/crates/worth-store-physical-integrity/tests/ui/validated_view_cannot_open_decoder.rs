use worth_store_physical_format::physical_work_obligation::decode_physical_work_obligation_v6;
use worth_store_physical_format::wal_frame::decode_bounded_wal_frame_v1;
use worth_store_physical_format::{
    inspect_inline_page, DurablePhysicalRootManifest, DurableRootSelector,
};
use worth_store_physical_integrity::{
    IntegrityValidatedCurrentRootSelector, IntegrityValidatedPreviousRootSelector,
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

fn main() {}
