use std::sync::Arc;

use worth_ui_host_contract::{
    UiMountedAllocationBasis, UiMountedCanonicalBox, UiMountedCollectionRowCorrelation,
    UiMountedContentGeneration, UiMountedInstanceIdentity, UiMountedNodeReceiptIdentity,
    UiMountedRgba8, UiSemanticTextProfile, UiSemanticTextSlot,
};

#[derive(Clone, Debug, PartialEq)]
pub struct UiHeadlessSemanticTextMechanic {
    content_generation: UiMountedContentGeneration,
    mounted_instance: UiMountedInstanceIdentity,
    node_receipt: UiMountedNodeReceiptIdentity,
    allocation_basis: UiMountedAllocationBasis,
    bounds: UiMountedCanonicalBox,
    origin_x: f32,
    origin_y: f32,
    text: Arc<str>,
    slot: UiSemanticTextSlot,
    collection_row: Option<UiMountedCollectionRowCorrelation>,
    color: UiMountedRgba8,
    profile: UiSemanticTextProfile,
    layer_semantic_order: u32,
    semantic_digest: u64,
}

pub(in crate::host::adapter) struct UiHeadlessSemanticTextMechanicInput {
    pub content_generation: UiMountedContentGeneration,
    pub mounted_instance: UiMountedInstanceIdentity,
    pub node_receipt: UiMountedNodeReceiptIdentity,
    pub allocation_basis: UiMountedAllocationBasis,
    pub bounds: UiMountedCanonicalBox,
    pub origin_x: f32,
    pub origin_y: f32,
    pub text: Arc<str>,
    pub slot: UiSemanticTextSlot,
    pub collection_row: Option<UiMountedCollectionRowCorrelation>,
    pub color: UiMountedRgba8,
    pub profile: UiSemanticTextProfile,
    pub layer_semantic_order: u32,
    pub semantic_digest: u64,
}

impl UiHeadlessSemanticTextMechanic {
    pub(in crate::host::adapter) fn new(input: UiHeadlessSemanticTextMechanicInput) -> Self {
        Self {
            content_generation: input.content_generation,
            mounted_instance: input.mounted_instance,
            node_receipt: input.node_receipt,
            allocation_basis: input.allocation_basis,
            bounds: input.bounds,
            origin_x: input.origin_x,
            origin_y: input.origin_y,
            text: input.text,
            slot: input.slot,
            collection_row: input.collection_row,
            color: input.color,
            profile: input.profile,
            layer_semantic_order: input.layer_semantic_order,
            semantic_digest: input.semantic_digest,
        }
    }

    pub const fn content_generation(&self) -> UiMountedContentGeneration {
        self.content_generation
    }
    pub const fn mounted_instance(&self) -> UiMountedInstanceIdentity {
        self.mounted_instance
    }
    pub const fn node_receipt(&self) -> UiMountedNodeReceiptIdentity {
        self.node_receipt
    }
    pub const fn allocation_basis(&self) -> UiMountedAllocationBasis {
        self.allocation_basis
    }
    pub const fn bounds(&self) -> UiMountedCanonicalBox {
        self.bounds
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
    pub const fn color(&self) -> UiMountedRgba8 {
        self.color
    }
    pub const fn profile(&self) -> UiSemanticTextProfile {
        self.profile
    }
    pub const fn layer_semantic_order(&self) -> u32 {
        self.layer_semantic_order
    }
    pub const fn semantic_digest(&self) -> u64 {
        self.semantic_digest
    }
}
