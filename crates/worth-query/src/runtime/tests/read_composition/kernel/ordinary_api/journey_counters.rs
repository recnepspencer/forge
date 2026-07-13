use super::super::super::support::*;
use super::fixtures::{admitted_policy_tenant_inputs, local_identity_read};
use crate::ordinary::read::{current, declare};
use crate::runtime::tests::support::bridge_runtime_with_support;
use crate::runtime::{
    WorthQueryRuntimeFacadeFamily, WorthQueryRuntimeFamilySupport, WorthQueryRuntimeSupportProfile,
};

#[test]
fn successful_read_records_each_journey_boundary_once() {
    let declaration = declare(local_identity_read).expect("read declaration should canonicalize");
    let mut workspace = read_runtime()
        .workspace("ordinary-read-journey-completed")
        .expect("ordinary workspace should open");

    let completion = declaration
        .using(current())
        .run(&mut workspace)
        .into_result()
        .expect("ordinary read should complete");
    let counters = completion.journey_counters();

    assert_eq!(counters.context_admission_attempt_count(), 1);
    assert_eq!(counters.lower_runtime_execution_attempt_count(), 1);
    assert_eq!(counters.lower_runtime_execution_completed_count(), 1);
}

#[test]
fn context_denial_records_zero_lower_runtime_work() {
    let declaration = declare(local_identity_read).expect("read declaration should canonicalize");
    let policy_tenant = admitted_policy_tenant_inputs(1, false);
    let context = current().under_policy_tenant(
        policy_tenant.policy,
        policy_tenant.tenant,
        policy_tenant.branch,
        policy_tenant.schema,
    );
    let mut workspace = read_runtime()
        .workspace("ordinary-read-journey-context-denied")
        .expect("ordinary workspace should open");

    let stop = declaration
        .using(context)
        .run(&mut workspace)
        .into_result()
        .expect_err("denied context must stop before runtime execution");
    let counters = stop.journey_counters();

    assert_eq!(counters.context_admission_attempt_count(), 1);
    assert_eq!(counters.lower_runtime_execution_attempt_count(), 0);
    assert_eq!(counters.lower_runtime_execution_completed_count(), 0);
    assert!(stop.runtime_error().is_none());
}

#[test]
fn runtime_stop_records_attempt_without_completion() {
    let unsupported_read_profile = WorthQueryRuntimeSupportProfile::bridge_backed(
        "ordinary-read-runtime-stop-subscription",
        "ordinary-read-runtime-stop-preview",
        "ordinary-read-runtime-stop-inspection",
    )
    .with_family_support(WorthQueryRuntimeFamilySupport::unsupported(
        WorthQueryRuntimeFacadeFamily::Read,
        "seeded ordinary read runtime denial",
    ));
    let mut workspace = bridge_runtime_with_support(unsupported_read_profile)
        .workspace("ordinary-read-journey-runtime-stopped")
        .expect("ordinary workspace should open");
    let declaration = declare(local_identity_read).expect("read declaration should canonicalize");

    let stop = declaration
        .using(current())
        .run(&mut workspace)
        .into_result()
        .expect_err("unsupported read runtime must stop after context admission");
    let counters = stop.journey_counters();

    assert_eq!(counters.context_admission_attempt_count(), 1);
    assert_eq!(counters.lower_runtime_execution_attempt_count(), 1);
    assert_eq!(counters.lower_runtime_execution_completed_count(), 0);
    assert!(stop.context_receipt().is_some());
    assert!(stop.runtime_error().is_some());
}
