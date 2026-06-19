use forge_server::ForgeServerLoweredOperationPlan;

fn main() {
    consume(impossible());
}

fn consume(plan: ForgeServerLoweredOperationPlan) {
    let _ = plan.into_query_handoff();
}

fn impossible() -> ForgeServerLoweredOperationPlan {
    loop {}
}
