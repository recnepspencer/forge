use worth_foundational::facade::{
    FoundationalBoundaryArtifactCategory, FoundationalBoundaryArtifactRole,
    FoundationalBoundaryAvailability, FoundationalBoundaryDeliveryClass,
};
use worth_query::facade::{
    WorthQueryDeclarationLegalityClass, WorthQueryDeclarationLegalityContract,
};

fn main() {
    let _ = WorthQueryDeclarationLegalityContract::new(
        WorthQueryDeclarationLegalityClass::AuthoritativeHotArtifact,
        FoundationalBoundaryArtifactCategory::Artifact,
        FoundationalBoundaryArtifactRole::AuthoritativeCurrent,
        FoundationalBoundaryDeliveryClass::MustBeHot,
        FoundationalBoundaryAvailability::Present,
    );
}
