#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryEvidenceReportAdoptionResidueClassification {
    CoveredQueryEvidenceAdoption,
    DefendedDomainArtifactIdentity,
    Unclassified,
}

impl ForgeQueryEvidenceReportAdoptionResidueClassification {
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
pub struct ForgeQueryEvidenceReportAdoptionSource {
    label: String,
    path: Option<String>,
    source: String,
    classification: ForgeQueryEvidenceReportAdoptionResidueClassification,
}

impl ForgeQueryEvidenceReportAdoptionSource {
    fn new(
        label: impl Into<String>,
        path: Option<String>,
        source: impl Into<String>,
        classification: ForgeQueryEvidenceReportAdoptionResidueClassification,
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

    pub fn classification(&self) -> ForgeQueryEvidenceReportAdoptionResidueClassification {
        self.classification
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryEvidenceReportAdoptionSourceSet {
    crate_name: String,
    sources: Vec<ForgeQueryEvidenceReportAdoptionSource>,
}

impl ForgeQueryEvidenceReportAdoptionSourceSet {
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
        classification: ForgeQueryEvidenceReportAdoptionResidueClassification,
    ) -> Self {
        self.sources
            .push(ForgeQueryEvidenceReportAdoptionSource::new(
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

    pub fn sources(&self) -> &[ForgeQueryEvidenceReportAdoptionSource] {
        &self.sources
    }

    pub(crate) fn validate(&self) -> Result<(), ForgeQueryEvidenceReportAdoptionError> {
        if self.crate_name.trim().is_empty() {
            return Err(ForgeQueryEvidenceReportAdoptionError::new(
                ForgeQueryEvidenceReportAdoptionErrorKind::EmptyCrateName,
                "evidence report adoption audit crate name must not be empty",
            ));
        }
        let mut labels = std::collections::BTreeSet::new();
        for source in &self.sources {
            validate_source_shape(source)?;
            if !labels.insert(source.label()) {
                return Err(ForgeQueryEvidenceReportAdoptionError::for_source(
                    ForgeQueryEvidenceReportAdoptionErrorKind::DuplicateSourceLabel,
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
    source: &ForgeQueryEvidenceReportAdoptionSource,
) -> Result<(), ForgeQueryEvidenceReportAdoptionError> {
    if source.label().trim().is_empty() {
        return Err(ForgeQueryEvidenceReportAdoptionError::new(
            ForgeQueryEvidenceReportAdoptionErrorKind::EmptySourceLabel,
            "evidence report adoption audit source label must not be empty",
        ));
    }
    if source.path().is_some_and(|path| path.trim().is_empty()) {
        return Err(ForgeQueryEvidenceReportAdoptionError::for_source(
            ForgeQueryEvidenceReportAdoptionErrorKind::EmptySourcePath,
            source.label(),
            format!(
                "evidence report adoption audit source `{}` path must not be empty",
                source.label()
            ),
        ));
    }
    if source.source().trim().is_empty() {
        return Err(ForgeQueryEvidenceReportAdoptionError::for_source(
            ForgeQueryEvidenceReportAdoptionErrorKind::EmptySourceText,
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
    ForgeQueryEvidenceReportAdoptionError, ForgeQueryEvidenceReportAdoptionErrorKind,
};
