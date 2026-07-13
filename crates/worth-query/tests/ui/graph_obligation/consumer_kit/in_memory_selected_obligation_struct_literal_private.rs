use worth_query::facade::consumer_kit::WorthQueryGraphObligationInMemorySelectedObligation;
use worth_query::facade::runtime::{WorthQueryGraphObligationKind, WorthQueryGraphObligationSupportLane, WorthQueryGraphObligationSupportStatus};

fn main() {
    let _ = WorthQueryGraphObligationInMemorySelectedObligation {
        obligation_kind: WorthQueryGraphObligationKind::BlockingInvariant,
        support_lane: WorthQueryGraphObligationSupportLane::AssemblyIndexSelection,
        support_status: WorthQueryGraphObligationSupportStatus::Supported,
        rule_identity_digest: "rule".to_string(),
        registration_digest: "registration".to_string(),
        execution_budget_digest: "budget".to_string(),
        row_digest: "row".to_string(),
    };
}
