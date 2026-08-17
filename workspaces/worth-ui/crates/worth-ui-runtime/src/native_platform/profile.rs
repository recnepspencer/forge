use std::sync::Arc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiNativeWindowSpec {
    title: Arc<str>,
    initial_logical_size: [u32; 2],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiNativePlatformProfile {
    window: UiNativeWindowSpec,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativePlatformPreparationDenial {
    EmptyWindowTitle,
    WindowTitleCapacityExceeded,
    EmptyWindowExtent,
    WindowExtentCapacityExceeded,
    QualifiedProfileMismatch,
    PreparationIdentityExhausted,
    UnsupportedPlatform,
    UnsupportedArchitecture,
}

impl UiNativeWindowSpec {
    pub fn new(title: impl Into<Arc<str>>, initial_logical_size: [u32; 2]) -> Self {
        Self {
            title: title.into(),
            initial_logical_size,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub const fn initial_logical_size(&self) -> [u32; 2] {
        self.initial_logical_size
    }
}

impl UiNativePlatformProfile {
    pub fn single_window(window: UiNativeWindowSpec) -> Self {
        Self { window }
    }

    pub fn window(&self) -> &UiNativeWindowSpec {
        &self.window
    }

    pub(crate) fn validate(&self) -> Result<(), UiNativePlatformPreparationDenial> {
        validate_environment(cfg!(target_os = "windows"), cfg!(target_arch = "x86_64"))?;
        if self.window.title.is_empty() {
            return Err(UiNativePlatformPreparationDenial::EmptyWindowTitle);
        }
        if self.window.title.len() > 256 {
            return Err(UiNativePlatformPreparationDenial::WindowTitleCapacityExceeded);
        }
        let [width, height] = self.window.initial_logical_size;
        if width == 0 || height == 0 {
            return Err(UiNativePlatformPreparationDenial::EmptyWindowExtent);
        }
        if width > 16_384 || height > 16_384 {
            return Err(UiNativePlatformPreparationDenial::WindowExtentCapacityExceeded);
        }
        if worth_ui_host_native::UiNativePlatformProfileIdentity::WORTH_UI_WINDOWS_DX12_V1.as_str()
            != "worth-ui-windows-dx12-v1"
        {
            return Err(UiNativePlatformPreparationDenial::QualifiedProfileMismatch);
        }
        Ok(())
    }
}

fn validate_environment(
    is_windows: bool,
    is_x86_64: bool,
) -> Result<(), UiNativePlatformPreparationDenial> {
    if !is_windows {
        return Err(UiNativePlatformPreparationDenial::UnsupportedPlatform);
    }
    if !is_x86_64 {
        return Err(UiNativePlatformPreparationDenial::UnsupportedArchitecture);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{validate_environment, UiNativePlatformPreparationDenial};

    #[test]
    fn closed_environment_classifier_rejects_each_platform_substitution() {
        assert_eq!(validate_environment(true, true), Ok(()));
        assert_eq!(
            validate_environment(false, true),
            Err(UiNativePlatformPreparationDenial::UnsupportedPlatform)
        );
        assert_eq!(
            validate_environment(true, false),
            Err(UiNativePlatformPreparationDenial::UnsupportedArchitecture)
        );
    }
}
