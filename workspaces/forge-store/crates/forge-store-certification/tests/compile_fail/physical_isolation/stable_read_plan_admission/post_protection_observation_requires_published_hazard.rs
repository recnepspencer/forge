use forge_store_physical_isolation::{
    CurrentPhysicalRoot, PhysicalReadStabilityAuthority, PostProtectionPhysicalReadObservation,
    ProtectedPhysicalReferenceSet,
};

fn main() {
    let authority: PhysicalReadStabilityAuthority = todo!();
    let root: CurrentPhysicalRoot = todo!();
    let references: ProtectedPhysicalReferenceSet = todo!();
    let _observed =
        PostProtectionPhysicalReadObservation::from_authority_current_root(&authority, root, references);
}
