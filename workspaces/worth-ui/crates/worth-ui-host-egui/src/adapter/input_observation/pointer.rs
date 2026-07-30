use worth_ui_host_contract::{
    UiHostObservationPayload, UiHostPointerButton, UiHostPointerButtonTransition,
    UiHostPointerCaptureEpoch, UiHostPointerIdentity, UiHostPressedPointerButtons,
    UiHostSurfacePosition,
};

use super::UiEguiCoordinateConversionDenial;

const EGUI_SUBPIXELS_PER_POINT: f64 = 1_000.0;
const CANONICAL_I64_EXCLUSIVE_MAX: f64 = 9_223_372_036_854_775_808.0;
const EGUI_PRIMARY_POINTER: UiHostPointerIdentity = UiHostPointerIdentity::new(1);

#[derive(Clone, Copy, Default)]
pub(super) struct UiEguiPointerTranslator;

#[derive(Clone)]
pub(super) struct UiEguiPointerTranslationState {
    capture_epoch: u64,
    pressed: [bool; 5],
}

impl UiEguiPointerTranslator {
    pub(super) const fn capability(self) -> worth_ui_host_contract::WorthUiHostCapability {
        worth_ui_host_contract::WorthUiHostCapability::PointerInput
    }

    pub(super) fn motion(
        self,
        state: &UiEguiPointerTranslationState,
        position: egui::Pos2,
    ) -> Result<UiHostObservationPayload, UiEguiCoordinateConversionDenial> {
        state.motion(position)
    }

    pub(super) fn button(
        self,
        state: &mut UiEguiPointerTranslationState,
        position: egui::Pos2,
        button: egui::PointerButton,
        pressed: bool,
    ) -> Result<UiHostObservationPayload, UiEguiCoordinateConversionDenial> {
        state.button(position, button, pressed)
    }

    pub(super) fn scroll(
        self,
        delta: egui::Vec2,
    ) -> Result<UiHostObservationPayload, UiEguiCoordinateConversionDenial> {
        UiEguiPointerTranslationState::scroll(delta)
    }

    pub(super) fn end_capture(
        self,
        state: &mut UiEguiPointerTranslationState,
    ) -> Result<(), ()> {
        state.end_capture()
    }
}

impl UiEguiPointerTranslationState {
    pub(super) fn motion(
        &self,
        position: egui::Pos2,
    ) -> Result<UiHostObservationPayload, UiEguiCoordinateConversionDenial> {
        Ok(UiHostObservationPayload::PointerMotion {
            pointer: EGUI_PRIMARY_POINTER,
            capture_epoch: UiHostPointerCaptureEpoch::new(self.capture_epoch),
            pressed_buttons: self.pressed_buttons(),
            position: surface_position(position)?,
        })
    }

    pub(super) fn button(
        &mut self,
        position: egui::Pos2,
        button: egui::PointerButton,
        pressed: bool,
    ) -> Result<UiHostObservationPayload, UiEguiCoordinateConversionDenial> {
        let position = surface_position(position)?;
        let button = pointer_button(button);
        self.pressed[button_index(button)] = pressed;
        Ok(UiHostObservationPayload::PointerButton {
            pointer: EGUI_PRIMARY_POINTER,
            capture_epoch: UiHostPointerCaptureEpoch::new(self.capture_epoch),
            button,
            transition: if pressed {
                UiHostPointerButtonTransition::Pressed
            } else {
                UiHostPointerButtonTransition::Released
            },
            position,
        })
    }

    pub(super) fn scroll(
        delta: egui::Vec2,
    ) -> Result<UiHostObservationPayload, UiEguiCoordinateConversionDenial> {
        Ok(UiHostObservationPayload::ScrollDelta {
            x_subpixels: canonical_subpixels(delta.x)?,
            y_subpixels: canonical_subpixels(delta.y)?,
        })
    }

    pub(super) fn end_capture(&mut self) -> Result<(), ()> {
        self.capture_epoch = self.capture_epoch.checked_add(1).ok_or(())?;
        self.pressed = [false; 5];
        Ok(())
    }

    fn pressed_buttons(&self) -> UiHostPressedPointerButtons {
        UiHostPressedPointerButtons::from_buttons(
            [
                UiHostPointerButton::Primary,
                UiHostPointerButton::Secondary,
                UiHostPointerButton::Middle,
                UiHostPointerButton::Extra1,
                UiHostPointerButton::Extra2,
            ]
            .into_iter()
            .enumerate()
            .filter_map(|(index, button)| self.pressed[index].then_some(button)),
        )
    }
}

impl Default for UiEguiPointerTranslationState {
    fn default() -> Self {
        Self {
            capture_epoch: 1,
            pressed: [false; 5],
        }
    }
}

fn surface_position(
    position: egui::Pos2,
) -> Result<UiHostSurfacePosition, UiEguiCoordinateConversionDenial> {
    Ok(UiHostSurfacePosition::new(
        canonical_subpixels(position.x)?,
        canonical_subpixels(position.y)?,
    ))
}

fn canonical_subpixels(value: f32) -> Result<i64, UiEguiCoordinateConversionDenial> {
    if !value.is_finite() {
        return Err(UiEguiCoordinateConversionDenial::NotFinite);
    }
    let scaled = (f64::from(value) * EGUI_SUBPIXELS_PER_POINT).round();
    if !(-CANONICAL_I64_EXCLUSIVE_MAX..CANONICAL_I64_EXCLUSIVE_MAX).contains(&scaled) {
        return Err(UiEguiCoordinateConversionDenial::OutsideCanonicalRange);
    }
    Ok(scaled as i64)
}

fn pointer_button(button: egui::PointerButton) -> UiHostPointerButton {
    match button {
        egui::PointerButton::Primary => UiHostPointerButton::Primary,
        egui::PointerButton::Secondary => UiHostPointerButton::Secondary,
        egui::PointerButton::Middle => UiHostPointerButton::Middle,
        egui::PointerButton::Extra1 => UiHostPointerButton::Extra1,
        egui::PointerButton::Extra2 => UiHostPointerButton::Extra2,
    }
}

const fn button_index(button: UiHostPointerButton) -> usize {
    match button {
        UiHostPointerButton::Primary => 0,
        UiHostPointerButton::Secondary => 1,
        UiHostPointerButton::Middle => 2,
        UiHostPointerButton::Extra1 => 3,
        UiHostPointerButton::Extra2 => 4,
    }
}
