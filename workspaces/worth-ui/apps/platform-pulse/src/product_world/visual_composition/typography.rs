#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlatformPulseTextRole {
    Display,
    Masthead,
    Section,
    Body,
    Meta,
    Action,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlatformPulseTextStyleContract {
    size_points: u16,
    line_height_points: u16,
    weight: u16,
}

impl PlatformPulseTextRole {
    pub const ALL: [Self; 6] = [
        Self::Display,
        Self::Masthead,
        Self::Section,
        Self::Body,
        Self::Meta,
        Self::Action,
    ];

    pub const fn style(self) -> PlatformPulseTextStyleContract {
        match self {
            Self::Display => PlatformPulseTextStyleContract::new(44, 52, 500),
            Self::Masthead => PlatformPulseTextStyleContract::new(16, 20, 600),
            Self::Section => PlatformPulseTextStyleContract::new(11, 16, 650),
            Self::Body => PlatformPulseTextStyleContract::new(13, 20, 450),
            Self::Meta => PlatformPulseTextStyleContract::new(12, 16, 500),
            Self::Action => PlatformPulseTextStyleContract::new(13, 20, 600),
        }
    }
}

impl PlatformPulseTextStyleContract {
    pub const fn new(size_points: u16, line_height_points: u16, weight: u16) -> Self {
        Self {
            size_points,
            line_height_points,
            weight,
        }
    }

    pub const fn size_points(self) -> u16 {
        self.size_points
    }

    pub const fn line_height_points(self) -> u16 {
        self.line_height_points
    }

    pub const fn weight(self) -> u16 {
        self.weight
    }

    pub fn qualified_style(self) -> UiTextStyle {
        UiTextStyle::new(UiTextStyleInput {
            language: Arc::from("und"),
            font_size_millipoints: u32::from(self.size_points) * 1_000,
            letter_spacing_millipoints: 0,
            word_spacing_millipoints: 0,
            family_stack: UiFontFamilyStack::profile_sans(),
            face_request: UiTextFaceRequest::new(self.weight, 100_000, UiFontSlant::Upright)
                .expect("Pulse text weights are qualified requests"),
            features: Box::new([]),
            variations: Box::new([]),
        })
        .expect("Pulse text roles are valid qualified styles")
    }

    pub fn semantic_text_contract(
        self,
        foreground: ThemeTokenId,
        layer_semantic_order: u32,
    ) -> Result<ComponentSemanticTextContract, ComponentSemanticTextContractDenial> {
        ComponentSemanticTextContract::qualified_with_line_height(
            foreground,
            layer_semantic_order,
            self.qualified_style(),
            u32::from(self.line_height_points) * 1_000,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_pulse_role_builds_a_real_qualified_text_contract() {
        for role in PlatformPulseTextRole::ALL {
            let expected = role.style();
            let contract = expected
                .semantic_text_contract(ThemeTokenId::new("theme.pulse.text").unwrap(), 1)
                .unwrap();
            let style = contract.style().unwrap();
            assert_eq!(
                style.font_size_millipoints(),
                u32::from(expected.size_points()) * 1_000
            );
            assert_eq!(style.face_request().weight(), expected.weight());
            assert_eq!(
                contract.line_height_millipoints(),
                Some(u32::from(expected.line_height_points()) * 1_000)
            );
        }
    }
}
use std::sync::Arc;

use worth_ui::facade::app::{
    UiFontFamilyStack, UiFontSlant, UiTextFaceRequest, UiTextStyle, UiTextStyleInput,
};
use worth_ui::facade::declaration::{
    ComponentSemanticTextContract, ComponentSemanticTextContractDenial, ThemeTokenId,
};
