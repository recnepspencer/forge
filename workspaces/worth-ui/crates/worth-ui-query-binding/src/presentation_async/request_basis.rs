use std::collections::HashSet;
use std::sync::Arc;

use worth_ui_host_contract::{
    UiGlyphRasterKey, UiGlyphRasterPinRequest, UiHostPresentationLineageIdentity,
    UiHostSurfaceIdentity, UiMountedContentGeneration, UiMountedFrameIdentity,
    UiMountedInstanceIdentity, UiMountedPaintCommandIdentity, UiMountedPresentationAttemptIdentity,
    UiMountedTextForegroundSpan, UiQualifiedTextLayoutIdentity,
    UiQualifiedTextLayoutRequestIdentity, UiQualifiedTextLayoutWidthBasis,
    UiSemanticSurfaceIdentity, UiSurfaceBindingGeneration, UiTextOriginalRange,
    UiTextScaleGeneration,
};

mod identity_parts;
mod raster_key_set;
pub use raster_key_set::WorthUiPresentationRasterKeySetBasis;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiPresentationPaintSpanBasis {
    identity: [u8; 32],
    original_range: UiTextOriginalRange,
    foreground: [u8; 4],
}

impl WorthUiPresentationPaintSpanBasis {
    pub fn from_mounted(span: UiMountedTextForegroundSpan) -> Self {
        Self {
            identity: span.identity().digest(),
            original_range: span.original_range(),
            foreground: span.color().channels(),
        }
    }

    pub const fn identity(self) -> [u8; 32] {
        self.identity
    }
    pub const fn original_range(self) -> UiTextOriginalRange {
        self.original_range
    }
    pub const fn foreground(self) -> [u8; 4] {
        self.foreground
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPresentationMechanicBasis {
    mounted_instance: UiMountedInstanceIdentity,
    mechanic: UiMountedPaintCommandIdentity,
    content_generation: UiMountedContentGeneration,
    content: Arc<str>,
    layout: UiQualifiedTextLayoutIdentity,
    layout_request: UiQualifiedTextLayoutRequestIdentity,
    layout_width: UiQualifiedTextLayoutWidthBasis,
    paint_spans: Box<[WorthUiPresentationPaintSpanBasis]>,
    raster_key_set: WorthUiPresentationRasterKeySetBasis,
    text_scale: UiTextScaleGeneration,
}

#[doc(hidden)]
#[derive(Clone)]
pub struct WorthUiPresentationMechanicBasisInput {
    pub mounted_instance: UiMountedInstanceIdentity,
    pub mechanic: UiMountedPaintCommandIdentity,
    pub content_generation: UiMountedContentGeneration,
    pub content: Arc<str>,
    pub layout: UiQualifiedTextLayoutIdentity,
    pub layout_request: UiQualifiedTextLayoutRequestIdentity,
    pub layout_width: UiQualifiedTextLayoutWidthBasis,
    pub paint_spans: Box<[WorthUiPresentationPaintSpanBasis]>,
    pub raster_keys: Box<[UiGlyphRasterKey]>,
    pub text_scale: UiTextScaleGeneration,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct WorthUiPresentationPinBasis {
    pin: UiGlyphRasterPinRequest,
}

impl WorthUiPresentationPinBasis {
    #[doc(hidden)]
    pub const fn from_runtime(pin: UiGlyphRasterPinRequest) -> Self {
        Self { pin }
    }
    pub const fn layout(self) -> UiQualifiedTextLayoutIdentity {
        self.pin.layout_identity()
    }
    pub const fn key(self) -> UiGlyphRasterKey {
        self.pin.key()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPresentationRequestBasis {
    mounted_frame: UiMountedFrameIdentity,
    semantic_surface: UiSemanticSurfaceIdentity,
    host_surface: UiHostSurfaceIdentity,
    binding: UiSurfaceBindingGeneration,
    complete: bool,
    mechanics: Box<[WorthUiPresentationMechanicBasis]>,
    removed_mechanics: Box<[UiMountedPaintCommandIdentity]>,
    binding_pins: Box<[WorthUiPresentationPinBasis]>,
    pin_additions: Box<[WorthUiPresentationPinBasis]>,
    pin_releases: Box<[WorthUiPresentationPinBasis]>,
    dpi_milli: u32,
    attempt: UiMountedPresentationAttemptIdentity,
    predecessor: Option<UiMountedFrameIdentity>,
    host_lineage: UiHostPresentationLineageIdentity,
}

#[doc(hidden)]
#[derive(Clone)]
pub struct WorthUiPresentationRequestBasisInput {
    pub mounted_frame: UiMountedFrameIdentity,
    pub semantic_surface: UiSemanticSurfaceIdentity,
    pub host_surface: UiHostSurfaceIdentity,
    pub binding: UiSurfaceBindingGeneration,
    pub complete: bool,
    pub mechanics: Box<[WorthUiPresentationMechanicBasisInput]>,
    pub removed_mechanics: Box<[UiMountedPaintCommandIdentity]>,
    pub binding_pins: Box<[WorthUiPresentationPinBasis]>,
    pub pin_additions: Box<[WorthUiPresentationPinBasis]>,
    pub pin_releases: Box<[WorthUiPresentationPinBasis]>,
    pub dpi_milli: u32,
    pub attempt: UiMountedPresentationAttemptIdentity,
    pub predecessor: Option<UiMountedFrameIdentity>,
    pub host_lineage: UiHostPresentationLineageIdentity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPresentationRequestBasisDenial {
    ZeroDpi,
    EmptyPresentationChange,
    EmptyPaintSpan,
    DuplicatePaintSpan,
    OverlappingPaintSpan,
    DuplicateMechanic,
    DuplicateRemovedMechanic,
    MechanicAlsoRemoved,
    DuplicatePinAddition,
    DuplicatePinRelease,
    DuplicateBindingPin,
    NonTextMechanic,
    MechanicMountedInstanceMismatch,
}

impl WorthUiPresentationRequestBasis {
    #[doc(hidden)]
    pub fn from_runtime_correspondence(
        input: WorthUiPresentationRequestBasisInput,
    ) -> Result<Self, WorthUiPresentationRequestBasisDenial> {
        if input.dpi_milli == 0 {
            return Err(WorthUiPresentationRequestBasisDenial::ZeroDpi);
        }
        if !input.complete
            && input.mechanics.is_empty()
            && input.pin_additions.is_empty()
            && input.pin_releases.is_empty()
            && input.binding_pins.is_empty()
            && input.removed_mechanics.is_empty()
        {
            return Err(WorthUiPresentationRequestBasisDenial::EmptyPresentationChange);
        }
        let mut mechanics = input
            .mechanics
            .into_vec()
            .into_iter()
            .map(admit_mechanic)
            .collect::<Result<Vec<_>, _>>()?;
        mechanics.sort_by_key(mechanic_sort_key);
        if mechanics
            .windows(2)
            .any(|pair| pair[0].mechanic == pair[1].mechanic)
        {
            return Err(WorthUiPresentationRequestBasisDenial::DuplicateMechanic);
        }
        let mut removed_mechanics = input.removed_mechanics.into_vec();
        removed_mechanics.sort_by_key(paint_command_sort_key);
        if removed_mechanics.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err(WorthUiPresentationRequestBasisDenial::DuplicateRemovedMechanic);
        }
        if mechanics.iter().any(|mechanic| {
            removed_mechanics
                .iter()
                .any(|removed| *removed == mechanic.mechanic)
        }) {
            return Err(WorthUiPresentationRequestBasisDenial::MechanicAlsoRemoved);
        }
        let mut pin_additions = input.pin_additions.into_vec();
        let mut pin_releases = input.pin_releases.into_vec();
        let mut binding_pins = input.binding_pins.into_vec();
        sort_pins(&mut pin_additions);
        sort_pins(&mut pin_releases);
        sort_pins(&mut binding_pins);
        validate_unique_pins(
            &pin_additions,
            WorthUiPresentationRequestBasisDenial::DuplicatePinAddition,
        )?;
        validate_unique_pins(
            &pin_releases,
            WorthUiPresentationRequestBasisDenial::DuplicatePinRelease,
        )?;
        validate_unique_pins(
            &binding_pins,
            WorthUiPresentationRequestBasisDenial::DuplicateBindingPin,
        )?;
        Ok(Self {
            mounted_frame: input.mounted_frame,
            semantic_surface: input.semantic_surface,
            host_surface: input.host_surface,
            binding: input.binding,
            complete: input.complete,
            mechanics: mechanics.into_boxed_slice(),
            removed_mechanics: removed_mechanics.into_boxed_slice(),
            binding_pins: binding_pins.into_boxed_slice(),
            pin_additions: pin_additions.into_boxed_slice(),
            pin_releases: pin_releases.into_boxed_slice(),
            dpi_milli: input.dpi_milli,
            attempt: input.attempt,
            predecessor: input.predecessor,
            host_lineage: input.host_lineage,
        })
    }

    pub fn mechanics(&self) -> &[WorthUiPresentationMechanicBasis] {
        &self.mechanics
    }
    pub fn removed_mechanics(&self) -> &[UiMountedPaintCommandIdentity] {
        &self.removed_mechanics
    }
    pub fn pin_additions(&self) -> &[WorthUiPresentationPinBasis] {
        &self.pin_additions
    }
    pub fn binding_pins(&self) -> &[WorthUiPresentationPinBasis] {
        &self.binding_pins
    }
    pub fn pin_releases(&self) -> &[WorthUiPresentationPinBasis] {
        &self.pin_releases
    }
    pub const fn dpi_milli(&self) -> u32 {
        self.dpi_milli
    }
    pub const fn attempt(&self) -> UiMountedPresentationAttemptIdentity {
        self.attempt
    }
    pub const fn semantic_surface(&self) -> UiSemanticSurfaceIdentity {
        self.semantic_surface
    }
    pub const fn host_surface(&self) -> UiHostSurfaceIdentity {
        self.host_surface
    }
    pub const fn binding(&self) -> UiSurfaceBindingGeneration {
        self.binding
    }
    pub const fn complete(&self) -> bool {
        self.complete
    }
    pub const fn host_lineage(&self) -> UiHostPresentationLineageIdentity {
        self.host_lineage
    }
    pub const fn mounted_frame(&self) -> UiMountedFrameIdentity {
        self.mounted_frame
    }
    pub const fn predecessor(&self) -> Option<UiMountedFrameIdentity> {
        self.predecessor
    }
}

impl WorthUiPresentationMechanicBasis {
    pub const fn mounted_instance(&self) -> UiMountedInstanceIdentity {
        self.mounted_instance
    }
    pub const fn mechanic(&self) -> UiMountedPaintCommandIdentity {
        self.mechanic
    }
    pub const fn layout(&self) -> UiQualifiedTextLayoutIdentity {
        self.layout
    }
    pub const fn content_generation(&self) -> UiMountedContentGeneration {
        self.content_generation
    }
    pub fn content(&self) -> &str {
        &self.content
    }
    pub const fn layout_request(&self) -> UiQualifiedTextLayoutRequestIdentity {
        self.layout_request
    }
    pub const fn layout_width(&self) -> UiQualifiedTextLayoutWidthBasis {
        self.layout_width
    }
    pub fn paint_spans(&self) -> &[WorthUiPresentationPaintSpanBasis] {
        &self.paint_spans
    }
    pub fn raster_key_set(&self) -> &WorthUiPresentationRasterKeySetBasis {
        &self.raster_key_set
    }
    pub const fn text_scale(&self) -> UiTextScaleGeneration {
        self.text_scale
    }
}

fn admit_mechanic(
    input: WorthUiPresentationMechanicBasisInput,
) -> Result<WorthUiPresentationMechanicBasis, WorthUiPresentationRequestBasisDenial> {
    input
        .mechanic
        .semantic_text_identity_parts()
        .ok_or(WorthUiPresentationRequestBasisDenial::NonTextMechanic)?;
    if input.mechanic.mounted_instance() != input.mounted_instance {
        return Err(WorthUiPresentationRequestBasisDenial::MechanicMountedInstanceMismatch);
    }
    let mut paint_spans = input.paint_spans.into_vec();
    paint_spans.sort_by_key(|span| {
        (
            span.original_range.start(),
            span.original_range.end(),
            span.identity,
        )
    });
    validate_paint_spans(&paint_spans)?;
    Ok(WorthUiPresentationMechanicBasis {
        mounted_instance: input.mounted_instance,
        mechanic: input.mechanic,
        content_generation: input.content_generation,
        content: input.content,
        layout: input.layout,
        layout_request: input.layout_request,
        layout_width: input.layout_width,
        paint_spans: paint_spans.into_boxed_slice(),
        raster_key_set: WorthUiPresentationRasterKeySetBasis::from_runtime(
            input.raster_keys.into_vec(),
        ),
        text_scale: input.text_scale,
    })
}

fn validate_paint_spans(
    spans: &[WorthUiPresentationPaintSpanBasis],
) -> Result<(), WorthUiPresentationRequestBasisDenial> {
    let mut identities = HashSet::with_capacity(spans.len());
    let mut prior_end = None;
    for span in spans {
        if span.original_range.is_empty() {
            return Err(WorthUiPresentationRequestBasisDenial::EmptyPaintSpan);
        }
        if !identities.insert(span.identity) {
            return Err(WorthUiPresentationRequestBasisDenial::DuplicatePaintSpan);
        }
        if prior_end.is_some_and(|end| end > span.original_range.start()) {
            return Err(WorthUiPresentationRequestBasisDenial::OverlappingPaintSpan);
        }
        prior_end = Some(span.original_range.end());
    }
    Ok(())
}

fn mechanic_sort_key(mechanic: &WorthUiPresentationMechanicBasis) -> (u64, u32, Option<[u8; 32]>) {
    let (slot, row) = mechanic
        .mechanic
        .semantic_text_identity_parts()
        .expect("admitted presentation mechanic remains semantic text");
    (
        mechanic.mounted_instance.diagnostic_value(),
        u32::from(slot),
        row,
    )
}

fn paint_command_sort_key(
    mechanic: &UiMountedPaintCommandIdentity,
) -> (u64, u32, Option<[u8; 32]>) {
    let (slot, row) = mechanic
        .semantic_text_identity_parts()
        .unwrap_or((u16::MAX, None));
    (
        mechanic.mounted_instance().diagnostic_value(),
        u32::from(slot),
        row,
    )
}

fn sort_pins(pins: &mut [WorthUiPresentationPinBasis]) {
    pins.sort_by_key(identity_parts::pin_sort_parts);
}

fn validate_unique_pins(
    pins: &[WorthUiPresentationPinBasis],
    denial: WorthUiPresentationRequestBasisDenial,
) -> Result<(), WorthUiPresentationRequestBasisDenial> {
    let mut seen = HashSet::with_capacity(pins.len());
    for pin in pins {
        if !seen.insert(pin.pin) {
            return Err(denial);
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "request_basis_tests.rs"]
mod tests;
