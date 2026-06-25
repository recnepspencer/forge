use super::super::digest::hash_text;
use super::WorthUiAppearanceStateName;
use crate::runtime::{WorthUiPrimitiveOperabilityPosture, WorthUiRuntimeFactId};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiPrimitiveHostAppearanceObservation {
    hovered: bool,
    pressed: bool,
    focused: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPrimitiveObservedPostureReceipt {
    surface_id: String,
    active_appearance_fact: WorthUiRuntimeFactId,
    operability_posture: WorthUiPrimitiveOperabilityPosture,
    posture: WorthUiAppearanceStatePosture,
    receipt_digest: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiAppearanceStatePosture {
    Enabled(WorthUiAppearanceEnabledPosture),
    Disabled(WorthUiAppearanceDisabledPosture),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiAppearanceEnabledPosture {
    hovered: bool,
    pressed: bool,
    focused: bool,
    selected: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiAppearanceDisabledPosture {
    operability_posture: WorthUiPrimitiveOperabilityPosture,
    selected: bool,
}

impl WorthUiPrimitiveHostAppearanceObservation {
    pub fn new(hovered: bool, pressed: bool, focused: bool) -> Self {
        Self {
            hovered,
            pressed,
            focused,
        }
    }

    pub fn rest() -> Self {
        Self::new(false, false, false)
    }

    pub fn hovered(&self) -> bool {
        self.hovered
    }

    pub fn pressed(&self) -> bool {
        self.pressed
    }

    pub fn focused(&self) -> bool {
        self.focused
    }
}

impl WorthUiPrimitiveObservedPostureReceipt {
    pub(crate) fn enabled(
        surface_id: &str,
        host_observation: WorthUiPrimitiveHostAppearanceObservation,
        selected: bool,
    ) -> Self {
        let posture = WorthUiAppearanceStatePosture::Enabled(WorthUiAppearanceEnabledPosture::new(
            host_observation.hovered(),
            host_observation.pressed(),
            host_observation.focused(),
            selected,
        ));
        let operability_posture = WorthUiPrimitiveOperabilityPosture::Enabled;
        Self {
            surface_id: surface_id.to_owned(),
            active_appearance_fact: WorthUiRuntimeFactId::primitive_active_appearance(surface_id),
            operability_posture,
            posture,
            receipt_digest: observed_posture_digest(surface_id, operability_posture, posture),
        }
    }

    pub(crate) fn non_enabled(
        surface_id: &str,
        operability_posture: WorthUiPrimitiveOperabilityPosture,
        selected: bool,
    ) -> Self {
        debug_assert!(operability_posture != WorthUiPrimitiveOperabilityPosture::Enabled);
        let posture = WorthUiAppearanceStatePosture::Disabled(
            WorthUiAppearanceDisabledPosture::new(operability_posture, selected),
        );
        Self {
            surface_id: surface_id.to_owned(),
            active_appearance_fact: WorthUiRuntimeFactId::primitive_active_appearance(surface_id),
            operability_posture,
            posture,
            receipt_digest: observed_posture_digest(surface_id, operability_posture, posture),
        }
    }

    pub fn surface_id(&self) -> &str {
        &self.surface_id
    }

    pub fn active_appearance_fact(&self) -> &WorthUiRuntimeFactId {
        &self.active_appearance_fact
    }

    pub fn operability_posture(&self) -> WorthUiPrimitiveOperabilityPosture {
        self.operability_posture
    }

    pub fn posture(&self) -> WorthUiAppearanceStatePosture {
        self.posture
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

impl WorthUiAppearanceEnabledPosture {
    fn new(hovered: bool, pressed: bool, focused: bool, selected: bool) -> Self {
        Self {
            hovered,
            pressed,
            focused,
            selected,
        }
    }

    pub fn hovered(self) -> bool {
        self.hovered
    }

    pub fn pressed(self) -> bool {
        self.pressed
    }

    pub fn focused(self) -> bool {
        self.focused
    }

    pub fn selected(self) -> bool {
        self.selected
    }
}

impl WorthUiAppearanceDisabledPosture {
    fn new(operability_posture: WorthUiPrimitiveOperabilityPosture, selected: bool) -> Self {
        Self {
            operability_posture,
            selected,
        }
    }

    pub fn operability_posture(self) -> WorthUiPrimitiveOperabilityPosture {
        self.operability_posture
    }

    pub fn selected(self) -> bool {
        self.selected
    }
}

fn observed_posture_digest(
    surface_id: &str,
    operability_posture: WorthUiPrimitiveOperabilityPosture,
    posture: WorthUiAppearanceStatePosture,
) -> u64 {
    hash_text(&format!(
        "primitive-observed-posture|surface:{surface_id}|operability:{operability_posture:?}|posture:{posture:?}"
    ))
}

impl WorthUiAppearanceStatePosture {
    pub fn enabled_rest() -> Self {
        Self::Enabled(WorthUiAppearanceEnabledPosture::new(
            false, false, false, false,
        ))
    }

    pub fn disabled_posture(operability_posture: WorthUiPrimitiveOperabilityPosture) -> Self {
        Self::Disabled(WorthUiAppearanceDisabledPosture::new(
            operability_posture,
            false,
        ))
    }

    pub fn active_states(self) -> Vec<WorthUiAppearanceStateName> {
        let mut active_states = vec![WorthUiAppearanceStateName::Rest];
        match self {
            Self::Enabled(posture) => {
                if posture.selected() {
                    active_states.push(WorthUiAppearanceStateName::Selected);
                }
                if posture.hovered() {
                    active_states.push(WorthUiAppearanceStateName::Hover);
                }
                if posture.pressed() {
                    active_states.push(WorthUiAppearanceStateName::Pressed);
                }
                if posture.focused() {
                    active_states.push(WorthUiAppearanceStateName::Focus);
                }
            }
            Self::Disabled(_) => active_states.push(WorthUiAppearanceStateName::Disabled),
        }
        active_states
    }

    pub fn hovered(self) -> bool {
        matches!(self, Self::Enabled(posture) if posture.hovered())
    }

    pub fn pressed(self) -> bool {
        matches!(self, Self::Enabled(posture) if posture.pressed())
    }

    pub fn focused(self) -> bool {
        matches!(self, Self::Enabled(posture) if posture.focused())
    }

    pub fn disabled(self) -> bool {
        matches!(self, Self::Disabled(_))
    }

    pub fn selected(self) -> bool {
        match self {
            Self::Enabled(posture) => posture.selected(),
            Self::Disabled(posture) => posture.selected(),
        }
    }
}
