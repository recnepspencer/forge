#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiInspectionAspectRelevanceDetail {
    include_direct_provenance_refs: bool,
}

impl UiInspectionAspectRelevanceDetail {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn include_direct_provenance_refs(mut self) -> Self {
        self.include_direct_provenance_refs = true;
        self
    }

    pub fn includes_direct_provenance_refs(self) -> bool {
        self.include_direct_provenance_refs
    }
}
