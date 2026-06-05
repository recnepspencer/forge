use forge_query::facade::runtime::{
    ForgeQueryLowerRuntimeBoundaryEnvelopeBindingTarget, ForgeQuerySupportContributionAuthoring,
};

fn cannot_use_binding_target_as_source(target: &ForgeQueryLowerRuntimeBoundaryEnvelopeBindingTarget) {
    let _ = ForgeQuerySupportContributionAuthoring::narrowed_support("routing", "detail")
        .for_lower_runtime_boundary_source(target);
}

fn main() {}
