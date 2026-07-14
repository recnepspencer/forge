use serde::{Deserialize, Serialize};

use crate::boundary::errors::WorthSignalJsError;
use crate::expression::model::SignalValue;
use crate::recipe::model::SetValue;

use super::{
    canonical_worker_certification_digest, WorkerHostBoundaryCausality,
    WorkerHostBoundaryPerformanceEnvelope,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerBrowserHistoryIngress {
    pub navigation_kind: String,
    pub raw_location: String,
    pub route_identity: String,
    #[serde(default)]
    pub runtime_route_source_id: Option<String>,
    #[serde(default)]
    pub route_value: Option<SignalValue>,
    #[serde(default)]
    pub runtime_continuity_source_id: Option<String>,
    #[serde(default)]
    pub continuity_value: Option<SignalValue>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerBrowserHistoryIngressReport {
    pub envelope_family: &'static str,
    pub causality: WorkerHostBoundaryCausality,
    pub browser_history_envelope_digest: String,
    pub route_truth_digest: String,
    pub continuity_digest: String,
    pub replay_restore_digest: String,
    pub runtime_admitted_route_count: u64,
    pub runtime_admitted_continuity_count: u64,
    pub runtime_mutation_breadth: u32,
    pub worker_first_truth_digest: String,
    pub performance: WorkerHostBoundaryPerformanceEnvelope,
    pub ambient_location_read_denied: bool,
}

impl WorkerBrowserHistoryIngressReport {
    pub(in crate::runtime::worker_host) fn from_ingress(
        ingress: WorkerBrowserHistoryIngress,
        causality: WorkerHostBoundaryCausality,
        runtime_admitted_route_count: u64,
        runtime_admitted_continuity_count: u64,
        runtime_mutation_breadth: u32,
        worker_first_truth_digest: String,
    ) -> Result<Self, WorthSignalJsError> {
        Ok(Self {
            envelope_family: "browserHistoryIngress",
            causality: causality.clone(),
            browser_history_envelope_digest: canonical_worker_certification_digest(&ingress)?,
            route_truth_digest: canonical_worker_certification_digest(&(
                ingress.navigation_kind.as_str(),
                ingress.route_identity.as_str(),
            ))?,
            continuity_digest: canonical_worker_certification_digest(&(
                "runtimeOwnedRouteContinuity",
                ingress.raw_location.as_str(),
                ingress.route_identity.as_str(),
                runtime_admitted_continuity_count,
                causality.clone(),
            ))?,
            replay_restore_digest: canonical_worker_certification_digest(&(
                "runtimeOwnedRouteReplayRestore",
                ingress.navigation_kind.as_str(),
                ingress.raw_location.as_str(),
                ingress.route_identity.as_str(),
                runtime_admitted_route_count,
                runtime_admitted_continuity_count,
                causality,
            ))?,
            runtime_admitted_route_count,
            runtime_admitted_continuity_count,
            runtime_mutation_breadth,
            worker_first_truth_digest,
            performance: WorkerHostBoundaryPerformanceEnvelope::browser_history_ingress(
                ingress.raw_location.as_str(),
                ingress.route_identity.as_str(),
                runtime_admitted_route_count + runtime_admitted_continuity_count,
                runtime_mutation_breadth,
            )?,
            ambient_location_read_denied: true,
        })
    }
}

pub(in crate::runtime::worker_host) fn runtime_values_for_browser_history_admission(
    ingress: &WorkerBrowserHistoryIngress,
) -> Result<Vec<SetValue>, WorthSignalJsError> {
    let mut runtime_values = Vec::with_capacity(2);
    match (&ingress.runtime_route_source_id, &ingress.route_value) {
        (Some(runtime_route_source_id), Some(route_value)) => runtime_values.push(SetValue {
            id: runtime_route_source_id.clone(),
            value: route_value.clone(),
            aspect: None,
            aspects: None,
        }),
        (None, None) => {}
        _ => {
            return Err(WorthSignalJsError::invalid_input(
                "browser history route admission requires a paired runtime route source id with route value",
            ));
        }
    }
    match (
        &ingress.runtime_continuity_source_id,
        &ingress.continuity_value,
    ) {
        (Some(runtime_continuity_source_id), Some(continuity_value)) => {
            runtime_values.push(SetValue {
                id: runtime_continuity_source_id.clone(),
                value: continuity_value.clone(),
                aspect: None,
                aspects: None,
            });
        }
        (None, None) => {}
        _ => {
            return Err(WorthSignalJsError::invalid_input(
                "browser history route continuity admission requires a paired runtime continuity source id with continuity value",
            ));
        }
    }

    Ok(runtime_values)
}

pub(in crate::runtime::worker_host) fn browser_history_admission_width(
    ingress: &WorkerBrowserHistoryIngress,
) -> BrowserHistoryAdmissionWidth {
    BrowserHistoryAdmissionWidth {
        runtime_admitted_route_count: u64::from(
            ingress.runtime_route_source_id.is_some() && ingress.route_value.is_some(),
        ),
        runtime_admitted_continuity_count: u64::from(
            ingress.runtime_continuity_source_id.is_some() && ingress.continuity_value.is_some(),
        ),
    }
}

pub(in crate::runtime::worker_host) struct BrowserHistoryAdmissionWidth {
    pub runtime_admitted_route_count: u64,
    pub runtime_admitted_continuity_count: u64,
}
