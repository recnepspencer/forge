use crate::identity::hash_parts;
use crate::policy_execution_seam::{
    PolicyAwareExecutionSeamError, PolicyAwareExecutionSeamFailureClass, PolicyAwareSeamCounters,
};

use super::PolicyAwareLivePlan;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PolicyDriftDisposition {
    NoChange,
    FreshAdmissionFromCheckpoint,
    FullRestartDebt,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyLiveEpochEvidence {
    checkpoint_policy_digest: String,
    checkpoint_tenant_basis_digest: String,
    current_policy_digest: String,
    current_tenant_basis_digest: String,
    disposition: PolicyDriftDisposition,
    digest: String,
}

impl PolicyLiveEpochEvidence {
    pub fn new(
        checkpoint_policy_digest: impl Into<String>,
        checkpoint_tenant_basis_digest: impl Into<String>,
        current_policy_digest: impl Into<String>,
        current_tenant_basis_digest: impl Into<String>,
    ) -> Self {
        let checkpoint_policy_digest = checkpoint_policy_digest.into();
        let checkpoint_tenant_basis_digest = checkpoint_tenant_basis_digest.into();
        let current_policy_digest = current_policy_digest.into();
        let current_tenant_basis_digest = current_tenant_basis_digest.into();
        let disposition = if checkpoint_policy_digest == current_policy_digest
            && checkpoint_tenant_basis_digest == current_tenant_basis_digest
        {
            PolicyDriftDisposition::NoChange
        } else {
            PolicyDriftDisposition::FreshAdmissionFromCheckpoint
        };
        let digest = hash_parts(&[
            format!("checkpoint_policy:{checkpoint_policy_digest}"),
            format!("checkpoint_tenant:{checkpoint_tenant_basis_digest}"),
            format!("current_policy:{current_policy_digest}"),
            format!("current_tenant:{current_tenant_basis_digest}"),
            format!("disposition:{}", disposition.as_str()),
        ]);
        Self {
            checkpoint_policy_digest,
            checkpoint_tenant_basis_digest,
            current_policy_digest,
            current_tenant_basis_digest,
            disposition,
            digest,
        }
    }

    pub fn disposition(&self) -> PolicyDriftDisposition {
        self.disposition
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn checkpoint_policy_digest(&self) -> &str {
        &self.checkpoint_policy_digest
    }

    pub fn checkpoint_tenant_basis_digest(&self) -> &str {
        &self.checkpoint_tenant_basis_digest
    }

    pub fn current_policy_digest(&self) -> &str {
        &self.current_policy_digest
    }

    pub fn current_tenant_basis_digest(&self) -> &str {
        &self.current_tenant_basis_digest
    }
}

impl PolicyDriftDisposition {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NoChange => "no_change",
            Self::FreshAdmissionFromCheckpoint => "fresh_admission_from_checkpoint",
            Self::FullRestartDebt => "full_restart_debt",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyLiveDensityEvidence {
    authorized_relevance_width: usize,
    observed_authorized_delta_width: usize,
    sparse_delta_width_limit: usize,
    posture: PolicyLiveDensityPosture,
    digest: String,
}

impl PolicyLiveDensityEvidence {
    pub fn new(
        authorized_relevance_width: usize,
        observed_authorized_delta_width: usize,
        sparse_delta_width_limit: usize,
    ) -> Self {
        let posture = if observed_authorized_delta_width <= sparse_delta_width_limit
            && observed_authorized_delta_width <= authorized_relevance_width
        {
            PolicyLiveDensityPosture::SparseDelta
        } else if observed_authorized_delta_width <= authorized_relevance_width {
            PolicyLiveDensityPosture::BurstReadmission
        } else {
            PolicyLiveDensityPosture::DenseRestartDebt
        };
        let digest = hash_parts(&[
            format!("authorized_relevance_width:{authorized_relevance_width}"),
            format!("observed_authorized_delta_width:{observed_authorized_delta_width}"),
            format!("sparse_delta_width_limit:{sparse_delta_width_limit}"),
            format!("posture:{}", posture.as_str()),
        ]);
        Self {
            authorized_relevance_width,
            observed_authorized_delta_width,
            sparse_delta_width_limit,
            posture,
            digest,
        }
    }

    pub fn authorized_relevance_width(&self) -> usize {
        self.authorized_relevance_width
    }

    pub fn observed_authorized_delta_width(&self) -> usize {
        self.observed_authorized_delta_width
    }

    pub fn sparse_delta_width_limit(&self) -> usize {
        self.sparse_delta_width_limit
    }

    pub fn posture(&self) -> PolicyLiveDensityPosture {
        self.posture
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyLiveDriftEvidenceReport {
    epoch_evidence: PolicyLiveEpochEvidence,
    density_evidence: PolicyLiveDensityEvidence,
    counters: PolicyAwareSeamCounters,
    digest: String,
}

impl PolicyLiveDriftEvidenceReport {
    pub fn epoch_evidence(&self) -> &PolicyLiveEpochEvidence {
        &self.epoch_evidence
    }

    pub fn density_evidence(&self) -> &PolicyLiveDensityEvidence {
        &self.density_evidence
    }

    pub fn counters(&self) -> &PolicyAwareSeamCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

pub fn certify_policy_live_drift_evidence(
    live_plan: &PolicyAwareLivePlan,
    epoch_evidence: PolicyLiveEpochEvidence,
    density_evidence: PolicyLiveDensityEvidence,
) -> Result<PolicyLiveDriftEvidenceReport, PolicyAwareExecutionSeamError> {
    let seam = live_plan.core().seam();
    if epoch_evidence.current_policy_digest() != seam.policy_digest()
        || epoch_evidence.current_tenant_basis_digest() != seam.tenant_truth_basis_digest()
    {
        return Err(PolicyAwareExecutionSeamError::new(
            PolicyAwareExecutionSeamFailureClass::UnsupportedPolicyExecutionMode,
            "live drift evidence must bind the current policy and tenant basis to the admitted live plan",
            PolicyAwareSeamCounters::denied_raw_live_relevance(),
        ));
    }

    if live_plan.report().drift_disposition() != epoch_evidence.disposition() {
        return Err(PolicyAwareExecutionSeamError::new(
            PolicyAwareExecutionSeamFailureClass::UnsupportedPolicyExecutionMode,
            "live drift evidence must match the admitted plan disposition",
            PolicyAwareSeamCounters::denied_raw_live_relevance(),
        ));
    }

    if live_plan.report().density_posture() != density_evidence.posture() {
        return Err(PolicyAwareExecutionSeamError::new(
            PolicyAwareExecutionSeamFailureClass::UnsupportedPolicyExecutionMode,
            "live density evidence must match the admitted plan posture",
            PolicyAwareSeamCounters::denied_raw_live_relevance(),
        ));
    }

    if density_evidence.authorized_relevance_width()
        != live_plan.relevance().authorized_fields().len()
    {
        return Err(PolicyAwareExecutionSeamError::new(
            PolicyAwareExecutionSeamFailureClass::UnsupportedPolicyExecutionMode,
            "live density evidence must bind to the admitted live relevance width",
            PolicyAwareSeamCounters::denied_raw_live_relevance(),
        ));
    }

    let mut counters = PolicyAwareSeamCounters::default();
    if epoch_evidence.checkpoint_policy_digest() != epoch_evidence.current_policy_digest() {
        counters = counters.record_policy_epoch_drift_readmission();
    }
    if epoch_evidence.checkpoint_tenant_basis_digest()
        != epoch_evidence.current_tenant_basis_digest()
    {
        counters = counters.record_tenant_basis_drift_readmission();
    }
    if density_evidence.posture() == PolicyLiveDensityPosture::BurstReadmission {
        counters = counters.record_policy_sparse_to_burst_readmission();
    }

    let digest = hash_parts(&[
        format!("live_plan:{}", live_plan.report().digest()),
        format!("epoch:{}", epoch_evidence.digest()),
        format!("density:{}", density_evidence.digest()),
        format!("counters:{}", hash_parts(&counters.digest_parts())),
    ]);

    Ok(PolicyLiveDriftEvidenceReport {
        epoch_evidence,
        density_evidence,
        counters,
        digest,
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PolicyLiveDensityPosture {
    SparseDelta,
    BurstReadmission,
    DenseRestartDebt,
}

impl PolicyLiveDensityPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SparseDelta => "sparse_delta",
            Self::BurstReadmission => "burst_readmission",
            Self::DenseRestartDebt => "dense_restart_debt",
        }
    }
}
