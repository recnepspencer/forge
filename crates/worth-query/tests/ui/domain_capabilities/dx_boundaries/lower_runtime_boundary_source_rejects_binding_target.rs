use worth_query::facade::domain::WorthQuerySupportContributionAuthoring;
use worth_query::facade::runtime::WorthQueryLowerRuntimeBoundaryEnvelopeBindingTarget;

fn cannot_use_binding_target_as_source(target: &WorthQueryLowerRuntimeBoundaryEnvelopeBindingTarget) {
    let _ = WorthQuerySupportContributionAuthoring::narrowed_support("routing", "detail")
        .for_lower_runtime_boundary_source(target);
}

fn main() {}
