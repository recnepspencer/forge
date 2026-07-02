use crate::{
    UiInspectionEvidenceSource, UiInspectionObligationDenialPosture,
    UiInspectionObligationFamily,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiInspectionObligationEvidenceQuery {
    family: Option<UiInspectionObligationFamily>,
    denial_posture: Option<UiInspectionObligationDenialPosture>,
    prerequisite_source: Option<UiInspectionEvidenceSource>,
}

impl UiInspectionObligationEvidenceQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_family(mut self, family: UiInspectionObligationFamily) -> Self {
        self.family = Some(family);
        self
    }

    pub fn with_denial_posture(
        mut self,
        denial_posture: UiInspectionObligationDenialPosture,
    ) -> Self {
        self.denial_posture = Some(denial_posture);
        self
    }

    pub fn with_prerequisite_source(
        mut self,
        prerequisite_source: UiInspectionEvidenceSource,
    ) -> Self {
        self.prerequisite_source = Some(prerequisite_source);
        self
    }

    pub fn family(&self) -> Option<UiInspectionObligationFamily> {
        self.family
    }

    pub fn denial_posture(&self) -> Option<UiInspectionObligationDenialPosture> {
        self.denial_posture
    }

    pub fn prerequisite_source(&self) -> Option<UiInspectionEvidenceSource> {
        self.prerequisite_source
    }
}
