use super::UiNativeOwnedWindow;

pub(super) type UiNativePointerInputPort = crate::native::UiNativePointerInputPort;

pub(super) fn install_pointer_input(
    window: &UiNativeOwnedWindow,
) -> Option<Box<UiNativePointerInputPort>> {
    crate::native::platform::install_pointer_input(std::sync::Arc::clone(&*window))
}
