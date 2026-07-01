#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiDeclarationContainmentIntent {
    RootTopology,
    DeclaredPageSetMembership { page_set_name: Box<str> },
    DeclaredRegionMembership { region_name: Box<str> },
    DeclaredMosaicMembership { mosaic_name: Box<str> },
    DeclaredLocalCompositionMembership { local_composition_name: Box<str> },
    DeclaredControlAttachment { control_name: Box<str> },
    DeclaredDiagnosticAttachment { diagnostic_surface_name: Box<str> },
}

impl UiDeclarationContainmentIntent {
    pub const fn is_root(&self) -> bool {
        matches!(self, Self::RootTopology)
    }

    pub fn claim_name(&self) -> Option<&str> {
        match self {
            Self::RootTopology => None,
            Self::DeclaredPageSetMembership { page_set_name } => Some(page_set_name),
            Self::DeclaredRegionMembership { region_name } => Some(region_name),
            Self::DeclaredMosaicMembership { mosaic_name } => Some(mosaic_name),
            Self::DeclaredLocalCompositionMembership {
                local_composition_name,
            } => Some(local_composition_name),
            Self::DeclaredControlAttachment { control_name } => Some(control_name),
            Self::DeclaredDiagnosticAttachment {
                diagnostic_surface_name,
            } => Some(diagnostic_surface_name),
        }
    }
}
