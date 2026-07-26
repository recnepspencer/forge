use super::{
    WorthQueryConvergenceAssessment, WorthQueryConvergenceDomainAssessmentOutcome,
    WorthQueryConvergenceDomainFailure, WorthQueryConvergenceProviderFamilies,
};

pub trait WorthQueryConvergenceDomainProvider: Send + Sync + 'static {
    fn convergence_families(&self) -> &WorthQueryConvergenceProviderFamilies;

    fn assess(
        &self,
        assessment: WorthQueryConvergenceAssessment<'_>,
    ) -> Result<WorthQueryConvergenceDomainAssessmentOutcome, WorthQueryConvergenceDomainFailure>;
}
