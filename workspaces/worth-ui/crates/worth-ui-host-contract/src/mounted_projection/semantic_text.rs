use std::sync::Arc;

use crate::{
    UiMountedContentGeneration, UiMountedFrameIdentity, UiMountedInstanceIdentity,
    UiMountedNodeReceiptIdentity, UiSemanticSurfaceIdentity, UiSurfaceBindingGeneration,
    WorthUiHostCapabilityObservationGeneration,
};

mod validation;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMountedTextSchemaVersion(u16);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiSemanticTextProfile {
    BodyDefault,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiSemanticTextWrapPosture {
    Clip,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiSemanticTextBaselinePosture {
    Alphabetic,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiSemanticTextSlot {
    Value,
    CollectionValue { selected_field_ordinal: u16 },
    Posture,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMountedCollectionRowCorrelation([u8; 32]);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMountedSemanticTextReference(u16);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMountedSemanticTextCompletionDenial {
    NonAreaGeometry,
    ClipMismatch,
    InvalidTextOrigin,
    NodeReceiptFrameMismatch,
    NodeReceiptInstanceMismatch,
    CollectionIdentityMismatch,
    ContentCapacityExceeded,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMountedSemanticTextTableDenial {
    CapacityExceeded,
    ByteCapacityExceeded,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UiMountedSemanticTextMechanic {
    schema: UiMountedTextSchemaVersion,
    content_generation: UiMountedContentGeneration,
    frame: UiMountedFrameIdentity,
    surface: UiSemanticSurfaceIdentity,
    binding: UiSurfaceBindingGeneration,
    mounted_instance: UiMountedInstanceIdentity,
    node_receipt: UiMountedNodeReceiptIdentity,
    allocation_basis: super::UiMountedAllocationBasis,
    bounds: super::UiMountedCanonicalBox,
    clip_bounds: super::UiMountedCanonicalBox,
    origin_x: f32,
    origin_y: f32,
    text: Arc<str>,
    slot: UiSemanticTextSlot,
    collection_row: Option<UiMountedCollectionRowCorrelation>,
    color: super::UiMountedRgba8,
    profile: UiSemanticTextProfile,
    layer_semantic_order: u32,
    capability_generation: WorthUiHostCapabilityObservationGeneration,
    capability_profile_digest: u64,
    semantic_digest: u64,
}

#[doc(hidden)]
#[derive(Clone)]
pub struct UiMountedSemanticTextCompletionInput {
    pub content_generation: UiMountedContentGeneration,
    pub frame: UiMountedFrameIdentity,
    pub surface: UiSemanticSurfaceIdentity,
    pub binding: UiSurfaceBindingGeneration,
    pub mounted_instance: UiMountedInstanceIdentity,
    pub node_receipt: UiMountedNodeReceiptIdentity,
    pub allocation_basis: super::UiMountedAllocationBasis,
    pub bounds: super::UiMountedCanonicalBox,
    pub clip_bounds: super::UiMountedCanonicalBox,
    pub origin_x: f32,
    pub origin_y: f32,
    pub text: Arc<str>,
    pub slot: UiSemanticTextSlot,
    pub collection_row: Option<UiMountedCollectionRowCorrelation>,
    pub color: super::UiMountedRgba8,
    pub profile: UiSemanticTextProfile,
    pub layer_semantic_order: u32,
    pub capability_generation: WorthUiHostCapabilityObservationGeneration,
    pub capability_profile_digest: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UiMountedSemanticTextTable {
    schema: UiMountedTextSchemaVersion,
    rows: Box<[UiMountedSemanticTextMechanic]>,
}

impl UiMountedTextSchemaVersion {
    pub const REQUIRED_MOUNTED_FRAME_REVISION: u16 = 3;

    pub const fn current() -> Self {
        Self(1)
    }

    pub const fn revision(self) -> u16 {
        self.0
    }
}

impl UiSemanticTextProfile {
    pub const fn size_millipoints(self) -> u16 {
        match self {
            Self::BodyDefault => 14_000,
        }
    }

    pub const fn weight(self) -> u16 {
        match self {
            Self::BodyDefault => 400,
        }
    }

    pub const fn wrap(self) -> UiSemanticTextWrapPosture {
        match self {
            Self::BodyDefault => UiSemanticTextWrapPosture::Clip,
        }
    }

    pub const fn baseline(self) -> UiSemanticTextBaselinePosture {
        match self {
            Self::BodyDefault => UiSemanticTextBaselinePosture::Alphabetic,
        }
    }
}

impl UiMountedSemanticTextReference {
    #[doc(hidden)]
    pub const fn from_runtime_mounting(index: u16) -> Self {
        Self(index)
    }

    pub const fn index(self) -> u16 {
        self.0
    }
}

impl UiMountedCollectionRowCorrelation {
    #[doc(hidden)]
    pub const fn from_runtime_mounting(identity: [u8; 32]) -> Self {
        Self(identity)
    }

    pub const fn correlation_digest(self) -> [u8; 32] {
        self.0
    }
}

impl UiMountedSemanticTextMechanic {
    pub const MAX_CONTENT_BYTES: usize = 4_096;

    #[doc(hidden)]
    pub fn complete_from_runtime_mounting(
        input: UiMountedSemanticTextCompletionInput,
    ) -> Result<Self, UiMountedSemanticTextCompletionDenial> {
        validation::validate_completion(&input)?;
        let semantic_digest = validation::semantic_digest(&input);
        Ok(Self {
            schema: UiMountedTextSchemaVersion::current(),
            content_generation: input.content_generation,
            frame: input.frame,
            surface: input.surface,
            binding: input.binding,
            mounted_instance: input.mounted_instance,
            node_receipt: input.node_receipt,
            allocation_basis: input.allocation_basis,
            bounds: input.bounds,
            clip_bounds: input.clip_bounds,
            origin_x: input.origin_x,
            origin_y: input.origin_y,
            text: input.text,
            slot: input.slot,
            collection_row: input.collection_row,
            color: input.color,
            profile: input.profile,
            layer_semantic_order: input.layer_semantic_order,
            capability_generation: input.capability_generation,
            capability_profile_digest: input.capability_profile_digest,
            semantic_digest,
        })
    }

    pub const fn schema(&self) -> UiMountedTextSchemaVersion {
        self.schema
    }
    pub const fn content_generation(&self) -> UiMountedContentGeneration {
        self.content_generation
    }
    pub const fn frame(&self) -> UiMountedFrameIdentity {
        self.frame
    }
    pub const fn surface(&self) -> UiSemanticSurfaceIdentity {
        self.surface
    }
    pub const fn binding(&self) -> UiSurfaceBindingGeneration {
        self.binding
    }
    pub const fn mounted_instance(&self) -> UiMountedInstanceIdentity {
        self.mounted_instance
    }
    pub const fn node_receipt(&self) -> UiMountedNodeReceiptIdentity {
        self.node_receipt
    }
    pub const fn allocation_basis(&self) -> super::UiMountedAllocationBasis {
        self.allocation_basis
    }
    pub const fn bounds(&self) -> super::UiMountedCanonicalBox {
        self.bounds
    }
    pub const fn clip_bounds(&self) -> super::UiMountedCanonicalBox {
        self.clip_bounds
    }
    pub fn origin_x(&self) -> f32 {
        self.origin_x
    }
    pub fn origin_y(&self) -> f32 {
        self.origin_y
    }
    pub fn text(&self) -> &str {
        &self.text
    }
    pub const fn slot(&self) -> UiSemanticTextSlot {
        self.slot
    }
    pub fn collection_row(&self) -> Option<&UiMountedCollectionRowCorrelation> {
        self.collection_row.as_ref()
    }
    pub const fn color(&self) -> super::UiMountedRgba8 {
        self.color
    }
    pub const fn profile(&self) -> UiSemanticTextProfile {
        self.profile
    }
    pub const fn layer_semantic_order(&self) -> u32 {
        self.layer_semantic_order
    }
    pub const fn capability_generation(&self) -> WorthUiHostCapabilityObservationGeneration {
        self.capability_generation
    }
    pub const fn capability_profile_digest(&self) -> u64 {
        self.capability_profile_digest
    }
    pub const fn semantic_digest(&self) -> u64 {
        self.semantic_digest
    }
}

impl UiMountedSemanticTextTable {
    pub const MAX_ROWS: usize = 2_048;
    pub const MAX_BYTES: usize = 1_048_576;

    pub fn empty() -> Self {
        Self {
            schema: UiMountedTextSchemaVersion::current(),
            rows: Box::new([]),
        }
    }

    #[doc(hidden)]
    pub fn from_runtime_mounting(
        rows: Vec<UiMountedSemanticTextMechanic>,
    ) -> Result<Self, UiMountedSemanticTextTableDenial> {
        if rows.len() > Self::MAX_ROWS {
            return Err(UiMountedSemanticTextTableDenial::CapacityExceeded);
        }
        let bytes = rows.iter().try_fold(0usize, |total, row| {
            total.checked_add(row.text.len()).and_then(|total| {
                total.checked_add(
                    row.collection_row
                        .as_ref()
                        .map_or(0, |_| std::mem::size_of::<[u8; 32]>()),
                )
            })
        });
        if bytes.is_none_or(|bytes| bytes > Self::MAX_BYTES) {
            return Err(UiMountedSemanticTextTableDenial::ByteCapacityExceeded);
        }
        Ok(Self {
            schema: UiMountedTextSchemaVersion::current(),
            rows: rows.into_boxed_slice(),
        })
    }

    pub const fn schema(&self) -> UiMountedTextSchemaVersion {
        self.schema
    }
    pub fn rows(&self) -> &[UiMountedSemanticTextMechanic] {
        &self.rows
    }
    pub fn resolve(
        &self,
        reference: UiMountedSemanticTextReference,
    ) -> Option<&UiMountedSemanticTextMechanic> {
        self.rows.get(usize::from(reference.index()))
    }
}

#[cfg(test)]
#[path = "semantic_text_tests.rs"]
mod tests;
