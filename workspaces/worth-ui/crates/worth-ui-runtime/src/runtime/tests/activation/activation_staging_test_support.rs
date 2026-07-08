use super::durable_state_inventory_test_support::platform_inventory;
use super::query_binding_comparison_test_support::{query_artifact, standard_query_app};
use crate::facade::WorthUiApp;
use crate::runtime::{
    WorthUiActivationStagingDenial, WorthUiAdmittedReplacementCandidate,
    WorthUiDurableStateReconciliationPlan, WorthUiNodeReplacementPlan, WorthUiPendingActivation,
    WorthUiPendingExecutionPlanLoweringInput, WorthUiQueryLiveRebindPlan,
    WorthUiReplacementImpactClassification, WorthUiRuntimeHost, WorthUiRuntimeImpactNarrowing,
};

pub(super) fn activation_staging_inputs() -> ActivationStagingInputs {
    let app = standard_query_app();
    let active = query_artifact(&app, "workspace.view_binding.selection");
    let candidate = query_artifact(&app, "workspace.view_binding.selection");
    let runtime = super::replacement_impact_test_support::launch_runtime(&app, active);
    let admitted =
        super::replacement_impact_test_support::admitted_candidate(&app, &runtime, candidate);
    let comparison = runtime
        .compare_admitted_replacement(&admitted)
        .expect("runtime comparison succeeds");
    let impact = runtime
        .classify_replacement_impact(&comparison, &admitted)
        .expect("impact classification succeeds");
    let narrowing = runtime
        .narrow_replacement_impact(&impact, &admitted)
        .expect("impact narrowing succeeds");
    let identity_report = runtime
        .build_identity_match_graph(&narrowing, &admitted)
        .expect("identity matching succeeds");
    let node_plan = runtime
        .classify_node_replacements(&impact, &narrowing, &identity_report)
        .expect("node replacement plan succeeds");
    let inventory = platform_inventory(&runtime)
        .build_for_replacement(&node_plan)
        .expect("inventory builds");
    let reconciliation_plan = runtime
        .reconcile_durable_state(&node_plan, &inventory)
        .expect("state reconciliation succeeds");
    let query_comparison = runtime
        .compare_query_bindings(&node_plan, &narrowing, &admitted)
        .expect("query comparison succeeds");
    let query_rebind_plan = runtime
        .plan_query_live_rebinds(&query_comparison, &node_plan, &narrowing, &admitted)
        .expect("query rebind planning succeeds");
    let pending_execution_plan_lowering_input = runtime
        .prepare_pending_execution_plan_lowering_input(
            &node_plan,
            &reconciliation_plan,
            &query_rebind_plan,
        );

    ActivationStagingInputs {
        app,
        runtime,
        admitted,
        impact,
        narrowing,
        node_plan,
        reconciliation_plan,
        query_rebind_plan,
        pending_execution_plan_lowering_input,
    }
}

pub(super) struct ActivationStagingInputs {
    pub(super) app: WorthUiApp,
    pub(super) runtime: WorthUiRuntimeHost,
    pub(super) admitted: WorthUiAdmittedReplacementCandidate,
    pub(super) impact: WorthUiReplacementImpactClassification,
    pub(super) narrowing: WorthUiRuntimeImpactNarrowing,
    pub(super) node_plan: WorthUiNodeReplacementPlan,
    pub(super) reconciliation_plan: WorthUiDurableStateReconciliationPlan,
    pub(super) query_rebind_plan: WorthUiQueryLiveRebindPlan,
    pub(super) pending_execution_plan_lowering_input: WorthUiPendingExecutionPlanLoweringInput,
}

impl ActivationStagingInputs {
    pub(super) fn into_app_runtime_and_pending(
        self,
    ) -> (WorthUiApp, WorthUiRuntimeHost, WorthUiPendingActivation) {
        let pending = self
            .runtime
            .stage_replacement_activation(
                self.admitted,
                &self.impact,
                &self.narrowing,
                &self.node_plan,
                Some(&self.reconciliation_plan),
                Some(&self.query_rebind_plan),
                Some(&self.pending_execution_plan_lowering_input),
            )
            .expect("activation staging succeeds");
        (self.app, self.runtime, pending)
    }

    pub(super) fn into_runtime_and_pending(self) -> (WorthUiRuntimeHost, WorthUiPendingActivation) {
        let pending = self
            .runtime
            .stage_replacement_activation(
                self.admitted,
                &self.impact,
                &self.narrowing,
                &self.node_plan,
                Some(&self.reconciliation_plan),
                Some(&self.query_rebind_plan),
                Some(&self.pending_execution_plan_lowering_input),
            )
            .expect("activation staging succeeds");
        (self.runtime, pending)
    }

    pub(super) fn stage(self) -> WorthUiPendingActivation {
        self.runtime
            .stage_replacement_activation(
                self.admitted,
                &self.impact,
                &self.narrowing,
                &self.node_plan,
                Some(&self.reconciliation_plan),
                Some(&self.query_rebind_plan),
                Some(&self.pending_execution_plan_lowering_input),
            )
            .expect("activation staging succeeds")
    }

    pub(super) fn stage_denial(self) -> WorthUiActivationStagingDenial {
        self.runtime
            .stage_replacement_activation(
                self.admitted,
                &self.impact,
                &self.narrowing,
                &self.node_plan,
                Some(&self.reconciliation_plan),
                Some(&self.query_rebind_plan),
                Some(&self.pending_execution_plan_lowering_input),
            )
            .expect_err("activation staging denies")
    }

    pub(super) fn stage_without_reconciliation(self) -> WorthUiActivationStagingDenial {
        self.runtime
            .stage_replacement_activation(
                self.admitted,
                &self.impact,
                &self.narrowing,
                &self.node_plan,
                None,
                Some(&self.query_rebind_plan),
                Some(&self.pending_execution_plan_lowering_input),
            )
            .expect_err("missing reconciliation denies")
    }

    pub(super) fn stage_without_query_rebind(self) -> WorthUiActivationStagingDenial {
        self.runtime
            .stage_replacement_activation(
                self.admitted,
                &self.impact,
                &self.narrowing,
                &self.node_plan,
                Some(&self.reconciliation_plan),
                None,
                Some(&self.pending_execution_plan_lowering_input),
            )
            .expect_err("missing query rebind denies")
    }

    pub(super) fn stage_with_reconciliation(
        self,
        reconciliation_plan: &WorthUiDurableStateReconciliationPlan,
    ) -> WorthUiActivationStagingDenial {
        self.runtime
            .stage_replacement_activation(
                self.admitted,
                &self.impact,
                &self.narrowing,
                &self.node_plan,
                Some(reconciliation_plan),
                Some(&self.query_rebind_plan),
                Some(&self.pending_execution_plan_lowering_input),
            )
            .expect_err("stale reconciliation denies")
    }

    pub(super) fn stage_with_query_rebind(
        self,
        query_rebind_plan: &WorthUiQueryLiveRebindPlan,
    ) -> WorthUiActivationStagingDenial {
        self.runtime
            .stage_replacement_activation(
                self.admitted,
                &self.impact,
                &self.narrowing,
                &self.node_plan,
                Some(&self.reconciliation_plan),
                Some(query_rebind_plan),
                Some(&self.pending_execution_plan_lowering_input),
            )
            .expect_err("stale query rebind denies")
    }

    pub(super) fn stage_with_node_plan(
        self,
        node_plan: &WorthUiNodeReplacementPlan,
    ) -> WorthUiActivationStagingDenial {
        self.runtime
            .stage_replacement_activation(
                self.admitted,
                &self.impact,
                &self.narrowing,
                node_plan,
                Some(&self.reconciliation_plan),
                Some(&self.query_rebind_plan),
                Some(&self.pending_execution_plan_lowering_input),
            )
            .expect_err("stale node plan denies")
    }

    pub(super) fn stage_without_plan_lowering_input(self) -> WorthUiActivationStagingDenial {
        self.runtime
            .stage_replacement_activation(
                self.admitted,
                &self.impact,
                &self.narrowing,
                &self.node_plan,
                Some(&self.reconciliation_plan),
                Some(&self.query_rebind_plan),
                None,
            )
            .expect_err("missing plan lowering input denies")
    }

    pub(super) fn stage_with_plan_lowering_input(
        self,
        input: &WorthUiPendingExecutionPlanLoweringInput,
    ) -> WorthUiActivationStagingDenial {
        self.runtime
            .stage_replacement_activation(
                self.admitted,
                &self.impact,
                &self.narrowing,
                &self.node_plan,
                Some(&self.reconciliation_plan),
                Some(&self.query_rebind_plan),
                Some(input),
            )
            .expect_err("stale plan lowering input denies")
    }
}
