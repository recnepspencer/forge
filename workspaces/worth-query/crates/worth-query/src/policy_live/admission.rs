#[cfg(test)]
use crate::authorized_projection::AuthorizedProjectionFieldPath;
#[cfg(test)]
use crate::identity::hash_parts;
#[cfg(test)]
use crate::policy_execution_seam::{
    PolicyAwareExecutionMode, PolicyAwareExecutionSeam, PolicyAwareExecutionSeamError,
    PolicyAwareExecutionSeamFailureClass, PolicyAwareSeamCounters,
};
#[cfg(test)]
use crate::policy_narrowing::NarrowedPolicyQueryArtifact;
use crate::policy_plan::PolicyAwarePlanCore;
#[cfg(test)]
use crate::policy_plan::PolicyAwarePlanCostPosture;

use super::{PolicyAwareLiveRelevanceContract, PolicyDriftDisposition, PolicyLiveDensityPosture};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyAwareLiveAdmissionReport {
    digest: String,
    drift_disposition: PolicyDriftDisposition,
    density_posture: PolicyLiveDensityPosture,
}

impl PolicyAwareLiveAdmissionReport {
    #[cfg(test)]
    pub(crate) fn new(
        seam: &PolicyAwareExecutionSeam,
        relevance: &PolicyAwareLiveRelevanceContract,
        drift_disposition: PolicyDriftDisposition,
        density_posture: PolicyLiveDensityPosture,
    ) -> Self {
        Self {
            digest: hash_parts(&[
                format!("seam:{}", seam.identity().as_str()),
                format!("relevance:{}", relevance.digest()),
                format!("drift:{}", drift_disposition.as_str()),
                format!("density:{}", density_posture.as_str()),
            ]),
            drift_disposition,
            density_posture,
        }
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn drift_disposition(&self) -> PolicyDriftDisposition {
        self.drift_disposition
    }

    pub fn density_posture(&self) -> PolicyLiveDensityPosture {
        self.density_posture
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyAwareLivePlan {
    core: PolicyAwarePlanCore,
    relevance: PolicyAwareLiveRelevanceContract,
    report: PolicyAwareLiveAdmissionReport,
}

impl PolicyAwareLivePlan {
    pub fn core(&self) -> &PolicyAwarePlanCore {
        &self.core
    }

    pub fn relevance(&self) -> &PolicyAwareLiveRelevanceContract {
        &self.relevance
    }

    pub fn report(&self) -> &PolicyAwareLiveAdmissionReport {
        &self.report
    }
}
#[cfg(test)]
pub(crate) fn admit_policy_aware_live_plan(
    artifact: &NarrowedPolicyQueryArtifact,
    requested_relevance_fields: &[AuthorizedProjectionFieldPath],
    drift_disposition: PolicyDriftDisposition,
    density_posture: PolicyLiveDensityPosture,
) -> Result<PolicyAwareLivePlan, PolicyAwareExecutionSeamError> {
    let masked = artifact
        .authorized_projection()
        .masked_projection()
        .masked_field_paths();
    if requested_relevance_fields
        .iter()
        .any(|requested| masked.iter().any(|masked| masked == requested))
    {
        return Err(PolicyAwareExecutionSeamError::new(
            PolicyAwareExecutionSeamFailureClass::RawLiveRelevanceForbidden,
            "live relevance cannot observe masked fields after policy narrowing",
            PolicyAwareSeamCounters::denied_raw_live_relevance(),
        ));
    }

    if matches!(density_posture, PolicyLiveDensityPosture::DenseRestartDebt) {
        return Err(PolicyAwareExecutionSeamError::new(
            PolicyAwareExecutionSeamFailureClass::UnsupportedPolicyExecutionMode,
            "dense policy-aware live maintenance remains explicit restart debt",
            PolicyAwareSeamCounters::denied_policy_dense_restart_debt(),
        ));
    }

    let authorized_field_paths = requested_relevance_fields
        .iter()
        .filter_map(|field| {
            artifact
                .authorized_projection()
                .visible_field_paths()
                .iter()
                .find(|path| *path == field)
                .cloned()
        })
        .collect::<Vec<_>>();
    let relevance = PolicyAwareLiveRelevanceContract::new(authorized_field_paths);
    let core = PolicyAwarePlanCore::from_narrowed_with_counter_adjustment(
        artifact,
        PolicyAwareExecutionMode::LiveSubscription,
        PolicyAwarePlanCostPosture::LiveSparseAuthorized,
        artifact.authorized_projection().visible_field_paths().len(),
        relevance.authorized_field_paths().len(),
        |mut counters| {
            if matches!(density_posture, PolicyLiveDensityPosture::BurstReadmission) {
                counters = counters.record_policy_sparse_to_burst_readmission();
            }
            counters
        },
    );
    let report = PolicyAwareLiveAdmissionReport::new(
        core.seam(),
        &relevance,
        drift_disposition,
        density_posture,
    );
    Ok(PolicyAwareLivePlan {
        core,
        relevance,
        report,
    })
}
