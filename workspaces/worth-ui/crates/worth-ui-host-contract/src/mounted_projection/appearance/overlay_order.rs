use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiOverlayParticipantIdentity {
    Portal(crate::UiMountedInstanceIdentity),
    Backdrop(super::UiMountedBackdropIdentity),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiOverlayStackSnapshot {
    semantic_surface: crate::UiSemanticSurfaceIdentity,
    presentation: crate::UiMountedPresentationAttemptIdentity,
    portal_revision: u64,
    backdrop_revision: u64,
    bottom_to_top: Box<[UiOverlayParticipantIdentity]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiOverlayStackSnapshotDenial {
    DuplicateParticipant(UiOverlayParticipantIdentity),
}

impl UiOverlayStackSnapshot {
    #[doc(hidden)]
    pub fn complete_from_runtime_overlay_order(
        semantic_surface: crate::UiSemanticSurfaceIdentity,
        presentation: crate::UiMountedPresentationAttemptIdentity,
        portal_revision: u64,
        backdrop_revision: u64,
        bottom_to_top: impl IntoIterator<Item = UiOverlayParticipantIdentity>,
    ) -> Result<Self, UiOverlayStackSnapshotDenial> {
        let bottom_to_top = bottom_to_top.into_iter().collect::<Vec<_>>();
        let mut seen = BTreeSet::new();
        if let Some(duplicate) = bottom_to_top
            .iter()
            .find(|participant| !seen.insert((*participant).clone()))
        {
            return Err(UiOverlayStackSnapshotDenial::DuplicateParticipant(
                duplicate.clone(),
            ));
        }
        Ok(Self {
            semantic_surface,
            presentation,
            portal_revision,
            backdrop_revision,
            bottom_to_top: bottom_to_top.into_boxed_slice(),
        })
    }

    pub const fn semantic_surface(&self) -> crate::UiSemanticSurfaceIdentity {
        self.semantic_surface
    }
    pub const fn presentation(&self) -> crate::UiMountedPresentationAttemptIdentity {
        self.presentation
    }
    pub const fn portal_revision(&self) -> u64 {
        self.portal_revision
    }
    pub const fn backdrop_revision(&self) -> u64 {
        self.backdrop_revision
    }
    pub fn bottom_to_top(&self) -> &[UiOverlayParticipantIdentity] {
        &self.bottom_to_top
    }
}
