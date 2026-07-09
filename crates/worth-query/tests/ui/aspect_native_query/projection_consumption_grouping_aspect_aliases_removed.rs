use worth_query::facade::{ConsumedMembershipFact, ConsumedRelationEndpointFact};

fn main() {
    let membership = membership_fixture();
    let _ = membership.grouping_aspect();

    let endpoint = endpoint_fixture();
    let _ = endpoint.grouping_aspect();
}

fn membership_fixture() -> ConsumedMembershipFact {
    panic!("fixture only")
}

fn endpoint_fixture() -> ConsumedRelationEndpointFact {
    panic!("fixture only")
}
