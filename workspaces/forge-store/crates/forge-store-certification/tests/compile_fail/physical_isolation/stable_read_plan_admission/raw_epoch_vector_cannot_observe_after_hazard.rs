use forge_store_physical_isolation::{PhysicalEpochVector, PublishedReaderHazard};

fn main() {
    let hazard: PublishedReaderHazard = todo!();
    let observed: PhysicalEpochVector = todo!();
    let _observation = hazard.observe_after_publication(observed);
}
