use forge_store::{SubscriptionSupportOperationalBasis, SupportAffectedSet};

fn attempt(bases: Vec<SubscriptionSupportOperationalBasis>) {
    let _ = SupportAffectedSet::from_retention_bases(bases);
}

fn main() {}
