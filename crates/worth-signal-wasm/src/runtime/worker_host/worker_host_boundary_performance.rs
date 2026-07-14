use serde::{Deserialize, Serialize};

use crate::boundary::errors::WorthSignalJsError;

use super::canonical_worker_certification_digest;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerHostBoundaryPerformanceEnvelope {
    pub bridge_envelope_count: u64,
    pub submitted_item_count: u64,
    pub coalesced_item_count: u64,
    pub runtime_admitted_item_count: u64,
    pub runtime_mutation_breadth: u32,
    pub ambient_worker_read_count: u64,
    pub diagnostics_cold_reconstruction_count: u64,
    pub payload_identity_byte_count: u64,
    pub performance_digest: String,
}

impl WorkerHostBoundaryPerformanceEnvelope {
    pub(in crate::runtime::worker_host) fn host_capability_ingress(
        submitted_item_count: u64,
        coalesced_item_count: u64,
        runtime_admitted_item_count: u64,
        runtime_mutation_breadth: u32,
    ) -> Result<Self, WorthSignalJsError> {
        Self::from_boundary_counts(WorkerHostBoundaryPerformanceCounts {
            boundary_family: "hostCapabilityIngress",
            bridge_envelope_count: 1,
            submitted_item_count,
            coalesced_item_count,
            runtime_admitted_item_count,
            runtime_mutation_breadth,
            ambient_worker_read_count: 0,
            diagnostics_cold_reconstruction_count: 0,
            payload_identity_byte_count: 0,
        })
    }

    pub(in crate::runtime::worker_host) fn browser_history_ingress(
        raw_location: &str,
        route_identity: &str,
        runtime_admitted_item_count: u64,
        runtime_mutation_breadth: u32,
    ) -> Result<Self, WorthSignalJsError> {
        Self::from_boundary_counts(WorkerHostBoundaryPerformanceCounts {
            boundary_family: "browserHistoryIngress",
            bridge_envelope_count: 1,
            submitted_item_count: 1,
            coalesced_item_count: 1,
            runtime_admitted_item_count,
            runtime_mutation_breadth,
            ambient_worker_read_count: 0,
            diagnostics_cold_reconstruction_count: 0,
            payload_identity_byte_count: payload_identity_byte_count([
                raw_location,
                route_identity,
            ]),
        })
    }

    pub(in crate::runtime::worker_host) fn host_effect_request(
        closed_payload_identity: &str,
    ) -> Result<Self, WorthSignalJsError> {
        Self::from_boundary_counts(WorkerHostBoundaryPerformanceCounts {
            boundary_family: "hostEffectRequest",
            bridge_envelope_count: 1,
            submitted_item_count: 1,
            coalesced_item_count: 1,
            runtime_admitted_item_count: 0,
            runtime_mutation_breadth: 0,
            ambient_worker_read_count: 0,
            diagnostics_cold_reconstruction_count: 0,
            payload_identity_byte_count: payload_identity_byte_count([closed_payload_identity]),
        })
    }

    pub(in crate::runtime::worker_host) fn host_effect_acknowledgement(
        artifact_identity: &str,
        runtime_admitted_item_count: u64,
        runtime_mutation_breadth: u32,
    ) -> Result<Self, WorthSignalJsError> {
        Self::from_boundary_counts(WorkerHostBoundaryPerformanceCounts {
            boundary_family: "hostEffectAcknowledgement",
            bridge_envelope_count: 1,
            submitted_item_count: 1,
            coalesced_item_count: 1,
            runtime_admitted_item_count,
            runtime_mutation_breadth,
            ambient_worker_read_count: 0,
            diagnostics_cold_reconstruction_count: 0,
            payload_identity_byte_count: payload_identity_byte_count([artifact_identity]),
        })
    }

    pub(in crate::runtime::worker_host) fn main_thread_hosted_callback_request(
        closed_input_count: u64,
        closed_payload_identity: &str,
    ) -> Result<Self, WorthSignalJsError> {
        Self::from_boundary_counts(WorkerHostBoundaryPerformanceCounts {
            boundary_family: "mainThreadHostedCallbackRequest",
            bridge_envelope_count: 1,
            submitted_item_count: closed_input_count,
            coalesced_item_count: closed_input_count,
            runtime_admitted_item_count: 0,
            runtime_mutation_breadth: 0,
            ambient_worker_read_count: 0,
            diagnostics_cold_reconstruction_count: 0,
            payload_identity_byte_count: payload_identity_byte_count([closed_payload_identity]),
        })
    }

    pub(in crate::runtime::worker_host) fn main_thread_hosted_callback_result(
        artifact_identity: &str,
        runtime_admitted_item_count: u64,
        runtime_mutation_breadth: u32,
    ) -> Result<Self, WorthSignalJsError> {
        Self::from_boundary_counts(WorkerHostBoundaryPerformanceCounts {
            boundary_family: "mainThreadHostedCallbackResult",
            bridge_envelope_count: 1,
            submitted_item_count: 1,
            coalesced_item_count: 1,
            runtime_admitted_item_count,
            runtime_mutation_breadth,
            ambient_worker_read_count: 0,
            diagnostics_cold_reconstruction_count: 0,
            payload_identity_byte_count: payload_identity_byte_count([artifact_identity]),
        })
    }

    pub(in crate::runtime::worker_host) fn observation_delivery(
        observation_delivery_breadth: u64,
        observation_digest: &str,
    ) -> Result<Self, WorthSignalJsError> {
        Self::from_boundary_counts(WorkerHostBoundaryPerformanceCounts {
            boundary_family: "observationDelivery",
            bridge_envelope_count: 1,
            submitted_item_count: observation_delivery_breadth,
            coalesced_item_count: observation_delivery_breadth,
            runtime_admitted_item_count: 0,
            runtime_mutation_breadth: 0,
            ambient_worker_read_count: 0,
            diagnostics_cold_reconstruction_count: 0,
            payload_identity_byte_count: payload_identity_byte_count([observation_digest]),
        })
    }

    pub(in crate::runtime::worker_host) fn output_delivery(
        output_delivery_breadth: u64,
        output_payload_byte_count: u64,
        output_digest: &str,
    ) -> Result<Self, WorthSignalJsError> {
        Self::from_boundary_counts(WorkerHostBoundaryPerformanceCounts {
            boundary_family: "outputDelivery",
            bridge_envelope_count: 1,
            submitted_item_count: output_delivery_breadth,
            coalesced_item_count: output_delivery_breadth,
            runtime_admitted_item_count: 0,
            runtime_mutation_breadth: 0,
            ambient_worker_read_count: 0,
            diagnostics_cold_reconstruction_count: 0,
            payload_identity_byte_count: output_payload_byte_count
                .saturating_add(payload_identity_byte_count([output_digest])),
        })
    }

    pub(in crate::runtime::worker_host) fn signal_readback(
        signal_readback_breadth: u64,
        signal_payload_byte_count: u64,
        signal_digest: &str,
    ) -> Result<Self, WorthSignalJsError> {
        Self::from_boundary_counts(WorkerHostBoundaryPerformanceCounts {
            boundary_family: "signalReadback",
            bridge_envelope_count: 1,
            submitted_item_count: signal_readback_breadth,
            coalesced_item_count: signal_readback_breadth,
            runtime_admitted_item_count: 0,
            runtime_mutation_breadth: 0,
            ambient_worker_read_count: 0,
            diagnostics_cold_reconstruction_count: 0,
            payload_identity_byte_count: signal_payload_byte_count
                .saturating_add(payload_identity_byte_count([signal_digest])),
        })
    }

    pub(in crate::runtime::worker_host) fn diagnostics_summary_read(
        summary_digest: &str,
        rich_read_availability_digest: &str,
    ) -> Result<Self, WorthSignalJsError> {
        Self::from_boundary_counts(WorkerHostBoundaryPerformanceCounts {
            boundary_family: "diagnosticsSummaryRead",
            bridge_envelope_count: 1,
            submitted_item_count: 1,
            coalesced_item_count: 1,
            runtime_admitted_item_count: 0,
            runtime_mutation_breadth: 0,
            ambient_worker_read_count: 0,
            diagnostics_cold_reconstruction_count: 0,
            payload_identity_byte_count: payload_identity_byte_count([
                summary_digest,
                rich_read_availability_digest,
            ]),
        })
    }

    pub(in crate::runtime::worker_host) fn diagnostics_rich_history_read(
        history_payload_byte_count: u64,
        history_digest: &str,
        diagnostics_cold_reconstruction_count: u64,
    ) -> Result<Self, WorthSignalJsError> {
        Self::from_boundary_counts(WorkerHostBoundaryPerformanceCounts {
            boundary_family: "diagnosticsRichHistoryRead",
            bridge_envelope_count: 1,
            submitted_item_count: 1,
            coalesced_item_count: 1,
            runtime_admitted_item_count: 0,
            runtime_mutation_breadth: 0,
            ambient_worker_read_count: 0,
            diagnostics_cold_reconstruction_count,
            payload_identity_byte_count: history_payload_byte_count
                .saturating_add(payload_identity_byte_count([history_digest])),
        })
    }

    pub(in crate::runtime::worker_host) fn lifecycle_control(
        lifecycle_payload_identity: &str,
        runtime_admitted_item_count: u64,
    ) -> Result<Self, WorthSignalJsError> {
        Self::from_boundary_counts(WorkerHostBoundaryPerformanceCounts {
            boundary_family: "lifecycleControl",
            bridge_envelope_count: 1,
            submitted_item_count: 1,
            coalesced_item_count: 1,
            runtime_admitted_item_count,
            runtime_mutation_breadth: 0,
            ambient_worker_read_count: 0,
            diagnostics_cold_reconstruction_count: 0,
            payload_identity_byte_count: payload_identity_byte_count([lifecycle_payload_identity]),
        })
    }

    fn from_boundary_counts(
        counts: WorkerHostBoundaryPerformanceCounts,
    ) -> Result<Self, WorthSignalJsError> {
        Ok(Self {
            bridge_envelope_count: counts.bridge_envelope_count,
            submitted_item_count: counts.submitted_item_count,
            coalesced_item_count: counts.coalesced_item_count,
            runtime_admitted_item_count: counts.runtime_admitted_item_count,
            runtime_mutation_breadth: counts.runtime_mutation_breadth,
            ambient_worker_read_count: counts.ambient_worker_read_count,
            diagnostics_cold_reconstruction_count: counts.diagnostics_cold_reconstruction_count,
            payload_identity_byte_count: counts.payload_identity_byte_count,
            performance_digest: canonical_worker_certification_digest(&counts)?,
        })
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct WorkerHostBoundaryPerformanceCounts {
    boundary_family: &'static str,
    bridge_envelope_count: u64,
    submitted_item_count: u64,
    coalesced_item_count: u64,
    runtime_admitted_item_count: u64,
    runtime_mutation_breadth: u32,
    ambient_worker_read_count: u64,
    diagnostics_cold_reconstruction_count: u64,
    payload_identity_byte_count: u64,
}

fn payload_identity_byte_count<'a>(identities: impl IntoIterator<Item = &'a str>) -> u64 {
    identities.into_iter().fold(0_u64, |byte_count, identity| {
        byte_count.saturating_add(identity.len() as u64)
    })
}
