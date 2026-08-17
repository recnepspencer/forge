use super::durable_state_inventory_test_support::platform_inventory;
use super::query_binding_comparison_test_support::{query_artifact, standard_query_app};
use crate::facade::WorthUiApp;
use crate::runtime::{
    WorthUiActivationStagingDenial, WorthUiActivationStagingPlans,
    WorthUiAdmittedReplacementCandidate, WorthUiDurableStateReconciliationPlan,
    WorthUiNodeReplacementPlan, WorthUiPendingActivation, WorthUiQueryLiveRebindPlan,
    WorthUiReplacementImpactClassification, WorthUiRuntimeImpactNarrowing,
};

fn staging_plans<'a>(
    reconciliation_plan: Option<&'a WorthUiDurableStateReconciliationPlan>,
    query_rebind_plan: Option<&'a WorthUiQueryLiveRebindPlan>,
) -> WorthUiActivationStagingPlans<'a> {
    WorthUiActivationStagingPlans::new(reconciliation_plan, query_rebind_plan)
}

pub(crate) fn activation_staging_inputs() -> ActivationStagingInputs {
    let app = standard_query_app();
    activation_staging_inputs_for(app)
}

pub(crate) fn activation_staging_inputs_with_installed_query_view(
    view: worth_ui_query_binding::WorthUiInstalledQueryView,
) -> ActivationStagingInputs {
    let binding_id = view.definition().identity().as_str().to_owned();
    let app = crate::facade::WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .register_query_view(view)
        .expect("installed Query view registers for activation")
        .freeze()
        .map(crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host)
        .expect("Query-bound activation application prepares");
    activation_staging_inputs_for_bindings(app, &binding_id, &binding_id)
}

pub(crate) fn activation_staging_inputs_with_query_change() -> ActivationStagingInputs {
    let app = standard_query_app();
    activation_staging_inputs_for_bindings(
        app,
        "workspace.view_binding.selection",
        "workspace.view_binding.detail",
    )
}

pub(super) fn activation_staging_inputs_for(app: WorthUiApp) -> ActivationStagingInputs {
    activation_staging_inputs_for_bindings(
        app,
        "workspace.view_binding.selection",
        "workspace.view_binding.selection",
    )
}

fn activation_staging_inputs_for_bindings(
    app: WorthUiApp,
    active_binding_id: &str,
    candidate_binding_id: &str,
) -> ActivationStagingInputs {
    let active = query_artifact(&app, active_binding_id);
    let candidate = query_artifact(&app, candidate_binding_id);
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
    ActivationStagingInputs {
        app,
        runtime,
        admitted,
        impact,
        narrowing,
        node_plan,
        reconciliation_plan,
        query_rebind_plan,
    }
}

pub(crate) struct ActivationStagingInputs {
    pub(super) app: WorthUiApp,
    pub(super) runtime: crate::runtime::WorthUiRuntimeFrameworkLoop,
    pub(super) admitted: WorthUiAdmittedReplacementCandidate,
    pub(super) impact: WorthUiReplacementImpactClassification,
    pub(super) narrowing: WorthUiRuntimeImpactNarrowing,
    pub(super) node_plan: WorthUiNodeReplacementPlan,
    pub(super) reconciliation_plan: WorthUiDurableStateReconciliationPlan,
    pub(super) query_rebind_plan: WorthUiQueryLiveRebindPlan,
}

impl ActivationStagingInputs {
    pub(crate) fn reconstructive_plan_input(
        &self,
        component_hooks: &[crate::runtime::WorthUiComponentLoweringHook],
    ) -> crate::runtime::WorthUiExecutionPlanInput {
        self.runtime
            .prepare_reconstructive_plan_input_for_test(&self.admitted, component_hooks)
    }

    pub(crate) fn into_app_runtime_and_pending(
        self,
    ) -> (
        WorthUiApp,
        crate::runtime::WorthUiRuntimeFrameworkLoop,
        WorthUiPendingActivation,
    ) {
        let pending = self
            .runtime
            .stage_replacement_activation(
                self.admitted,
                &self.impact,
                &self.narrowing,
                &self.node_plan,
                staging_plans(
                    Some(&self.reconciliation_plan),
                    Some(&self.query_rebind_plan),
                ),
            )
            .expect("activation staging succeeds");
        (self.app, self.runtime, pending)
    }

    pub(crate) fn into_runtime_and_pending(
        self,
    ) -> (
        crate::runtime::WorthUiRuntimeFrameworkLoop,
        WorthUiPendingActivation,
    ) {
        let pending = self
            .runtime
            .stage_replacement_activation(
                self.admitted,
                &self.impact,
                &self.narrowing,
                &self.node_plan,
                staging_plans(
                    Some(&self.reconciliation_plan),
                    Some(&self.query_rebind_plan),
                ),
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
                staging_plans(
                    Some(&self.reconciliation_plan),
                    Some(&self.query_rebind_plan),
                ),
            )
            .expect("activation staging succeeds")
    }

    pub(super) fn stage_without_reconciliation(self) -> WorthUiActivationStagingDenial {
        self.runtime
            .stage_replacement_activation(
                self.admitted,
                &self.impact,
                &self.narrowing,
                &self.node_plan,
                staging_plans(None, Some(&self.query_rebind_plan)),
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
                staging_plans(Some(&self.reconciliation_plan), None),
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
                staging_plans(Some(reconciliation_plan), Some(&self.query_rebind_plan)),
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
                staging_plans(Some(&self.reconciliation_plan), Some(query_rebind_plan)),
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
                staging_plans(
                    Some(&self.reconciliation_plan),
                    Some(&self.query_rebind_plan),
                ),
            )
            .expect_err("stale node plan denies")
    }
}
