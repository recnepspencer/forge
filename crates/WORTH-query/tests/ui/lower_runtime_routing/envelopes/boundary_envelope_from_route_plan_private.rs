use worth_query::facade::{
    WorthQueryLowerRuntimeBoundaryEnvelope, WorthQueryLowerRuntimeBoundaryExecutionReceipt,
    WorthQueryLowerRuntimeRoutePlan, WorthQueryLowerRuntimeSeamKey,
};

fn from_route_plan(
    seam: WorthQueryLowerRuntimeSeamKey,
    plan: &WorthQueryLowerRuntimeRoutePlan,
    receipt: &WorthQueryLowerRuntimeBoundaryExecutionReceipt,
) {
    let _ = WorthQueryLowerRuntimeBoundaryEnvelope::from_route_plan(
        seam,
        plan,
        receipt,
        todo!(),
    );
}

fn main() {}
