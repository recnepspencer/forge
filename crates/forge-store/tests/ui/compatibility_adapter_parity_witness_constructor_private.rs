use forge_store::{
    CompatibilityAdapterCostClass, CompatibilityAdapterDigest, CompatibilityAdapterId,
    CompatibilityAdapterParityWitness,
};

fn main() {
    let _ = CompatibilityAdapterParityWitness::new(
        CompatibilityAdapterId::new("adapter"),
        CompatibilityAdapterDigest::new("digest"),
        CompatibilityAdapterCostClass::ZeroCopy,
    );
}
