use super::{
    WorthUiAppearanceReloadPackage, WorthUiCapabilityReloadFamilyKind,
    WorthUiCommandProjectionReloadPackage, WorthUiCommandReloadPackage,
    WorthUiComponentReloadPackage, WorthUiDensityReloadPackage, WorthUiThemeTokenReloadPackage,
};

/// Transitional source-shaped capability adapter request.
///
/// This remains available while authored source-package ingress is being unified
/// across the older capability-family proof slices. It is not the ordinary
/// authored-source ingress boundary introduced by Phase 23.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiCapabilityReloadRequest {
    ThemeTokens(WorthUiThemeTokenReloadPackage),
    Commands(WorthUiCommandReloadPackage),
    CommandProjections(WorthUiCommandProjectionReloadPackage),
    Components(WorthUiComponentReloadPackage),
    Appearance(WorthUiAppearanceReloadPackage),
    Density(WorthUiDensityReloadPackage),
    Batch(Vec<WorthUiCapabilityReloadRequest>),
}

impl WorthUiCapabilityReloadRequest {
    pub fn from_theme_tokens(theme_tokens: WorthUiThemeTokenReloadPackage) -> Self {
        Self::ThemeTokens(theme_tokens)
    }

    pub fn from_commands(commands: WorthUiCommandReloadPackage) -> Self {
        Self::Commands(commands)
    }

    pub fn from_command_projections(projections: WorthUiCommandProjectionReloadPackage) -> Self {
        Self::CommandProjections(projections)
    }

    pub fn from_components(components: WorthUiComponentReloadPackage) -> Self {
        Self::Components(components)
    }

    pub fn from_appearance(appearance: WorthUiAppearanceReloadPackage) -> Self {
        Self::Appearance(appearance)
    }

    pub fn from_density(density: WorthUiDensityReloadPackage) -> Self {
        Self::Density(density)
    }

    pub fn batch(requests: impl IntoIterator<Item = WorthUiCapabilityReloadRequest>) -> Self {
        Self::Batch(requests.into_iter().collect())
    }

    pub(crate) fn flattened(self) -> Vec<WorthUiCapabilityReloadRequest> {
        match self {
            Self::Batch(requests) => requests
                .into_iter()
                .flat_map(WorthUiCapabilityReloadRequest::flattened)
                .collect(),
            request => vec![request],
        }
    }

    pub(crate) fn family_kind(&self) -> WorthUiCapabilityReloadFamilyKind {
        match self {
            Self::ThemeTokens(_) => WorthUiCapabilityReloadFamilyKind::ThemeTokens,
            Self::Commands(_) => WorthUiCapabilityReloadFamilyKind::Commands,
            Self::CommandProjections(_) => WorthUiCapabilityReloadFamilyKind::CommandProjections,
            Self::Components(_) => WorthUiCapabilityReloadFamilyKind::Components,
            Self::Appearance(_) => WorthUiCapabilityReloadFamilyKind::Appearance,
            Self::Density(_) => WorthUiCapabilityReloadFamilyKind::Density,
            Self::Batch(_) => panic!("batch requests must be flattened before family inspection"),
        }
    }

    pub(crate) fn source_digest(&self) -> u64 {
        match self {
            Self::ThemeTokens(package) => package.source_digest(),
            Self::Commands(package) => package.source_digest(),
            Self::CommandProjections(package) => package.source_digest(),
            Self::Components(package) => package.source_digest(),
            Self::Appearance(package) => package.source_digest(),
            Self::Density(package) => package.source_digest(),
            Self::Batch(requests) => {
                requests
                    .iter()
                    .fold(0xcbf2_9ce4_8422_2325, |digest, request| {
                        digest.wrapping_mul(0x0000_0100_0000_01b3) ^ request.source_digest()
                    })
            }
        }
    }
}
