use worth_query::facade::runtime::{WorthQueryLowerRuntimeDirectImportPosture, WorthQueryLowerRuntimeSeamKey};
use worth_query::facade::certification::WorthQueryLowerRuntimeDirectImportAuditRow;

fn main() {
    let _ = WorthQueryLowerRuntimeDirectImportAuditRow::new(
        WorthQueryLowerRuntimeSeamKey::FrontierSignalAdapterModule,
        "Worthd/module.rs",
        WorthQueryLowerRuntimeDirectImportPosture::AllowedAdapter,
        "Worthd rationale",
    );
}
