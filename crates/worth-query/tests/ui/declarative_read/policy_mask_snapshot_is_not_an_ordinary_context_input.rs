use worth_query::facade::foundation::{PolicyAspectMask, PolicyMaskSnapshot};
use worth_query::facade::read::WorthQueryCurrentPolicyTenantReadContext;

fn inject_phase_mask(context: WorthQueryCurrentPolicyTenantReadContext) {
    let phase_artifact =
        PolicyMaskSnapshot::synthetic_authority("policy", PolicyAspectMask::allow_all());
    let _ordinary_context = context.with_policy_narrowing(phase_artifact);
}

fn main() {}
