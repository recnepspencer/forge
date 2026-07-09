use worth_query::facade::runtime::{
    WorthQueryLowerRuntimeBoundaryEnvelopeBindingTarget, WorthQuerySupportContributionAuthoring,
};

fn cannot_use_binding_target_as_source(target: &WorthQueryLowerRuntimeBoundaryEnvelopeBindingTarget) {
    let _ = WorthQuerySupportContributionAuthoring::narrowed_support("routing", "detail")
        .for_lower_runtime_boundary_source(target);
}

fn main() {}
