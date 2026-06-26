use forge_query::facade::{ForgeQueryCapabilityFamily, ForgeQueryCapabilityStatus};
use worth_ui::facade::QueryViewCapabilityReference;

fn main() {
    let _witness = QueryViewCapabilityReference {
        family: ForgeQueryCapabilityFamily::QueryComposition,
        status: ForgeQueryCapabilityStatus::Admitted,
        reason: "local admission claim",
    };
}
