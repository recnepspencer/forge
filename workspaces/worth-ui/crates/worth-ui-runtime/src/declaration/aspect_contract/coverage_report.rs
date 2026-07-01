use crate::declaration::{UiAspectContract, UiAspectFamily, UiAspectName, UiAspectSemanticSlice};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAspectCoverageReport {
    published: Box<[UiAspectCoverageEntry]>,
    consumed: Box<[UiAspectCoverageEntry]>,
}

impl UiAspectCoverageReport {
    pub(crate) fn new(
        published: &[UiAspectCoverageEntry],
        consumed: &[UiAspectCoverageEntry],
    ) -> Self {
        Self {
            published: published.to_vec().into_boxed_slice(),
            consumed: consumed.to_vec().into_boxed_slice(),
        }
    }

    pub fn published(&self) -> &[UiAspectCoverageEntry] {
        &self.published
    }

    pub fn consumed(&self) -> &[UiAspectCoverageEntry] {
        &self.consumed
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAspectCoverageEntry {
    aspect: UiAspectName,
}

impl UiAspectCoverageEntry {
    pub(crate) fn from_contract(aspect: &UiAspectName) -> Self {
        Self {
            aspect: aspect.clone(),
        }
    }

    pub fn family(&self) -> UiAspectFamily {
        self.aspect.family()
    }

    pub fn semantic_slice(&self) -> UiAspectSemanticSlice {
        self.aspect.semantic_slice()
    }

    pub fn canonical_label(&self) -> &str {
        self.aspect.canonical_label()
    }
}

impl UiAspectContract {
    pub fn coverage_report(&self) -> UiAspectCoverageReport {
        UiAspectCoverageReport::new(
            &self
                .published()
                .aspects()
                .iter()
                .map(UiAspectCoverageEntry::from_contract)
                .collect::<Vec<_>>(),
            &self
                .consumed()
                .aspects()
                .iter()
                .map(UiAspectCoverageEntry::from_contract)
                .collect::<Vec<_>>(),
        )
    }
}
