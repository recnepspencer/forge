use crate::declaration::stable_text_digest;

use crate::evidence::{UiConstraintAxisScope, UiConstraintSiblingNegotiationMode};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiConstraintSiblingNegotiationGroup {
    neighborhood_identity_digest: u64,
    mode: UiConstraintSiblingNegotiationMode,
    axis_scope: UiConstraintAxisScope,
    member_identity_digests: Box<[u64]>,
    identity_digest: u64,
}

impl UiConstraintSiblingNegotiationGroup {
    pub(crate) fn new(
        neighborhood_identity_digest: u64,
        mode: UiConstraintSiblingNegotiationMode,
        axis_scope: UiConstraintAxisScope,
        mut member_identity_digests: Vec<u64>,
    ) -> Self {
        member_identity_digests.sort_unstable();
        let identity_digest = member_identity_digests.iter().fold(
            stable_text_digest("worth-ui.constraint-sibling-negotiation-group")
                ^ neighborhood_identity_digest.rotate_left(7)
                ^ sibling_mode_digest(mode).rotate_left(13)
                ^ axis_scope_digest(axis_scope).rotate_left(19),
            |digest, member_identity_digest| digest.rotate_left(11) ^ member_identity_digest,
        );
        Self {
            neighborhood_identity_digest,
            mode,
            axis_scope,
            member_identity_digests: member_identity_digests.into_boxed_slice(),
            identity_digest,
        }
    }

    pub fn neighborhood_identity_digest(&self) -> u64 {
        self.neighborhood_identity_digest
    }

    pub fn mode(&self) -> UiConstraintSiblingNegotiationMode {
        self.mode
    }

    pub fn axis_scope(&self) -> UiConstraintAxisScope {
        self.axis_scope
    }

    pub fn member_identity_digests(&self) -> &[u64] {
        &self.member_identity_digests
    }

    pub fn identity_digest(&self) -> u64 {
        self.identity_digest
    }
}

fn axis_scope_digest(axis_scope: UiConstraintAxisScope) -> u64 {
    match axis_scope {
        UiConstraintAxisScope::Primary => stable_text_digest("worth-ui.constraint-axis.primary"),
        UiConstraintAxisScope::Cross => stable_text_digest("worth-ui.constraint-axis.cross"),
        UiConstraintAxisScope::Both => stable_text_digest("worth-ui.constraint-axis.both"),
    }
}

fn sibling_mode_digest(mode: UiConstraintSiblingNegotiationMode) -> u64 {
    match mode {
        UiConstraintSiblingNegotiationMode::None => {
            stable_text_digest("worth-ui.constraint-sibling.none")
        }
        UiConstraintSiblingNegotiationMode::StablePeerPrimaryAxis => {
            stable_text_digest("worth-ui.constraint-sibling.primary-axis")
        }
        UiConstraintSiblingNegotiationMode::StablePeerTwoDimensional => {
            stable_text_digest("worth-ui.constraint-sibling.two-dimensional")
        }
    }
}
