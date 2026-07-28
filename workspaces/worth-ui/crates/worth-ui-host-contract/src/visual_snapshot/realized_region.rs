#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostRealizedRegionParticipation {
    Paint,
    HitTest,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiHostRealizedRegion {
    mounted_receipt: crate::UiMountedNodeReceiptIdentity,
    bounds: crate::UiMountedCanonicalBox,
    clip: crate::UiMountedCanonicalBox,
    semantic_order: u32,
    participation: UiHostRealizedRegionParticipation,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiHostRealizedGeometry {
    bounds: crate::UiMountedCanonicalBox,
    clip: crate::UiMountedCanonicalBox,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostRealizedOrdering {
    semantic_order: u32,
    participation: UiHostRealizedRegionParticipation,
}

impl UiHostRealizedRegion {
    #[doc(hidden)]
    pub const fn observed_by_host(
        mounted_receipt: crate::UiMountedNodeReceiptIdentity,
        geometry: UiHostRealizedGeometry,
        ordering: UiHostRealizedOrdering,
    ) -> Self {
        Self {
            mounted_receipt,
            bounds: geometry.bounds,
            clip: geometry.clip,
            semantic_order: ordering.semantic_order,
            participation: ordering.participation,
        }
    }

    pub const fn mounted_receipt(self) -> crate::UiMountedNodeReceiptIdentity {
        self.mounted_receipt
    }

    pub const fn bounds(self) -> crate::UiMountedCanonicalBox {
        self.bounds
    }

    pub const fn clip(self) -> crate::UiMountedCanonicalBox {
        self.clip
    }

    pub const fn semantic_order(self) -> u32 {
        self.semantic_order
    }

    pub const fn participation(self) -> UiHostRealizedRegionParticipation {
        self.participation
    }
}

impl UiHostRealizedGeometry {
    #[doc(hidden)]
    pub const fn observed_by_host(
        bounds: crate::UiMountedCanonicalBox,
        clip: crate::UiMountedCanonicalBox,
    ) -> Self {
        Self { bounds, clip }
    }
}

impl UiHostRealizedOrdering {
    #[doc(hidden)]
    pub const fn observed_by_host(
        semantic_order: u32,
        participation: UiHostRealizedRegionParticipation,
    ) -> Self {
        Self {
            semantic_order,
            participation,
        }
    }
}
