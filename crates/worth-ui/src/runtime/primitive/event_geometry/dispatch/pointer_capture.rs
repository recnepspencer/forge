use super::super::receipt::WorthUiPrimitivePointerCapture;
use super::{
    WorthUiPrimitiveEventDispatchPlan, WorthUiPrimitiveEventDispatchReceipt,
    WorthUiPrimitiveEventHitTestPoint,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitivePointerPhase {
    Hover,
    Press,
    Drag,
    Release,
    Cancel,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitivePointerCaptureHostSupport {
    Certified,
    Emulated,
    Unsupported,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitivePointerCaptureState {
    Uncaptured,
    Captured {
        surface_id: String,
        capture_digest: u64,
    },
    Released,
    Cancelled,
    Unsupported {
        surface_id: String,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitivePointerFrameInput {
    point: WorthUiPrimitiveEventHitTestPoint,
    phase: WorthUiPrimitivePointerPhase,
    prior_capture: WorthUiPrimitivePointerCaptureState,
    host_support: WorthUiPrimitivePointerCaptureHostSupport,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitivePointerFrameReceipt {
    dispatch: WorthUiPrimitiveEventDispatchReceipt,
    capture_state: WorthUiPrimitivePointerCaptureState,
    phase: WorthUiPrimitivePointerPhase,
    host_support: WorthUiPrimitivePointerCaptureHostSupport,
}

impl WorthUiPrimitivePointerFrameInput {
    pub fn new(
        point: WorthUiPrimitiveEventHitTestPoint,
        phase: WorthUiPrimitivePointerPhase,
        prior_capture: WorthUiPrimitivePointerCaptureState,
        host_support: WorthUiPrimitivePointerCaptureHostSupport,
    ) -> Self {
        Self {
            point,
            phase,
            prior_capture,
            host_support,
        }
    }

    pub fn hover(point: WorthUiPrimitiveEventHitTestPoint) -> Self {
        Self::new(
            point,
            WorthUiPrimitivePointerPhase::Hover,
            WorthUiPrimitivePointerCaptureState::Uncaptured,
            WorthUiPrimitivePointerCaptureHostSupport::Certified,
        )
    }

    pub fn point(&self) -> WorthUiPrimitiveEventHitTestPoint {
        self.point
    }

    pub fn phase(&self) -> WorthUiPrimitivePointerPhase {
        self.phase
    }

    pub fn prior_capture(&self) -> &WorthUiPrimitivePointerCaptureState {
        &self.prior_capture
    }

    pub fn host_support(&self) -> WorthUiPrimitivePointerCaptureHostSupport {
        self.host_support
    }
}

impl WorthUiPrimitivePointerFrameReceipt {
    pub fn from_dispatch_plan(
        plan: &WorthUiPrimitiveEventDispatchPlan,
        input: WorthUiPrimitivePointerFrameInput,
    ) -> Self {
        let dispatch = match input.phase {
            WorthUiPrimitivePointerPhase::Hover => plan.cursor_receipt_at(input.point),
            WorthUiPrimitivePointerPhase::Drag => {
                if let Some(surface_id) = input.prior_capture.captured_surface_id() {
                    plan.dispatch_captured_drag(surface_id)
                } else {
                    plan.dispatch_primary_click(input.point)
                }
            }
            WorthUiPrimitivePointerPhase::Press
            | WorthUiPrimitivePointerPhase::Release
            | WorthUiPrimitivePointerPhase::Cancel => plan.dispatch_primary_click(input.point),
        };
        let capture_state = next_capture_state(plan, &input, &dispatch);
        Self {
            dispatch,
            capture_state,
            phase: input.phase,
            host_support: input.host_support,
        }
    }

    pub fn dispatch(&self) -> &WorthUiPrimitiveEventDispatchReceipt {
        &self.dispatch
    }

    pub fn capture_state(&self) -> &WorthUiPrimitivePointerCaptureState {
        &self.capture_state
    }

    pub fn phase(&self) -> WorthUiPrimitivePointerPhase {
        self.phase
    }

    pub fn host_support(&self) -> WorthUiPrimitivePointerCaptureHostSupport {
        self.host_support
    }
}

impl WorthUiPrimitivePointerCaptureState {
    pub fn captured_surface_id(&self) -> Option<&str> {
        match self {
            Self::Captured { surface_id, .. } | Self::Unsupported { surface_id } => {
                Some(surface_id)
            }
            Self::Uncaptured | Self::Released | Self::Cancelled => None,
        }
    }
}

fn next_capture_state(
    plan: &WorthUiPrimitiveEventDispatchPlan,
    input: &WorthUiPrimitivePointerFrameInput,
    dispatch: &WorthUiPrimitiveEventDispatchReceipt,
) -> WorthUiPrimitivePointerCaptureState {
    match input.phase {
        WorthUiPrimitivePointerPhase::Hover => input.prior_capture.clone(),
        WorthUiPrimitivePointerPhase::Cancel => WorthUiPrimitivePointerCaptureState::Cancelled,
        WorthUiPrimitivePointerPhase::Release => WorthUiPrimitivePointerCaptureState::Released,
        WorthUiPrimitivePointerPhase::Drag => input.prior_capture.clone(),
        WorthUiPrimitivePointerPhase::Press => capture_on_press(plan, input, dispatch),
    }
}

fn capture_on_press(
    plan: &WorthUiPrimitiveEventDispatchPlan,
    input: &WorthUiPrimitivePointerFrameInput,
    dispatch: &WorthUiPrimitiveEventDispatchReceipt,
) -> WorthUiPrimitivePointerCaptureState {
    let Some(surface_id) = dispatch.primary_surface_id() else {
        return WorthUiPrimitivePointerCaptureState::Uncaptured;
    };
    let Some(region) = plan
        .regions()
        .iter()
        .find(|region| region.surface_id() == surface_id)
    else {
        return WorthUiPrimitivePointerCaptureState::Uncaptured;
    };
    if region.capture() != WorthUiPrimitivePointerCapture::PressDrag || !region.can_activate() {
        return WorthUiPrimitivePointerCaptureState::Uncaptured;
    }
    match input.host_support {
        WorthUiPrimitivePointerCaptureHostSupport::Unsupported => {
            WorthUiPrimitivePointerCaptureState::Unsupported {
                surface_id: surface_id.to_owned(),
            }
        }
        WorthUiPrimitivePointerCaptureHostSupport::Certified
        | WorthUiPrimitivePointerCaptureHostSupport::Emulated => {
            WorthUiPrimitivePointerCaptureState::Captured {
                surface_id: surface_id.to_owned(),
                capture_digest: capture_digest(surface_id, input.point),
            }
        }
    }
}

fn capture_digest(surface_id: &str, point: WorthUiPrimitiveEventHitTestPoint) -> u64 {
    format!("capture|surface:{surface_id}|point:{point:?}")
        .bytes()
        .fold(0xcbf2_9ce4_8422_2325, |mut acc, byte| {
            acc ^= u64::from(byte);
            acc.wrapping_mul(0x0000_0100_0000_01b3)
        })
}
