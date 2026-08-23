#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct WorthUiPresentationSemanticSubscriberIdentity {
    pub(super) mounted_instance: Option<worth_ui_host_contract::UiMountedInstanceIdentity>,
    pub(super) mechanic: Option<worth_ui_host_contract::UiMountedPaintCommandIdentity>,
    pub(super) mounted_frame: worth_ui_host_contract::UiMountedFrameIdentity,
    pub(super) removal: bool,
    pub(super) content_digest: [u8; 32],
    pub(super) layout_digest: [u8; 32],
    pub(super) foreground_digest: [u8; 32],
    pub(super) raster_key_set_digest: [u8; 32],
    pub(super) source_digest: [u8; 32],
    pub(super) dependency_digests: [[u8; 32]; super::super::DEPENDENCY_COUNT],
    pub(super) attempt: worth_ui_host_contract::UiMountedPresentationAttemptIdentity,
    pub(super) semantic_surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
    pub(super) host_surface: worth_ui_host_contract::UiHostSurfaceIdentity,
    pub(super) binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    pub(super) host_lineage: worth_ui_host_contract::UiHostPresentationLineageIdentity,
}

impl WorthUiPresentationSemanticSubscriberIdentity {
    pub const fn mounted_instance(
        self,
    ) -> Option<worth_ui_host_contract::UiMountedInstanceIdentity> {
        self.mounted_instance
    }

    pub const fn mechanic(self) -> Option<worth_ui_host_contract::UiMountedPaintCommandIdentity> {
        self.mechanic
    }

    pub const fn mounted_frame(self) -> worth_ui_host_contract::UiMountedFrameIdentity {
        self.mounted_frame
    }

    pub const fn removal(self) -> bool {
        self.removal
    }

    pub const fn content_digest(self) -> [u8; 32] {
        self.content_digest
    }

    pub const fn layout_digest(self) -> [u8; 32] {
        self.layout_digest
    }

    pub const fn foreground_digest(self) -> [u8; 32] {
        self.foreground_digest
    }

    pub const fn raster_key_set_digest(self) -> [u8; 32] {
        self.raster_key_set_digest
    }

    pub const fn source_digest(self) -> [u8; 32] {
        self.source_digest
    }

    pub const fn immediate_dependency_digest(
        self,
        change: super::super::WorthUiPresentationSemanticChange,
    ) -> [u8; 32] {
        self.dependency_digests[change.ordinal()]
    }

    pub const fn attempt(self) -> worth_ui_host_contract::UiMountedPresentationAttemptIdentity {
        self.attempt
    }

    pub const fn semantic_surface(self) -> worth_ui_host_contract::UiSemanticSurfaceIdentity {
        self.semantic_surface
    }

    pub const fn host_surface(self) -> worth_ui_host_contract::UiHostSurfaceIdentity {
        self.host_surface
    }

    pub const fn binding(self) -> worth_ui_host_contract::UiSurfaceBindingGeneration {
        self.binding
    }

    pub const fn host_lineage(self) -> worth_ui_host_contract::UiHostPresentationLineageIdentity {
        self.host_lineage
    }
}
