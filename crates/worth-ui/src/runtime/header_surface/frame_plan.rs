use crate::capability::CapabilitySnapshot;
use crate::runtime::{
    WorthUiDropdownAppearanceRequest, WorthUiProjectionDependencyDeclaration,
    WorthUiProjectionDependencySet, WorthUiProjectionEquivalenceBasisKind, WorthUiProjectionFamily,
    WorthUiProjectionIdentity, WorthUiProjectionPlanContract,
};

use super::{
    WorthUiHeaderAppearanceFrameReceipt, WorthUiHeaderAppearancePlan,
    WorthUiHeaderAppearancePlanDenial, WorthUiHeaderAppearanceRequest, WorthUiHeaderFrameReceipt,
    WorthUiHeaderMenuPlan, WorthUiHeaderMenuPlanDenial, WorthUiHeaderMenuProjectionRequest,
    WorthUiHeaderThemeFrameReceipt, WorthUiHeaderThemePlan, WorthUiHeaderThemePlanDenial,
    WorthUiHeaderThemeTokenRequest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiHeaderFramePlan {
    menu_plan: WorthUiHeaderMenuPlan,
    theme_plan: WorthUiHeaderThemePlan,
    appearance_plan: WorthUiHeaderAppearancePlan,
    frame_digest: u64,
    dependencies: WorthUiProjectionDependencySet,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiHeaderFrame<'a> {
    menu: &'a WorthUiHeaderFrameReceipt,
    theme: &'a WorthUiHeaderThemeFrameReceipt,
    appearance: &'a WorthUiHeaderAppearanceFrameReceipt,
    frame_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiHeaderFramePlanDenial {
    Menu(WorthUiHeaderMenuPlanDenial),
    Theme(WorthUiHeaderThemePlanDenial),
    Appearance(WorthUiHeaderAppearancePlanDenial),
}

impl WorthUiHeaderFramePlan {
    pub fn from_snapshot(
        snapshot: &CapabilitySnapshot,
        menu_requests: impl IntoIterator<Item = WorthUiHeaderMenuProjectionRequest>,
        theme_request: WorthUiHeaderThemeTokenRequest,
        appearance_request: WorthUiHeaderAppearanceRequest,
    ) -> Result<Self, WorthUiHeaderFramePlanDenial> {
        let menu_plan = WorthUiHeaderMenuPlan::from_snapshot(
            snapshot,
            menu_requests,
            dropdown_appearance_request(&appearance_request),
        )
        .map_err(Self::menu)?;
        let theme_plan =
            WorthUiHeaderThemePlan::from_snapshot(snapshot, theme_request).map_err(Self::theme)?;
        let appearance_plan =
            WorthUiHeaderAppearancePlan::from_snapshot(snapshot, appearance_request)
                .map_err(Self::appearance)?;
        let frame_digest = digest_triplet(
            menu_plan.projection_digest(),
            theme_plan.theme_digest(),
            appearance_plan.appearance_digest(),
        );
        let dependencies = menu_plan
            .dependencies()
            .clone()
            .merge(theme_plan.dependencies())
            .merge(appearance_plan.dependencies());
        Ok(Self {
            menu_plan,
            theme_plan,
            appearance_plan,
            frame_digest,
            dependencies,
        })
    }

    pub(crate) fn from_composed_plans(
        menu_plan: WorthUiHeaderMenuPlan,
        theme_plan: WorthUiHeaderThemePlan,
        appearance_plan: WorthUiHeaderAppearancePlan,
    ) -> Self {
        let frame_digest = digest_triplet(
            menu_plan.projection_digest(),
            theme_plan.theme_digest(),
            appearance_plan.appearance_digest(),
        );
        let dependencies = menu_plan
            .dependencies()
            .clone()
            .merge(theme_plan.dependencies())
            .merge(appearance_plan.dependencies());
        Self {
            menu_plan,
            theme_plan,
            appearance_plan,
            frame_digest,
            dependencies,
        }
    }

    pub fn execute_frame(&self) -> WorthUiHeaderFrame<'_> {
        WorthUiHeaderFrame {
            menu: self.menu_plan.execute_frame(),
            theme: self.theme_plan.execute_frame(),
            appearance: self.appearance_plan.execute_frame(),
            frame_digest: self.frame_digest,
        }
    }

    pub fn menu_plan(&self) -> &WorthUiHeaderMenuPlan {
        &self.menu_plan
    }

    pub fn theme_plan(&self) -> &WorthUiHeaderThemePlan {
        &self.theme_plan
    }

    pub fn appearance_plan(&self) -> &WorthUiHeaderAppearancePlan {
        &self.appearance_plan
    }

    pub fn frame_digest(&self) -> u64 {
        self.frame_digest
    }

    pub fn dependencies(&self) -> &WorthUiProjectionDependencySet {
        &self.dependencies
    }

    fn menu(denial: WorthUiHeaderMenuPlanDenial) -> WorthUiHeaderFramePlanDenial {
        WorthUiHeaderFramePlanDenial::Menu(denial)
    }

    fn theme(denial: WorthUiHeaderThemePlanDenial) -> WorthUiHeaderFramePlanDenial {
        WorthUiHeaderFramePlanDenial::Theme(denial)
    }

    fn appearance(denial: WorthUiHeaderAppearancePlanDenial) -> WorthUiHeaderFramePlanDenial {
        WorthUiHeaderFramePlanDenial::Appearance(denial)
    }
}

impl WorthUiProjectionPlanContract for WorthUiHeaderFramePlan {
    fn projection_identity(&self) -> WorthUiProjectionIdentity {
        WorthUiProjectionIdentity::runtime("worth-ui.header.frame")
    }

    fn projection_family(&self) -> WorthUiProjectionFamily {
        WorthUiProjectionFamily::HeaderFrame
    }

    fn projection_dependency_declaration(&self) -> WorthUiProjectionDependencyDeclaration {
        WorthUiProjectionDependencyDeclaration::from_set(self.dependencies.clone())
    }

    fn projection_equivalence_digest(&self) -> u64 {
        self.frame_digest
    }

    fn projection_equivalence_basis_kind(&self) -> WorthUiProjectionEquivalenceBasisKind {
        WorthUiProjectionEquivalenceBasisKind::FrameDigest
    }
}

impl crate::runtime::projection_contract::plan_contract::private::Sealed
    for WorthUiHeaderFramePlan
{
}

impl<'a> WorthUiHeaderFrame<'a> {
    pub fn menu(&self) -> &'a WorthUiHeaderFrameReceipt {
        self.menu
    }

    pub fn theme(&self) -> &'a WorthUiHeaderThemeFrameReceipt {
        self.theme
    }

    pub fn appearance(&self) -> &'a WorthUiHeaderAppearanceFrameReceipt {
        self.appearance
    }

    pub fn frame_digest(&self) -> u64 {
        self.frame_digest
    }
}

impl From<WorthUiHeaderMenuPlanDenial> for WorthUiHeaderFramePlanDenial {
    fn from(denial: WorthUiHeaderMenuPlanDenial) -> Self {
        Self::Menu(denial)
    }
}

impl From<WorthUiHeaderThemePlanDenial> for WorthUiHeaderFramePlanDenial {
    fn from(denial: WorthUiHeaderThemePlanDenial) -> Self {
        Self::Theme(denial)
    }
}

impl From<WorthUiHeaderAppearancePlanDenial> for WorthUiHeaderFramePlanDenial {
    fn from(denial: WorthUiHeaderAppearancePlanDenial) -> Self {
        Self::Appearance(denial)
    }
}

fn digest_triplet(left: u64, middle: u64, right: u64) -> u64 {
    [
        left.to_le_bytes(),
        middle.to_le_bytes(),
        right.to_le_bytes(),
    ]
    .into_iter()
    .fold(0xcbf2_9ce4_8422_2325, |digest, bytes| {
        fold_bytes(digest, &bytes)
    })
}

fn fold_bytes(mut accumulator: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        accumulator ^= u64::from(*byte);
        accumulator = accumulator.wrapping_mul(0x0000_0100_0000_01b3);
    }
    accumulator
}

pub(crate) fn dropdown_appearance_request(
    appearance_request: &WorthUiHeaderAppearanceRequest,
) -> WorthUiDropdownAppearanceRequest {
    WorthUiDropdownAppearanceRequest::new(
        appearance_request.menu_min_width.clone(),
        appearance_request.row_padding.clone(),
        appearance_request.control_spacing.clone(),
    )
}
