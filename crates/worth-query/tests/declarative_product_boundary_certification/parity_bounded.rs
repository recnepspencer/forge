use super::product_support as fixture;

#[test]
fn reference_read_matches_internal_phase_chain_oracle() {
    use crate::ordinary::read::{admit_read_context_declaration, current, declare};
    use crate::runtime::{WorthQueryReadBuilder, WorthQueryReadFamily};
    let declaration = declare(fixture::identity_detail).expect("ordinary read should declare");
    let mut workspace = fixture::workspace("phase-12-reference-parity");
    let completion = declaration
        .using(current())
        .run(&mut workspace)
        .into_result()
        .expect("ordinary read should complete");
    let ordinary_context = completion.context_receipt().clone();
    let ordinary = completion.into_result();

    let intent = fixture::identity_detail(WorthQueryReadBuilder::declaration())
        .expect("oracle declaration should build");
    let admitted = admit_read_context_declaration(&intent, current().into())
        .expect("oracle context should admit");
    let (authority, planning_authority, oracle_context) = admitted.into_parts();
    let graph = intent.plan(planning_authority).expect("oracle should plan");
    let family = WorthQueryReadFamily::for_product_boundary_certification("declared_read", graph);
    let oracle = workspace
        .read_family_intent_in_graph_read_authority(&family, &authority)
        .execute()
        .expect("oracle should execute");

    assert_eq!(ordinary, oracle);
    assert_eq!(ordinary.receipt(), oracle.receipt());
    assert_eq!(ordinary_context, oracle_context);
    assert_eq!(
        ordinary_context.graph_authority_admission_digest(),
        oracle_context.graph_authority_admission_digest()
    );
}

#[test]
fn managed_lifecycle_has_exact_open_and_close_work() {
    use worth_query::facade::live::{
        current, declare, WorthQueryLiveOpenOutcome, WorthQueryManagedLiveCloseOutcome,
    };
    let mut workspace = fixture::workspace("phase-12-lifecycle");
    let opened = declare("phase12.lifecycle", fixture::identity_collection)
        .unwrap()
        .using(current())
        .open(&mut workspace);
    let completion = match opened {
        WorthQueryLiveOpenOutcome::Opened(value) => value,
        WorthQueryLiveOpenOutcome::Stopped(stop) => panic!("live stopped: {:?}", stop.source()),
    };
    assert_eq!(
        completion
            .journey_counters()
            .context_admission_attempt_count(),
        1
    );
    let handle = completion.into_handle();
    let observation = handle
        .observe(&mut workspace)
        .expect("live should be observable");
    assert_eq!(
        observation.activation_work().active_lane_creation_count(),
        1
    );
    assert_eq!(
        observation.activation_work().consumer_attachment_count(),
        1
    );
    match handle.close(&mut workspace) {
        WorthQueryManagedLiveCloseOutcome::Closed(receipt) => {
            assert_eq!(
                receipt
                    .disposal_work()
                    .consumer_attachment_close_count(),
                1
            );
            assert_eq!(receipt.disposal_work().active_lane_close_count(), 1);
            assert_eq!(receipt.disposal_work().lifecycle_closeout_count(), 1);
        }
        WorthQueryManagedLiveCloseOutcome::Stopped(stop) => {
            panic!("close stopped: {:?}", stop.error())
        }
    }
}

#[test]
fn invalid_context_has_zero_planning_and_runtime_work() {
    let mut workspace = fixture::workspace("phase-12-invalid-context");
    let stop = denied_read(&mut workspace);
    assert_eq!(stop.context_admission_attempt_count(), 1);
    assert_eq!(stop.planning_attempt_count(), 0);
    assert_eq!(stop.planning_completed_count(), 0);
    assert_eq!(stop.lower_runtime_execution_attempt_count(), 0);
    assert_eq!(stop.lower_runtime_execution_completed_count(), 0);
}

#[test]
fn unrelated_workspace_size_does_not_change_ergonomic_lowering_work() {
    let mut empty = fixture::workspace("phase-12-bounded-empty");
    let mut unrelated = fixture::workspace("phase-12-bounded-unrelated");
    for index in 0..64 {
        fixture::write_task(&mut unrelated, &format!("unrelated-{index}"));
    }
    assert_eq!(denied_read(&mut empty), denied_read(&mut unrelated));
}

fn denied_read(
    workspace: &mut worth_query::facade::runtime::WorthQueryWorkspace,
) -> worth_query::facade::read::WorthQueryReadJourneyCounters {
    use worth_query::facade::read::{
        current, declare, BranchAccessGrant, PolicyEpoch, PolicyRuleSnapshot,
        SchemaVariantSnapshot, TenantBasisEpoch, TenantBindingSnapshot,
    };
    let policy = PolicyRuleSnapshot::synthetic_authority_with_query_admission(
        "phase-12-policy",
        "phase-12-rules",
        PolicyEpoch::Synthetic(1),
        false,
    );
    let tenant = TenantBindingSnapshot::synthetic_direct(
        "tenant-a",
        "main",
        "schema-a",
        TenantBasisEpoch::Synthetic(7),
    );
    let branch = BranchAccessGrant::synthetic_granted("main", &policy);
    let schema = SchemaVariantSnapshot::synthetic_authority("tenant-a", "schema-a", "exact");
    let context = current().under_policy_tenant(policy, tenant, branch, schema);
    *declare(fixture::identity_detail)
        .unwrap()
        .using(context)
        .run(workspace)
        .stop()
        .expect("denied policy must stop")
        .journey_counters()
}
