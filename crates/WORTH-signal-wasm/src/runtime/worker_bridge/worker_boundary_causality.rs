use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerBoundaryCausalityStamp {
    transaction_sequence: u64,
    generation: u64,
}

impl WorkerBoundaryCausalityStamp {
    pub(in crate::runtime::worker_bridge) fn new(
        transaction_sequence: u64,
        generation: u64,
    ) -> Self {
        Self {
            transaction_sequence,
            generation,
        }
    }

    pub(in crate::runtime::worker_bridge) fn ordering_basis(&self) -> &'static str {
        "transactionSequenceThenGeneration"
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerBoundaryCausalityModel {
    pub ordering_basis: &'static str,
    pub transaction_sequence_required: bool,
    pub generation_required: bool,
    pub host_acknowledgement_is_authoritative: bool,
}

impl WorkerBoundaryCausalityModel {
    pub(in crate::runtime::worker_bridge) fn transaction_sequence_then_generation() -> Self {
        let zero = WorkerBoundaryCausalityStamp::new(0, 0);
        Self {
            ordering_basis: zero.ordering_basis(),
            transaction_sequence_required: true,
            generation_required: true,
            host_acknowledgement_is_authoritative: false,
        }
    }
}
