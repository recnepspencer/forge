use super::AdmittedLayoutStrategy;
use crate::catalog::{
    ArtifactFamilyAccessLane, ArtifactFamilyAuthorityClass, DurableArtifactProjectionClass,
};

impl AdmittedLayoutStrategy {
    pub const fn supports_point_access(&self) -> bool {
        self.declaration.capability().supports_point()
    }
    pub const fn supports_range_access(&self) -> bool {
        self.declaration.capability().supports_range()
    }
    pub const fn supports_prefix_access(&self) -> bool {
        self.declaration.capability().supports_prefix()
    }
    pub const fn supports_scan_access(&self) -> bool {
        self.declaration.capability().supports_scan()
    }
    pub const fn supports_streaming_access(&self) -> bool {
        self.declaration.capability().supports_streaming()
    }
    pub const fn allows_access_lane(&self, lane: ArtifactFamilyAccessLane) -> bool {
        self.declaration.capability().allows_lane(lane)
    }
    pub const fn declared_access_lane(&self) -> ArtifactFamilyAccessLane {
        self.declaration.access_lane()
    }
    pub const fn authority_class(&self) -> ArtifactFamilyAuthorityClass {
        self.declaration.authority_class()
    }
    pub fn supports_projection_class(&self, class: DurableArtifactProjectionClass) -> bool {
        self.declaration.projection_classes().contains(&class)
    }
}
