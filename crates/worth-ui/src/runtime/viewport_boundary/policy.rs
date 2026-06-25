#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiScrollOwner {
    None,
    Composition,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiClipPosture {
    None,
    ClipToViewport,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiViewportBasis {
    AllocatedFrame,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiViewportParticipationPolicy {
    AllDescendants,
    VisibleDescendantsOnly,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiScrollRestorationPolicy {
    None,
    ByCompositionIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiViewportBoundaryPolicyReceipt {
    policy_identity: String,
    scroll_owner: WorthUiScrollOwner,
    clip_posture: WorthUiClipPosture,
    viewport_basis: WorthUiViewportBasis,
    hit_policy: WorthUiViewportParticipationPolicy,
    focus_policy: WorthUiViewportParticipationPolicy,
    accessibility_policy: WorthUiViewportParticipationPolicy,
    measurement_policy: WorthUiViewportParticipationPolicy,
    restoration_policy: WorthUiScrollRestorationPolicy,
}

impl WorthUiViewportBoundaryPolicyReceipt {
    pub(super) fn admit(policy_identity: &str) -> Option<Self> {
        match policy_identity {
            "validation.viewport.local.card_scroll" => Some(Self::composition_scroll(
                policy_identity,
                WorthUiClipPosture::ClipToViewport,
                WorthUiViewportParticipationPolicy::VisibleDescendantsOnly,
            )),
            "validation.viewport.local.clip" => Some(Self {
                policy_identity: policy_identity.to_owned(),
                scroll_owner: WorthUiScrollOwner::None,
                clip_posture: WorthUiClipPosture::ClipToViewport,
                viewport_basis: WorthUiViewportBasis::AllocatedFrame,
                hit_policy: WorthUiViewportParticipationPolicy::VisibleDescendantsOnly,
                focus_policy: WorthUiViewportParticipationPolicy::VisibleDescendantsOnly,
                accessibility_policy: WorthUiViewportParticipationPolicy::VisibleDescendantsOnly,
                measurement_policy: WorthUiViewportParticipationPolicy::VisibleDescendantsOnly,
                restoration_policy: WorthUiScrollRestorationPolicy::None,
            }),
            "validation.viewport.local.unclipped" => Some(Self {
                policy_identity: policy_identity.to_owned(),
                scroll_owner: WorthUiScrollOwner::None,
                clip_posture: WorthUiClipPosture::None,
                viewport_basis: WorthUiViewportBasis::AllocatedFrame,
                hit_policy: WorthUiViewportParticipationPolicy::AllDescendants,
                focus_policy: WorthUiViewportParticipationPolicy::AllDescendants,
                accessibility_policy: WorthUiViewportParticipationPolicy::AllDescendants,
                measurement_policy: WorthUiViewportParticipationPolicy::AllDescendants,
                restoration_policy: WorthUiScrollRestorationPolicy::None,
            }),
            _ => None,
        }
    }

    fn composition_scroll(
        policy_identity: &str,
        clip_posture: WorthUiClipPosture,
        participation: WorthUiViewportParticipationPolicy,
    ) -> Self {
        Self {
            policy_identity: policy_identity.to_owned(),
            scroll_owner: WorthUiScrollOwner::Composition,
            clip_posture,
            viewport_basis: WorthUiViewportBasis::AllocatedFrame,
            hit_policy: participation,
            focus_policy: participation,
            accessibility_policy: participation,
            measurement_policy: participation,
            restoration_policy: WorthUiScrollRestorationPolicy::ByCompositionIdentity,
        }
    }

    pub fn policy_identity(&self) -> &str {
        &self.policy_identity
    }

    pub fn scroll_owner(&self) -> WorthUiScrollOwner {
        self.scroll_owner
    }

    pub fn clip_posture(&self) -> WorthUiClipPosture {
        self.clip_posture
    }

    pub fn viewport_basis(&self) -> WorthUiViewportBasis {
        self.viewport_basis
    }

    pub fn hit_policy(&self) -> WorthUiViewportParticipationPolicy {
        self.hit_policy
    }

    pub fn focus_policy(&self) -> WorthUiViewportParticipationPolicy {
        self.focus_policy
    }

    pub fn accessibility_policy(&self) -> WorthUiViewportParticipationPolicy {
        self.accessibility_policy
    }

    pub fn measurement_policy(&self) -> WorthUiViewportParticipationPolicy {
        self.measurement_policy
    }

    pub fn restoration_policy(&self) -> WorthUiScrollRestorationPolicy {
        self.restoration_policy
    }
}

impl WorthUiScrollOwner {
    pub const fn token(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Composition => "composition",
        }
    }
}

impl WorthUiClipPosture {
    pub const fn token(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::ClipToViewport => "clip_to_viewport",
        }
    }
}

impl WorthUiViewportParticipationPolicy {
    pub const fn token(self) -> &'static str {
        match self {
            Self::AllDescendants => "all_descendants",
            Self::VisibleDescendantsOnly => "visible_descendants_only",
        }
    }
}
