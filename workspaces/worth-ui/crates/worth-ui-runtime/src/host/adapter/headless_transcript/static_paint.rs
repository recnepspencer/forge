use worth_ui_host_contract::{
    UiMountedAllocationBasis, UiMountedCanonicalBox, UiMountedFrameIdentity,
    UiMountedInstanceIdentity, UiMountedNodeReceiptIdentity, UiMountedRgba8,
    UiMountedStaticPaintSchemaVersion, UiSemanticSurfaceIdentity, UiSurfaceBindingGeneration,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiHeadlessFilledRectMechanic {
    schema: UiMountedStaticPaintSchemaVersion,
    frame: UiMountedFrameIdentity,
    surface: UiSemanticSurfaceIdentity,
    binding: UiSurfaceBindingGeneration,
    mounted_instance: UiMountedInstanceIdentity,
    node_receipt: UiMountedNodeReceiptIdentity,
    allocation_basis: UiMountedAllocationBasis,
    bounds: UiMountedCanonicalBox,
    color: UiMountedRgba8,
    layer_semantic_order: u32,
    clip_bounds: UiMountedCanonicalBox,
    semantic_digest: u64,
}

pub(in crate::host::adapter) struct UiHeadlessFilledRectMechanicInput {
    pub schema: UiMountedStaticPaintSchemaVersion,
    pub frame: UiMountedFrameIdentity,
    pub surface: UiSemanticSurfaceIdentity,
    pub binding: UiSurfaceBindingGeneration,
    pub mounted_instance: UiMountedInstanceIdentity,
    pub node_receipt: UiMountedNodeReceiptIdentity,
    pub allocation_basis: UiMountedAllocationBasis,
    pub bounds: UiMountedCanonicalBox,
    pub color: UiMountedRgba8,
    pub layer_semantic_order: u32,
    pub clip_bounds: UiMountedCanonicalBox,
    pub semantic_digest: u64,
}

impl UiHeadlessFilledRectMechanic {
    pub(in crate::host::adapter) const fn new(input: UiHeadlessFilledRectMechanicInput) -> Self {
        Self {
            schema: input.schema,
            frame: input.frame,
            surface: input.surface,
            binding: input.binding,
            mounted_instance: input.mounted_instance,
            node_receipt: input.node_receipt,
            allocation_basis: input.allocation_basis,
            bounds: input.bounds,
            color: input.color,
            layer_semantic_order: input.layer_semantic_order,
            clip_bounds: input.clip_bounds,
            semantic_digest: input.semantic_digest,
        }
    }

    pub const fn schema(self) -> UiMountedStaticPaintSchemaVersion {
        self.schema
    }

    pub const fn frame(self) -> UiMountedFrameIdentity {
        self.frame
    }

    pub const fn surface(self) -> UiSemanticSurfaceIdentity {
        self.surface
    }

    pub const fn binding(self) -> UiSurfaceBindingGeneration {
        self.binding
    }

    pub const fn mounted_instance(self) -> UiMountedInstanceIdentity {
        self.mounted_instance
    }

    pub const fn node_receipt(self) -> UiMountedNodeReceiptIdentity {
        self.node_receipt
    }

    pub const fn allocation_basis(self) -> UiMountedAllocationBasis {
        self.allocation_basis
    }

    pub const fn bounds(self) -> UiMountedCanonicalBox {
        self.bounds
    }

    pub const fn color(self) -> UiMountedRgba8 {
        self.color
    }

    pub const fn layer_semantic_order(self) -> u32 {
        self.layer_semantic_order
    }

    pub const fn clip_bounds(self) -> UiMountedCanonicalBox {
        self.clip_bounds
    }

    pub const fn semantic_digest(self) -> u64 {
        self.semantic_digest
    }
}
