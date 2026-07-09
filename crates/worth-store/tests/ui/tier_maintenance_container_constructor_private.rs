use worth_store::{MaintenanceDeclarationId, TierMoveMaintenanceDeclaration};

fn main() {
    let _ = TierMoveMaintenanceDeclaration::new(
        "snapshot_family",
        "family:tier-local",
        "move:cold-placement",
        true,
    );
    let _ = MaintenanceDeclarationId::new("maintenance-tier-move").as_str();
}
