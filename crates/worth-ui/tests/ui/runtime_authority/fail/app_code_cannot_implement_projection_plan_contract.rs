use worth_ui::facade::{
    WorthUiProjectionDependencyDeclaration, WorthUiProjectionEquivalenceBasisKind,
    WorthUiProjectionFamily, WorthUiProjectionIdentity, WorthUiProjectionPlanContract,
};

#[derive(Clone)]
struct AppMintedPlan;

impl WorthUiProjectionPlanContract for AppMintedPlan {
    fn projection_identity(&self) -> WorthUiProjectionIdentity {
        unreachable!()
    }

    fn projection_family(&self) -> WorthUiProjectionFamily {
        WorthUiProjectionFamily::HeaderMenu
    }

    fn projection_dependency_declaration(&self) -> WorthUiProjectionDependencyDeclaration {
        unreachable!()
    }

    fn projection_equivalence_digest(&self) -> u64 {
        0
    }

    fn projection_equivalence_basis_kind(&self) -> WorthUiProjectionEquivalenceBasisKind {
        WorthUiProjectionEquivalenceBasisKind::ProjectionDigest
    }
}

fn main() {}
