use crate::capability::{
    AppearanceTokenId, CapabilitySnapshot, DensityTokenId, WorthUiLengthValue, WorthUiPaddingValue,
    WorthUiSpacingValue,
};
use crate::runtime::{WorthUiProjectionDependencySet, WorthUiRuntimeFactId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiDropdownAppearanceRequest {
    menu_min_width: AppearanceTokenId,
    row_padding: DensityTokenId,
    control_spacing: DensityTokenId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiDropdownAppearanceFrameReceipt {
    menu_min_width: WorthUiLengthValue,
    row_padding: WorthUiPaddingValue,
    control_spacing: WorthUiSpacingValue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiDropdownAppearancePlanDenial {
    MissingAppearanceToken(String),
    MissingDensityToken(String),
    WrongAppearanceValue { id: String, expected: &'static str },
    WrongDensityValue { id: String, expected: &'static str },
}

impl WorthUiDropdownAppearanceRequest {
    pub fn new(
        menu_min_width: AppearanceTokenId,
        row_padding: DensityTokenId,
        control_spacing: DensityTokenId,
    ) -> Self {
        Self {
            menu_min_width,
            row_padding,
            control_spacing,
        }
    }

    pub(crate) fn resolve(
        &self,
        snapshot: &CapabilitySnapshot,
    ) -> Result<WorthUiDropdownAppearanceFrameReceipt, WorthUiDropdownAppearancePlanDenial> {
        let menu_min_width = snapshot
            .appearance_tokens()
            .get(&self.menu_min_width)
            .ok_or_else(|| {
                WorthUiDropdownAppearancePlanDenial::MissingAppearanceToken(
                    self.menu_min_width.as_str().to_owned(),
                )
            })?;
        let row_padding = snapshot
            .density_tokens()
            .get(&self.row_padding)
            .ok_or_else(|| {
                WorthUiDropdownAppearancePlanDenial::MissingDensityToken(
                    self.row_padding.as_str().to_owned(),
                )
            })?;
        let control_spacing = snapshot
            .density_tokens()
            .get(&self.control_spacing)
            .ok_or_else(|| {
                WorthUiDropdownAppearancePlanDenial::MissingDensityToken(
                    self.control_spacing.as_str().to_owned(),
                )
            })?;

        let menu_min_width = match menu_min_width.value() {
            crate::capability::WorthUiAppearanceValue::Length(value) => *value,
            _ => {
                return Err(WorthUiDropdownAppearancePlanDenial::WrongAppearanceValue {
                    id: self.menu_min_width.as_str().to_owned(),
                    expected: "Length",
                });
            }
        };
        let row_padding = match row_padding.value() {
            crate::capability::WorthUiDensityValue::Padding(value) => value.clone(),
            _ => {
                return Err(WorthUiDropdownAppearancePlanDenial::WrongDensityValue {
                    id: self.row_padding.as_str().to_owned(),
                    expected: "Padding",
                });
            }
        };
        let control_spacing = match control_spacing.value() {
            crate::capability::WorthUiDensityValue::Spacing(value) => *value,
            _ => {
                return Err(WorthUiDropdownAppearancePlanDenial::WrongDensityValue {
                    id: self.control_spacing.as_str().to_owned(),
                    expected: "Spacing",
                });
            }
        };

        Ok(WorthUiDropdownAppearanceFrameReceipt {
            menu_min_width,
            row_padding,
            control_spacing,
        })
    }

    pub(crate) fn dependencies(&self) -> WorthUiProjectionDependencySet {
        WorthUiProjectionDependencySet::empty()
            .depends_on(WorthUiRuntimeFactId::appearance_token(&self.menu_min_width))
            .depends_on(WorthUiRuntimeFactId::density_token(&self.row_padding))
            .depends_on(WorthUiRuntimeFactId::density_token(&self.control_spacing))
    }
}

impl WorthUiDropdownAppearanceFrameReceipt {
    pub fn menu_min_width(&self) -> WorthUiLengthValue {
        self.menu_min_width
    }

    pub fn row_padding(&self) -> &WorthUiPaddingValue {
        &self.row_padding
    }

    pub fn control_spacing(&self) -> WorthUiSpacingValue {
        self.control_spacing
    }

    pub(crate) fn digest(&self) -> u64 {
        [
            self.menu_min_width.digest_basis(),
            self.row_padding.digest_basis(),
            self.control_spacing.digest_basis(),
        ]
        .into_iter()
        .fold(0xcbf2_9ce4_8422_2325, |digest, value| {
            fold_bytes(digest, value.as_bytes())
        })
    }
}

fn fold_bytes(mut accumulator: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        accumulator ^= u64::from(*byte);
        accumulator = accumulator.wrapping_mul(0x0000_0100_0000_01b3);
    }
    accumulator
}
