use super::activation_staging_test_support::activation_staging_inputs;
use super::allocation_planning_test_support::{
    allocation_planning, independent_allocation_planning,
};
use crate::facade::WorthUi;
use crate::runtime::tests::source_ingress_boundary_test_support::lower_file_submission;
use crate::runtime::{
    WorthUiExecutionLane, WorthUiExecutionLaneSupport, WorthUiExecutionPlan,
    WorthUiExecutionPlanInput, WorthUiLaneAdmission, WorthUiPlanNodeInputFamily,
    WorthUiSourceProvider, WorthUiVirtualizedDataFrameDenial, WorthUiVirtualizedDataFrameReceipt,
    WorthUiVirtualizedDataFrameTarget, WorthUiVirtualizedDataPlanDenial,
    WorthUiVirtualizedPlanSummary, WorthUiVirtualizedPlanSummaryRequest, WorthUiWatcherEvent,
};

pub(super) fn virtualized_data_fixture() -> VirtualizedDataFixture {
    VirtualizedDataFixture::new("virtualized-data.active")
}

pub(super) fn virtualized_data_denial_for_missing_support(
    removed_lane: WorthUiExecutionLane,
) -> WorthUiVirtualizedDataPlanDenial {
    let context = virtualized_data_context();
    let admission = if removed_lane == WorthUiExecutionLane::QueryBound {
        admission_without_query_bound(&context)
    } else {
        let planning = independent_allocation_planning("virtualized-data.missing-support");
        let facts = context
            .runtime
            .detached_execution_plan_lowering_facts_for_test(&planning, context.plan_input.clone());
        context
            .runtime
            .admit_execution_lanes(
                &facts,
                &WorthUiExecutionLaneSupport::without_lane_for_test(removed_lane),
            )
            .expect("narrower lane admission succeeds")
    };
    crate::runtime::execution::virtualized_data_lane::WorthUiVirtualizedDataPlanBuilder::build(
        &context.execution_plan,
        &admission,
    )
    .expect_err("virtualized data plan rejects missing support")
}

pub(super) fn virtualized_data_denial_for_stale_lane_admission() -> WorthUiVirtualizedDataPlanDenial
{
    let context = virtualized_data_context();
    let drifted_plan_input = plan_input_with_duplicate_query_input(&context.plan_input);
    let receipt_runtime = fresh_runtime();
    let drifted_planning = independent_allocation_planning("virtualized-data.stale-admission");
    let drifted_facts = receipt_runtime
        .detached_execution_plan_lowering_facts_for_test(&drifted_planning, drifted_plan_input);
    let stale_admission = receipt_runtime
        .admit_execution_lanes(
            &drifted_facts,
            &WorthUiExecutionLaneSupport::platform_default(),
        )
        .expect("drifted lane admission still has data and Query support");
    crate::runtime::execution::virtualized_data_lane::WorthUiVirtualizedDataPlanBuilder::build(
        &context.execution_plan,
        &stale_admission,
    )
    .expect_err("virtualized data plan rejects stale lane admission")
}

pub(super) struct VirtualizedDataFixture {
    pub(super) session: crate::facade::WorthUiActiveApplicationSession,
}

impl VirtualizedDataFixture {
    fn new(label: &str) -> Self {
        let mut query =
            worth_ui_query_binding::certification::WorthUiInstalledQueryTestFixture::new(label);
        let snapshot = WorthUi::app()
            .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
            .register_query_view(query.installed_view())
            .expect("installed view registers")
            .freeze()
            .expect("query snapshot prepares");
        let submission = lower_file_submission(
            WorthUiSourceProvider::in_memory(label)
                .with_file("app/main.wui", "binding inspector.measurements {}"),
            [WorthUiWatcherEvent::provider_revision(label)],
            snapshot.capabilities(),
        );
        let mut session = WorthUi::app()
            .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
            .register_query_view(query.installed_view())
            .expect("installed view registers for active app")
            .with_candidate_submission(submission)
            .freeze()
            .expect("query source app prepares")
            .launch()
            .expect("query source app launches");
        let projection = query.settle_snapshot();
        let fact_link = session
            .query_fact_link("inspector.measurements")
            .expect("active plan retains the Query fact link");
        let mut admission = None;
        let completion = session
            .execute_framework_turn(|turn| {
                turn.query_projection(|source| {
                    admission = Some(
                        source
                            .admit_settled(projection)
                            .map(|_| source.submit_settled(&fact_link)),
                    );
                });
            })
            .expect("no mounted presentation lease is active");
        drop(completion.into_completion());
        admission
            .expect("projection source executes")
            .expect("projection admits through exact installed binding")
            .expect("settled fact submits through the compact plan link");
        Self { session }
    }

    pub(super) fn summary(&self) -> WorthUiVirtualizedPlanSummary {
        self.session
            .inspect_virtualized_plan(WorthUiVirtualizedPlanSummaryRequest::first_view())
            .expect("active virtualized summary resolves")
    }

    pub(super) fn execute(
        &mut self,
        target: WorthUiVirtualizedDataFrameTarget,
    ) -> Result<WorthUiVirtualizedDataFrameReceipt, WorthUiVirtualizedDataFrameDenial> {
        let completion = self
            .session
            .execute_framework_turn(|_| {})
            .expect("no mounted presentation lease is active");
        let execution = match completion.into_execution() {
            Ok(execution) => execution,
            Err(_) => panic!("no-ingress framework turn yields execution"),
        };
        execution
            .execute_virtualized_data_frame(target)
            .map(|completion| completion.receipt().clone())
    }
}

struct VirtualizedDataContext {
    runtime: crate::runtime::WorthUiRuntimeFrameworkLoop,
    plan_input: WorthUiExecutionPlanInput,
    execution_plan: WorthUiExecutionPlan,
}

fn virtualized_data_context() -> VirtualizedDataContext {
    let inputs = activation_staging_inputs();
    let plan_input = inputs
        .runtime
        .prepare_reconstructive_plan_input_for_test(&inputs.admitted, &[]);
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let planning = allocation_planning(&runtime, &pending, "virtualized-data.fixture");
    let facts =
        runtime.detached_execution_plan_lowering_facts_for_test(&planning, plan_input.clone());
    let allocation = runtime
        .allocate_runtime_handles(&facts)
        .expect("handle allocation succeeds");
    let lane_admission = runtime
        .admit_execution_lanes(&facts, &WorthUiExecutionLaneSupport::platform_default())
        .expect("lane admission succeeds");
    let execution_plan = runtime
        .assemble_execution_plan_topology_with_lane_admission(&facts, &allocation, &lane_admission)
        .expect("execution plan topology assembles");
    VirtualizedDataContext {
        runtime,
        plan_input,
        execution_plan,
    }
}

fn admission_without_query_bound(context: &VirtualizedDataContext) -> WorthUiLaneAdmission {
    let no_query_plan_input = plan_input_without_family(
        &context.plan_input,
        WorthUiPlanNodeInputFamily::QueryViewBinding,
    );
    let receipt_runtime = fresh_runtime();
    let planning = independent_allocation_planning("virtualized-data.without-query-bound");
    let facts = receipt_runtime
        .detached_execution_plan_lowering_facts_for_test(&planning, no_query_plan_input);
    receipt_runtime
        .admit_execution_lanes(
            &facts,
            &WorthUiExecutionLaneSupport::without_lane_for_test(WorthUiExecutionLane::QueryBound),
        )
        .expect("query-free input can be admitted without QueryBound support")
}

fn fresh_runtime() -> crate::runtime::WorthUiRuntimeFrameworkLoop {
    activation_staging_inputs().into_runtime_and_pending().0
}

fn plan_input_without_family(
    plan_input: &WorthUiExecutionPlanInput,
    family: WorthUiPlanNodeInputFamily,
) -> WorthUiExecutionPlanInput {
    WorthUiExecutionPlanInput::new(
        plan_input.basis().clone(),
        plan_input.context().clone(),
        plan_input
            .node_inputs()
            .iter()
            .filter(|node_input| node_input.family() != family)
            .cloned()
            .collect(),
        plan_input.counters(),
    )
}

fn plan_input_with_duplicate_query_input(
    plan_input: &WorthUiExecutionPlanInput,
) -> WorthUiExecutionPlanInput {
    let duplicated = plan_input
        .node_inputs()
        .iter()
        .find(|node_input| node_input.family() == WorthUiPlanNodeInputFamily::QueryViewBinding)
        .expect("fixture has a Query input")
        .clone();
    let mut node_inputs = plan_input.node_inputs().to_vec();
    node_inputs.push(duplicated);
    WorthUiExecutionPlanInput::new(
        plan_input.basis().clone(),
        plan_input.context().clone(),
        node_inputs,
        plan_input.counters(),
    )
}
