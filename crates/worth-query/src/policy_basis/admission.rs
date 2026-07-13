use crate::canonicalization::CanonicalQueryArtifact;
use crate::identity::CanonicalQueryDigest;
use crate::policy_basis::artifacts::policy_basis_identity;
use crate::tenant_basis::{
    admit_tenant_bases as admit_tenant_bases_inner, SchemaVariantSnapshot, TenantBindingSnapshot,
    TenantSchemaBasis, TenantTruthBasis,
};

use super::{
    AdmittedPolicyTenantContext, BranchAccessGrant, BranchAccessGrantClass,
    PolicyAdmissionDisposition, PolicyBasis, PolicyBasisCounters, PolicyExecutionModeRequest,
    PolicyRuleSnapshot, PolicyTenantAdmissionBundle, PolicyTenantAdmissionCounters,
    PolicyTenantAdmissionError, PolicyTenantAdmissionFailureClass,
};

pub fn admit_policy_tenant_context(
    query: &CanonicalQueryArtifact,
    policy: PolicyRuleSnapshot,
    tenant: TenantBindingSnapshot,
    branch: BranchAccessGrant,
    schema: SchemaVariantSnapshot,
    mode: PolicyExecutionModeRequest,
) -> Result<AdmittedPolicyTenantContext, PolicyTenantAdmissionError> {
    admit_policy_tenant_context_for_query_identity(
        query.digest(),
        policy,
        tenant,
        branch,
        schema,
        mode,
    )
}

pub(crate) fn admit_policy_tenant_context_for_query_identity(
    canonical_query_digest: &CanonicalQueryDigest,
    policy: PolicyRuleSnapshot,
    tenant: TenantBindingSnapshot,
    branch: BranchAccessGrant,
    schema: SchemaVariantSnapshot,
    mode: PolicyExecutionModeRequest,
) -> Result<AdmittedPolicyTenantContext, PolicyTenantAdmissionError> {
    let policy_basis = admit_policy_basis(&policy, &branch, mode)?;
    let (tenant_truth_basis, tenant_schema_basis, tenant_counters) =
        admit_tenant_bases(&tenant, &schema)?;

    if branch.branch_identity() != tenant_truth_basis.branch_identity() {
        return Err(PolicyTenantAdmissionError::new(
            PolicyTenantAdmissionFailureClass::TenantAdmissionDenied,
            "branch access grant must match tenant truth branch",
            PolicyTenantAdmissionCounters::admitted(
                PolicyBasisCounters::admitted(),
                crate::tenant_basis::TenantBasisCounters::denied_missing_truth(),
            ),
        ));
    }

    let counters =
        PolicyTenantAdmissionCounters::admitted(PolicyBasisCounters::admitted(), tenant_counters);
    let bundle = PolicyTenantAdmissionBundle::admitted(
        canonical_query_digest.as_str().to_string(),
        &policy_basis,
        &tenant_truth_basis,
        &tenant_schema_basis,
        &branch,
        schema.digest().to_string(),
        mode,
        counters,
    );

    Ok(AdmittedPolicyTenantContext::admitted(
        policy_basis,
        tenant_truth_basis,
        tenant_schema_basis,
        bundle,
    ))
}

fn admit_policy_basis(
    policy: &PolicyRuleSnapshot,
    branch: &BranchAccessGrant,
    mode: PolicyExecutionModeRequest,
) -> Result<PolicyBasis, PolicyTenantAdmissionError> {
    if !mode.phase_one_admitted() {
        return Err(PolicyTenantAdmissionError::new(
            PolicyTenantAdmissionFailureClass::UnsupportedExecutionMode,
            "policy tenant admission admits current, branch, historical read, and graph mutation contexts only",
            PolicyTenantAdmissionCounters::admitted(
                PolicyBasisCounters::denied_mode(),
                crate::tenant_basis::TenantBasisCounters::default(),
            ),
        ));
    }

    if !policy.admits_query_family() {
        return Err(PolicyTenantAdmissionError::new(
            PolicyTenantAdmissionFailureClass::PolicyQueryFamilyDenied,
            "policy snapshot denies the query family",
            PolicyTenantAdmissionCounters::admitted(
                PolicyBasisCounters::denied_policy(),
                crate::tenant_basis::TenantBasisCounters::default(),
            ),
        ));
    }

    if branch.grant_class() == BranchAccessGrantClass::Denied {
        return Err(PolicyTenantAdmissionError::new(
            PolicyTenantAdmissionFailureClass::BranchAccessDenied,
            "branch access is denied before truth touch",
            PolicyTenantAdmissionCounters::admitted(
                PolicyBasisCounters::denied_branch(),
                crate::tenant_basis::TenantBasisCounters::default(),
            ),
        ));
    }

    if branch.policy_digest() != policy.digest() {
        return Err(PolicyTenantAdmissionError::new(
            PolicyTenantAdmissionFailureClass::RawMiddlewarePolicySourceForbidden,
            "branch grant policy digest must match the policy snapshot",
            PolicyTenantAdmissionCounters::admitted(
                PolicyBasisCounters::denied_middleware(),
                crate::tenant_basis::TenantBasisCounters::default(),
            ),
        ));
    }

    if !policy.cost_posture().phase_one_admitted() || policy.work_budget().is_none() {
        return Err(PolicyTenantAdmissionError::new(
            PolicyTenantAdmissionFailureClass::PolicyWorkBudgetDenied,
            "policy admission requires an explicit bounded work budget before truth touch",
            PolicyTenantAdmissionCounters::admitted(
                PolicyBasisCounters::denied_work_budget(),
                crate::tenant_basis::TenantBasisCounters::default(),
            ),
        ));
    }

    let disposition = if policy.admits_non_disclosing_use() {
        PolicyAdmissionDisposition::AdmittedWithNonDisclosingUse
    } else if policy.narrows_projection() {
        PolicyAdmissionDisposition::AdmittedNarrowed
    } else {
        PolicyAdmissionDisposition::AdmittedUnchanged
    };
    Ok(PolicyBasis::admitted(
        policy_basis_identity(
            policy.policy_basis_label(),
            policy.rule_set_digest(),
            policy.policy_epoch(),
            policy.digest(),
        ),
        policy.policy_epoch(),
        policy.rule_set_digest().to_string(),
        disposition,
        policy.cost_posture(),
        policy
            .work_budget()
            .expect("checked above: policy work budget exists"),
        policy.projection_mask().cloned(),
    ))
}

fn admit_tenant_bases(
    tenant: &TenantBindingSnapshot,
    schema: &SchemaVariantSnapshot,
) -> Result<
    (
        TenantTruthBasis,
        TenantSchemaBasis,
        crate::tenant_basis::TenantBasisCounters,
    ),
    PolicyTenantAdmissionError,
> {
    let (truth, schema_basis, counters) =
        admit_tenant_bases_inner(tenant, schema).map_err(|err| {
            PolicyTenantAdmissionError::new(
                PolicyTenantAdmissionFailureClass::TenantAdmissionDenied,
                "tenant basis admission failed before truth touch",
                PolicyTenantAdmissionCounters::admitted(
                    PolicyBasisCounters::admitted(),
                    err.counters().clone(),
                ),
            )
        })?;
    Ok((truth, schema_basis, counters))
}
