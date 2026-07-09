use worth_foundational::facade::DiagnosticRichnessProfile;

use crate::operator_evidence::WorthServerEvidenceTransform;

#[derive(Clone, Debug)]
pub struct WorthServerOperatorEvidenceConfig {
    default_transform: WorthServerEvidenceTransform,
    minimum_diagnostics_profile: DiagnosticRichnessProfile,
}

impl WorthServerOperatorEvidenceConfig {
    pub fn builder() -> WorthServerOperatorEvidenceConfigBuilder {
        WorthServerOperatorEvidenceConfigBuilder::default()
    }

    pub fn default_transform(&self) -> WorthServerEvidenceTransform {
        self.default_transform
    }

    pub fn minimum_diagnostics_profile(&self) -> DiagnosticRichnessProfile {
        self.minimum_diagnostics_profile
    }
}

#[derive(Clone, Debug)]
pub struct WorthServerOperatorEvidenceConfigBuilder {
    default_transform: WorthServerEvidenceTransform,
    minimum_diagnostics_profile: DiagnosticRichnessProfile,
}

impl Default for WorthServerOperatorEvidenceConfigBuilder {
    fn default() -> Self {
        Self {
            default_transform: WorthServerEvidenceTransform::OperatorDefault,
            minimum_diagnostics_profile: DiagnosticRichnessProfile::OperationalMinimal,
        }
    }
}

impl WorthServerOperatorEvidenceConfigBuilder {
    pub fn with_default_transform(mut self, transform: WorthServerEvidenceTransform) -> Self {
        self.default_transform = transform;
        self
    }

    pub fn with_minimum_diagnostics_profile(mut self, profile: DiagnosticRichnessProfile) -> Self {
        self.minimum_diagnostics_profile = profile;
        self
    }

    pub fn build(
        self,
    ) -> Result<WorthServerOperatorEvidenceConfig, WorthServerOperatorEvidenceConfigError> {
        Ok(WorthServerOperatorEvidenceConfig {
            default_transform: self.default_transform,
            minimum_diagnostics_profile: self.minimum_diagnostics_profile,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerOperatorEvidenceConfigError {}
