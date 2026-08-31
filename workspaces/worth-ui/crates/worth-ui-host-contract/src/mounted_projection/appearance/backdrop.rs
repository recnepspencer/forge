#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiMountedBackdropIdentity(Box<str>);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiOverlayPlacementReceipt {
    overlay_revision: u64,
    ordinal: u32,
}

/// Inert host transport attribution for a backdrop mechanic.
///
/// This is not the runtime-owned semantic backdrop projection and grants no
/// publication authority. Its lineage only prevents mechanics from combining
/// fields completed for different surfaces or overlay revisions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMountedBackdropAppearanceAttribution {
    semantic_surface: crate::UiSemanticSurfaceIdentity,
    overlay_revision: u64,
    identity: u64,
    revision: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMountedBackdropMechanic {
    identity: UiMountedBackdropIdentity,
    semantic_surface: crate::UiSemanticSurfaceIdentity,
    placement: UiOverlayPlacementReceipt,
    bounds: super::UiAppearanceDamageRegion,
    clip: super::UiAppearanceClip,
    background: super::UiMountedAppearanceColor,
    opacity: super::UiMountedAppearanceOpacity,
    attribution: UiMountedBackdropAppearanceAttribution,
}

#[doc(hidden)]
pub struct UiMountedBackdropCompletionInput {
    pub identity: UiMountedBackdropIdentity,
    pub semantic_surface: crate::UiSemanticSurfaceIdentity,
    pub placement: UiOverlayPlacementReceipt,
    pub bounds: super::UiAppearanceDamageRegion,
    pub clip: super::UiAppearanceClip,
    pub background: super::UiMountedAppearanceColor,
    pub opacity: super::UiMountedAppearanceOpacity,
    pub attribution: UiMountedBackdropAppearanceAttribution,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMountedBackdropCompletionDenial {
    AttributionSurfaceMismatch,
    AttributionOverlayRevisionMismatch,
}

impl UiMountedBackdropMechanic {
    #[doc(hidden)]
    pub fn complete_from_runtime_mounting(
        input: UiMountedBackdropCompletionInput,
    ) -> Result<Self, UiMountedBackdropCompletionDenial> {
        if input.attribution.semantic_surface != input.semantic_surface {
            return Err(UiMountedBackdropCompletionDenial::AttributionSurfaceMismatch);
        }
        if input.attribution.overlay_revision != input.placement.overlay_revision {
            return Err(UiMountedBackdropCompletionDenial::AttributionOverlayRevisionMismatch);
        }
        Ok(Self {
            identity: input.identity,
            semantic_surface: input.semantic_surface,
            placement: input.placement,
            bounds: input.bounds,
            clip: input.clip,
            background: input.background,
            opacity: input.opacity,
            attribution: input.attribution,
        })
    }

    pub const fn identity(&self) -> &UiMountedBackdropIdentity {
        &self.identity
    }
    pub const fn semantic_surface(&self) -> crate::UiSemanticSurfaceIdentity {
        self.semantic_surface
    }
    pub const fn placement(&self) -> UiOverlayPlacementReceipt {
        self.placement
    }
    pub const fn bounds(&self) -> super::UiAppearanceDamageRegion {
        self.bounds
    }
    pub const fn clip(&self) -> super::UiAppearanceClip {
        self.clip
    }
    pub const fn background(&self) -> super::UiMountedAppearanceColor {
        self.background
    }
    pub const fn opacity(&self) -> super::UiMountedAppearanceOpacity {
        self.opacity
    }
    pub const fn attribution(&self) -> UiMountedBackdropAppearanceAttribution {
        self.attribution
    }
    pub const fn participates_in_hit_testing(&self) -> bool {
        false
    }
}

impl UiMountedBackdropIdentity {
    #[doc(hidden)]
    pub fn from_runtime_mounting(value: impl Into<Box<str>>) -> Option<Self> {
        let value = value.into();
        (!value.is_empty()).then_some(Self(value))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl UiOverlayPlacementReceipt {
    #[doc(hidden)]
    pub const fn from_runtime_overlay_order(overlay_revision: u64, ordinal: u32) -> Option<Self> {
        if overlay_revision == 0 {
            None
        } else {
            Some(Self {
                overlay_revision,
                ordinal,
            })
        }
    }
    pub const fn overlay_revision(self) -> u64 {
        self.overlay_revision
    }
    pub const fn ordinal(self) -> u32 {
        self.ordinal
    }
}

impl UiMountedBackdropAppearanceAttribution {
    #[doc(hidden)]
    pub const fn from_runtime_transport(
        semantic_surface: crate::UiSemanticSurfaceIdentity,
        placement: UiOverlayPlacementReceipt,
        identity: u64,
        revision: u64,
    ) -> Option<Self> {
        if identity == 0 || revision == 0 {
            None
        } else {
            Some(Self {
                semantic_surface,
                overlay_revision: placement.overlay_revision,
                identity,
                revision,
            })
        }
    }

    pub const fn identity(self) -> u64 {
        self.identity
    }
    pub const fn revision(self) -> u64 {
        self.revision
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> UiMountedBackdropCompletionInput {
        let semantic_surface = crate::UiSemanticSurfaceIdentity::mint_unbound().unwrap();
        let placement = UiOverlayPlacementReceipt::from_runtime_overlay_order(3, 0).unwrap();
        UiMountedBackdropCompletionInput {
            identity: UiMountedBackdropIdentity::from_runtime_mounting("dialog.backdrop").unwrap(),
            semantic_surface,
            placement,
            bounds: super::super::UiAppearanceDamageRegion::new(0, 0, 100, 80).unwrap(),
            clip: super::super::UiAppearanceClip::new(0, 0, 100, 80).unwrap(),
            background: super::super::UiMountedAppearanceColor::from_straight_srgba([0, 0, 0, 128]),
            opacity: super::super::UiMountedAppearanceOpacity::ONE,
            attribution: UiMountedBackdropAppearanceAttribution::from_runtime_transport(
                semantic_surface,
                placement,
                7,
                1,
            )
            .unwrap(),
        }
    }

    #[test]
    fn backdrop_completion_uses_distinct_non_node_attribution() {
        let mechanic = UiMountedBackdropMechanic::complete_from_runtime_mounting(input()).unwrap();
        assert_eq!(mechanic.attribution().identity(), 7);
        assert!(!mechanic.participates_in_hit_testing());
    }

    #[test]
    fn backdrop_completion_denies_cross_overlay_attribution() {
        let mut input = input();
        input.placement = UiOverlayPlacementReceipt::from_runtime_overlay_order(4, 0).unwrap();
        assert_eq!(
            UiMountedBackdropMechanic::complete_from_runtime_mounting(input),
            Err(UiMountedBackdropCompletionDenial::AttributionOverlayRevisionMismatch)
        );
    }

    #[test]
    fn backdrop_completion_denies_cross_surface_attribution() {
        let mut input = input();
        input.semantic_surface = crate::UiSemanticSurfaceIdentity::mint_unbound().unwrap();
        assert_eq!(
            UiMountedBackdropMechanic::complete_from_runtime_mounting(input),
            Err(UiMountedBackdropCompletionDenial::AttributionSurfaceMismatch)
        );
    }
}
