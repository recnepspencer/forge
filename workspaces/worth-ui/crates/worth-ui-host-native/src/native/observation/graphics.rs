use crate::native::UiNativePresentationAccess;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiNativeGraphicsObservation {
    adapter_name: Box<str>,
    vendor: u32,
    device: u32,
    driver: Box<str>,
    driver_info: Box<str>,
    device_type: Box<str>,
    backend: Box<str>,
    surface_format: Box<str>,
    present_mode: Box<str>,
    alpha_mode: Box<str>,
    retained_format: Box<str>,
    max_texture_dimension_2d: u32,
    device_generation: u64,
    surface_generation: u64,
    surface_suspensions: u64,
    targetless_surface_suspensions: u64,
}

impl UiNativeGraphicsObservation {
    pub(crate) fn from_presentation_access(access: &UiNativePresentationAccess) -> Self {
        let info = access.adapter_info();
        Self {
            adapter_name: info.name.clone().into_boxed_str(),
            vendor: info.vendor,
            device: info.device,
            driver: info.driver.clone().into_boxed_str(),
            driver_info: info.driver_info.clone().into_boxed_str(),
            device_type: format!("{:?}", info.device_type).into_boxed_str(),
            backend: format!("{:?}", info.backend).into_boxed_str(),
            surface_format: format!("{:?}", access.surface_configuration().format).into_boxed_str(),
            present_mode: format!("{:?}", access.surface_configuration().present_mode)
                .into_boxed_str(),
            alpha_mode: format!("{:?}", access.surface_configuration().alpha_mode).into_boxed_str(),
            retained_format: "Rgba8UnormSrgb".into(),
            max_texture_dimension_2d: access.adapter_limits().max_texture_dimension_2d,
            device_generation: access.device_generation_identity(),
            surface_generation: access.surface_generation(),
            surface_suspensions: access.surface_suspensions(),
            targetless_surface_suspensions: access.targetless_surface_suspensions(),
        }
    }

    pub fn adapter_name(&self) -> &str {
        &self.adapter_name
    }
    pub const fn vendor(&self) -> u32 {
        self.vendor
    }
    pub const fn device(&self) -> u32 {
        self.device
    }
    pub fn driver(&self) -> &str {
        &self.driver
    }
    pub fn driver_info(&self) -> &str {
        &self.driver_info
    }
    pub fn device_type(&self) -> &str {
        &self.device_type
    }
    pub fn backend(&self) -> &str {
        &self.backend
    }
    pub fn surface_format(&self) -> &str {
        &self.surface_format
    }
    pub fn present_mode(&self) -> &str {
        &self.present_mode
    }
    pub fn alpha_mode(&self) -> &str {
        &self.alpha_mode
    }
    pub fn retained_format(&self) -> &str {
        &self.retained_format
    }
    pub const fn max_texture_dimension_2d(&self) -> u32 {
        self.max_texture_dimension_2d
    }
    pub const fn device_generation(&self) -> u64 {
        self.device_generation
    }
    pub const fn surface_generation(&self) -> u64 {
        self.surface_generation
    }
    pub const fn surface_suspensions(&self) -> u64 {
        self.surface_suspensions
    }
    pub const fn targetless_surface_suspensions(&self) -> u64 {
        self.targetless_surface_suspensions
    }
}
