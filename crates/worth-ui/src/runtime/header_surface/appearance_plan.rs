use crate::capability::{
    AppearanceTokenId, CapabilitySnapshot, DensityTokenId, WorthUiAppearanceTokenDescriptor,
    WorthUiAppearanceValue, WorthUiBorderWidthValue, WorthUiDensityTokenDescriptor,
    WorthUiDensityValue, WorthUiFontSizeValue, WorthUiLengthValue, WorthUiPaddingValue,
    WorthUiShadowValue, WorthUiSpacingValue,
};
use crate::runtime::{
    WorthUiProjectionDependencyDeclaration, WorthUiProjectionDependencySet,
    WorthUiProjectionEquivalenceBasisKind, WorthUiProjectionFamily, WorthUiProjectionIdentity,
    WorthUiProjectionPlanContract, WorthUiRuntimeFactId,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiHeaderAppearanceRequest {
    pub(crate) font_size: AppearanceTokenId,
    pub(crate) menu_min_width: AppearanceTokenId,
    pub(crate) border_width: AppearanceTokenId,
    pub(crate) panel_shadow: AppearanceTokenId,
    pub(crate) row_padding: DensityTokenId,
    pub(crate) container_padding: DensityTokenId,
    pub(crate) control_spacing: DensityTokenId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiHeaderAppearancePlan {
    receipt: WorthUiHeaderAppearanceFrameReceipt,
    appearance_digest: u64,
    dependencies: WorthUiProjectionDependencySet,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiHeaderAppearanceFrameReceipt {
    font_size: WorthUiFontSizeValue,
    menu_min_width: WorthUiLengthValue,
    border_width: WorthUiBorderWidthValue,
    panel_shadow: WorthUiShadowValue,
    row_padding: WorthUiPaddingValue,
    container_padding: WorthUiPaddingValue,
    control_spacing: WorthUiSpacingValue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiHeaderAppearancePlanDenial {
    MissingAppearanceToken(String),
    MissingDensityToken(String),
    WrongAppearanceValue { id: String, expected: &'static str },
    WrongDensityValue { id: String, expected: &'static str },
}

impl WorthUiHeaderAppearanceRequest {
    pub fn new(
        font_size: AppearanceTokenId,
        menu_min_width: AppearanceTokenId,
        border_width: AppearanceTokenId,
        panel_shadow: AppearanceTokenId,
        row_padding: DensityTokenId,
        container_padding: DensityTokenId,
        control_spacing: DensityTokenId,
    ) -> Self {
        Self {
            font_size,
            menu_min_width,
            border_width,
            panel_shadow,
            row_padding,
            container_padding,
            control_spacing,
        }
    }

    fn dependencies(&self) -> WorthUiProjectionDependencySet {
        let appearance_dependencies = [
            &self.font_size,
            &self.menu_min_width,
            &self.border_width,
            &self.panel_shadow,
        ]
        .into_iter()
        .fold(
            WorthUiProjectionDependencySet::empty(),
            |dependencies, token_id| {
                dependencies.depends_on(WorthUiRuntimeFactId::appearance_token(token_id))
            },
        );

        [
            &self.row_padding,
            &self.container_padding,
            &self.control_spacing,
        ]
        .into_iter()
        .fold(appearance_dependencies, |dependencies, token_id| {
            dependencies.depends_on(WorthUiRuntimeFactId::density_token(token_id))
        })
    }
}

impl WorthUiHeaderAppearancePlan {
    pub fn from_snapshot(
        snapshot: &CapabilitySnapshot,
        request: WorthUiHeaderAppearanceRequest,
    ) -> Result<Self, WorthUiHeaderAppearancePlanDenial> {
        let receipt = WorthUiHeaderAppearanceFrameReceipt {
            font_size: resolve_font_size(snapshot, &request.font_size)?,
            menu_min_width: resolve_length(snapshot, &request.menu_min_width, "Length")?,
            border_width: resolve_border_width(snapshot, &request.border_width)?,
            panel_shadow: resolve_shadow(snapshot, &request.panel_shadow)?,
            row_padding: resolve_padding(snapshot, &request.row_padding, "Padding")?,
            container_padding: resolve_padding(snapshot, &request.container_padding, "Padding")?,
            control_spacing: resolve_spacing(snapshot, &request.control_spacing)?,
        };
        let appearance_digest = receipt.digest();
        Ok(Self {
            receipt,
            appearance_digest,
            dependencies: request.dependencies(),
        })
    }

    pub fn execute_frame(&self) -> &WorthUiHeaderAppearanceFrameReceipt {
        &self.receipt
    }

    pub fn appearance_digest(&self) -> u64 {
        self.appearance_digest
    }

    pub fn dependencies(&self) -> &WorthUiProjectionDependencySet {
        &self.dependencies
    }
}

impl WorthUiProjectionPlanContract for WorthUiHeaderAppearancePlan {
    fn projection_identity(&self) -> WorthUiProjectionIdentity {
        WorthUiProjectionIdentity::runtime("worth-ui.header.appearance")
    }

    fn projection_family(&self) -> WorthUiProjectionFamily {
        WorthUiProjectionFamily::HeaderAppearance
    }

    fn projection_dependency_declaration(&self) -> WorthUiProjectionDependencyDeclaration {
        WorthUiProjectionDependencyDeclaration::from_set(self.dependencies.clone())
    }

    fn projection_equivalence_digest(&self) -> u64 {
        self.appearance_digest
    }

    fn projection_equivalence_basis_kind(&self) -> WorthUiProjectionEquivalenceBasisKind {
        WorthUiProjectionEquivalenceBasisKind::ProjectionDigest
    }
}

impl crate::runtime::projection_contract::plan_contract::private::Sealed
    for WorthUiHeaderAppearancePlan
{
}

impl WorthUiHeaderAppearanceFrameReceipt {
    pub fn font_size(&self) -> WorthUiFontSizeValue {
        self.font_size
    }

    pub fn menu_min_width(&self) -> WorthUiLengthValue {
        self.menu_min_width
    }

    pub fn border_width(&self) -> WorthUiBorderWidthValue {
        self.border_width
    }

    pub fn panel_shadow(&self) -> &WorthUiShadowValue {
        &self.panel_shadow
    }

    pub fn row_padding(&self) -> &WorthUiPaddingValue {
        &self.row_padding
    }

    pub fn container_padding(&self) -> &WorthUiPaddingValue {
        &self.container_padding
    }

    pub fn control_spacing(&self) -> WorthUiSpacingValue {
        self.control_spacing
    }

    fn digest(&self) -> u64 {
        [
            self.font_size.digest_basis(),
            self.menu_min_width.digest_basis(),
            self.border_width.digest_basis(),
            self.panel_shadow.digest_basis(),
            self.row_padding.digest_basis(),
            self.container_padding.digest_basis(),
            self.control_spacing.digest_basis(),
        ]
        .into_iter()
        .fold(0xcbf2_9ce4_8422_2325, |digest, value| {
            fold_bytes(digest, value.as_bytes())
        })
    }
}

fn resolve_font_size(
    snapshot: &CapabilitySnapshot,
    token_id: &AppearanceTokenId,
) -> Result<WorthUiFontSizeValue, WorthUiHeaderAppearancePlanDenial> {
    match appearance_descriptor(snapshot, token_id)?.value() {
        WorthUiAppearanceValue::FontSize(value) => Ok(*value),
        _ => Err(WorthUiHeaderAppearancePlanDenial::WrongAppearanceValue {
            id: token_id.as_str().to_owned(),
            expected: "FontSize",
        }),
    }
}

fn resolve_length(
    snapshot: &CapabilitySnapshot,
    token_id: &AppearanceTokenId,
    expected: &'static str,
) -> Result<WorthUiLengthValue, WorthUiHeaderAppearancePlanDenial> {
    match appearance_descriptor(snapshot, token_id)?.value() {
        WorthUiAppearanceValue::Length(value) => Ok(*value),
        _ => Err(WorthUiHeaderAppearancePlanDenial::WrongAppearanceValue {
            id: token_id.as_str().to_owned(),
            expected,
        }),
    }
}

fn resolve_border_width(
    snapshot: &CapabilitySnapshot,
    token_id: &AppearanceTokenId,
) -> Result<WorthUiBorderWidthValue, WorthUiHeaderAppearancePlanDenial> {
    match appearance_descriptor(snapshot, token_id)?.value() {
        WorthUiAppearanceValue::BorderWidth(value) => Ok(*value),
        _ => Err(WorthUiHeaderAppearancePlanDenial::WrongAppearanceValue {
            id: token_id.as_str().to_owned(),
            expected: "BorderWidth",
        }),
    }
}

fn resolve_shadow(
    snapshot: &CapabilitySnapshot,
    token_id: &AppearanceTokenId,
) -> Result<WorthUiShadowValue, WorthUiHeaderAppearancePlanDenial> {
    match appearance_descriptor(snapshot, token_id)?.value() {
        WorthUiAppearanceValue::Shadow(value) => Ok(value.clone()),
        _ => Err(WorthUiHeaderAppearancePlanDenial::WrongAppearanceValue {
            id: token_id.as_str().to_owned(),
            expected: "Shadow",
        }),
    }
}

fn resolve_padding(
    snapshot: &CapabilitySnapshot,
    token_id: &DensityTokenId,
    expected: &'static str,
) -> Result<WorthUiPaddingValue, WorthUiHeaderAppearancePlanDenial> {
    match density_descriptor(snapshot, token_id)?.value() {
        WorthUiDensityValue::Padding(value) => Ok(value.clone()),
        _ => Err(WorthUiHeaderAppearancePlanDenial::WrongDensityValue {
            id: token_id.as_str().to_owned(),
            expected,
        }),
    }
}

fn resolve_spacing(
    snapshot: &CapabilitySnapshot,
    token_id: &DensityTokenId,
) -> Result<WorthUiSpacingValue, WorthUiHeaderAppearancePlanDenial> {
    match density_descriptor(snapshot, token_id)?.value() {
        WorthUiDensityValue::Spacing(value) => Ok(*value),
        _ => Err(WorthUiHeaderAppearancePlanDenial::WrongDensityValue {
            id: token_id.as_str().to_owned(),
            expected: "Spacing",
        }),
    }
}

fn appearance_descriptor<'a>(
    snapshot: &'a CapabilitySnapshot,
    token_id: &AppearanceTokenId,
) -> Result<&'a WorthUiAppearanceTokenDescriptor, WorthUiHeaderAppearancePlanDenial> {
    snapshot.appearance_tokens().get(token_id).ok_or_else(|| {
        WorthUiHeaderAppearancePlanDenial::MissingAppearanceToken(token_id.as_str().to_owned())
    })
}

fn density_descriptor<'a>(
    snapshot: &'a CapabilitySnapshot,
    token_id: &DensityTokenId,
) -> Result<&'a WorthUiDensityTokenDescriptor, WorthUiHeaderAppearancePlanDenial> {
    snapshot.density_tokens().get(token_id).ok_or_else(|| {
        WorthUiHeaderAppearancePlanDenial::MissingDensityToken(token_id.as_str().to_owned())
    })
}

fn fold_bytes(mut accumulator: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        accumulator ^= u64::from(*byte);
        accumulator = accumulator.wrapping_mul(0x0000_0100_0000_01b3);
    }
    accumulator
}

#[cfg(test)]
mod tests {
    use crate::facade::{
        ThemeColorValue, WorthUi, WorthUiAppearanceFamily, WorthUiAppearanceTokenDescriptor,
        WorthUiAppearanceTokenSource, WorthUiAppearanceValue, WorthUiBorderWidthValue,
        WorthUiDensityFamily, WorthUiDensityTokenDescriptor, WorthUiDensityValue,
        WorthUiFontSizeValue, WorthUiLengthValue, WorthUiPaddingValue, WorthUiShadowValue,
        WorthUiSpacingValue,
    };
    use crate::runtime::WorthUiRuntimeFactSet;

    use super::*;

    #[test]
    fn header_appearance_plan_declares_exact_appearance_and_density_dependencies() {
        let snapshot = snapshot_with_font_size("13px");
        let request = header_request();

        let plan = WorthUiHeaderAppearancePlan::from_snapshot(&snapshot, request)
            .expect("header appearance plan builds");
        let changed = WorthUiRuntimeFactSet::empty()
            .with(WorthUiRuntimeFactId::appearance_token(
                &AppearanceTokenId::new("appearance.header.font_size").unwrap(),
            ))
            .with(WorthUiRuntimeFactId::density_token(
                &DensityTokenId::new("density.header.container_padding").unwrap(),
            ));

        assert!(plan.dependencies().intersects(&changed));
        assert!(plan
            .dependencies()
            .contains_exact(&WorthUiRuntimeFactId::appearance_token(
                &AppearanceTokenId::new("appearance.header.font_size").unwrap(),
            )));
        assert!(plan
            .dependencies()
            .contains_exact(&WorthUiRuntimeFactId::density_token(
                &DensityTokenId::new("density.header.container_padding").unwrap(),
            )));
    }

    #[test]
    fn header_appearance_plan_digest_changes_when_font_size_changes() {
        let compact = WorthUiHeaderAppearancePlan::from_snapshot(
            &snapshot_with_font_size("13px"),
            header_request(),
        )
        .expect("compact plan builds");
        let roomy = WorthUiHeaderAppearancePlan::from_snapshot(
            &snapshot_with_font_size("15px"),
            header_request(),
        )
        .expect("roomy plan builds");

        assert_ne!(compact.appearance_digest(), roomy.appearance_digest());
        assert_eq!(compact.receipt.row_padding(), roomy.receipt.row_padding());
    }

    fn snapshot_with_font_size(font_size: &str) -> CapabilitySnapshot {
        WorthUi::app()
            .register_appearance_token(WorthUiAppearanceTokenDescriptor::define(
                AppearanceTokenId::new("appearance.header.font_size").unwrap(),
                WorthUiAppearanceFamily::Typography,
                WorthUiAppearanceTokenSource::Application,
                WorthUiAppearanceValue::FontSize(WorthUiFontSizeValue::from_px(font_size).unwrap()),
            ))
            .register_appearance_token(WorthUiAppearanceTokenDescriptor::define(
                AppearanceTokenId::new("appearance.header.menu_min_width").unwrap(),
                WorthUiAppearanceFamily::Layout,
                WorthUiAppearanceTokenSource::Application,
                WorthUiAppearanceValue::Length(WorthUiLengthValue::from_px("220px").unwrap()),
            ))
            .register_appearance_token(WorthUiAppearanceTokenDescriptor::define(
                AppearanceTokenId::new("appearance.header.border_width").unwrap(),
                WorthUiAppearanceFamily::Border,
                WorthUiAppearanceTokenSource::Application,
                WorthUiAppearanceValue::BorderWidth(
                    WorthUiBorderWidthValue::from_px("1px").unwrap(),
                ),
            ))
            .register_appearance_token(WorthUiAppearanceTokenDescriptor::define(
                AppearanceTokenId::new("appearance.header.panel_shadow").unwrap(),
                WorthUiAppearanceFamily::Elevation,
                WorthUiAppearanceTokenSource::Application,
                WorthUiAppearanceValue::Shadow(
                    WorthUiShadowValue::from_authored_parts(
                        ThemeColorValue::hex("#00000066").unwrap(),
                        "0px",
                        "1px",
                        "3px",
                        "0px",
                    )
                    .unwrap(),
                ),
            ))
            .register_density_token(WorthUiDensityTokenDescriptor::define(
                DensityTokenId::new("density.header.row_padding").unwrap(),
                WorthUiDensityFamily::RowPadding,
                WorthUiDensityValue::Padding(
                    WorthUiPaddingValue::from_shorthand_px("1px 6px").unwrap(),
                ),
            ))
            .register_density_token(WorthUiDensityTokenDescriptor::define(
                DensityTokenId::new("density.header.container_padding").unwrap(),
                WorthUiDensityFamily::ContainerPadding,
                WorthUiDensityValue::Padding(
                    WorthUiPaddingValue::from_shorthand_px("4px 8px").unwrap(),
                ),
            ))
            .register_density_token(WorthUiDensityTokenDescriptor::define(
                DensityTokenId::new("density.header.control_spacing").unwrap(),
                WorthUiDensityFamily::ControlSpacing,
                WorthUiDensityValue::Spacing(WorthUiSpacingValue::from_px("8px").unwrap()),
            ))
            .freeze()
            .capabilities()
            .clone()
    }

    fn header_request() -> WorthUiHeaderAppearanceRequest {
        WorthUiHeaderAppearanceRequest::new(
            AppearanceTokenId::new("appearance.header.font_size").unwrap(),
            AppearanceTokenId::new("appearance.header.menu_min_width").unwrap(),
            AppearanceTokenId::new("appearance.header.border_width").unwrap(),
            AppearanceTokenId::new("appearance.header.panel_shadow").unwrap(),
            DensityTokenId::new("density.header.row_padding").unwrap(),
            DensityTokenId::new("density.header.container_padding").unwrap(),
            DensityTokenId::new("density.header.control_spacing").unwrap(),
        )
    }
}
