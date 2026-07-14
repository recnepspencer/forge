use worth_store_physical_isolation::{
    PhysicalReadStabilityAuthority, SemanticVisibilityReference,
};

fn require_physical_authority(_: PhysicalReadStabilityAuthority) {}

fn main() {
    let semantic = SemanticVisibilityReference::relational_snapshot("runtime-a", "snapshot-1");
    require_physical_authority(semantic);
}
