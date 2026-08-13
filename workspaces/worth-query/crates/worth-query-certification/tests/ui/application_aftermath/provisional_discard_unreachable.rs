//! ProvisionalDiscard is unreachable from installed aftermath types (R8.58).

use worth_query_host::facade::domain::{
    InstalledAftermathNextActionContract, PublishedAftermathPosture,
};

fn main() {
    let _ = PublishedAftermathPosture::ProvisionalDiscard;
    let _ = InstalledAftermathNextActionContract::ProvisionalDiscard;
}
