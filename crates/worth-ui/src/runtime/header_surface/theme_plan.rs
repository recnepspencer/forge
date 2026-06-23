use crate::capability::{CapabilitySnapshot, ThemeTokenId, ThemeTokenValue};
use crate::runtime::{
    WorthUiProjectionDependencyDeclaration, WorthUiProjectionDependencySet,
    WorthUiProjectionEquivalenceBasisKind, WorthUiProjectionFamily, WorthUiProjectionIdentity,
    WorthUiProjectionPlanContract, WorthUiRuntimeFactId,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiHeaderThemeTokenRequest {
    panel_fill: ThemeTokenId,
    menu_fill: ThemeTokenId,
    menu_hover_fill: ThemeTokenId,
    menu_active_fill: ThemeTokenId,
    text: ThemeTokenId,
    border: ThemeTokenId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiHeaderThemePlan {
    receipt: WorthUiHeaderThemeFrameReceipt,
    theme_digest: u64,
    dependencies: WorthUiProjectionDependencySet,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiHeaderThemeFrameReceipt {
    panel_fill: String,
    menu_fill: String,
    menu_hover_fill: String,
    menu_active_fill: String,
    text: String,
    border: String,
    source_parse_count: usize,
    registry_lookup_count: usize,
    artifact_tree_scan_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiHeaderThemePlanDenial {
    MissingToken(String),
    NonColorToken(String),
}

impl WorthUiHeaderThemeTokenRequest {
    pub fn new(
        panel_fill: ThemeTokenId,
        menu_fill: ThemeTokenId,
        menu_hover_fill: ThemeTokenId,
        menu_active_fill: ThemeTokenId,
        text: ThemeTokenId,
        border: ThemeTokenId,
    ) -> Self {
        Self {
            panel_fill,
            menu_fill,
            menu_hover_fill,
            menu_active_fill,
            text,
            border,
        }
    }
}

impl WorthUiHeaderThemePlan {
    pub fn from_snapshot(
        snapshot: &CapabilitySnapshot,
        request: WorthUiHeaderThemeTokenRequest,
    ) -> Result<Self, WorthUiHeaderThemePlanDenial> {
        let dependencies = request.dependencies();
        let receipt = WorthUiHeaderThemeFrameReceipt::new(
            resolve_color(snapshot, &request.panel_fill)?,
            resolve_color(snapshot, &request.menu_fill)?,
            resolve_color(snapshot, &request.menu_hover_fill)?,
            resolve_color(snapshot, &request.menu_active_fill)?,
            resolve_color(snapshot, &request.text)?,
            resolve_color(snapshot, &request.border)?,
        );
        let theme_digest = receipt.digest();
        Ok(Self {
            receipt,
            theme_digest,
            dependencies,
        })
    }

    pub fn execute_frame(&self) -> &WorthUiHeaderThemeFrameReceipt {
        &self.receipt
    }

    pub fn theme_digest(&self) -> u64 {
        self.theme_digest
    }

    pub fn dependencies(&self) -> &WorthUiProjectionDependencySet {
        &self.dependencies
    }
}

impl WorthUiProjectionPlanContract for WorthUiHeaderThemePlan {
    fn projection_identity(&self) -> WorthUiProjectionIdentity {
        WorthUiProjectionIdentity::runtime("worth-ui.header.theme")
    }

    fn projection_family(&self) -> WorthUiProjectionFamily {
        WorthUiProjectionFamily::HeaderTheme
    }

    fn projection_dependency_declaration(&self) -> WorthUiProjectionDependencyDeclaration {
        WorthUiProjectionDependencyDeclaration::from_set(self.dependencies.clone())
    }

    fn projection_equivalence_digest(&self) -> u64 {
        self.theme_digest
    }

    fn projection_equivalence_basis_kind(&self) -> WorthUiProjectionEquivalenceBasisKind {
        WorthUiProjectionEquivalenceBasisKind::ThemeDigest
    }
}

impl crate::runtime::projection_contract::plan_contract::private::Sealed
    for WorthUiHeaderThemePlan
{
}

impl WorthUiHeaderThemeTokenRequest {
    fn dependencies(&self) -> WorthUiProjectionDependencySet {
        [
            &self.panel_fill,
            &self.menu_fill,
            &self.menu_hover_fill,
            &self.menu_active_fill,
            &self.text,
            &self.border,
        ]
        .into_iter()
        .fold(
            WorthUiProjectionDependencySet::empty(),
            |dependencies, token_id| {
                dependencies.depends_on(WorthUiRuntimeFactId::theme_token(token_id))
            },
        )
    }
}

impl WorthUiHeaderThemeFrameReceipt {
    fn new(
        panel_fill: String,
        menu_fill: String,
        menu_hover_fill: String,
        menu_active_fill: String,
        text: String,
        border: String,
    ) -> Self {
        Self {
            panel_fill,
            menu_fill,
            menu_hover_fill,
            menu_active_fill,
            text,
            border,
            source_parse_count: 0,
            registry_lookup_count: 0,
            artifact_tree_scan_count: 0,
        }
    }

    pub fn panel_fill(&self) -> &str {
        &self.panel_fill
    }

    pub fn menu_fill(&self) -> &str {
        &self.menu_fill
    }

    pub fn menu_hover_fill(&self) -> &str {
        &self.menu_hover_fill
    }

    pub fn menu_active_fill(&self) -> &str {
        &self.menu_active_fill
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn border(&self) -> &str {
        &self.border
    }

    pub fn source_parse_count(&self) -> usize {
        self.source_parse_count
    }

    pub fn registry_lookup_count(&self) -> usize {
        self.registry_lookup_count
    }

    pub fn artifact_tree_scan_count(&self) -> usize {
        self.artifact_tree_scan_count
    }

    fn digest(&self) -> u64 {
        [
            self.panel_fill.as_str(),
            self.menu_fill.as_str(),
            self.menu_hover_fill.as_str(),
            self.menu_active_fill.as_str(),
            self.text.as_str(),
            self.border.as_str(),
        ]
        .into_iter()
        .fold(0xcbf2_9ce4_8422_2325, |digest, value| {
            fold_bytes(digest, value.as_bytes())
        })
    }
}

fn resolve_color(
    snapshot: &CapabilitySnapshot,
    token_id: &ThemeTokenId,
) -> Result<String, WorthUiHeaderThemePlanDenial> {
    let descriptor = snapshot
        .theme_tokens()
        .get(token_id)
        .ok_or_else(|| WorthUiHeaderThemePlanDenial::MissingToken(token_id.as_str().to_owned()))?;
    match descriptor.value() {
        Some(ThemeTokenValue::Color(color)) => Ok(color.as_str().to_owned()),
        _ => Err(WorthUiHeaderThemePlanDenial::NonColorToken(
            token_id.as_str().to_owned(),
        )),
    }
}

fn fold_bytes(mut accumulator: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        accumulator ^= u64::from(*byte);
        accumulator = accumulator.wrapping_mul(0x0000_0100_0000_01b3);
    }
    accumulator
}
