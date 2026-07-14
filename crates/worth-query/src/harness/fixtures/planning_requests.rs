use crate::facade::policy::{
    planning_request_context_for_bound, planning_request_context_for_direct,
    PlanningAmbientContext, PlanningRequestContext, PlanningSemanticInputs,
};
use crate::facade::runtime::ValidatedQueryBundle;

pub fn direct_runtime_request(bundle: &ValidatedQueryBundle) -> PlanningRequestContext {
    planning_request_context_for_direct(bundle, super::resolved_bases::runtime_basis_intent())
        .unwrap()
}

pub fn bound_runtime_request(bundle: &ValidatedQueryBundle, value: &str) -> PlanningRequestContext {
    planning_request_context_for_bound(
        bundle,
        super::resolved_bases::runtime_basis_intent(),
        super::binding_resolutions::root_bound_bindings(value),
        Vec::new(),
    )
    .unwrap()
}

pub fn pre_resolved_bound_runtime_request(
    bundle: &ValidatedQueryBundle,
    value: &str,
) -> PlanningRequestContext {
    PlanningRequestContext::new(
        PlanningSemanticInputs::new(
            Some(super::binding_resolutions::resolved_root_binding(
                bundle, value,
            )),
            super::resolved_bases::runtime_basis_intent(),
        ),
        PlanningAmbientContext::new(Vec::new()),
    )
}
