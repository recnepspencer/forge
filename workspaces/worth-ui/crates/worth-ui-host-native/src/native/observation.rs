use worth_ui_host_contract::UiHostPresentationCostReport;

use super::UiNativeGraphics;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiNativePresentationObservation {
    client_physical_size: [u32; 2],
    scale_factor_milli: u32,
    source_rgba8: [u8; 4],
    retained_center_rgba8: [u8; 4],
    retained_baseline_rgba8: [u8; 4],
    presented_frame: u64,
    semantic_surface: u64,
    binding_generation: u64,
    mounted_instance: u64,
    node_receipt: u64,
    presentation_attempt: u64,
    logical_bounds_milli: [i64; 4],
    order_ordinal: u16,
    port_crossings: u8,
    cost: UiHostPresentationCostReport,
}

pub(crate) struct UiNativePresentationInput {
    pub(crate) client_physical_size: [u32; 2],
    pub(crate) scale_factor_milli: u32,
    pub(crate) source_rgba8: [u8; 4],
    pub(crate) retained_center_rgba8: [u8; 4],
    pub(crate) retained_baseline_rgba8: [u8; 4],
    pub(crate) presented_frame: u64,
    pub(crate) semantic_surface: u64,
    pub(crate) binding_generation: u64,
    pub(crate) mounted_instance: u64,
    pub(crate) node_receipt: u64,
    pub(crate) presentation_attempt: u64,
    pub(crate) logical_bounds_milli: [i64; 4],
    pub(crate) order_ordinal: u16,
    pub(crate) port_crossings: u8,
    pub(crate) cost: UiHostPresentationCostReport,
}

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
}

impl UiNativePresentationObservation {
    pub(crate) const fn new(input: UiNativePresentationInput) -> Self {
        Self {
            client_physical_size: input.client_physical_size,
            scale_factor_milli: input.scale_factor_milli,
            source_rgba8: input.source_rgba8,
            retained_center_rgba8: input.retained_center_rgba8,
            retained_baseline_rgba8: input.retained_baseline_rgba8,
            presented_frame: input.presented_frame,
            semantic_surface: input.semantic_surface,
            binding_generation: input.binding_generation,
            mounted_instance: input.mounted_instance,
            node_receipt: input.node_receipt,
            presentation_attempt: input.presentation_attempt,
            logical_bounds_milli: input.logical_bounds_milli,
            order_ordinal: input.order_ordinal,
            port_crossings: input.port_crossings,
            cost: input.cost,
        }
    }

    pub const fn client_physical_size(&self) -> [u32; 2] {
        self.client_physical_size
    }

    pub const fn scale_factor_milli(&self) -> u32 {
        self.scale_factor_milli
    }

    pub const fn source_rgba8(&self) -> [u8; 4] {
        self.source_rgba8
    }

    pub const fn retained_center_rgba8(&self) -> [u8; 4] {
        self.retained_center_rgba8
    }

    pub const fn retained_baseline_rgba8(&self) -> [u8; 4] {
        self.retained_baseline_rgba8
    }

    pub const fn presented_frame(&self) -> u64 {
        self.presented_frame
    }

    pub const fn semantic_surface(&self) -> u64 {
        self.semantic_surface
    }

    pub const fn binding_generation(&self) -> u64 {
        self.binding_generation
    }

    pub const fn mounted_instance(&self) -> u64 {
        self.mounted_instance
    }

    pub const fn node_receipt(&self) -> u64 {
        self.node_receipt
    }

    pub const fn presentation_attempt(&self) -> u64 {
        self.presentation_attempt
    }

    pub const fn logical_bounds_milli(&self) -> [i64; 4] {
        self.logical_bounds_milli
    }

    pub const fn order_ordinal(&self) -> u16 {
        self.order_ordinal
    }

    pub(crate) fn record_presentation_port_crossing(&mut self) {
        self.port_crossings = self.port_crossings.saturating_add(1);
    }

    pub const fn port_crossings(&self) -> u8 {
        self.port_crossings
    }

    pub const fn cost(&self) -> UiHostPresentationCostReport {
        self.cost
    }
}

impl UiNativeGraphicsObservation {
    pub(crate) fn from_graphics(graphics: &UiNativeGraphics) -> Self {
        let info = &graphics.adapter_info;
        Self {
            adapter_name: info.name.clone().into_boxed_str(),
            vendor: info.vendor,
            device: info.device,
            driver: info.driver.clone().into_boxed_str(),
            driver_info: info.driver_info.clone().into_boxed_str(),
            device_type: format!("{:?}", info.device_type).into_boxed_str(),
            backend: format!("{:?}", info.backend).into_boxed_str(),
            surface_format: format!("{:?}", graphics.surface_configuration.format).into_boxed_str(),
            present_mode: format!("{:?}", graphics.surface_configuration.present_mode)
                .into_boxed_str(),
            alpha_mode: format!("{:?}", graphics.surface_configuration.alpha_mode).into_boxed_str(),
            retained_format: "Rgba8UnormSrgb".into(),
            max_texture_dimension_2d: graphics._adapter.limits().max_texture_dimension_2d,
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
}
