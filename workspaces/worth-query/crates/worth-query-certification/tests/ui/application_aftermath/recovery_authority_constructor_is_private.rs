//! Q8.20 — consumers outside worth-query-execution cannot mint recovery authority.

use worth_query_execution::facade::primary_graph::{
    WorthQueryRecoveryEffectAuthority, WorthQueryRecoveryInspectAuthority,
};

fn main() {
    let _ = WorthQueryRecoveryEffectAuthority {};
    let _ = WorthQueryRecoveryInspectAuthority {};
    let _ = <WorthQueryRecoveryEffectAuthority>::mint;
    let _ = <WorthQueryRecoveryInspectAuthority>::mint;
}
