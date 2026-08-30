#[derive(Debug)]
pub(crate) enum UiPortalFocusTransitionDenial {
    DuplicateProposal,
    UnknownProposal,
    ForeignPreparedFrame,
    StaleFocusRevision,
    Routing,
}

#[must_use = "a prepared portal focus transition changes no focus truth until publication settles"]
pub(super) struct UiPreparedPortalFocusTransition {
    boundary: super::UiPortalFocusBoundaryIdentity,
    opening: bool,
    expected_revision: u64,
    snapshot: crate::mounting::UiMountedFocusParticipationSnapshot,
    next: Option<super::UiSemanticKeyboardFocus>,
    restoration: Option<super::UiFocusRestorationToken>,
    closed_descendants: Box<[super::UiPortalFocusBoundaryIdentity]>,
}

impl UiPreparedPortalFocusTransition {
    pub(super) fn new(
        boundary: super::UiPortalFocusBoundaryIdentity,
        opening: bool,
        expected_revision: u64,
        snapshot: crate::mounting::UiMountedFocusParticipationSnapshot,
        next: Option<super::UiSemanticKeyboardFocus>,
        restoration: Option<super::UiFocusRestorationToken>,
        closed_descendants: Box<[super::UiPortalFocusBoundaryIdentity]>,
    ) -> Self {
        Self {
            boundary,
            opening,
            expected_revision,
            snapshot,
            next,
            restoration,
            closed_descendants,
        }
    }

    pub(super) const fn frame(&self) -> worth_ui_host_contract::UiMountedFrameIdentity {
        self.snapshot.frame()
    }

    pub(super) const fn boundary(&self) -> super::UiPortalFocusBoundaryIdentity {
        self.boundary
    }

    pub(super) const fn opening(&self) -> bool {
        self.opening
    }

    pub(super) const fn expected_revision(&self) -> u64 {
        self.expected_revision
    }

    pub(super) fn snapshot(&self) -> &crate::mounting::UiMountedFocusParticipationSnapshot {
        &self.snapshot
    }

    pub(super) const fn next(&self) -> Option<super::UiSemanticKeyboardFocus> {
        self.next
    }

    pub(super) const fn restoration(&self) -> Option<super::UiFocusRestorationToken> {
        self.restoration
    }

    pub(super) fn closed_descendants(&self) -> &[super::UiPortalFocusBoundaryIdentity] {
        &self.closed_descendants
    }
}
