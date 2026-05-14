use forge_query::facade::{
    BasisLifecycleMigrationAuditRow, BasisLifecycleMigrationPosture,
    BasisLifecycleMigrationSurface,
};

fn main() {
    let _ = BasisLifecycleMigrationAuditRow {
        surface: BasisLifecycleMigrationSurface::ReadCompositionBasisContext,
        posture: BasisLifecycleMigrationPosture::CompatibilityDebt,
        existing_consumer: "legacy",
        lifecycle_artifact: "lifecycle",
        compatibility_debt: Some("debt"),
        row_digest: String::new(),
    };
}
