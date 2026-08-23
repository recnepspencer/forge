use crate::native::UiNativePresentationObservation;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiNativeClientPresentationAttribution {
    frame: u64,
    surface: u64,
    binding: u64,
    mounted_instance: u64,
    node_receipt: u64,
    presentation_attempt: u64,
    authored_provenance_digest: u64,
    authored_semantic_identity_digest: u64,
}

impl UiNativeClientPresentationAttribution {
    pub const fn reported(mechanical: [u64; 6], authored: [u64; 2]) -> Self {
        let [frame, surface, binding, mounted_instance, node_receipt, presentation_attempt] =
            mechanical;
        let [authored_provenance_digest, authored_semantic_identity_digest] = authored;
        Self {
            frame,
            surface,
            binding,
            mounted_instance,
            node_receipt,
            presentation_attempt,
            authored_provenance_digest,
            authored_semantic_identity_digest,
        }
    }

    pub(in crate::native::event_loop) const fn matches(
        self,
        observation: &UiNativePresentationObservation,
    ) -> bool {
        self.frame == observation.presented_frame()
            && self.surface == observation.semantic_surface()
            && self.binding == observation.binding_generation()
            && self.mounted_instance == observation.mounted_instance()
            && self.node_receipt == observation.node_receipt()
            && self.presentation_attempt == observation.presentation_attempt()
    }

    pub const fn frame(self) -> u64 {
        self.frame
    }

    pub const fn surface(self) -> u64 {
        self.surface
    }

    pub const fn binding(self) -> u64 {
        self.binding
    }

    pub const fn mounted_instance(self) -> u64 {
        self.mounted_instance
    }

    pub const fn node_receipt(self) -> u64 {
        self.node_receipt
    }

    pub const fn presentation_attempt(self) -> u64 {
        self.presentation_attempt
    }

    pub const fn authored_provenance_digest(self) -> u64 {
        self.authored_provenance_digest
    }

    pub const fn authored_semantic_identity_digest(self) -> u64 {
        self.authored_semantic_identity_digest
    }
}
