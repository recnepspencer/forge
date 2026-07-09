use worth_query::facade::{ActiveSubscriptionLaneDigest, ActiveSubscriptionLaneHandle};

fn main() {
    let _handle = ActiveSubscriptionLaneHandle {
        lane_digest: ActiveSubscriptionLaneDigest(String::new()),
        lane_index: 0,
        registry_generation: 1,
    };
}
