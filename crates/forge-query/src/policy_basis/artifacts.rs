use crate::identity::hash_parts;
use crate::tenant_basis::{
    TenantBasisEpoch, TenantSchemaBasis, TenantSchemaBasisIdentity, TenantTruthBasis,
    TenantTruthBasisIdentity,
};

use super::{
    BranchAccessGrant, PolicyCostPosture, PolicyEpoch, PolicyTenantAdmissionCounters,
    PolicyWorkBudget,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyBasisIdentity(String);

impl PolicyBasisIdentity {
    pub(crate) fn new(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyTenantAdmissionDigest(String);

impl PolicyTenantAdmissionDigest {
    pub(crate) fn new(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PolicyAdmissionDisposition {
    AdmittedUnchanged,
    AdmittedNarrowed,
    AdmittedWithNonDisclosingUse,
    Denied,
}

impl PolicyAdmissionDisposition {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AdmittedUnchanged => "admitted_unchanged",
            Self::AdmittedNarrowed => "admitted_narrowed",
            Self::AdmittedWithNonDisclosingUse => "admitted_with_non_disclosing_use",
            Self::Denied => "denied",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PolicyExecutionModeRequest {
    CurrentRead,
    BranchRead,
    HistoricalRead,
    HistoricalDiff,
    LiveSubscription,
    DeliveryOnly,
}

impl PolicyExecutionModeRequest {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CurrentRead => "current_read",
            Self::BranchRead => "branch_read",
            Self::HistoricalRead => "historical_read",
            Self::HistoricalDiff => "historical_diff",
            Self::LiveSubscription => "live_subscription",
            Self::DeliveryOnly => "delivery_only",
        }
    }

    pub(crate) fn phase_one_admitted(&self) -> bool {
        matches!(
            self,
            Self::CurrentRead | Self::BranchRead | Self::HistoricalRead
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyBasis {
    identity: PolicyBasisIdentity,
    policy_epoch: PolicyEpoch,
    rule_set_digest: String,
    disposition: PolicyAdmissionDisposition,
    cost_posture: PolicyCostPosture,
    work_budget: PolicyWorkBudget,
}

impl PolicyBasis {
    pub(crate) fn admitted(
        identity: PolicyBasisIdentity,
        policy_epoch: PolicyEpoch,
        rule_set_digest: String,
        disposition: PolicyAdmissionDisposition,
        cost_posture: PolicyCostPosture,
        work_budget: PolicyWorkBudget,
    ) -> Self {
        Self {
            identity,
            policy_epoch,
            rule_set_digest,
            disposition,
            cost_posture,
            work_budget,
        }
    }

    pub fn identity(&self) -> &PolicyBasisIdentity {
        &self.identity
    }

    pub fn policy_epoch(&self) -> PolicyEpoch {
        self.policy_epoch
    }

    pub fn rule_set_digest(&self) -> &str {
        &self.rule_set_digest
    }

    pub fn disposition(&self) -> PolicyAdmissionDisposition {
        self.disposition
    }

    pub fn cost_posture(&self) -> PolicyCostPosture {
        self.cost_posture
    }

    pub fn work_budget(&self) -> PolicyWorkBudget {
        self.work_budget
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyTenantAdmissionBundle {
    canonical_query_digest: String,
    policy_digest: String,
    policy_epoch: PolicyEpoch,
    tenant_truth_basis_digest: String,
    tenant_schema_basis_digest: String,
    tenant_basis_epoch: TenantBasisEpoch,
    branch_access_digest: String,
    schema_variant_digest: String,
    execution_mode: PolicyExecutionModeRequest,
    admission_disposition: PolicyAdmissionDisposition,
    policy_cost_posture: PolicyCostPosture,
    policy_work_budget: PolicyWorkBudget,
    counters: PolicyTenantAdmissionCounters,
    digest: PolicyTenantAdmissionDigest,
}

impl PolicyTenantAdmissionBundle {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn admitted(
        canonical_query_digest: String,
        policy_basis: &PolicyBasis,
        tenant_truth_basis: &TenantTruthBasis,
        tenant_schema_basis: &TenantSchemaBasis,
        branch_access: &BranchAccessGrant,
        schema_variant_digest: String,
        execution_mode: PolicyExecutionModeRequest,
        counters: PolicyTenantAdmissionCounters,
    ) -> Self {
        let policy_digest = policy_basis.identity().as_str().to_string();
        let tenant_truth_basis_digest = tenant_truth_basis.identity().as_str().to_string();
        let tenant_schema_basis_digest = tenant_schema_basis.identity().as_str().to_string();
        let branch_access_digest = branch_access.digest().to_string();
        let digest = PolicyTenantAdmissionDigest::new(hash_parts(&[
            format!("query:{canonical_query_digest}"),
            format!("policy:{policy_digest}"),
            format!("policy_epoch:{}", policy_basis.policy_epoch().as_u64()),
            format!("tenant_truth:{tenant_truth_basis_digest}"),
            format!("tenant_schema:{tenant_schema_basis_digest}"),
            format!("tenant_epoch:{}", tenant_truth_basis.epoch().as_u64()),
            format!("branch:{branch_access_digest}"),
            format!("schema_variant:{schema_variant_digest}"),
            format!("mode:{}", execution_mode.as_str()),
            format!("disposition:{}", policy_basis.disposition().as_str()),
            format!("cost_posture:{}", policy_basis.cost_posture().as_str()),
            policy_basis.work_budget().digest_part(),
        ]));
        Self {
            canonical_query_digest,
            policy_digest,
            policy_epoch: policy_basis.policy_epoch(),
            tenant_truth_basis_digest,
            tenant_schema_basis_digest,
            tenant_basis_epoch: tenant_truth_basis.epoch(),
            branch_access_digest,
            schema_variant_digest,
            execution_mode,
            admission_disposition: policy_basis.disposition(),
            policy_cost_posture: policy_basis.cost_posture(),
            policy_work_budget: policy_basis.work_budget(),
            counters,
            digest,
        }
    }

    pub fn canonical_query_digest(&self) -> &str {
        &self.canonical_query_digest
    }

    pub fn policy_digest(&self) -> &str {
        &self.policy_digest
    }

    pub fn policy_epoch(&self) -> PolicyEpoch {
        self.policy_epoch
    }

    pub fn tenant_truth_basis_digest(&self) -> &str {
        &self.tenant_truth_basis_digest
    }

    pub fn tenant_schema_basis_digest(&self) -> &str {
        &self.tenant_schema_basis_digest
    }

    pub fn tenant_basis_epoch(&self) -> TenantBasisEpoch {
        self.tenant_basis_epoch
    }

    pub fn branch_access_digest(&self) -> &str {
        &self.branch_access_digest
    }

    pub fn schema_variant_digest(&self) -> &str {
        &self.schema_variant_digest
    }

    pub fn execution_mode(&self) -> PolicyExecutionModeRequest {
        self.execution_mode
    }

    pub fn admission_disposition(&self) -> PolicyAdmissionDisposition {
        self.admission_disposition
    }

    pub fn policy_cost_posture(&self) -> PolicyCostPosture {
        self.policy_cost_posture
    }

    pub fn policy_work_budget(&self) -> PolicyWorkBudget {
        self.policy_work_budget
    }

    pub fn counters(&self) -> &PolicyTenantAdmissionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &PolicyTenantAdmissionDigest {
        &self.digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedPolicyTenantContext {
    policy_basis: PolicyBasis,
    tenant_truth_basis: TenantTruthBasis,
    tenant_schema_basis: TenantSchemaBasis,
    bundle: PolicyTenantAdmissionBundle,
}

impl AdmittedPolicyTenantContext {
    pub(crate) fn admitted(
        policy_basis: PolicyBasis,
        tenant_truth_basis: TenantTruthBasis,
        tenant_schema_basis: TenantSchemaBasis,
        bundle: PolicyTenantAdmissionBundle,
    ) -> Self {
        Self {
            policy_basis,
            tenant_truth_basis,
            tenant_schema_basis,
            bundle,
        }
    }

    pub fn policy_basis(&self) -> &PolicyBasis {
        &self.policy_basis
    }

    pub fn tenant_truth_basis(&self) -> &TenantTruthBasis {
        &self.tenant_truth_basis
    }

    pub fn tenant_schema_basis(&self) -> &TenantSchemaBasis {
        &self.tenant_schema_basis
    }

    pub fn bundle(&self) -> &PolicyTenantAdmissionBundle {
        &self.bundle
    }
}

pub(crate) fn policy_basis_identity(
    label: &str,
    rule_set_digest: &str,
    epoch: PolicyEpoch,
) -> PolicyBasisIdentity {
    PolicyBasisIdentity::new(hash_parts(&[
        format!("policy_basis:{label}"),
        format!("rule_set:{rule_set_digest}"),
        format!("epoch:{}", epoch.as_u64()),
    ]))
}

pub(crate) fn tenant_truth_identity(
    tenant_identity: &str,
    branch_identity: &str,
    resolution_class: crate::tenant_basis::TenantResolutionClass,
    epoch: TenantBasisEpoch,
) -> TenantTruthBasisIdentity {
    TenantTruthBasisIdentity::new(hash_parts(&[
        format!("tenant:{tenant_identity}"),
        format!("branch:{branch_identity}"),
        format!("resolution:{}", resolution_class.as_str()),
        format!("epoch:{}", epoch.as_u64()),
    ]))
}

pub(crate) fn tenant_schema_identity(
    tenant_identity: &str,
    schema_identity: &str,
    epoch: TenantBasisEpoch,
) -> TenantSchemaBasisIdentity {
    TenantSchemaBasisIdentity::new(hash_parts(&[
        format!("tenant:{tenant_identity}"),
        format!("schema:{schema_identity}"),
        format!("epoch:{}", epoch.as_u64()),
    ]))
}
