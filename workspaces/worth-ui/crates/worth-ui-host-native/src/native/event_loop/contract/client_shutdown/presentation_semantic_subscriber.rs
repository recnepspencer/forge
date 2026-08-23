#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UiNativeClientPresentationSemanticSubscriberObservation {
    mounted_instance: Option<u64>,
    semantic_slot: Option<u16>,
    collection_row: Option<[u8; 32]>,
    mounted_frame: u64,
    removal: bool,
    content_digest: [u8; 32],
    layout_digest: [u8; 32],
    foreground_digest: [u8; 32],
    raster_key_set_digest: [u8; 32],
    source_digest: [u8; 32],
    immediate_dependency_digest: [u8; 32],
    attempt: u64,
    semantic_surface: u64,
    host_surface: u64,
    binding: u64,
    host_lineage: u64,
}

impl UiNativeClientPresentationSemanticSubscriberObservation {
    #[doc(hidden)]
    pub const fn reported(
        mounted_instance: Option<u64>,
        semantic_slot: Option<u16>,
        collection_row: Option<[u8; 32]>,
        mounted_frame: u64,
        removal: bool,
        content_digest: [u8; 32],
        layout_digest: [u8; 32],
        foreground_digest: [u8; 32],
        raster_key_set_digest: [u8; 32],
        source_digest: [u8; 32],
        immediate_dependency_digest: [u8; 32],
        request_identity: [u64; 5],
    ) -> Self {
        Self {
            mounted_instance,
            semantic_slot,
            collection_row,
            mounted_frame,
            removal,
            content_digest,
            layout_digest,
            foreground_digest,
            raster_key_set_digest,
            source_digest,
            immediate_dependency_digest,
            attempt: request_identity[0],
            semantic_surface: request_identity[1],
            host_surface: request_identity[2],
            binding: request_identity[3],
            host_lineage: request_identity[4],
        }
    }

    pub const fn mounted_instance(self) -> Option<u64> {
        self.mounted_instance
    }

    pub const fn semantic_slot(self) -> Option<u16> {
        self.semantic_slot
    }

    pub const fn collection_row(self) -> Option<[u8; 32]> {
        self.collection_row
    }

    pub const fn mounted_frame(self) -> u64 {
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

    pub const fn immediate_dependency_digest(self) -> [u8; 32] {
        self.immediate_dependency_digest
    }

    pub const fn attempt(self) -> u64 {
        self.attempt
    }

    pub const fn semantic_surface(self) -> u64 {
        self.semantic_surface
    }

    pub const fn host_surface(self) -> u64 {
        self.host_surface
    }

    pub const fn binding(self) -> u64 {
        self.binding
    }

    pub const fn host_lineage(self) -> u64 {
        self.host_lineage
    }
}
