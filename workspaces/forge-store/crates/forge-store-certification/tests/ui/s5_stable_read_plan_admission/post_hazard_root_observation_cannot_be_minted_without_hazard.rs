use forge_store_physical_isolation::{
    CurrentPhysicalRoot, PhysicalReadStabilityAuthority, PostHazardRootObservation,
    ProtectedPhysicalReference,
};

fn main() {
    let authority: PhysicalReadStabilityAuthority = todo!();
    let root: CurrentPhysicalRoot = todo!();
    let references: &[ProtectedPhysicalReference] = &[];
    let _observed =
        PostHazardRootObservation::from_authority_current_root(&authority, root, references);
}
