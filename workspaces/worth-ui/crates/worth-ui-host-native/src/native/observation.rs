use worth_ui_host_contract::{UiHostPresentationCostReport, UiMountedPresentationProductionCost};

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
    host_surface: u64,
    binding_generation: u64,
    mounted_instance: u64,
    node_receipt: u64,
    presentation_attempt: u64,
    logical_bounds_milli: [i64; 4],
    order_ordinal: u16,
    port_crossings: u8,
    production_cost: UiMountedPresentationProductionCost,
    cost: UiHostPresentationCostReport,
    alpha_glyphs: Box<[UiNativeGlyphObservation]>,
    intrinsic_glyphs: Box<[UiNativeGlyphObservation]>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiNativeGlyphObservation {
    glyph_id: u32,
    palette: u16,
    source: worth_ui_host_contract::UiGlyphRasterSource,
    raster_key_digest: [u8; 32],
    original_range: [u32; 2],
    foreground_rgba8: [u8; 4],
    target_bounds: [u32; 4],
    transcript_digest: [u8; 32],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativePresentationWorkKind {
    Initial,
    Delta,
    Reconstruction,
    Unchanged,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiNativeRetainedFrameObservation {
    frame: u64,
    kind: UiNativePresentationWorkKind,
    retained_baseline_rgba8: [u8; 4],
    retained_center_rgba8: [u8; 4],
    cost: UiHostPresentationCostReport,
    intrinsic_glyphs: Box<[UiNativeGlyphObservation]>,
    presentation: Option<UiNativePresentationObservation>,
}

pub(crate) struct UiNativePresentationInput {
    pub(crate) client_physical_size: [u32; 2],
    pub(crate) scale_factor_milli: u32,
    pub(crate) source_rgba8: [u8; 4],
    pub(crate) retained_center_rgba8: [u8; 4],
    pub(crate) retained_baseline_rgba8: [u8; 4],
    pub(crate) presented_frame: u64,
    pub(crate) semantic_surface: u64,
    pub(crate) host_surface: u64,
    pub(crate) binding_generation: u64,
    pub(crate) mounted_instance: u64,
    pub(crate) node_receipt: u64,
    pub(crate) presentation_attempt: u64,
    pub(crate) logical_bounds_milli: [i64; 4],
    pub(crate) order_ordinal: u16,
    pub(crate) port_crossings: u8,
    pub(crate) production_cost: UiMountedPresentationProductionCost,
    pub(crate) cost: UiHostPresentationCostReport,
    pub(crate) alpha_glyphs: Box<[UiNativeGlyphObservation]>,
    pub(crate) intrinsic_glyphs: Box<[UiNativeGlyphObservation]>,
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
    pub(crate) fn new(input: UiNativePresentationInput) -> Self {
        Self {
            client_physical_size: input.client_physical_size,
            scale_factor_milli: input.scale_factor_milli,
            source_rgba8: input.source_rgba8,
            retained_center_rgba8: input.retained_center_rgba8,
            retained_baseline_rgba8: input.retained_baseline_rgba8,
            presented_frame: input.presented_frame,
            semantic_surface: input.semantic_surface,
            host_surface: input.host_surface,
            binding_generation: input.binding_generation,
            mounted_instance: input.mounted_instance,
            node_receipt: input.node_receipt,
            presentation_attempt: input.presentation_attempt,
            logical_bounds_milli: input.logical_bounds_milli,
            order_ordinal: input.order_ordinal,
            port_crossings: input.port_crossings,
            production_cost: input.production_cost,
            cost: input.cost,
            alpha_glyphs: input.alpha_glyphs,
            intrinsic_glyphs: input.intrinsic_glyphs,
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

    pub const fn host_surface(&self) -> u64 {
        self.host_surface
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

    pub const fn port_crossings(&self) -> u8 {
        self.port_crossings
    }

    pub const fn cost(&self) -> UiHostPresentationCostReport {
        self.cost
    }

    pub const fn production_cost(&self) -> UiMountedPresentationProductionCost {
        self.production_cost
    }

    pub fn alpha_glyphs(&self) -> &[UiNativeGlyphObservation] {
        &self.alpha_glyphs
    }

    pub fn intrinsic_glyphs(&self) -> &[UiNativeGlyphObservation] {
        &self.intrinsic_glyphs
    }

    pub fn glyph_transcript_digest(&self) -> [u8; 32] {
        glyph_transcript_digest(self.alpha_glyphs.iter().chain(self.intrinsic_glyphs.iter()))
    }

    pub fn intrinsic_glyph_transcript_digest(&self) -> [u8; 32] {
        intrinsic_transcript_digest(&self.intrinsic_glyphs)
    }
}

impl UiNativeGlyphObservation {
    pub(crate) fn from_native_command(
        command: super::presentation::text::UiNativeGlyphCommand,
    ) -> Self {
        use sha2::{Digest, Sha256};
        let key = command.run.raster_key();
        let range = command.run.original_range();
        let [x, y, width, height] = command.target;
        Self {
            glyph_id: key.glyph_id(),
            palette: key.palette().index(),
            source: key.source(),
            raster_key_digest: Sha256::digest(super::text_atlas::canonical_raster_key_bytes(key))
                .into(),
            original_range: [range.start(), range.end()],
            foreground_rgba8: command.run.foreground().channels(),
            target_bounds: [
                x.floor().max(0.0) as u32,
                y.floor().max(0.0) as u32,
                (x + width).ceil().max(0.0) as u32,
                (y + height).ceil().max(0.0) as u32,
            ],
            transcript_digest: Sha256::digest(command.run.canonical_transcript_bytes()).into(),
        }
    }

    pub const fn glyph_id(self) -> u32 {
        self.glyph_id
    }
    pub const fn palette(self) -> u16 {
        self.palette
    }
    pub const fn source(self) -> worth_ui_host_contract::UiGlyphRasterSource {
        self.source
    }
    pub const fn raster_key_digest(self) -> [u8; 32] {
        self.raster_key_digest
    }
    pub const fn original_range(self) -> [u32; 2] {
        self.original_range
    }
    pub const fn foreground_rgba8(self) -> [u8; 4] {
        self.foreground_rgba8
    }
    pub const fn target_bounds(self) -> [u32; 4] {
        self.target_bounds
    }
    pub const fn transcript_digest(self) -> [u8; 32] {
        self.transcript_digest
    }
}

fn intrinsic_transcript_digest(glyphs: &[UiNativeGlyphObservation]) -> [u8; 32] {
    glyph_transcript_digest(glyphs.iter())
}

fn glyph_transcript_digest<'a>(
    glyphs: impl IntoIterator<Item = &'a UiNativeGlyphObservation>,
) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    let mut rows = glyphs
        .into_iter()
        .map(|glyph| glyph.transcript_digest)
        .collect::<Vec<_>>();
    rows.sort_unstable();
    let mut digest = Sha256::new();
    digest.update((rows.len() as u64).to_le_bytes());
    for row in rows {
        digest.update(32_u64.to_le_bytes());
        digest.update(row);
    }
    digest.finalize().into()
}

impl UiNativeRetainedFrameObservation {
    pub(crate) fn observed(
        frame: u64,
        kind: UiNativePresentationWorkKind,
        pixels: [[u8; 4]; 2],
        cost: UiHostPresentationCostReport,
        presentation: Option<UiNativePresentationObservation>,
    ) -> Self {
        let intrinsic_glyphs = presentation
            .as_ref()
            .map(|observation| observation.intrinsic_glyphs().to_vec().into_boxed_slice())
            .unwrap_or_default();
        Self {
            frame,
            kind,
            retained_baseline_rgba8: pixels[0],
            retained_center_rgba8: pixels[1],
            cost,
            intrinsic_glyphs,
            presentation,
        }
    }

    pub const fn frame(&self) -> u64 {
        self.frame
    }

    pub const fn kind(&self) -> UiNativePresentationWorkKind {
        self.kind
    }

    pub const fn retained_baseline_rgba8(&self) -> [u8; 4] {
        self.retained_baseline_rgba8
    }

    pub const fn retained_center_rgba8(&self) -> [u8; 4] {
        self.retained_center_rgba8
    }

    pub const fn cost(&self) -> UiHostPresentationCostReport {
        self.cost
    }

    pub fn intrinsic_glyphs(&self) -> &[UiNativeGlyphObservation] {
        &self.intrinsic_glyphs
    }

    pub const fn presentation(&self) -> Option<&UiNativePresentationObservation> {
        self.presentation.as_ref()
    }

    pub fn intrinsic_glyph_transcript_digest(&self) -> [u8; 32] {
        intrinsic_transcript_digest(&self.intrinsic_glyphs)
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
