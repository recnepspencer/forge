#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct UiGraphReplanConsequences {
    scroll_owned: Box<[super::UiScrollReplanConsequence]>,
    portal_anchors: Box<[super::UiPortalReplanConsequence]>,
}

impl UiGraphReplanConsequences {
    pub(crate) fn seal(
        mut scroll_owned: Vec<super::UiScrollReplanConsequence>,
        mut portal_anchors: Vec<super::UiPortalReplanConsequence>,
    ) -> Self {
        scroll_owned.sort_by_key(super::UiScrollReplanConsequence::identity_digest);
        portal_anchors.sort_by_key(super::UiPortalReplanConsequence::identity_digest);
        Self {
            scroll_owned: scroll_owned.into_boxed_slice(),
            portal_anchors: portal_anchors.into_boxed_slice(),
        }
    }

    pub(crate) fn portal_anchors(&self) -> &[super::UiPortalReplanConsequence] {
        &self.portal_anchors
    }

    pub(crate) fn scroll_owned(&self) -> &[super::UiScrollReplanConsequence] {
        &self.scroll_owned
    }

    pub(crate) fn identity_digest(&self) -> u64 {
        let digest = self.scroll_owned.iter().fold(
            crate::declaration::stable_text_digest("worth-ui.graph-replan-consequences"),
            |digest, consequence| {
                digest.rotate_left(7) ^ consequence.identity_digest().rotate_left(29)
            },
        );
        self.portal_anchors
            .iter()
            .fold(digest, |digest, consequence| {
                digest.rotate_left(11) ^ consequence.identity_digest().rotate_left(37)
            })
    }
}
