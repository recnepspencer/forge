#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum UiRelevanceFamily {
    Declaration,
    Admission,
    Graph,
    Aspect,
    Obligation,
    Planning,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum UiEvidenceLinkKind {
    DerivedFrom,
    Summarizes,
    Explains,
    CausedBy,
    SelectedBy,
    InvalidatedBy,
    AttachedTo,
    CorrespondsTo,
    BlockedBy,
    CitesForeignEvidence,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiRelevanceFilter {
    family: Option<UiRelevanceFamily>,
    cross_family: Option<UiRelevanceFamily>,
    link_kind: Option<UiEvidenceLinkKind>,
}

impl UiRelevanceFilter {
    pub fn target_local() -> Self {
        Self::default()
    }

    pub fn family(family: UiRelevanceFamily) -> Self {
        Self {
            family: Some(family),
            ..Self::default()
        }
    }

    pub fn include_family(mut self, family: UiRelevanceFamily) -> Self {
        self.cross_family = Some(family);
        self
    }

    pub fn include_link(mut self, link_kind: UiEvidenceLinkKind) -> Self {
        self.link_kind = Some(link_kind);
        self
    }

    pub fn family_filter(self) -> Option<UiRelevanceFamily> {
        self.family
    }

    pub fn cross_family(self) -> Option<UiRelevanceFamily> {
        self.cross_family
    }

    pub fn link_kind(self) -> Option<UiEvidenceLinkKind> {
        self.link_kind
    }

    pub fn widens_beyond_local(self) -> bool {
        self.cross_family.is_some() || self.link_kind.is_some()
    }
}
