use forge_query::facade::consumer_kit::ForgeQueryGraphObligationInMemorySelectedObligation;
use forge_query::facade::runtime::{
    ForgeQueryGraphObligationKind, ForgeQueryGraphObligationSupportLane,
    ForgeQueryGraphObligationSupportStatus,
};

fn main() {
    let _ = ForgeQueryGraphObligationInMemorySelectedObligation {
        obligation_kind: ForgeQueryGraphObligationKind::BlockingInvariant,
        support_lane: ForgeQueryGraphObligationSupportLane::AssemblyIndexSelection,
        support_status: ForgeQueryGraphObligationSupportStatus::Supported,
        rule_identity_digest: "rule".to_string(),
        registration_digest: "registration".to_string(),
        execution_budget_digest: "budget".to_string(),
        row_digest: "row".to_string(),
    };
}
