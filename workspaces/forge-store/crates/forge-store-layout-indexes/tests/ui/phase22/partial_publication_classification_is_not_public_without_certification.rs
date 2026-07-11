use forge_store_recovery_physics::{PartialPublicationClassification, PartialPublicationObservationSet};

fn main() {
    let _ = PartialPublicationClassification::classify_observations(
        PartialPublicationObservationSet::new(),
    );
}
