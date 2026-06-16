use crate::capability::CapabilitySnapshot;

use super::{
    WorthUiHeaderFrameReceipt, WorthUiHeaderMenuPlan, WorthUiHeaderMenuPlanDenial,
    WorthUiHeaderMenuProjectionRequest, WorthUiHeaderThemeFrameReceipt, WorthUiHeaderThemePlan,
    WorthUiHeaderThemePlanDenial, WorthUiHeaderThemeTokenRequest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiHeaderFramePlan {
    menu_plan: WorthUiHeaderMenuPlan,
    theme_plan: WorthUiHeaderThemePlan,
    frame_digest: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiHeaderFrame<'a> {
    menu: &'a WorthUiHeaderFrameReceipt,
    theme: &'a WorthUiHeaderThemeFrameReceipt,
    frame_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiHeaderFramePlanDenial {
    Menu(WorthUiHeaderMenuPlanDenial),
    Theme(WorthUiHeaderThemePlanDenial),
}

impl WorthUiHeaderFramePlan {
    pub fn from_snapshot(
        snapshot: &CapabilitySnapshot,
        menu_requests: impl IntoIterator<Item = WorthUiHeaderMenuProjectionRequest>,
        theme_request: WorthUiHeaderThemeTokenRequest,
    ) -> Result<Self, WorthUiHeaderFramePlanDenial> {
        let menu_plan =
            WorthUiHeaderMenuPlan::from_snapshot(snapshot, menu_requests).map_err(Self::menu)?;
        let theme_plan =
            WorthUiHeaderThemePlan::from_snapshot(snapshot, theme_request).map_err(Self::theme)?;
        let frame_digest = digest_pair(menu_plan.projection_digest(), theme_plan.theme_digest());
        Ok(Self {
            menu_plan,
            theme_plan,
            frame_digest,
        })
    }

    pub fn execute_frame(&self) -> WorthUiHeaderFrame<'_> {
        WorthUiHeaderFrame {
            menu: self.menu_plan.execute_frame(),
            theme: self.theme_plan.execute_frame(),
            frame_digest: self.frame_digest,
        }
    }

    pub fn menu_plan(&self) -> &WorthUiHeaderMenuPlan {
        &self.menu_plan
    }

    pub fn theme_plan(&self) -> &WorthUiHeaderThemePlan {
        &self.theme_plan
    }

    pub fn frame_digest(&self) -> u64 {
        self.frame_digest
    }

    fn menu(denial: WorthUiHeaderMenuPlanDenial) -> WorthUiHeaderFramePlanDenial {
        WorthUiHeaderFramePlanDenial::Menu(denial)
    }

    fn theme(denial: WorthUiHeaderThemePlanDenial) -> WorthUiHeaderFramePlanDenial {
        WorthUiHeaderFramePlanDenial::Theme(denial)
    }
}

impl<'a> WorthUiHeaderFrame<'a> {
    pub fn menu(&self) -> &'a WorthUiHeaderFrameReceipt {
        self.menu
    }

    pub fn theme(&self) -> &'a WorthUiHeaderThemeFrameReceipt {
        self.theme
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

fn digest_pair(left: u64, right: u64) -> u64 {
    [left.to_le_bytes(), right.to_le_bytes()]
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
