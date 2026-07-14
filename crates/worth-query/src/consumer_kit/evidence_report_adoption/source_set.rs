#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryEvidenceReportAdoptionResidueClassification {
    CoveredQueryEvidenceAdoption,
    DefendedDomainArtifactIdentity,
    Unclassified,
}

impl WorthQueryEvidenceReportAdoptionResidueClassification {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CoveredQueryEvidenceAdoption => "covered-query-evidence-adoption",
            Self::DefendedDomainArtifactIdentity => "defended-domain-artifact-identity",
            Self::Unclassified => "unclassified",
        }
    }

    pub(crate) fn permits_digest_residue(self) -> bool {
        matches!(self, Self::DefendedDomainArtifactIdentity)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryEvidenceReportAdoptionSource {
    label: String,
    path: Option<String>,
    source: String,
    classification: WorthQueryEvidenceReportAdoptionResidueClassification,
}

impl WorthQueryEvidenceReportAdoptionSource {
    fn new(
        label: impl Into<String>,
        path: Option<String>,
        source: impl Into<String>,
        classification: WorthQueryEvidenceReportAdoptionResidueClassification,
    ) -> Self {
        Self {
            label: label.into(),
            path,
            source: source.into(),
            classification,
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn classification(&self) -> WorthQueryEvidenceReportAdoptionResidueClassification {
        self.classification
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryEvidenceReportAdoptionSourceSet {
    crate_name: String,
    sources: Vec<WorthQueryEvidenceReportAdoptionSource>,
}

impl WorthQueryEvidenceReportAdoptionSourceSet {
    pub fn new(crate_name: impl Into<String>) -> Self {
        Self {
            crate_name: crate_name.into(),
            sources: Vec::new(),
        }
    }

    pub fn source_file(
        mut self,
        label: impl Into<String>,
        path: impl Into<String>,
        source: impl Into<String>,
        classification: WorthQueryEvidenceReportAdoptionResidueClassification,
    ) -> Self {
        self.sources
            .push(WorthQueryEvidenceReportAdoptionSource::new(
                label,
                Some(path.into()),
                source,
                classification,
            ));
        self
    }

    pub fn crate_name(&self) -> &str {
        &self.crate_name
    }

    pub fn sources(&self) -> &[WorthQueryEvidenceReportAdoptionSource] {
        &self.sources
    }

    pub(crate) fn validate(&self) -> Result<(), WorthQueryEvidenceReportAdoptionError> {
        if self.crate_name.trim().is_empty() {
            return Err(WorthQueryEvidenceReportAdoptionError::new(
                WorthQueryEvidenceReportAdoptionErrorKind::EmptyCrateName,
                "evidence report adoption audit crate name must not be empty",
            ));
        }
        let mut labels = std::collections::BTreeSet::new();
        for source in &self.sources {
            validate_source_shape(source)?;
            if !labels.insert(source.label()) {
                return Err(WorthQueryEvidenceReportAdoptionError::for_source(
                    WorthQueryEvidenceReportAdoptionErrorKind::DuplicateSourceLabel,
                    source.label(),
                    format!(
                        "duplicate evidence report adoption audit source label `{}`",
                        source.label()
                    ),
                ));
            }
        }
        Ok(())
    }
}

fn validate_source_shape(
    source: &WorthQueryEvidenceReportAdoptionSource,
) -> Result<(), WorthQueryEvidenceReportAdoptionError> {
    if source.label().trim().is_empty() {
        return Err(WorthQueryEvidenceReportAdoptionError::new(
            WorthQueryEvidenceReportAdoptionErrorKind::EmptySourceLabel,
            "evidence report adoption audit source label must not be empty",
        ));
    }
    if source.path().is_some_and(|path| path.trim().is_empty()) {
        return Err(WorthQueryEvidenceReportAdoptionError::for_source(
            WorthQueryEvidenceReportAdoptionErrorKind::EmptySourcePath,
            source.label(),
            format!(
                "evidence report adoption audit source `{}` path must not be empty",
                source.label()
            ),
        ));
    }
    if source.source().trim().is_empty() {
        return Err(WorthQueryEvidenceReportAdoptionError::for_source(
            WorthQueryEvidenceReportAdoptionErrorKind::EmptySourceText,
            source.label(),
            format!(
                "evidence report adoption audit source `{}` must not be empty",
                source.label()
            ),
        ));
    }
    Ok(())
}
use super::error::{
    WorthQueryEvidenceReportAdoptionError, WorthQueryEvidenceReportAdoptionErrorKind,
};
