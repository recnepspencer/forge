use super::{
    WorthUiProjectionDependencyDeclaration, WorthUiProjectionEquivalenceBasisKind,
    WorthUiProjectionIdentity,
};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthUiProjectionFamily {
    Dropdown,
    HeaderMenu,
    HeaderAppearance,
    HeaderTheme,
    HeaderFrame,
    PageHost,
    PrimitiveProof,
    QueryProjectionConsumption,
    LiveViewExpression,
}

impl WorthUiProjectionFamily {
    pub(crate) fn token(self) -> &'static str {
        match self {
            Self::Dropdown => "dropdown",
            Self::HeaderMenu => "header_menu",
            Self::HeaderAppearance => "header_appearance",
            Self::HeaderTheme => "header_theme",
            Self::HeaderFrame => "header_frame",
            Self::PageHost => "page_host",
            Self::PrimitiveProof => "primitive_proof",
            Self::QueryProjectionConsumption => "query_projection_consumption",
            Self::LiveViewExpression => "live_view_expression",
        }
    }
}

pub trait WorthUiProjectionPlanContract: private::Sealed + Clone {
    fn projection_identity(&self) -> WorthUiProjectionIdentity;
    fn projection_family(&self) -> WorthUiProjectionFamily;
    fn projection_dependency_declaration(&self) -> WorthUiProjectionDependencyDeclaration;
    fn projection_equivalence_digest(&self) -> u64;
    fn projection_equivalence_basis_kind(&self) -> WorthUiProjectionEquivalenceBasisKind;
}

pub(crate) mod private {
    pub trait Sealed {}
}
