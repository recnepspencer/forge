use forge_foundational::facade::{
    FoundationalBoundaryArtifactCategory, FoundationalBoundaryArtifactRole,
    FoundationalBoundaryAvailability, FoundationalBoundaryDeliveryClass,
};
use forge_query::facade::{
    ForgeQueryDeclarationLegalityClass, ForgeQueryDeclarationLegalityContract,
};

fn main() {
    let _ = ForgeQueryDeclarationLegalityContract::new(
        ForgeQueryDeclarationLegalityClass::AuthoritativeHotArtifact,
        FoundationalBoundaryArtifactCategory::Artifact,
        FoundationalBoundaryArtifactRole::AuthoritativeCurrent,
        FoundationalBoundaryDeliveryClass::MustBeHot,
        FoundationalBoundaryAvailability::Present,
    );
}
