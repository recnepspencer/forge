use crate::harness::certification::digest_parts;
use crate::harness::milestone_nine_certification::bundles::MilestoneNineRejectionBundle;
use crate::harness::milestone_nine_certification::bundles::MilestoneNineRejectionRow;
use crate::harness::milestone_nine_certification::classifications::MilestoneNineFailureClass;
use crate::harness::milestone_nine_certification::classifications::MilestoneNinePerturbationClass;
use crate::harness::milestone_nine_certification::fixtures::admitted_bundle;
use crate::harness::milestone_nine_certification::fixtures::base_policy;
use crate::harness::milestone_nine_certification::fixtures::canonical_query;
use crate::harness::milestone_nine_certification::fixtures::rejection_bundle;
use crate::harness::milestone_nine_certification::fixtures::rejection_for_mode;
use crate::harness::milestone_nine_certification::fixtures::saved_query_reuse_bundle;
use crate::harness::milestone_nine_certification::fixtures::schema;
use crate::harness::milestone_nine_certification::fixtures::tenant;
use crate::policy_basis::admit_policy_tenant_context;
use crate::policy_basis::classify_saved_query_policy_tenant_reuse;
use crate::policy_basis::BranchAccessGrant;
use crate::policy_basis::PolicyCostPosture;
use crate::policy_basis::PolicyEpoch;
use crate::policy_basis::PolicyExecutionModeRequest;
use crate::policy_basis::PolicyRuleSnapshot;
use crate::policy_basis::PolicyWorkBudget;
use crate::policy_basis::SavedQueryPolicyReuseDescriptor;
use crate::policy_basis::SavedQueryPolicyReuseDisposition;
use crate::tenant_basis::SchemaVariantSnapshot;
use crate::tenant_basis::TenantBasisEpoch;
use crate::tenant_basis::TenantBindingSnapshot;

pub(super) fn rejection_admission_rows() -> Vec<MilestoneNineRejectionRow> {
    let control = admitted_bundle(PolicyExecutionModeRequest::CurrentRead, false);
    let canonical = canonical_query();
    let policy = base_policy(false);
    let branch_denial = admit_policy_tenant_context(
        canonical.query(),
        policy.clone(),
        tenant(),
        BranchAccessGrant::synthetic_denied("branch-a", "no_relationship_path", &policy),
        schema(),
        PolicyExecutionModeRequest::CurrentRead,
    )
    .unwrap_err();
    let unknown_cost_policy = PolicyRuleSnapshot::synthetic_authority_with_budget(
        "runtime-policy",
        "rules-v1",
        PolicyEpoch::Synthetic(7),
        true,
        PolicyCostPosture::UnknownCost,
        Some(PolicyWorkBudget::bounded(1, 1, 1)),
    );
    let unknown_cost = admit_policy_tenant_context(
        canonical.query(),
        unknown_cost_policy.clone(),
        tenant(),
        BranchAccessGrant::synthetic_granted("branch-a", &unknown_cost_policy),
        schema(),
        PolicyExecutionModeRequest::CurrentRead,
    )
    .unwrap_err();
    let hidden_tenant = admit_policy_tenant_context(
        canonical.query(),
        policy.clone(),
        TenantBindingSnapshot::synthetic_hidden_filter(
            "tenant-a",
            "branch-a",
            "schema-a",
            TenantBasisEpoch::Synthetic(3),
        ),
        BranchAccessGrant::synthetic_granted("branch-a", &policy),
        schema(),
        PolicyExecutionModeRequest::CurrentRead,
    )
    .unwrap_err();
    let global_fallback = admit_policy_tenant_context(
        canonical.query(),
        policy.clone(),
        tenant(),
        BranchAccessGrant::synthetic_granted("branch-a", &policy),
        SchemaVariantSnapshot::synthetic_global_fallback("tenant-a", "schema-a"),
        PolicyExecutionModeRequest::CurrentRead,
    )
    .unwrap_err();
    let drift = SavedQueryPolicyReuseDescriptor::new(
        "saved-a",
        "policy-a",
        "tenant-truth-a",
        "tenant-schema-a",
        "branch-a",
        PolicyExecutionModeRequest::CurrentRead,
        "policy-b",
        "tenant-truth-b",
        "tenant-schema-b",
        "branch-b",
        PolicyExecutionModeRequest::CurrentRead,
    );
    let drift_class = classify_saved_query_policy_tenant_reuse(&drift);
    vec![
        MilestoneNineRejectionRow {
            row_name: "live-subscription-deferred-before-truth",
            perturbation_class: MilestoneNinePerturbationClass::LiveSubscriptionDeferred,
            control_lane: control.clone(),
            hostile_lane: rejection_for_mode(PolicyExecutionModeRequest::LiveSubscription),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "historical-diff-deferred-before-truth",
            perturbation_class: MilestoneNinePerturbationClass::HistoricalDiffDeferred,
            control_lane: control.clone(),
            hostile_lane: rejection_for_mode(PolicyExecutionModeRequest::HistoricalDiff),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "unknown-policy-cost-denied-before-truth",
            perturbation_class: MilestoneNinePerturbationClass::UnknownPolicyCostDenied,
            control_lane: control.clone(),
            hostile_lane: rejection_bundle(unknown_cost.clone()),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "branch-denial-before-tenant-truth",
            perturbation_class: MilestoneNinePerturbationClass::BranchDeniedBeforeTruth,
            control_lane: control.clone(),
            hostile_lane: rejection_bundle(branch_denial),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "hidden-tenant-filter-denied",
            perturbation_class: MilestoneNinePerturbationClass::HiddenTenantFilterDenied,
            control_lane: control.clone(),
            hostile_lane: rejection_bundle(hidden_tenant),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "global-schema-fallback-denied",
            perturbation_class: MilestoneNinePerturbationClass::GlobalSchemaFallbackDenied,
            control_lane: control.clone(),
            hostile_lane: rejection_bundle(global_fallback),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "saved-query-policy-tenant-drift",
            perturbation_class: MilestoneNinePerturbationClass::SavedQueryPolicyTenantDrift,
            control_lane: saved_query_reuse_bundle(
                SavedQueryPolicyReuseDisposition::LegalNoSemanticChange,
            ),
            hostile_lane: MilestoneNineRejectionBundle {
                failure_class: MilestoneNineFailureClass::SavedQueryPolicyTenantDrift,
                failure_digest: digest_parts(&[drift_class.as_str().to_string()]),
                counter_snapshot_digest: digest_parts(&[format!("reuse:{}", drift_class.as_str())]),
            },
            parity_lane: saved_query_reuse_bundle(
                SavedQueryPolicyReuseDisposition::LegalNoSemanticChange,
            ),
        },
    ]
}
