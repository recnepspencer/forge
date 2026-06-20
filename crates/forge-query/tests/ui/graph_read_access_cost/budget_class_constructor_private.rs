use forge_query::facade::runtime::{
    ForgeQueryGraphReadBudgetClass, ForgeQueryGraphReadBudgetClassKind,
};

fn main() {
    let _ = ForgeQueryGraphReadBudgetClass {
        kind: ForgeQueryGraphReadBudgetClassKind::InlineEphemeralCandidate,
    };
}
