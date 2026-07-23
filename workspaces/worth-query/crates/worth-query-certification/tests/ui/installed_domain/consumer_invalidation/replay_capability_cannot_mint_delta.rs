use worth_query::facade::{certification, domain, foundation};

fn mint_from_replay<D, O, F, L: foundation::BasisOperationLane>(
    replay: certification::WorthQueryCertificationReplayResult<D, O, F, L>,
    delivery: domain::WorthQuerySharedProjectionDelivery,
) -> domain::WorthQueryConsumerInvalidationDelta {
    replay.consumer_invalidation_delta(delivery)
}

fn main() {}
