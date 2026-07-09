use worth_query::facade::runtime::{
    WorthQueryLowerRuntimeDirectImportAuditRow, WorthQueryLowerRuntimeDirectImportPosture,
    WorthQueryLowerRuntimeSeamKey,
};

fn main() {
    let _ = WorthQueryLowerRuntimeDirectImportAuditRow::new(
        WorthQueryLowerRuntimeSeamKey::FrontierSignalAdapterModule,
        "Worthd/module.rs",
        WorthQueryLowerRuntimeDirectImportPosture::AllowedAdapter,
        "Worthd rationale",
    );
}
