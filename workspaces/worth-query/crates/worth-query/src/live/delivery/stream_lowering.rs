use super::super::locality::StreamLoweringCostPosture;
use super::super::LivePolicyCounters;
use super::member_projection::{StreamMemberProjection, StreamWindowCompatibility};
use super::query_contract::QueryDeliveryContract;
use super::replay_record::DeliveryContractReplayRecord;
use super::stream_admission::{
    AdmittedStreamConsumerContract, StreamConsumerShape, StreamContractDigest,
    StreamContractRequest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StreamLoweredDeliveryContract {
    pub(in crate::live) query_digest: String,
    pub(in crate::live) locality_digest: String,
    pub(in crate::live) delivery_digest: String,
    pub(in crate::live) query_delivery_contract: QueryDeliveryContract,
    pub(in crate::live) stream_contract_digest: StreamContractDigest,
    pub(in crate::live) delivery_contract_lowering: DeliveryContractLowering,
    pub(in crate::live) request: StreamContractRequest,
    pub(in crate::live) admitted_consumer_contract: AdmittedStreamConsumerContract,
    pub(in crate::live) member_projection: StreamMemberProjection,
    pub(in crate::live) window_compatibility: StreamWindowCompatibility,
    pub(in crate::live) replay_record: DeliveryContractReplayRecord,
    pub(in crate::live) counter_snapshot: LivePolicyCounters,
    pub(in crate::live) member_count: usize,
    pub(in crate::live) window_width: usize,
    pub(in crate::live) delivery_width: usize,
    pub(in crate::live) cost_posture: StreamLoweringCostPosture,
}

impl StreamLoweredDeliveryContract {
    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }

    pub fn locality_digest(&self) -> &str {
        &self.locality_digest
    }

    pub fn delivery_digest(&self) -> &str {
        &self.delivery_digest
    }

    pub fn query_delivery_contract(&self) -> &QueryDeliveryContract {
        &self.query_delivery_contract
    }

    pub fn stream_contract_digest(&self) -> &str {
        self.stream_contract_digest.as_str()
    }

    pub fn delivery_contract_lowering(&self) -> &DeliveryContractLowering {
        &self.delivery_contract_lowering
    }

    pub fn consumer_shape(&self) -> &StreamConsumerShape {
        self.request.consumer_shape()
    }

    pub fn request(&self) -> &StreamContractRequest {
        &self.request
    }

    pub fn admitted_consumer_contract(&self) -> &AdmittedStreamConsumerContract {
        &self.admitted_consumer_contract
    }

    pub fn member_projection(&self) -> &StreamMemberProjection {
        &self.member_projection
    }

    pub fn window_compatibility(&self) -> &StreamWindowCompatibility {
        &self.window_compatibility
    }

    pub fn replay_record(&self) -> &DeliveryContractReplayRecord {
        &self.replay_record
    }

    pub fn counter_snapshot(&self) -> &LivePolicyCounters {
        &self.counter_snapshot
    }

    pub fn member_count(&self) -> usize {
        self.member_count
    }

    pub fn window_width(&self) -> usize {
        self.window_width
    }

    pub fn delivery_width(&self) -> usize {
        self.delivery_width
    }

    pub fn cost_posture(&self) -> &StreamLoweringCostPosture {
        &self.cost_posture
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeliveryContractLowering {
    pub(in crate::live) digest: String,
    pub(in crate::live) query_delivery_digest: String,
    pub(in crate::live) request_digest: String,
    pub(in crate::live) admitted_consumer_contract_digest: String,
    pub(in crate::live) stream_contract_digest: String,
}

impl DeliveryContractLowering {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn query_delivery_digest(&self) -> &str {
        &self.query_delivery_digest
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub fn admitted_consumer_contract_digest(&self) -> &str {
        &self.admitted_consumer_contract_digest
    }

    pub fn stream_contract_digest(&self) -> &str {
        &self.stream_contract_digest
    }
}
