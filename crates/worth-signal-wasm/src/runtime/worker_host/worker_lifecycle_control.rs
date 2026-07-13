use serde::{Deserialize, Serialize};
use worth_signal::facade::runtime::ObservationHandle;

use crate::boundary::errors::WorthSignalJsError;

use super::{
    canonical_worker_certification_digest, committed_truth_digest_for_runtime,
    WorkerHostBoundaryCausality, WorkerHostBoundaryPerformanceEnvelope, WorkerRuntimeShell,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerObservationDeliveryAttachRequest {
    pub signal_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerObservationDeliveryDetachRequest {
    pub lifecycle_subscription_id: u64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerLifecycleControlPacket {
    pub envelope_family: &'static str,
    pub lifecycle_event: &'static str,
    pub lifecycle_artifact: &'static str,
    pub runtime_authority: &'static str,
    pub lifecycle_subscription_id: u64,
    pub signal_id: Option<String>,
    pub observer_attached_count: u64,
    pub observer_detached_count: u64,
    pub detach_denial_count: u64,
    pub active_observer_count: u64,
    pub worker_first_truth_digest: String,
    pub lifecycle_digest: String,
    pub causality: WorkerHostBoundaryCausality,
    pub boundary_performance: WorkerHostBoundaryPerformanceEnvelope,
    pub packet_digest: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerLifecycleControlCertificationPackage {
    pub certification_family: &'static str,
    pub covered_suite_count: u64,
    pub lifecycle_event: &'static str,
    pub observer_attached_count: u64,
    pub observer_detached_count: u64,
    pub detach_denial_count: u64,
    pub active_observer_count: u64,
    pub worker_first_truth_digest: String,
    pub lifecycle_digest: String,
    pub boundary_performance_digest: String,
    pub packet_digest: String,
    pub certification_digest: String,
}

pub(in crate::runtime::worker_host) struct WorkerObservationDeliverySubscription {
    pub signal_id: String,
    pub handle: ObservationHandle,
}

impl WorkerLifecycleControlPacket {
    pub(in crate::runtime::worker_host) fn attached(
        lifecycle_subscription_id: u64,
        signal_id: String,
        active_observer_count: u64,
        worker_first_truth_digest: String,
        causality: WorkerHostBoundaryCausality,
    ) -> Result<Self, WorthSignalJsError> {
        Self::from_lifecycle_event(WorkerLifecycleEventEvidence {
            lifecycle_event: "ObserverAttached",
            lifecycle_subscription_id,
            signal_id: Some(signal_id),
            observer_attached_count: 1,
            observer_detached_count: 0,
            detach_denial_count: 0,
            active_observer_count,
            runtime_admitted_item_count: 1,
            worker_first_truth_digest,
            causality,
        })
    }

    pub(in crate::runtime::worker_host) fn detached(
        lifecycle_subscription_id: u64,
        signal_id: String,
        active_observer_count: u64,
        worker_first_truth_digest: String,
        causality: WorkerHostBoundaryCausality,
    ) -> Result<Self, WorthSignalJsError> {
        Self::from_lifecycle_event(WorkerLifecycleEventEvidence {
            lifecycle_event: "ObserverDetached",
            lifecycle_subscription_id,
            signal_id: Some(signal_id),
            observer_attached_count: 0,
            observer_detached_count: 1,
            detach_denial_count: 0,
            active_observer_count,
            runtime_admitted_item_count: 1,
            worker_first_truth_digest,
            causality,
        })
    }

    pub(in crate::runtime::worker_host) fn detach_denied(
        lifecycle_subscription_id: u64,
        active_observer_count: u64,
        worker_first_truth_digest: String,
        causality: WorkerHostBoundaryCausality,
    ) -> Result<Self, WorthSignalJsError> {
        Self::from_lifecycle_event(WorkerLifecycleEventEvidence {
            lifecycle_event: "ObserverDetachDenied",
            lifecycle_subscription_id,
            signal_id: None,
            observer_attached_count: 0,
            observer_detached_count: 0,
            detach_denial_count: 1,
            active_observer_count,
            runtime_admitted_item_count: 0,
            worker_first_truth_digest,
            causality,
        })
    }

    fn from_lifecycle_event(
        evidence: WorkerLifecycleEventEvidence,
    ) -> Result<Self, WorthSignalJsError> {
        let lifecycle_digest = canonical_worker_certification_digest(&evidence)?;
        let boundary_performance = WorkerHostBoundaryPerformanceEnvelope::lifecycle_control(
            lifecycle_digest.as_str(),
            evidence.runtime_admitted_item_count,
        )?;
        let packet_digest = canonical_worker_certification_digest(&(
            "lifecycleControl",
            evidence.lifecycle_event,
            lifecycle_digest.as_str(),
            boundary_performance.performance_digest.as_str(),
            evidence.worker_first_truth_digest.as_str(),
        ))?;

        Ok(Self {
            envelope_family: "lifecycleControl",
            lifecycle_event: evidence.lifecycle_event,
            lifecycle_artifact: "observationDeliverySubscription",
            runtime_authority: "workerOwnedRuntime",
            lifecycle_subscription_id: evidence.lifecycle_subscription_id,
            signal_id: evidence.signal_id,
            observer_attached_count: evidence.observer_attached_count,
            observer_detached_count: evidence.observer_detached_count,
            detach_denial_count: evidence.detach_denial_count,
            active_observer_count: evidence.active_observer_count,
            worker_first_truth_digest: evidence.worker_first_truth_digest,
            lifecycle_digest,
            causality: evidence.causality,
            boundary_performance,
            packet_digest,
        })
    }
}

impl WorkerLifecycleControlCertificationPackage {
    pub(in crate::runtime::worker_host) fn from_worker_retained_packet(
        shell: &WorkerRuntimeShell,
    ) -> Result<Self, WorthSignalJsError> {
        let packet = shell.latest_worker_lifecycle_control_packet()?;
        let worker_first_truth_digest = committed_truth_digest_for_runtime(&shell.core)?;
        if packet.worker_first_truth_digest != worker_first_truth_digest {
            return Err(WorthSignalJsError::invalid_input(
                "worker lifecycle control certification requires current lifecycle evidence",
            ));
        }
        let certification_digest = canonical_worker_certification_digest(&(
            "workerLifecycleControlCertification",
            packet.lifecycle_event,
            packet.lifecycle_digest.as_str(),
            packet.boundary_performance.performance_digest.as_str(),
            packet.packet_digest.as_str(),
            worker_first_truth_digest.as_str(),
        ))?;

        Ok(Self {
            certification_family: "workerLifecycleControlCertification",
            covered_suite_count: 1,
            lifecycle_event: packet.lifecycle_event,
            observer_attached_count: packet.observer_attached_count,
            observer_detached_count: packet.observer_detached_count,
            detach_denial_count: packet.detach_denial_count,
            active_observer_count: packet.active_observer_count,
            worker_first_truth_digest,
            lifecycle_digest: packet.lifecycle_digest.clone(),
            boundary_performance_digest: packet.boundary_performance.performance_digest.clone(),
            packet_digest: packet.packet_digest.clone(),
            certification_digest,
        })
    }
}

impl WorkerRuntimeShell {
    pub fn attach_observation_delivery(
        &mut self,
        request: WorkerObservationDeliveryAttachRequest,
    ) -> Result<WorkerLifecycleControlPacket, WorthSignalJsError> {
        validate_observation_delivery_attach_request(&request)?;
        if !self.core.is_web_output_signal(&request.signal_id) {
            return Err(WorthSignalJsError::invalid_input(format!(
                "worker lifecycle observation id `{}` is not a published output",
                request.signal_id
            )));
        }
        let handle = self
            .core
            .observe_signal_for_runtime_certification(&request.signal_id)?;
        let lifecycle_subscription_id = self.next_worker_lifecycle_subscription_id;
        self.next_worker_lifecycle_subscription_id =
            self.next_worker_lifecycle_subscription_id.saturating_add(1);
        self.worker_observation_delivery_subscriptions.insert(
            lifecycle_subscription_id,
            WorkerObservationDeliverySubscription {
                signal_id: request.signal_id.clone(),
                handle,
            },
        );
        let packet = WorkerLifecycleControlPacket::attached(
            lifecycle_subscription_id,
            request.signal_id,
            self.worker_observation_delivery_subscriptions.len() as u64,
            committed_truth_digest_for_runtime(&self.core)?,
            self.next_host_boundary_causality(),
        )?;
        self.latest_worker_lifecycle_control_packet = Some(packet.clone());
        Ok(packet)
    }

    pub fn detach_observation_delivery(
        &mut self,
        request: WorkerObservationDeliveryDetachRequest,
    ) -> Result<WorkerLifecycleControlPacket, WorthSignalJsError> {
        let packet = match self
            .worker_observation_delivery_subscriptions
            .remove(&request.lifecycle_subscription_id)
        {
            Some(subscription) => {
                let _ = self.core.unobserve_handle(subscription.handle);
                WorkerLifecycleControlPacket::detached(
                    request.lifecycle_subscription_id,
                    subscription.signal_id,
                    self.worker_observation_delivery_subscriptions.len() as u64,
                    committed_truth_digest_for_runtime(&self.core)?,
                    self.next_host_boundary_causality(),
                )?
            }
            None => WorkerLifecycleControlPacket::detach_denied(
                request.lifecycle_subscription_id,
                self.worker_observation_delivery_subscriptions.len() as u64,
                committed_truth_digest_for_runtime(&self.core)?,
                self.next_host_boundary_causality(),
            )?,
        };
        self.latest_worker_lifecycle_control_packet = Some(packet.clone());
        Ok(packet)
    }

    pub fn certify_worker_lifecycle_control(
        &self,
    ) -> Result<WorkerLifecycleControlCertificationPackage, WorthSignalJsError> {
        WorkerLifecycleControlCertificationPackage::from_worker_retained_packet(self)
    }

    pub(in crate::runtime::worker_host) fn has_observation_delivery_subscription(&self) -> bool {
        !self.worker_observation_delivery_subscriptions.is_empty()
    }

    pub(in crate::runtime::worker_host) fn active_observation_delivery_subscription_count(
        &self,
    ) -> u64 {
        self.worker_observation_delivery_subscriptions.len() as u64
    }

    pub(in crate::runtime::worker_host) fn active_observation_delivery_lifecycle_digest(
        &self,
    ) -> Result<String, WorthSignalJsError> {
        let subscriptions = self
            .worker_observation_delivery_subscriptions
            .iter()
            .map(|(lifecycle_subscription_id, subscription)| {
                ActiveObservationDeliverySubscription {
                    lifecycle_subscription_id: *lifecycle_subscription_id,
                    signal_id: subscription.signal_id.as_str(),
                }
            })
            .collect::<Vec<_>>();
        canonical_worker_certification_digest(&ActiveObservationDeliveryLifecycle {
            lifecycle_artifact: "observationDeliverySubscription",
            subscriptions,
        })
    }

    pub(in crate::runtime::worker_host) fn latest_worker_lifecycle_control_packet(
        &self,
    ) -> Result<&WorkerLifecycleControlPacket, WorthSignalJsError> {
        self.latest_worker_lifecycle_control_packet
            .as_ref()
            .ok_or_else(|| {
                WorthSignalJsError::invalid_input(
                    "worker lifecycle control certification requires lifecycle evidence",
                )
            })
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ActiveObservationDeliveryLifecycle<'a> {
    lifecycle_artifact: &'static str,
    subscriptions: Vec<ActiveObservationDeliverySubscription<'a>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ActiveObservationDeliverySubscription<'a> {
    lifecycle_subscription_id: u64,
    signal_id: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct WorkerLifecycleEventEvidence {
    lifecycle_event: &'static str,
    lifecycle_subscription_id: u64,
    signal_id: Option<String>,
    observer_attached_count: u64,
    observer_detached_count: u64,
    detach_denial_count: u64,
    active_observer_count: u64,
    runtime_admitted_item_count: u64,
    worker_first_truth_digest: String,
    causality: WorkerHostBoundaryCausality,
}

fn validate_observation_delivery_attach_request(
    request: &WorkerObservationDeliveryAttachRequest,
) -> Result<(), WorthSignalJsError> {
    if request.signal_id.trim().is_empty() {
        return Err(WorthSignalJsError::invalid_input(
            "worker lifecycle observation attachment requires a signal id",
        ));
    }
    Ok(())
}
