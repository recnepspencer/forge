use forge_store::{
    SubscriptionSupportAccessStructure, SubscriptionSupportAccessStructureReport,
};

fn main() {
    let _ = SubscriptionSupportAccessStructureReport {
        required: vec![SubscriptionSupportAccessStructure::FamilyLookup],
        debted: vec![SubscriptionSupportAccessStructure::FamilyLookup],
    };
}
