use super::error::{WorthQueryBoundaryAuditError, WorthQueryBoundaryAuditErrorKind};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBoundaryAuditSource {
    label: String,
    path: Option<String>,
    source: String,
}

impl WorthQueryBoundaryAuditSource {
    fn new(label: impl Into<String>, source: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            path: None,
            source: source.into(),
        }
    }

    fn new_file(
        label: impl Into<String>,
        path: impl Into<String>,
        source: impl Into<String>,
    ) -> Self {
        Self {
            label: label.into(),
            path: Some(path.into()),
            source: source.into(),
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBoundaryAuditSourceSet {
    crate_name: String,
    sources: Vec<WorthQueryBoundaryAuditSource>,
}

impl WorthQueryBoundaryAuditSourceSet {
    pub fn new(crate_name: impl Into<String>) -> Self {
        Self {
            crate_name: crate_name.into(),
            sources: Vec::new(),
        }
    }

    pub fn source(mut self, label: impl Into<String>, source: impl Into<String>) -> Self {
        self.sources
            .push(WorthQueryBoundaryAuditSource::new(label, source));
        self
    }

    pub fn source_file(
        mut self,
        label: impl Into<String>,
        path: impl Into<String>,
        source: impl Into<String>,
    ) -> Self {
        self.sources
            .push(WorthQueryBoundaryAuditSource::new_file(label, path, source));
        self
    }

    pub fn crate_name(&self) -> &str {
        &self.crate_name
    }

    pub fn sources(&self) -> &[WorthQueryBoundaryAuditSource] {
        &self.sources
    }

    pub fn source_labels(&self) -> Vec<&str> {
        self.sources.iter().map(|source| source.label()).collect()
    }

    pub(crate) fn validate(&self) -> Result<(), WorthQueryBoundaryAuditError> {
        if self.crate_name.trim().is_empty() {
            return Err(WorthQueryBoundaryAuditError::new(
                WorthQueryBoundaryAuditErrorKind::EmptyCrateName,
                "boundary audit crate name must not be empty",
            ));
        }
        let mut labels = std::collections::BTreeSet::new();
        for source in &self.sources {
            validate_source_shape(source)?;
            if !labels.insert(source.label()) {
                return Err(WorthQueryBoundaryAuditError::for_source(
                    WorthQueryBoundaryAuditErrorKind::DuplicateSourceLabel,
                    source.label(),
                    format!("duplicate boundary audit source label `{}`", source.label()),
                ));
            }
        }
        Ok(())
    }
}

fn validate_source_shape(
    source: &WorthQueryBoundaryAuditSource,
) -> Result<(), WorthQueryBoundaryAuditError> {
    if source.label().trim().is_empty() {
        return Err(WorthQueryBoundaryAuditError::new(
            WorthQueryBoundaryAuditErrorKind::EmptySourceLabel,
            "boundary audit source label must not be empty",
        ));
    }
    if source.path().is_some_and(|path| path.trim().is_empty()) {
        return Err(WorthQueryBoundaryAuditError::for_source(
            WorthQueryBoundaryAuditErrorKind::EmptySourcePath,
            source.label(),
            format!(
                "boundary audit source `{}` path must not be empty",
                source.label()
            ),
        ));
    }
    if source.source().trim().is_empty() {
        return Err(WorthQueryBoundaryAuditError::for_source(
            WorthQueryBoundaryAuditErrorKind::EmptySourceText,
            source.label(),
            format!(
                "boundary audit source `{}` must not be empty",
                source.label()
            ),
        ));
    }
    Ok(())
}
