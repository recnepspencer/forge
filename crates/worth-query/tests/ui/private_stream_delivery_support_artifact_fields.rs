use worth_query::facade::foundation::{DeliveryContractLowering, DeliveryContractReplayRecord, QueryDeliveryContract, StreamMemberProjection, StreamWindowCompatibility};

fn main() {
    let _ = QueryDeliveryContract {
        digest: todo!(),
        query_digest: todo!(),
        locality_digest: todo!(),
        delivery_digest: todo!(),
        family: todo!(),
        locality_outcome: todo!(),
    };

    let _ = DeliveryContractLowering {
        digest: todo!(),
        query_delivery_digest: todo!(),
        request_digest: todo!(),
        admitted_consumer_contract_digest: todo!(),
        stream_contract_digest: todo!(),
    };

    let _ = StreamMemberProjection {
        digest: todo!(),
        consumer_shape: todo!(),
        member_count: todo!(),
        delivery_width: todo!(),
    };

    let _ = StreamWindowCompatibility {
        digest: todo!(),
        consumer_shape: todo!(),
        window_width: todo!(),
        budget_limit: todo!(),
    };

    let _ = DeliveryContractReplayRecord {
        digest: todo!(),
        query_digest: todo!(),
        delivery_digest: todo!(),
        replay_digest: todo!(),
        locality_outcome: todo!(),
        stream_contract_digest: todo!(),
    };
}
