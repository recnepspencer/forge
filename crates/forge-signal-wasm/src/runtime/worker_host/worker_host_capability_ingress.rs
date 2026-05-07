use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::boundary::errors::ForgeSignalJsError;
use crate::expression::model::SignalValue;
use crate::recipe::model::SetValue;

use super::{
    canonical_worker_certification_digest, WorkerHostBoundaryCausality,
    WorkerHostBoundaryPerformanceEnvelope,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerHostCapabilityUpdate {
    pub family: String,
    pub registration_id: String,
    pub semantic_value_identity: String,
    #[serde(default)]
    pub boundary_artifact: WorkerHostCapabilityBoundaryArtifact,
    #[serde(default)]
    pub runtime_source_id: Option<String>,
    #[serde(default)]
    pub runtime_value: Option<SignalValue>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WorkerHostCapabilityBoundaryArtifact {
    #[default]
    Admitted,
    Stale,
    Denied,
    Detached,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerHostCapabilityIngressBatch {
    pub updates: Vec<WorkerHostCapabilityUpdate>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerHostCapabilityIngressReport {
    pub envelope_family: &'static str,
    pub causality: WorkerHostBoundaryCausality,
    pub submitted_update_count: u64,
    pub submitted_admitted_update_count: u64,
    pub submitted_stale_update_count: u64,
    pub submitted_denied_update_count: u64,
    pub submitted_detached_update_count: u64,
    pub submitted_unavailable_update_count: u64,
    pub coalesced_admitted_update_count: u64,
    pub coalesced_update_count: u64,
    pub coalesced_stale_update_count: u64,
    pub coalesced_denied_update_count: u64,
    pub coalesced_detached_update_count: u64,
    pub coalesced_unavailable_update_count: u64,
    pub runtime_admitted_update_count: u64,
    pub runtime_mutation_breadth: u32,
    pub performance: WorkerHostBoundaryPerformanceEnvelope,
    pub host_capability_envelope_digest: String,
    pub lifecycle_digest: String,
    pub truth_digest: String,
    pub worker_first_truth_digest: String,
    pub coalescing_digest: String,
    pub host_boundary_artifact_digest: String,
    pub ambient_worker_read_denied: bool,
}

impl WorkerHostCapabilityIngressReport {
    pub(in crate::runtime::worker_host) fn from_coalesced_updates(
        coalesced_updates: Vec<WorkerHostCapabilityUpdate>,
        submitted_artifact_counts: HostCapabilityArtifactCounts,
        causality: WorkerHostBoundaryCausality,
        worker_first_truth_digest: String,
        runtime_mutation_breadth: u32,
    ) -> Result<Self, ForgeSignalJsError> {
        let coalesced_artifact_counts =
            HostCapabilityArtifactCounts::from_updates(&coalesced_updates);
        let submitted_update_count = submitted_artifact_counts.total_update_count();
        let runtime_admitted_update_count = coalesced_updates
            .iter()
            .filter(|update| update.runtime_source_id.is_some() && update.runtime_value.is_some())
            .count() as u64;
        let performance = WorkerHostBoundaryPerformanceEnvelope::host_capability_ingress(
            submitted_update_count,
            coalesced_updates.len() as u64,
            runtime_admitted_update_count,
            runtime_mutation_breadth,
        )?;

        Ok(Self {
            envelope_family: "hostCapabilityIngress",
            causality,
            submitted_update_count,
            submitted_admitted_update_count: submitted_artifact_counts.admitted_update_count,
            submitted_stale_update_count: submitted_artifact_counts.stale_update_count,
            submitted_denied_update_count: submitted_artifact_counts.denied_update_count,
            submitted_detached_update_count: submitted_artifact_counts.detached_update_count,
            submitted_unavailable_update_count: submitted_artifact_counts.unavailable_update_count,
            coalesced_admitted_update_count: coalesced_artifact_counts.admitted_update_count,
            coalesced_update_count: coalesced_updates.len() as u64,
            coalesced_stale_update_count: coalesced_artifact_counts.stale_update_count,
            coalesced_denied_update_count: coalesced_artifact_counts.denied_update_count,
            coalesced_detached_update_count: coalesced_artifact_counts.detached_update_count,
            coalesced_unavailable_update_count: coalesced_artifact_counts.unavailable_update_count,
            runtime_admitted_update_count,
            runtime_mutation_breadth,
            performance,
            host_capability_envelope_digest: canonical_worker_certification_digest(
                &coalesced_updates,
            )?,
            lifecycle_digest: canonical_worker_certification_digest(&(
                "hostCapabilityIngressLifecycle",
                causality,
            ))?,
            truth_digest: canonical_worker_certification_digest(&(
                "hostCapabilityIngressTruth",
                &coalesced_updates,
            ))?,
            worker_first_truth_digest,
            coalescing_digest: canonical_worker_certification_digest(&(
                "lastUpdatePerCapabilityRegistration",
                &coalesced_updates,
            ))?,
            host_boundary_artifact_digest: canonical_worker_certification_digest(&(
                "hostCapabilityBoundaryArtifacts",
                submitted_artifact_counts,
                coalesced_artifact_counts,
                &coalesced_updates,
            ))?,
            ambient_worker_read_denied: true,
        })
    }
}

pub(in crate::runtime::worker_host) fn host_capability_artifact_counts(
    updates: &[WorkerHostCapabilityUpdate],
) -> HostCapabilityArtifactCounts {
    HostCapabilityArtifactCounts::from_updates(updates)
}

pub(in crate::runtime::worker_host) fn runtime_values_for_host_capability_admission(
    updates: &[WorkerHostCapabilityUpdate],
) -> Result<Vec<SetValue>, ForgeSignalJsError> {
    let mut runtime_values = Vec::with_capacity(updates.len());
    for update in updates {
        validate_host_capability_update_runtime_admission(update)?;
        if let (Some(runtime_source_id), Some(runtime_value)) =
            (&update.runtime_source_id, &update.runtime_value)
        {
            runtime_values.push(SetValue {
                id: runtime_source_id.clone(),
                value: runtime_value.clone(),
                aspect: None,
                aspects: None,
            });
        }
    }
    Ok(runtime_values)
}

pub(in crate::runtime::worker_host) fn reject_malformed_host_capability_updates(
    updates: &[WorkerHostCapabilityUpdate],
) -> Result<(), ForgeSignalJsError> {
    for update in updates {
        validate_host_capability_update_runtime_admission(update)?;
    }
    Ok(())
}

fn validate_host_capability_update_runtime_admission(
    update: &WorkerHostCapabilityUpdate,
) -> Result<(), ForgeSignalJsError> {
    if !update.boundary_artifact.allows_runtime_admission()
        && (update.runtime_source_id.is_some() || update.runtime_value.is_some())
    {
        return Err(ForgeSignalJsError::invalid_input(
            "non-admitted host capability artifacts cannot mutate worker runtime truth",
        ));
    }

    match (&update.runtime_source_id, &update.runtime_value) {
        (Some(_), Some(_)) | (None, None) => Ok(()),
        _ => Err(ForgeSignalJsError::invalid_input(
            "host capability runtime admission requires a paired runtime source id with runtime value",
        )),
    }
}

pub(in crate::runtime::worker_host) fn coalesce_host_capability_updates(
    updates: Vec<WorkerHostCapabilityUpdate>,
) -> Vec<WorkerHostCapabilityUpdate> {
    let mut coalesced_by_registration = BTreeMap::new();
    for update in updates {
        coalesced_by_registration.insert(
            (update.family.clone(), update.registration_id.clone()),
            update,
        );
    }
    coalesced_by_registration.into_values().collect()
}

impl WorkerHostCapabilityBoundaryArtifact {
    fn allows_runtime_admission(self) -> bool {
        matches!(self, Self::Admitted)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::runtime::worker_host) struct HostCapabilityArtifactCounts {
    admitted_update_count: u64,
    stale_update_count: u64,
    denied_update_count: u64,
    detached_update_count: u64,
    unavailable_update_count: u64,
}

impl HostCapabilityArtifactCounts {
    fn from_updates(updates: &[WorkerHostCapabilityUpdate]) -> Self {
        let mut counts = Self {
            admitted_update_count: 0,
            stale_update_count: 0,
            denied_update_count: 0,
            detached_update_count: 0,
            unavailable_update_count: 0,
        };

        for update in updates {
            match update.boundary_artifact {
                WorkerHostCapabilityBoundaryArtifact::Admitted => {
                    counts.admitted_update_count += 1;
                }
                WorkerHostCapabilityBoundaryArtifact::Stale => {
                    counts.stale_update_count += 1;
                }
                WorkerHostCapabilityBoundaryArtifact::Denied => {
                    counts.denied_update_count += 1;
                }
                WorkerHostCapabilityBoundaryArtifact::Detached => {
                    counts.detached_update_count += 1;
                }
                WorkerHostCapabilityBoundaryArtifact::Unavailable => {
                    counts.unavailable_update_count += 1;
                }
            }
        }

        counts
    }

    fn total_update_count(self) -> u64 {
        self.admitted_update_count
            + self.stale_update_count
            + self.denied_update_count
            + self.detached_update_count
            + self.unavailable_update_count
    }
}
