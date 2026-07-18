/// Comparison evidence for the exact Query plan retained by a prepared
/// generation. The plan stays private so this identity cannot activate Query.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiPreparedQueryBindingPlanIdentity {
    plan: worth_ui_query_binding::WorthUiQueryBindingPlan,
}

impl WorthUiPreparedQueryBindingPlanIdentity {
    pub(crate) fn derive(plan: &worth_ui_query_binding::WorthUiQueryBindingPlan) -> Self {
        Self { plan: plan.clone() }
    }
}
