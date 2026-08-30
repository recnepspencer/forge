use worth_store_physical_format::{DurablePhysicalRootManifest, DurableRootSelector};
use worth_store_physical_integrity::{
    IntegrityValidatedCurrentRootSelector, IntegrityValidatedPreviousRootSelector,
    IntegrityValidatedRootManifest,
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

fn main() {}
