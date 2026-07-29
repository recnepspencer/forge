#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiRebindPlanBasis {
    classification: crate::runtime::observation::UiChangeClassificationBasis,
    candidate_generation:
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity,
}

impl UiRebindPlanBasis {
    pub(crate) const fn new(
        classification: crate::runtime::observation::UiChangeClassificationBasis,
        candidate_generation: crate::facade::prepared_application_authority::
            WorthUiPreparedApplicationGenerationIdentity,
    ) -> Self {
        Self {
            classification,
            candidate_generation,
        }
    }

    pub const fn classification(
        &self,
    ) -> &crate::runtime::observation::UiChangeClassificationBasis {
        &self.classification
    }

    pub fn candidate_generation(
        &self,
    ) -> &crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity
    {
        &self.candidate_generation
    }
}
