use forge_query::facade::{
    ForgeQueryLowerRuntimeBoundaryEnvelope, ForgeQueryLowerRuntimeBoundaryExecutionReceipt,
    ForgeQueryLowerRuntimeRoutePlan, ForgeQueryLowerRuntimeSeamKey,
};

fn from_route_plan(
    seam: ForgeQueryLowerRuntimeSeamKey,
    plan: &ForgeQueryLowerRuntimeRoutePlan,
    receipt: &ForgeQueryLowerRuntimeBoundaryExecutionReceipt,
) {
    let _ = ForgeQueryLowerRuntimeBoundaryEnvelope::from_route_plan(
        seam, plan, receipt, "retained",
    );
}

fn main() {}
