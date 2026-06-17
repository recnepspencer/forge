use forge_query::facade::runtime::{
    ForgeQueryLowerRuntimeBoundaryExecutionReceipt, ForgeQueryLowerRuntimeRoutePlan,
};

fn main() {
    let plan: ForgeQueryLowerRuntimeRoutePlan = todo!();
    let _ = ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan(&plan, todo!());
}
