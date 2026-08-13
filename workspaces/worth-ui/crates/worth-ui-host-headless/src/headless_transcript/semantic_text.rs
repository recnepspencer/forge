use std::sync::Arc;

use worth_ui_host_contract::{
    UiMountedAllocationBasis, UiMountedCanonicalBox, UiMountedCollectionRowCorrelation,
    UiMountedContentGeneration, UiMountedInstanceIdentity, UiMountedNodeReceiptIdentity,
    UiMountedPaintCommandIdentity, UiMountedTextForegroundSpan, UiSemanticTextProfile,
    UiSemanticTextSlot,
};

#[derive(Clone, Debug, PartialEq)]
pub struct UiHeadlessSemanticTextMechanic {
    command_identity: UiMountedPaintCommandIdentity,
    content_generation: UiMountedContentGeneration,
    mounted_instance: UiMountedInstanceIdentity,
    node_receipt: UiMountedNodeReceiptIdentity,
    allocation_basis: UiMountedAllocationBasis,
    bounds: UiMountedCanonicalBox,
    origin_x: f32,
    origin_y: f32,
    source: Arc<str>,
    layout_identity: worth_ui_host_contract::UiQualifiedTextLayoutIdentity,
    graphemes: Arc<[worth_ui_host_contract::UiQualifiedTextGraphemeRecord]>,
    styles: Arc<[worth_ui_host_contract::UiQualifiedTextStyleRecord]>,
    coverage: Arc<[worth_ui_host_contract::UiQualifiedTextCoverageRecord]>,
    lines: Arc<[worth_ui_host_contract::UiQualifiedTextLineRecord]>,
    logical_bounds: worth_ui_host_contract::UiTextRect,
    ink_bounds: worth_ui_host_contract::UiTextRect,
    visual_runs: Arc<[worth_ui_host_contract::UiQualifiedTextVisualRunRecord]>,
    carets: Arc<[worth_ui_host_contract::UiQualifiedTextCaretRecord]>,
    cost: worth_ui_host_contract::UiQualifiedTextCostRecord,
    profile_generation: worth_ui_host_contract::UiTextProfileGeneration,
    font_collection_generation: worth_ui_host_contract::UiFontCollectionGeneration,
    text_scale_generation: worth_ui_host_contract::UiTextScaleGeneration,
    slot: UiSemanticTextSlot,
    collection_row: Option<UiMountedCollectionRowCorrelation>,
    foregrounds: Arc<[UiMountedTextForegroundSpan]>,
    profile: UiSemanticTextProfile,
    layer_semantic_order: u32,
    semantic_digest: u64,
}

pub(crate) struct UiHeadlessSemanticTextMechanicInput<'a> {
    pub command_identity: UiMountedPaintCommandIdentity,
    pub content_generation: UiMountedContentGeneration,
    pub mounted_instance: UiMountedInstanceIdentity,
    pub node_receipt: UiMountedNodeReceiptIdentity,
    pub allocation_basis: UiMountedAllocationBasis,
    pub bounds: UiMountedCanonicalBox,
    pub origin_x: f32,
    pub origin_y: f32,
    pub layout: worth_ui_host_contract::UiQualifiedTextLayoutView<'a>,
    pub slot: UiSemanticTextSlot,
    pub collection_row: Option<UiMountedCollectionRowCorrelation>,
    pub foregrounds: Arc<[UiMountedTextForegroundSpan]>,
    pub profile: UiSemanticTextProfile,
    pub layer_semantic_order: u32,
    pub semantic_digest: u64,
}

impl UiHeadlessSemanticTextMechanic {
    pub(crate) fn new(input: UiHeadlessSemanticTextMechanicInput<'_>) -> Self {
        let layout = input.layout;
        Self {
            command_identity: input.command_identity,
            content_generation: input.content_generation,
            mounted_instance: input.mounted_instance,
            node_receipt: input.node_receipt,
            allocation_basis: input.allocation_basis,
            bounds: input.bounds,
            origin_x: input.origin_x,
            origin_y: input.origin_y,
            source: Arc::from(layout.source()),
            layout_identity: layout.identity(),
            graphemes: Arc::from(layout.graphemes()),
            styles: Arc::from(layout.styles()),
            coverage: Arc::from(layout.coverage()),
            lines: Arc::from(layout.lines()),
            logical_bounds: layout.logical_bounds(),
            ink_bounds: layout.ink_bounds(),
            visual_runs: Arc::from(layout.visual_runs()),
            carets: Arc::from(layout.carets()),
            cost: layout.cost(),
            profile_generation: layout.profile_generation(),
            font_collection_generation: layout.font_collection_generation(),
            text_scale_generation: layout.text_scale_generation(),
            slot: input.slot,
            collection_row: input.collection_row,
            foregrounds: input.foregrounds,
            profile: input.profile,
            layer_semantic_order: input.layer_semantic_order,
            semantic_digest: input.semantic_digest,
        }
    }

    pub const fn command_identity(&self) -> UiMountedPaintCommandIdentity {
        self.command_identity
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
        &self.source
    }
    pub fn layout_identity(&self) -> worth_ui_host_contract::UiQualifiedTextLayoutIdentity {
        self.layout_identity
    }
    pub fn graphemes(&self) -> &[worth_ui_host_contract::UiQualifiedTextGraphemeRecord] {
        &self.graphemes
    }
    pub fn styles(&self) -> &[worth_ui_host_contract::UiQualifiedTextStyleRecord] {
        &self.styles
    }
    pub fn coverage(&self) -> &[worth_ui_host_contract::UiQualifiedTextCoverageRecord] {
        &self.coverage
    }
    pub(crate) fn lines(&self) -> &[worth_ui_host_contract::UiQualifiedTextLineRecord] {
        &self.lines
    }
    pub const fn logical_bounds(&self) -> worth_ui_host_contract::UiTextRect {
        self.logical_bounds
    }
    pub const fn ink_bounds(&self) -> worth_ui_host_contract::UiTextRect {
        self.ink_bounds
    }
    pub(crate) fn visual_runs(&self) -> &[worth_ui_host_contract::UiQualifiedTextVisualRunRecord] {
        &self.visual_runs
    }
    pub(crate) fn carets(&self) -> &[worth_ui_host_contract::UiQualifiedTextCaretRecord] {
        &self.carets
    }
    pub const fn qualified_layout_cost(&self) -> worth_ui_host_contract::UiQualifiedTextCostRecord {
        self.cost
    }
    pub(crate) const fn profile_generation(
        &self,
    ) -> worth_ui_host_contract::UiTextProfileGeneration {
        self.profile_generation
    }
    pub const fn font_collection_generation(
        &self,
    ) -> worth_ui_host_contract::UiFontCollectionGeneration {
        self.font_collection_generation
    }
    pub(crate) const fn text_scale_generation(
        &self,
    ) -> worth_ui_host_contract::UiTextScaleGeneration {
        self.text_scale_generation
    }
    pub const fn slot(&self) -> UiSemanticTextSlot {
        self.slot
    }
    pub fn collection_row(&self) -> Option<&UiMountedCollectionRowCorrelation> {
        self.collection_row.as_ref()
    }
    pub fn foregrounds(&self) -> &[UiMountedTextForegroundSpan] {
        &self.foregrounds
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
