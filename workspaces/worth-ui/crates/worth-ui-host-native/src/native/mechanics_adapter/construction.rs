use super::WorthUiNativeMechanicsAdapter;

impl WorthUiNativeMechanicsAdapter {
    pub(crate) fn from_preparation(
        state: std::rc::Rc<std::cell::RefCell<crate::native::UiNativeHostState>>,
        profile: crate::UiNativePlatformProfileIdentity,
    ) -> Self {
        Self { state, profile }
    }
}
