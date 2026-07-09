#![allow(invalid_value)]

use worth_store::{SupportReplicationEquivalenceWitness, SupportTrustEquivalenceWitness};

fn main() {
    let _ = SupportReplicationEquivalenceWitness {
        witness: unsafe { std::mem::zeroed::<SupportTrustEquivalenceWitness>() },
    };
}
