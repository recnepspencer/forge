use winit::dpi::PhysicalPosition;
use winit::event::{
    DeviceId, ElementState, Ime, MouseButton, MouseScrollDelta, TouchPhase, WindowEvent,
};
use worth_ui_host_contract::{UiHostProtocolContract, UiHostProtocolNegotiation};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum UiNativeLifecycleEvent {
    Pointer,
    Keyboard,
    Preedit,
    ImeCommit,
    ImeCancel,
    Scroll,
    Button,
    ButtonUnavailable,
    BeginSuccessor,
    BeginProfileTransition,
    BeginZeroSizedProfile,
    ExactCapacityText,
    OverCapacityText,
    ValidImeRange,
    UnprovableImeRange,
    TextWithoutRecipient,
    TextWithStaleRecipient,
    CompletePresentation,
    Close,
}

pub(super) fn window_event(event: UiNativeLifecycleEvent) -> WindowEvent {
    match event {
        UiNativeLifecycleEvent::Pointer => WindowEvent::CursorMoved {
            device_id: DeviceId::dummy(),
            position: PhysicalPosition::new(10.0, 20.0),
        },
        UiNativeLifecycleEvent::Keyboard => unreachable!(),
        UiNativeLifecycleEvent::Preedit => {
            WindowEvent::Ime(Ime::Preedit("draft".to_owned(), Some((0, 5))))
        }
        UiNativeLifecycleEvent::ImeCommit => WindowEvent::Ime(Ime::Commit("draft".to_owned())),
        UiNativeLifecycleEvent::ImeCancel => WindowEvent::Ime(Ime::Preedit(String::new(), None)),
        UiNativeLifecycleEvent::Scroll => WindowEvent::MouseWheel {
            device_id: DeviceId::dummy(),
            delta: MouseScrollDelta::PixelDelta(PhysicalPosition::new(2.0, 3.0)),
            phase: TouchPhase::Moved,
        },
        UiNativeLifecycleEvent::Button | UiNativeLifecycleEvent::ButtonUnavailable => {
            WindowEvent::MouseInput {
                device_id: DeviceId::dummy(),
                state: ElementState::Pressed,
                button: MouseButton::Left,
            }
        }
        _ => unreachable!("lifecycle control is not an input event"),
    }
}

pub(super) fn protocol() -> worth_ui_host_contract::UiHostProtocolAgreement {
    match UiHostProtocolContract::current().negotiate() {
        UiHostProtocolNegotiation::Compatible(agreement) => agreement,
        UiHostProtocolNegotiation::Incompatible(_) => unreachable!("qualified protocol is current"),
    }
}
