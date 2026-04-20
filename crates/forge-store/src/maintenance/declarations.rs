mod declaration_enum;
mod declaration_id;
mod payloads;

pub use declaration_enum::MaintenanceDeclaration;
pub use declaration_id::MaintenanceDeclarationId;
pub use payloads::{
    AuthoritativeReclaimMaintenanceDeclaration, CompactionMaintenanceDeclaration,
    MaintenanceDeclarationClass, RebuildMaintenanceDeclaration, ReclaimMaintenanceDeclaration,
    RetentionMaintenanceDeclaration,
};
