use forge_foundational::facade::DiagnosticRichnessProfile;

use crate::operator_evidence::ForgeServerEvidenceTransform;

#[derive(Clone, Debug)]
pub struct ForgeServerOperatorEvidenceConfig {
    default_transform: ForgeServerEvidenceTransform,
    minimum_diagnostics_profile: DiagnosticRichnessProfile,
}

impl ForgeServerOperatorEvidenceConfig {
    pub fn builder() -> ForgeServerOperatorEvidenceConfigBuilder {
        ForgeServerOperatorEvidenceConfigBuilder::default()
    }

    pub fn default_transform(&self) -> ForgeServerEvidenceTransform {
        self.default_transform
    }

    pub fn minimum_diagnostics_profile(&self) -> DiagnosticRichnessProfile {
        self.minimum_diagnostics_profile
    }
}

#[derive(Clone, Debug)]
pub struct ForgeServerOperatorEvidenceConfigBuilder {
    default_transform: ForgeServerEvidenceTransform,
    minimum_diagnostics_profile: DiagnosticRichnessProfile,
}

impl Default for ForgeServerOperatorEvidenceConfigBuilder {
    fn default() -> Self {
        Self {
            default_transform: ForgeServerEvidenceTransform::OperatorDefault,
            minimum_diagnostics_profile: DiagnosticRichnessProfile::OperationalMinimal,
        }
    }
}

impl ForgeServerOperatorEvidenceConfigBuilder {
    pub fn with_default_transform(mut self, transform: ForgeServerEvidenceTransform) -> Self {
        self.default_transform = transform;
        self
    }

    pub fn with_minimum_diagnostics_profile(mut self, profile: DiagnosticRichnessProfile) -> Self {
        self.minimum_diagnostics_profile = profile;
        self
    }

    pub fn build(
        self,
    ) -> Result<ForgeServerOperatorEvidenceConfig, ForgeServerOperatorEvidenceConfigError> {
        Ok(ForgeServerOperatorEvidenceConfig {
            default_transform: self.default_transform,
            minimum_diagnostics_profile: self.minimum_diagnostics_profile,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerOperatorEvidenceConfigError {}
