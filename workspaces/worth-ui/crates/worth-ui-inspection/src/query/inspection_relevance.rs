use crate::{
    UiInspectionAspectRelevanceDetail, UiInspectionObligationRelevanceDetail, UiRelevanceFilter,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiInspectionRelevance {
    filter: UiRelevanceFilter,
    aspect_detail: Option<UiInspectionAspectRelevanceDetail>,
    obligation_detail: Option<UiInspectionObligationRelevanceDetail>,
}

impl UiInspectionRelevance {
    pub fn local(filter: UiRelevanceFilter) -> Self {
        Self {
            filter,
            aspect_detail: None,
            obligation_detail: None,
        }
    }

    pub fn filter(self) -> UiRelevanceFilter {
        self.filter
    }

    pub fn with_obligation_detail(
        mut self,
        obligation_detail: UiInspectionObligationRelevanceDetail,
    ) -> Self {
        self.obligation_detail = Some(obligation_detail);
        self
    }

    pub fn with_aspect_detail(mut self, aspect_detail: UiInspectionAspectRelevanceDetail) -> Self {
        self.aspect_detail = Some(aspect_detail);
        self
    }

    pub fn aspect_detail(self) -> Option<UiInspectionAspectRelevanceDetail> {
        self.aspect_detail
    }

    pub fn obligation_detail(self) -> Option<UiInspectionObligationRelevanceDetail> {
        self.obligation_detail
    }
}

impl Default for UiInspectionRelevance {
    fn default() -> Self {
        Self::local(UiRelevanceFilter::target_local())
    }
}
