use worth_foundational::{
    claim_derived_projection_boundary_surface, FoundationalBoundaryReportSurface,
};
use worth_store_physical_isolation::PhysicalReadStabilityAuthority;

fn require_physical_authority(_: PhysicalReadStabilityAuthority) {}

fn main() {
    let report = FoundationalBoundaryReportSurface::new(vec!["semantic"], 1).unwrap();
    let claim = claim_derived_projection_boundary_surface(report);
    require_physical_authority(claim);
}
