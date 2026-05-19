use forge_query::facade::runtime::{
    ForgeQueryLowerRuntimeDirectImportAuditRow, ForgeQueryLowerRuntimeDirectImportPosture,
    ForgeQueryLowerRuntimeSeamKey,
};

fn main() {
    let _ = ForgeQueryLowerRuntimeDirectImportAuditRow::new(
        ForgeQueryLowerRuntimeSeamKey::FrontierSignalAdapterModule,
        "forged/module.rs",
        ForgeQueryLowerRuntimeDirectImportPosture::AllowedAdapter,
        "forged rationale",
    );
}
