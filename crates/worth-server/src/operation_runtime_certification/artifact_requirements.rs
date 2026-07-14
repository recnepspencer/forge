#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerProductOperationRuntimeRequirementStatus {
    Ready,
    Blocked,
}

impl WorthServerProductOperationRuntimeRequirementStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerProductOperationRuntimeRequirementRow {
    artifact_name: String,
    status: WorthServerProductOperationRuntimeRequirementStatus,
    digest: String,
    detail: String,
}

impl WorthServerProductOperationRuntimeRequirementRow {
    pub(crate) fn new(
        artifact_name: impl Into<String>,
        status: WorthServerProductOperationRuntimeRequirementStatus,
        digest: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            artifact_name: artifact_name.into(),
            status,
            digest: digest.into(),
            detail: detail.into(),
        }
    }

    pub fn artifact_name(&self) -> &str {
        &self.artifact_name
    }

    pub fn status(&self) -> WorthServerProductOperationRuntimeRequirementStatus {
        self.status
    }

    pub fn status_label(&self) -> &'static str {
        self.status.as_str()
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerProductOperationRuntimeArtifactRequirements {
    rows: Vec<WorthServerProductOperationRuntimeRequirementRow>,
    canonical_digest: String,
}

impl WorthServerProductOperationRuntimeArtifactRequirements {
    pub(crate) fn new(rows: Vec<WorthServerProductOperationRuntimeRequirementRow>) -> Self {
        let canonical_digest = rows
            .iter()
            .map(|row| {
                format!(
                    "{}:{}:{}:{}",
                    row.artifact_name(),
                    row.status_label(),
                    row.digest(),
                    row.detail()
                )
            })
            .collect::<Vec<_>>()
            .join("|");
        Self {
            rows,
            canonical_digest,
        }
    }

    pub fn rows(&self) -> &[WorthServerProductOperationRuntimeRequirementRow] {
        &self.rows
    }

    pub fn is_ready(&self) -> bool {
        self.rows
            .iter()
            .all(|row| row.status() == WorthServerProductOperationRuntimeRequirementStatus::Ready)
    }

    pub fn blocking_artifact_names(&self) -> Vec<&str> {
        self.rows
            .iter()
            .filter(|row| {
                row.status() == WorthServerProductOperationRuntimeRequirementStatus::Blocked
            })
            .map(WorthServerProductOperationRuntimeRequirementRow::artifact_name)
            .collect()
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
