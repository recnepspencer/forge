use serde::Serialize;

use crate::boundary::errors::WORTHSignalJsError;
use crate::runtime::summaries::ObservationSurfaceSummary;

use super::{
    canonical_worker_certification_digest, committed_truth_digest_for_runtime,
    WorkerHostBoundaryPerformanceEnvelope, WorkerRuntimeShell,
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerObservationDeliveryPacket {
    pub envelope_family: &'static str,
    pub delivery_mode: &'static str,
    pub runtime_authority: &'static str,
    pub observation_delivery_packet_count: u64,
    pub observation_delivery_breadth: u64,
    pub delivered_observation_count: u64,
    pub rollback_suppressed_delivery_count: u64,
    pub callback_node_count: u64,
    pub active_lifecycle_subscription_count: u64,
    pub worker_first_truth_digest: String,
    pub observation_digest: String,
    pub observation_lifecycle_digest: String,
    pub boundary_performance: WorkerHostBoundaryPerformanceEnvelope,
    pub packet_digest: String,
    pub observation: ObservationSurfaceSummary,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerObservationDeliveryCertificationPackage {
    pub certification_family: &'static str,
    pub covered_suite_count: u64,
    pub observation_delivery_packet_count: u64,
    pub observation_delivery_breadth: u64,
    pub delivered_observation_count: u64,
    pub rollback_suppressed_delivery_count: u64,
    pub active_lifecycle_subscription_count: u64,
    pub worker_first_truth_digest: String,
    pub observation_digest: String,
    pub observation_lifecycle_digest: String,
    pub boundary_performance_digest: String,
    pub packet_digest: String,
    pub certification_digest: String,
}

impl WorkerObservationDeliveryPacket {
    pub(in crate::runtime::worker_host) fn from_latest_observation(
        observation: ObservationSurfaceSummary,
        active_lifecycle_subscription_count: u64,
        observation_lifecycle_digest: String,
        worker_first_truth_digest: String,
    ) -> Result<Self, WORTHSignalJsError> {
        let observation_digest = canonical_worker_certification_digest(&observation)?;
        let observation_delivery_breadth = observation.observation.boundary_events.len() as u64;
        let delivered_observation_count = observation.observation.delivered_event_count as u64;
        let rollback_suppressed_delivery_count =
            observation.observation.rollback_suppressed_event_count as u64;
        let callback_node_count = observation.callback_nodes.len() as u64;
        let boundary_performance = WorkerHostBoundaryPerformanceEnvelope::observation_delivery(
            observation_delivery_breadth,
            observation_digest.as_str(),
        )?;
        let packet_digest = canonical_worker_certification_digest(&(
            "observationDelivery",
            "CommittedObservationDelivery",
            observation_delivery_breadth,
            delivered_observation_count,
            rollback_suppressed_delivery_count,
            callback_node_count,
            active_lifecycle_subscription_count,
            worker_first_truth_digest.as_str(),
            observation_digest.as_str(),
            observation_lifecycle_digest.as_str(),
            boundary_performance.performance_digest.as_str(),
        ))?;

        Ok(Self {
            envelope_family: "observationDelivery",
            delivery_mode: "CommittedObservationDelivery",
            runtime_authority: "workerOwnedRuntime",
            observation_delivery_packet_count: 1,
            observation_delivery_breadth,
            delivered_observation_count,
            rollback_suppressed_delivery_count,
            callback_node_count,
            active_lifecycle_subscription_count,
            worker_first_truth_digest,
            observation_digest,
            observation_lifecycle_digest,
            boundary_performance,
            packet_digest,
            observation,
        })
    }
}

impl WorkerObservationDeliveryCertificationPackage {
    pub(in crate::runtime::worker_host) fn from_worker_retained_packet(
        shell: &WorkerRuntimeShell,
    ) -> Result<Self, WORTHSignalJsError> {
        let packet = shell.latest_worker_observation_delivery_packet()?;
        if !shell.has_observation_delivery_subscription() {
            return Err(WORTHSignalJsError::invalid_input(
                "worker observation delivery certification requires an active lifecycle subscription",
            ));
        }
        let worker_first_truth_digest = committed_truth_digest_for_runtime(&shell.core)?;
        if packet.worker_first_truth_digest != worker_first_truth_digest {
            return Err(WORTHSignalJsError::invalid_input(
                "worker observation delivery certification requires current delivery evidence",
            ));
        }
        let active_lifecycle_subscription_count =
            shell.active_observation_delivery_subscription_count();
        let observation_lifecycle_digest = shell.active_observation_delivery_lifecycle_digest()?;
        if packet.active_lifecycle_subscription_count != active_lifecycle_subscription_count
            || packet.observation_lifecycle_digest != observation_lifecycle_digest
        {
            return Err(WORTHSignalJsError::invalid_input(
                "worker observation delivery certification requires current lifecycle evidence",
            ));
        }
        let certification_digest = canonical_worker_certification_digest(&(
            "workerObservationDeliveryCertification",
            packet.observation_digest.as_str(),
            packet.observation_lifecycle_digest.as_str(),
            packet.boundary_performance.performance_digest.as_str(),
            packet.packet_digest.as_str(),
            worker_first_truth_digest.as_str(),
        ))?;

        Ok(Self {
            certification_family: "workerObservationDeliveryCertification",
            covered_suite_count: 1,
            observation_delivery_packet_count: packet.observation_delivery_packet_count,
            observation_delivery_breadth: packet.observation_delivery_breadth,
            delivered_observation_count: packet.delivered_observation_count,
            rollback_suppressed_delivery_count: packet.rollback_suppressed_delivery_count,
            active_lifecycle_subscription_count,
            worker_first_truth_digest,
            observation_digest: packet.observation_digest.clone(),
            observation_lifecycle_digest,
            boundary_performance_digest: packet.boundary_performance.performance_digest.clone(),
            packet_digest: packet.packet_digest.clone(),
            certification_digest,
        })
    }
}

impl WorkerRuntimeShell {
    pub fn deliver_latest_observation(
        &mut self,
    ) -> Result<WorkerObservationDeliveryPacket, WORTHSignalJsError> {
        if !self.has_observation_delivery_subscription() {
            return Err(WORTHSignalJsError::invalid_input(
                "worker observation delivery requires an active lifecycle subscription",
            ));
        }
        let observation = self.core.latest_observation()?.ok_or_else(|| {
            WORTHSignalJsError::invalid_input(
                "worker observation delivery requires committed observation evidence",
            )
        })?;
        let packet = WorkerObservationDeliveryPacket::from_latest_observation(
            observation,
            self.active_observation_delivery_subscription_count(),
            self.active_observation_delivery_lifecycle_digest()?,
            committed_truth_digest_for_runtime(&self.core)?,
        )?;
        self.latest_worker_observation_delivery_packet = Some(packet.clone());
        Ok(packet)
    }

    pub fn certify_worker_observation_delivery(
        &self,
    ) -> Result<WorkerObservationDeliveryCertificationPackage, WORTHSignalJsError> {
        WorkerObservationDeliveryCertificationPackage::from_worker_retained_packet(self)
    }

    pub(in crate::runtime::worker_host) fn latest_worker_observation_delivery_packet(
        &self,
    ) -> Result<&WorkerObservationDeliveryPacket, WORTHSignalJsError> {
        self.latest_worker_observation_delivery_packet
            .as_ref()
            .ok_or_else(|| {
                WORTHSignalJsError::invalid_input(
                    "worker observation delivery certification requires delivery evidence",
                )
            })
    }
}
