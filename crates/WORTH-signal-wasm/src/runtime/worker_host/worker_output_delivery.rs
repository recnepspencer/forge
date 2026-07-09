use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::boundary::errors::WORTHSignalJsError;
use crate::expression::model::SignalValue;

use super::{
    canonical_worker_certification_digest, committed_truth_digest_for_runtime,
    WorkerHostBoundaryPerformanceEnvelope, WorkerRuntimeShell,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerOutputDeliveryRequest {
    pub output_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerDeliveredOutput {
    pub id: String,
    pub value: SignalValue,
    pub payload_byte_count: u64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerOutputDeliveryPacket {
    pub envelope_family: &'static str,
    pub delivery_mode: &'static str,
    pub runtime_authority: &'static str,
    pub output_delivery_packet_count: u64,
    pub output_delivery_breadth: u64,
    pub output_payload_byte_count: u64,
    pub worker_first_truth_digest: String,
    pub output_digest: String,
    pub boundary_performance: WorkerHostBoundaryPerformanceEnvelope,
    pub packet_digest: String,
    pub outputs: Vec<WorkerDeliveredOutput>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerOutputDeliveryCertificationPackage {
    pub certification_family: &'static str,
    pub covered_suite_count: u64,
    pub output_delivery_packet_count: u64,
    pub output_delivery_breadth: u64,
    pub output_payload_byte_count: u64,
    pub worker_first_truth_digest: String,
    pub output_digest: String,
    pub boundary_performance_digest: String,
    pub packet_digest: String,
    pub certification_digest: String,
}

impl WorkerOutputDeliveryPacket {
    pub(in crate::runtime::worker_host) fn from_outputs(
        outputs: Vec<WorkerDeliveredOutput>,
        worker_first_truth_digest: String,
    ) -> Result<Self, WORTHSignalJsError> {
        let output_delivery_breadth = outputs.len() as u64;
        let output_payload_byte_count = outputs.iter().fold(0_u64, |total, output| {
            total.saturating_add(output.payload_byte_count)
        });
        let output_digest = canonical_worker_certification_digest(&outputs)?;
        let boundary_performance = WorkerHostBoundaryPerformanceEnvelope::output_delivery(
            output_delivery_breadth,
            output_payload_byte_count,
            output_digest.as_str(),
        )?;
        let packet_digest = canonical_worker_certification_digest(&(
            "outputDelivery",
            "CommittedOutputDelivery",
            output_delivery_breadth,
            output_payload_byte_count,
            worker_first_truth_digest.as_str(),
            output_digest.as_str(),
            boundary_performance.performance_digest.as_str(),
        ))?;

        Ok(Self {
            envelope_family: "outputDelivery",
            delivery_mode: "CommittedOutputDelivery",
            runtime_authority: "workerOwnedRuntime",
            output_delivery_packet_count: 1,
            output_delivery_breadth,
            output_payload_byte_count,
            worker_first_truth_digest,
            output_digest,
            boundary_performance,
            packet_digest,
            outputs,
        })
    }
}

impl WorkerOutputDeliveryCertificationPackage {
    pub(in crate::runtime::worker_host) fn from_worker_retained_packet(
        shell: &WorkerRuntimeShell,
    ) -> Result<Self, WORTHSignalJsError> {
        let packet = shell.latest_worker_output_delivery_packet()?;
        let worker_first_truth_digest = committed_truth_digest_for_runtime(&shell.core)?;
        if packet.worker_first_truth_digest != worker_first_truth_digest {
            return Err(WORTHSignalJsError::invalid_input(
                "worker output delivery certification requires current delivery evidence",
            ));
        }
        let certification_digest = canonical_worker_certification_digest(&(
            "workerOutputDeliveryCertification",
            packet.output_digest.as_str(),
            packet.boundary_performance.performance_digest.as_str(),
            packet.packet_digest.as_str(),
            worker_first_truth_digest.as_str(),
        ))?;

        Ok(Self {
            certification_family: "workerOutputDeliveryCertification",
            covered_suite_count: 1,
            output_delivery_packet_count: packet.output_delivery_packet_count,
            output_delivery_breadth: packet.output_delivery_breadth,
            output_payload_byte_count: packet.output_payload_byte_count,
            worker_first_truth_digest,
            output_digest: packet.output_digest.clone(),
            boundary_performance_digest: packet.boundary_performance.performance_digest.clone(),
            packet_digest: packet.packet_digest.clone(),
            certification_digest,
        })
    }
}

impl WorkerRuntimeShell {
    pub fn deliver_outputs(
        &mut self,
        request: WorkerOutputDeliveryRequest,
    ) -> Result<WorkerOutputDeliveryPacket, WORTHSignalJsError> {
        validate_output_delivery_request(&request)?;
        let outputs = request
            .output_ids
            .into_iter()
            .map(|id| self.deliver_output_value(id))
            .collect::<Result<Vec<_>, _>>()?;
        let packet = WorkerOutputDeliveryPacket::from_outputs(
            outputs,
            committed_truth_digest_for_runtime(&self.core)?,
        )?;
        self.latest_worker_output_delivery_packet = Some(packet.clone());
        Ok(packet)
    }

    pub fn certify_worker_output_delivery(
        &self,
    ) -> Result<WorkerOutputDeliveryCertificationPackage, WORTHSignalJsError> {
        WorkerOutputDeliveryCertificationPackage::from_worker_retained_packet(self)
    }

    pub(in crate::runtime::worker_host) fn latest_worker_output_delivery_packet(
        &self,
    ) -> Result<&WorkerOutputDeliveryPacket, WORTHSignalJsError> {
        self.latest_worker_output_delivery_packet
            .as_ref()
            .ok_or_else(|| {
                WORTHSignalJsError::invalid_input(
                    "worker output delivery certification requires delivery evidence",
                )
            })
    }

    fn deliver_output_value(
        &mut self,
        id: String,
    ) -> Result<WorkerDeliveredOutput, WORTHSignalJsError> {
        if !self.core.is_web_output_signal(&id) {
            return Err(WORTHSignalJsError::invalid_input(format!(
                "worker output delivery id `{id}` is not a published output"
            )));
        }
        let value = self.core.read_value(&id)?;
        let payload_byte_count = serde_json::to_vec(&value)
            .map_err(|error| {
                WORTHSignalJsError::internal(format!(
                    "failed to measure output delivery payload `{id}`: {error}"
                ))
            })?
            .len() as u64;
        Ok(WorkerDeliveredOutput {
            id,
            value,
            payload_byte_count,
        })
    }
}

fn validate_output_delivery_request(
    request: &WorkerOutputDeliveryRequest,
) -> Result<(), WORTHSignalJsError> {
    if request.output_ids.is_empty() {
        return Err(WORTHSignalJsError::invalid_input(
            "worker output delivery requires at least one output id",
        ));
    }
    let mut seen = BTreeSet::new();
    for id in &request.output_ids {
        if id.trim().is_empty() {
            return Err(WORTHSignalJsError::invalid_input(
                "worker output delivery rejects blank output ids",
            ));
        }
        if !seen.insert(id) {
            return Err(WORTHSignalJsError::invalid_input(format!(
                "worker output delivery rejects duplicate output id `{id}`"
            )));
        }
    }
    Ok(())
}
