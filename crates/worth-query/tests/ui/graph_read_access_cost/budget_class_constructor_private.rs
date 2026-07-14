use worth_query::facade::runtime::{WorthQueryGraphReadBudgetClass, WorthQueryGraphReadBudgetClassKind};

fn main() {
    let _ = WorthQueryGraphReadBudgetClass {
        kind: WorthQueryGraphReadBudgetClassKind::InlineEphemeralCandidate,
    };
}
