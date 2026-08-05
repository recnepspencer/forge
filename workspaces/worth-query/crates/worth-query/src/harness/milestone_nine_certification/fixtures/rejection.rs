use crate::harness::certification::digest_parts;
use crate::harness::milestone_nine_certification::bundles::MilestoneNineRejectionBundle;
use crate::harness::milestone_nine_certification::classifications::MilestoneNineFailureClass;
use crate::harness::milestone_nine_certification::fixtures::base_policy;
use crate::harness::milestone_nine_certification::fixtures::canonical_query;
use crate::harness::milestone_nine_certification::fixtures::schema;
use crate::harness::milestone_nine_certification::fixtures::tenant;
use crate::policy_basis::admit_policy_tenant_context;
use crate::policy_basis::BranchAccessGrant;
use crate::policy_basis::PolicyExecutionModeRequest;
use crate::policy_basis::PolicyTenantAdmissionFailureClass;

pub(in crate::harness::milestone_nine_certification) fn rejection_bundle(
    error: crate::policy_basis::PolicyTenantAdmissionError,
) -> MilestoneNineRejectionBundle {
    let failure_class = match error.failure_class() {
        PolicyTenantAdmissionFailureClass::UnsupportedExecutionMode => {
            MilestoneNineFailureClass::UnsupportedExecutionMode
        }
        PolicyTenantAdmissionFailureClass::BranchAccessDenied => {
            MilestoneNineFailureClass::BranchAccessDenied
        }
        PolicyTenantAdmissionFailureClass::PolicyWorkBudgetDenied => {
            MilestoneNineFailureClass::UnsupportedExecutionMode
        }
        PolicyTenantAdmissionFailureClass::TenantAdmissionDenied => {
            MilestoneNineFailureClass::TenantAdmissionDenied
        }
        _ => MilestoneNineFailureClass::TenantAdmissionDenied,
    };
    MilestoneNineRejectionBundle {
        failure_class,
        failure_digest: digest_parts(&[
            error.failure_class().as_str().to_string(),
            error.message().to_string(),
        ]),
        counter_snapshot_digest: digest_parts(&[
            format!(
                "policy_denials:{}",
                error.counters().policy().policy_basis_denial_count()
            ),
            format!(
                "branch_denials:{}",
                error.counters().policy().branch_access_denial_count()
            ),
            format!(
                "mode_denials:{}",
                error
                    .counters()
                    .policy()
                    .unsupported_execution_mode_denial_count()
            ),
            format!(
                "work_budget_denials:{}",
                error.counters().policy().policy_work_budget_denial_count()
            ),
            format!(
                "hidden_filters:{}",
                error
                    .counters()
                    .tenant()
                    .hidden_tenant_filter_denial_count()
            ),
            format!(
                "schema_fallbacks:{}",
                error
                    .counters()
                    .tenant()
                    .global_schema_fallback_denial_count()
            ),
        ]),
    }
}

pub(in crate::harness::milestone_nine_certification) fn policy_narrowing_rejection_bundle(
    error: crate::policy_narrowing::PolicyNarrowingError,
) -> MilestoneNineRejectionBundle {
    let failure_class = match error.failure_class() {
        crate::policy_narrowing::PolicyNarrowingFailureClass::RelationshipProofDenied(_) => {
            MilestoneNineFailureClass::RelationshipProofDenied
        }
        _ => MilestoneNineFailureClass::PolicyNarrowingDenied,
    };
    MilestoneNineRejectionBundle {
        failure_class,
        failure_digest: digest_parts(&[
            error.failure_class().as_str().to_string(),
            error.message().to_string(),
        ]),
        counter_snapshot_digest: digest_parts(&error.counters().digest_parts()),
    }
}

pub(in crate::harness::milestone_nine_certification) fn policy_execution_seam_rejection_bundle(
    error: crate::policy_execution_seam::PolicyAwareExecutionSeamError,
) -> MilestoneNineRejectionBundle {
    MilestoneNineRejectionBundle {
        failure_class: MilestoneNineFailureClass::PolicyExecutionSeamDenied,
        failure_digest: digest_parts(&[
            error.failure_class().as_str().to_string(),
            error.message().to_string(),
        ]),
        counter_snapshot_digest: digest_parts(&error.counters().digest_parts()),
    }
}

pub(in crate::harness::milestone_nine_certification) fn rejection_for_mode(
    mode: PolicyExecutionModeRequest,
) -> MilestoneNineRejectionBundle {
    let canonical = canonical_query();
    let policy = base_policy(false);
    let branch = BranchAccessGrant::synthetic_granted("branch-a", &policy);
    let error =
        admit_policy_tenant_context(canonical.query(), policy, tenant(), branch, schema(), mode)
            .unwrap_err();
    rejection_bundle(error)
}
