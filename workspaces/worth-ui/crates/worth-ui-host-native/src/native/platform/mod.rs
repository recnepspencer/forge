#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub(crate) use windows::{
    install_pointer_input, observe_reduced_motion_posture, UiNativePointerInputPort,
};

#[cfg(not(target_os = "windows"))]
pub(crate) struct UiNativePointerInputPort;

#[cfg(not(target_os = "windows"))]
pub(crate) fn install_pointer_input(
    _window: std::sync::Arc<winit::window::Window>,
) -> Option<Box<UiNativePointerInputPort>> {
    None
}

#[cfg(not(target_os = "windows"))]
pub(crate) const fn observe_reduced_motion_posture() -> crate::native::UiNativeReducedMotionPosture
{
    crate::native::UiNativeReducedMotionPosture::Unavailable
}

#[cfg(not(target_os = "windows"))]
impl UiNativePointerInputPort {
    pub(crate) fn refresh_client_origin(&mut self) {}

    pub(crate) fn take_button_position(
        &mut self,
        _button: winit::event::MouseButton,
        _state: winit::event::ElementState,
    ) -> Option<winit::dpi::PhysicalPosition<f64>> {
        None
    }
}
