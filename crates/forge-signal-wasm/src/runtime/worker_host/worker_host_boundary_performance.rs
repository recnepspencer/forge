use serde::Serialize;

use crate::boundary::errors::ForgeSignalJsError;

use super::canonical_worker_certification_digest;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerHostBoundaryPerformanceEnvelope {
    pub bridge_envelope_count: u64,
    pub submitted_item_count: u64,
    pub coalesced_item_count: u64,
    pub runtime_admitted_item_count: u64,
    pub runtime_mutation_breadth: u32,
    pub ambient_worker_read_count: u64,
    pub payload_identity_byte_count: u64,
    pub performance_digest: String,
}

impl WorkerHostBoundaryPerformanceEnvelope {
    pub(in crate::runtime::worker_host) fn host_capability_ingress(
        submitted_item_count: u64,
        coalesced_item_count: u64,
        runtime_admitted_item_count: u64,
        runtime_mutation_breadth: u32,
    ) -> Result<Self, ForgeSignalJsError> {
        Self::from_boundary_counts(WorkerHostBoundaryPerformanceCounts {
            boundary_family: "hostCapabilityIngress",
            bridge_envelope_count: 1,
            submitted_item_count,
            coalesced_item_count,
            runtime_admitted_item_count,
            runtime_mutation_breadth,
            ambient_worker_read_count: 0,
            payload_identity_byte_count: 0,
        })
    }

    pub(in crate::runtime::worker_host) fn browser_history_ingress(
        raw_location: &str,
        route_identity: &str,
        runtime_admitted_item_count: u64,
        runtime_mutation_breadth: u32,
    ) -> Result<Self, ForgeSignalJsError> {
        Self::from_boundary_counts(WorkerHostBoundaryPerformanceCounts {
            boundary_family: "browserHistoryIngress",
            bridge_envelope_count: 1,
            submitted_item_count: 1,
            coalesced_item_count: 1,
            runtime_admitted_item_count,
            runtime_mutation_breadth,
            ambient_worker_read_count: 0,
            payload_identity_byte_count: payload_identity_byte_count([
                raw_location,
                route_identity,
            ]),
        })
    }

    pub(in crate::runtime::worker_host) fn host_effect_request(
        closed_payload_identity: &str,
    ) -> Result<Self, ForgeSignalJsError> {
        Self::from_boundary_counts(WorkerHostBoundaryPerformanceCounts {
            boundary_family: "hostEffectRequest",
            bridge_envelope_count: 1,
            submitted_item_count: 1,
            coalesced_item_count: 1,
            runtime_admitted_item_count: 0,
            runtime_mutation_breadth: 0,
            ambient_worker_read_count: 0,
            payload_identity_byte_count: payload_identity_byte_count([closed_payload_identity]),
        })
    }

    pub(in crate::runtime::worker_host) fn host_effect_acknowledgement(
        artifact_identity: &str,
        runtime_admitted_item_count: u64,
        runtime_mutation_breadth: u32,
    ) -> Result<Self, ForgeSignalJsError> {
        Self::from_boundary_counts(WorkerHostBoundaryPerformanceCounts {
            boundary_family: "hostEffectAcknowledgement",
            bridge_envelope_count: 1,
            submitted_item_count: 1,
            coalesced_item_count: 1,
            runtime_admitted_item_count,
            runtime_mutation_breadth,
            ambient_worker_read_count: 0,
            payload_identity_byte_count: payload_identity_byte_count([artifact_identity]),
        })
    }

    fn from_boundary_counts(
        counts: WorkerHostBoundaryPerformanceCounts,
    ) -> Result<Self, ForgeSignalJsError> {
        Ok(Self {
            bridge_envelope_count: counts.bridge_envelope_count,
            submitted_item_count: counts.submitted_item_count,
            coalesced_item_count: counts.coalesced_item_count,
            runtime_admitted_item_count: counts.runtime_admitted_item_count,
            runtime_mutation_breadth: counts.runtime_mutation_breadth,
            ambient_worker_read_count: counts.ambient_worker_read_count,
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
    payload_identity_byte_count: u64,
}

fn payload_identity_byte_count<'a>(identities: impl IntoIterator<Item = &'a str>) -> u64 {
    identities.into_iter().fold(0_u64, |byte_count, identity| {
        byte_count.saturating_add(identity.len() as u64)
    })
}
