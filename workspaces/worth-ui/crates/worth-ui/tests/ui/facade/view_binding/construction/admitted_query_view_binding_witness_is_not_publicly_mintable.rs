use worth_query::facade::{WorthQueryCapabilityFamily, WorthQueryCapabilityStatus};
use worth_ui::facade::QueryViewCapabilityReference;

fn main() {
    let _witness = QueryViewCapabilityReference {
        family: WorthQueryCapabilityFamily::QueryComposition,
        status: WorthQueryCapabilityStatus::Admitted,
        reason: "local admission claim",
    };
}
