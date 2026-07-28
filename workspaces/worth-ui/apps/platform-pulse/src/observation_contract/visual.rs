use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PlatformPulseVisualSnapshotRelationObservation {
    Current,
    RetainedPredecessor,
    Historical,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PlatformPulseVisualCoordinateOrientationObservation {
    TopLeftOrigin,
    BottomLeftOrigin,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PlatformPulseVisualCoordinateRoundingObservation {
    PixelCenterNearest,
    FloorEdges,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PlatformPulseVisualPixelColorSpaceObservation {
    Srgb,
    AdapterDeclared,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PlatformPulseVisualEvidenceFamilyObservation {
    Declaration,
    Admission,
    Graph,
    Planning,
    Aspect,
    Obligation,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseVisualSnapshotAffinityObservation {
    pub(super) snapshot: u64,
    pub(super) presentation_attempt: u64,
    pub(super) frame: u64,
    pub(super) semantic_surface: u64,
    pub(super) host_surface: u64,
    pub(super) binding_generation: u64,
    pub(super) presentation_epoch: u64,
    pub(super) relation: PlatformPulseVisualSnapshotRelationObservation,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseVisualCoordinateObservation {
    pub(super) native_client_origin: [i32; 2],
    pub(super) client_physical_dimensions: [u32; 2],
    pub(super) viewport_logical_dimension_bits: [u32; 2],
    pub(super) scale_bits: [u32; 2],
    pub(super) translation_bits: [u32; 2],
    pub(super) orientation: PlatformPulseVisualCoordinateOrientationObservation,
    pub(super) rounding: PlatformPulseVisualCoordinateRoundingObservation,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseVisualPixelObservation {
    pub(super) dimensions: [u32; 2],
    pub(super) stride: u32,
    pub(super) byte_count: u64,
    pub(super) color_space: PlatformPulseVisualPixelColorSpaceObservation,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseVisualSnapshotCaptured {
    pub(super) affinity: PlatformPulseVisualSnapshotAffinityObservation,
    pub(super) captured_client_extent: [u32; 4],
    pub(super) coordinates: PlatformPulseVisualCoordinateObservation,
    pub(super) pixels: PlatformPulseVisualPixelObservation,
    pub(super) visible_region_count: u64,
    pub(super) hit_test_region_count: u64,
    pub(super) cost_counters: [u64; 11],
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseVisualMountedNodeObservation {
    pub(super) node_receipt: u64,
    pub(super) mounted_instance: u64,
    pub(super) incarnation: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseVisualEvidenceObservation {
    pub(super) family: PlatformPulseVisualEvidenceFamilyObservation,
    pub(super) authority_generation: u64,
    pub(super) identity_digest: u64,
    pub(super) handle_digest: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseVisualIdentityTraceObservation {
    pub(super) mounted: PlatformPulseVisualMountedNodeObservation,
    pub(super) graph_node: u64,
    pub(super) declaration: u64,
    pub(super) authored_semantic_name: String,
    pub(super) source_artifact_path: String,
    pub(super) source_generation: u64,
    pub(super) declaration_index: u64,
    pub(super) evidence: Box<[PlatformPulseVisualEvidenceObservation]>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseVisualPointResolutionObservation {
    pub(super) point: [u32; 2],
    pub(super) visible_region: [u32; 4],
    pub(super) visible: PlatformPulseVisualIdentityTraceObservation,
    pub(super) hit: PlatformPulseVisualIdentityTraceObservation,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseVisualPointTrace {
    pub(super) snapshot: u64,
    pub(super) target: PlatformPulseVisualPointResolutionObservation,
    pub(super) background: PlatformPulseVisualPointResolutionObservation,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseVisualOverlayPublished {
    pub(super) overlay: u64,
    pub(super) base_snapshot: u64,
    pub(super) base_frame: u64,
    pub(super) target_region: [u32; 4],
    pub(super) published_frame: u64,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseVisualOverlayCleared {
    pub(super) overlay: u64,
    pub(super) published_frame: u64,
    pub(super) cleared_frame: u64,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlatformPulseVisualSnapshotRetired {
    pub(super) snapshot: u64,
    pub(super) predecessor_frame: u64,
    pub(super) successor_frame: u64,
    pub(super) explicitly_superseded: bool,
    pub(super) released_registered_resource: bool,
}

macro_rules! copy_accessors {
    ($type:ty, $($name:ident : $return:ty),+ $(,)?) => {
        impl $type {
            $(pub const fn $name(&self) -> $return { self.$name })+
        }
    };
}

copy_accessors!(
    PlatformPulseVisualSnapshotAffinityObservation,
    snapshot: u64,
    presentation_attempt: u64,
    frame: u64,
    semantic_surface: u64,
    host_surface: u64,
    binding_generation: u64,
    presentation_epoch: u64,
    relation: PlatformPulseVisualSnapshotRelationObservation,
);
copy_accessors!(
    PlatformPulseVisualCoordinateObservation,
    native_client_origin: [i32; 2],
    client_physical_dimensions: [u32; 2],
    viewport_logical_dimension_bits: [u32; 2],
    scale_bits: [u32; 2],
    translation_bits: [u32; 2],
    orientation: PlatformPulseVisualCoordinateOrientationObservation,
    rounding: PlatformPulseVisualCoordinateRoundingObservation,
);
copy_accessors!(
    PlatformPulseVisualPixelObservation,
    dimensions: [u32; 2],
    stride: u32,
    byte_count: u64,
    color_space: PlatformPulseVisualPixelColorSpaceObservation,
);
copy_accessors!(
    PlatformPulseVisualMountedNodeObservation,
    node_receipt: u64,
    mounted_instance: u64,
    incarnation: u64,
);
copy_accessors!(
    PlatformPulseVisualOverlayPublished,
    overlay: u64,
    base_snapshot: u64,
    base_frame: u64,
    target_region: [u32; 4],
    published_frame: u64,
);
copy_accessors!(
    PlatformPulseVisualOverlayCleared,
    overlay: u64,
    published_frame: u64,
    cleared_frame: u64,
);
copy_accessors!(
    PlatformPulseVisualSnapshotRetired,
    snapshot: u64,
    predecessor_frame: u64,
    successor_frame: u64,
    explicitly_superseded: bool,
    released_registered_resource: bool,
);

impl PlatformPulseVisualSnapshotCaptured {
    pub const fn affinity(&self) -> PlatformPulseVisualSnapshotAffinityObservation {
        self.affinity
    }

    pub const fn captured_client_extent(&self) -> [u32; 4] {
        self.captured_client_extent
    }

    pub const fn coordinates(&self) -> PlatformPulseVisualCoordinateObservation {
        self.coordinates
    }

    pub const fn pixels(&self) -> PlatformPulseVisualPixelObservation {
        self.pixels
    }

    pub const fn visible_region_count(&self) -> u64 {
        self.visible_region_count
    }

    pub const fn hit_test_region_count(&self) -> u64 {
        self.hit_test_region_count
    }

    pub const fn cost_counters(&self) -> [u64; 11] {
        self.cost_counters
    }
}

impl PlatformPulseVisualEvidenceObservation {
    pub const fn family(&self) -> PlatformPulseVisualEvidenceFamilyObservation {
        self.family
    }

    pub const fn authority_generation(&self) -> u64 {
        self.authority_generation
    }

    pub const fn identity_digest(&self) -> u64 {
        self.identity_digest
    }

    pub const fn handle_digest(&self) -> u64 {
        self.handle_digest
    }
}

impl PlatformPulseVisualIdentityTraceObservation {
    pub const fn mounted(&self) -> PlatformPulseVisualMountedNodeObservation {
        self.mounted
    }

    pub const fn graph_node(&self) -> u64 {
        self.graph_node
    }

    pub const fn declaration(&self) -> u64 {
        self.declaration
    }

    pub fn authored_semantic_name(&self) -> &str {
        &self.authored_semantic_name
    }

    pub fn source_artifact_path(&self) -> &str {
        &self.source_artifact_path
    }

    pub const fn source_generation(&self) -> u64 {
        self.source_generation
    }

    pub const fn declaration_index(&self) -> u64 {
        self.declaration_index
    }

    pub fn evidence(&self) -> &[PlatformPulseVisualEvidenceObservation] {
        &self.evidence
    }
}

impl PlatformPulseVisualPointResolutionObservation {
    pub const fn point(&self) -> [u32; 2] {
        self.point
    }

    pub const fn visible_region(&self) -> [u32; 4] {
        self.visible_region
    }

    pub const fn visible(&self) -> &PlatformPulseVisualIdentityTraceObservation {
        &self.visible
    }

    pub const fn hit(&self) -> &PlatformPulseVisualIdentityTraceObservation {
        &self.hit
    }
}

impl PlatformPulseVisualPointTrace {
    pub const fn snapshot(&self) -> u64 {
        self.snapshot
    }

    pub const fn target(&self) -> &PlatformPulseVisualPointResolutionObservation {
        &self.target
    }

    pub const fn background(&self) -> &PlatformPulseVisualPointResolutionObservation {
        &self.background
    }
}
