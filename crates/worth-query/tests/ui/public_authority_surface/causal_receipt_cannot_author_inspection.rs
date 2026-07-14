use worth_query::facade::runtime::{CausalInspection, QueryObservationReceipt};

fn main() {
    let receipt: QueryObservationReceipt = todo!();
    let _ = CausalInspection::for_observation(receipt);
}
