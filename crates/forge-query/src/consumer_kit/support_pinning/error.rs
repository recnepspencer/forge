#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQuerySupportPinningErrorKind {
    BlankConsumerName,
    SnapshotRowMissing,
    DuplicateSnapshotFamily,
    DuplicateRequiredFamily,
    DuplicateObservedFamily,
    RequiredObservedFamilyConflict,
    MissingRequiredStatus,
    MissingRequiredTeachingPosture,
    MissingLiveRowDigestBinding,
    JsonDecodeFailed,
    JsonEncodeFailed,
    SchemaMismatch,
    ContractDigestMismatch,
    VocabularyMismatch,
    InvalidFacadeFamily,
    InvalidPinnedStatus,
    InvalidPinnedTeachingPosture,
    BlockingFindings,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQuerySupportPinningError {
    kind: ForgeQuerySupportPinningErrorKind,
    message: String,
    family: Option<String>,
    expected: Option<String>,
    found: Option<String>,
    consumer_name: Option<String>,
    report_digest: Option<String>,
    blocking_findings: Vec<super::evaluation::ForgeQuerySupportPinFinding>,
}

impl ForgeQuerySupportPinningError {
    pub(crate) fn new(kind: ForgeQuerySupportPinningErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            family: None,
            expected: None,
            found: None,
            consumer_name: None,
            report_digest: None,
            blocking_findings: Vec::new(),
        }
    }

    pub(crate) fn with_family(
        kind: ForgeQuerySupportPinningErrorKind,
        message: impl Into<String>,
        family: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            family: Some(family.into()),
            expected: None,
            found: None,
            consumer_name: None,
            report_digest: None,
            blocking_findings: Vec::new(),
        }
    }

    pub(crate) fn with_found(
        kind: ForgeQuerySupportPinningErrorKind,
        message: impl Into<String>,
        found: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            family: None,
            expected: None,
            found: Some(found.into()),
            consumer_name: None,
            report_digest: None,
            blocking_findings: Vec::new(),
        }
    }

    pub(crate) fn with_expected_found(
        kind: ForgeQuerySupportPinningErrorKind,
        message: impl Into<String>,
        expected: impl Into<String>,
        found: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            family: None,
            expected: Some(expected.into()),
            found: Some(found.into()),
            consumer_name: None,
            report_digest: None,
            blocking_findings: Vec::new(),
        }
    }

    pub(crate) fn with_blocking_findings(
        consumer_name: impl Into<String>,
        report_digest: impl Into<String>,
        blocking_findings: Vec<super::evaluation::ForgeQuerySupportPinFinding>,
    ) -> Self {
        let consumer_name = consumer_name.into();
        Self {
            kind: ForgeQuerySupportPinningErrorKind::BlockingFindings,
            message: format!(
                "support pin report for {consumer_name} has {} blocking finding(s)",
                blocking_findings.len()
            ),
            family: None,
            expected: None,
            found: None,
            consumer_name: Some(consumer_name),
            report_digest: Some(report_digest.into()),
            blocking_findings,
        }
    }

    pub fn kind(&self) -> ForgeQuerySupportPinningErrorKind {
        self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn family(&self) -> Option<&str> {
        self.family.as_deref()
    }

    pub fn expected(&self) -> Option<&str> {
        self.expected.as_deref()
    }

    pub fn found(&self) -> Option<&str> {
        self.found.as_deref()
    }

    pub fn consumer_name(&self) -> Option<&str> {
        self.consumer_name.as_deref()
    }

    pub fn report_digest(&self) -> Option<&str> {
        self.report_digest.as_deref()
    }

    pub fn blocking_findings(&self) -> &[super::evaluation::ForgeQuerySupportPinFinding] {
        &self.blocking_findings
    }
}

impl std::fmt::Display for ForgeQuerySupportPinningError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for ForgeQuerySupportPinningError {}
