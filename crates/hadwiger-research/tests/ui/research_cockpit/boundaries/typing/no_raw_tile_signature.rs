use hadwiger_research::facade::{TileEquivalenceScope, TileEquivalenceWitness};

fn main() {
    let _ = TileEquivalenceWitness::builder("tile", TileEquivalenceScope::ContactConstraint)
        .with_left_contact_signature("raw-signature");
}
