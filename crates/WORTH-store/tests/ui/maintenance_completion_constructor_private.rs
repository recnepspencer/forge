use worth_store::{
    CompletedMaintenance, MaintenanceDeclaration, MaintenanceDeclarationId,
    RetentionMaintenanceDeclaration,
};

fn main() {
    let declaration = MaintenanceDeclaration::retention(
        MaintenanceDeclarationId::new("synthetic"),
        RetentionMaintenanceDeclaration::new("batch", 0, 0),
    );
    let _ = CompletedMaintenance::new(declaration, "WORTHd");
}
