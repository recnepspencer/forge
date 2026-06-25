use super::super::digest::digest_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionContextReport {
    denials: Vec<WorthUiCompositionContextDenial>,
    presentation_rows: Vec<WorthUiCompositionContextDenialPresentationRow>,
    report_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionContextDenial {
    code: WorthUiCompositionContextDenialCode,
    subject: String,
    context_kind: String,
    inherited_value: Option<String>,
    attempted_value: Option<String>,
    expected_policy: String,
    affected_descendants: Vec<String>,
    source_span_ready: bool,
    message: String,
    denial_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionContextDenialPresentationRow {
    label: &'static str,
    value: String,
    row_digest: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiCompositionContextDenialCode {
    MissingScopeNode,
    DuplicateContextKind,
    OverrideWithoutEligibility,
}

impl WorthUiCompositionContextReport {
    pub(crate) fn denied(mut denials: Vec<WorthUiCompositionContextDenial>) -> Self {
        denials.sort_by(|left, right| {
            left.subject()
                .cmp(right.subject())
                .then_with(|| format!("{:?}", left.code()).cmp(&format!("{:?}", right.code())))
        });
        let presentation_rows = denials
            .iter()
            .flat_map(WorthUiCompositionContextDenial::presentation_rows)
            .collect::<Vec<_>>();
        let report_digest = digest_parts(denials.iter().map(|denial| {
            format!(
                "{:?}:{}:{}:{}",
                denial.code(),
                denial.subject(),
                denial.context_kind(),
                denial.message()
            )
        }));
        Self {
            denials,
            presentation_rows,
            report_digest,
        }
    }

    pub fn denials(&self) -> &[WorthUiCompositionContextDenial] {
        &self.denials
    }

    pub fn presentation_rows(&self) -> &[WorthUiCompositionContextDenialPresentationRow] {
        &self.presentation_rows
    }

    pub fn report_digest(&self) -> u64 {
        self.report_digest
    }
}

impl WorthUiCompositionContextDenial {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn detailed(
        code: WorthUiCompositionContextDenialCode,
        subject: impl Into<String>,
        context_kind: impl Into<String>,
        inherited_value: Option<String>,
        attempted_value: Option<String>,
        expected_policy: impl Into<String>,
        affected_descendants: Vec<String>,
        source_span_ready: bool,
        message: impl Into<String>,
    ) -> Self {
        let subject = subject.into();
        let context_kind = context_kind.into();
        let expected_policy = expected_policy.into();
        let message = message.into();
        let denial_digest = digest_parts(
            [
                format!("{code:?}"),
                subject.clone(),
                context_kind.clone(),
                inherited_value.clone().unwrap_or_default(),
                attempted_value.clone().unwrap_or_default(),
                expected_policy.clone(),
                message.clone(),
            ]
            .into_iter()
            .chain(affected_descendants.iter().cloned()),
        );
        Self {
            code,
            subject,
            context_kind,
            inherited_value,
            attempted_value,
            expected_policy,
            affected_descendants,
            source_span_ready,
            message,
            denial_digest,
        }
    }

    pub fn code(&self) -> WorthUiCompositionContextDenialCode {
        self.code
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }

    pub fn context_kind(&self) -> &str {
        &self.context_kind
    }

    pub fn inherited_value(&self) -> Option<&str> {
        self.inherited_value.as_deref()
    }

    pub fn attempted_value(&self) -> Option<&str> {
        self.attempted_value.as_deref()
    }

    pub fn expected_policy(&self) -> &str {
        &self.expected_policy
    }

    pub fn affected_descendants(&self) -> &[String] {
        &self.affected_descendants
    }

    pub fn source_span_ready(&self) -> bool {
        self.source_span_ready
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn denial_digest(&self) -> u64 {
        self.denial_digest
    }

    fn presentation_rows(&self) -> Vec<WorthUiCompositionContextDenialPresentationRow> {
        [
            (
                "composition_context_denial",
                format!(
                    "code={:?} subject={} kind={} expected={}",
                    self.code, self.subject, self.context_kind, self.expected_policy
                ),
            ),
            ("affected_descendants", self.affected_descendants.join(",")),
        ]
        .into_iter()
        .map(|(label, value)| WorthUiCompositionContextDenialPresentationRow::new(label, value))
        .collect()
    }
}

impl WorthUiCompositionContextDenialPresentationRow {
    fn new(label: &'static str, value: String) -> Self {
        let row_digest = digest_parts([label.to_owned(), value.clone()]);
        Self {
            label,
            value,
            row_digest,
        }
    }

    pub fn label(&self) -> &'static str {
        self.label
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn row_digest(&self) -> u64 {
        self.row_digest
    }
}
