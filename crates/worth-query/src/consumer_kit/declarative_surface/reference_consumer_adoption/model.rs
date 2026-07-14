#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryReferenceConsumerResidueKind {
    LocalType,
    LocalHelper,
    LocalTransition,
    DeepImport,
    BackendDecision,
}

impl WorthQueryReferenceConsumerResidueKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LocalType => "local_type",
            Self::LocalHelper => "local_helper",
            Self::LocalTransition => "local_transition",
            Self::DeepImport => "deep_import",
            Self::BackendDecision => "backend_decision",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryReferenceConsumerDxCounters {
    import_count: usize,
    intermediate_type_count: usize,
    manual_transition_count: usize,
    backend_decision_count: usize,
    local_authority_decision_count: usize,
}

impl WorthQueryReferenceConsumerDxCounters {
    pub const fn new(
        import_count: usize,
        intermediate_type_count: usize,
        manual_transition_count: usize,
        backend_decision_count: usize,
        local_authority_decision_count: usize,
    ) -> Self {
        Self {
            import_count,
            intermediate_type_count,
            manual_transition_count,
            backend_decision_count,
            local_authority_decision_count,
        }
    }

    pub fn import_count(self) -> usize {
        self.import_count
    }

    pub fn intermediate_type_count(self) -> usize {
        self.intermediate_type_count
    }

    pub fn manual_transition_count(self) -> usize {
        self.manual_transition_count
    }

    pub fn backend_decision_count(self) -> usize {
        self.backend_decision_count
    }

    pub fn local_authority_decision_count(self) -> usize {
        self.local_authority_decision_count
    }

    pub fn ceremony_count(self) -> usize {
        self.import_count
            + self.intermediate_type_count
            + self.manual_transition_count
            + self.backend_decision_count
            + self.local_authority_decision_count
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryReferenceConsumerAdoptionRow {
    consumer: &'static str,
    source_path: &'static str,
    current_probe: &'static str,
    before: WorthQueryReferenceConsumerDxCounters,
    after: WorthQueryReferenceConsumerDxCounters,
}

impl WorthQueryReferenceConsumerAdoptionRow {
    pub(crate) const fn new(
        consumer: &'static str,
        source_path: &'static str,
        current_probe: &'static str,
        before: WorthQueryReferenceConsumerDxCounters,
        after: WorthQueryReferenceConsumerDxCounters,
    ) -> Self {
        Self {
            consumer,
            source_path,
            current_probe,
            before,
            after,
        }
    }

    pub fn consumer(&self) -> &'static str {
        self.consumer
    }

    pub fn source_path(&self) -> &'static str {
        self.source_path
    }

    pub fn current_probe(&self) -> &'static str {
        self.current_probe
    }

    pub fn before(&self) -> WorthQueryReferenceConsumerDxCounters {
        self.before
    }

    pub fn after(&self) -> WorthQueryReferenceConsumerDxCounters {
        self.after
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryReferenceConsumerDeletedResidue {
    consumer: &'static str,
    source_path: &'static str,
    probe: &'static str,
    kind: WorthQueryReferenceConsumerResidueKind,
}

impl WorthQueryReferenceConsumerDeletedResidue {
    pub(crate) const fn new(
        consumer: &'static str,
        source_path: &'static str,
        probe: &'static str,
        kind: WorthQueryReferenceConsumerResidueKind,
    ) -> Self {
        Self {
            consumer,
            source_path,
            probe,
            kind,
        }
    }

    pub fn consumer(&self) -> &'static str {
        self.consumer
    }

    pub fn source_path(&self) -> &'static str {
        self.source_path
    }

    pub fn probe(&self) -> &'static str {
        self.probe
    }

    pub fn kind(&self) -> WorthQueryReferenceConsumerResidueKind {
        self.kind
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryReferenceConsumerSource<'a> {
    path: &'a str,
    text: &'a str,
}

impl<'a> WorthQueryReferenceConsumerSource<'a> {
    pub const fn new(path: &'a str, text: &'a str) -> Self {
        Self { path, text }
    }

    pub fn path(&self) -> &str {
        self.path
    }

    pub fn text(&self) -> &str {
        self.text
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryReferenceConsumerAdoptionFindingKind {
    MissingCurrentSource,
    MissingCurrentProbe,
    AmbiguousCurrentProbe,
    DeletedResiduePresent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryReferenceConsumerAdoptionFinding {
    kind: WorthQueryReferenceConsumerAdoptionFindingKind,
    consumer: &'static str,
    source_path: &'static str,
    probe: &'static str,
    residue_kind: Option<WorthQueryReferenceConsumerResidueKind>,
}

impl WorthQueryReferenceConsumerAdoptionFinding {
    pub(crate) fn current(
        kind: WorthQueryReferenceConsumerAdoptionFindingKind,
        row: &WorthQueryReferenceConsumerAdoptionRow,
    ) -> Self {
        Self {
            kind,
            consumer: row.consumer(),
            source_path: row.source_path(),
            probe: row.current_probe(),
            residue_kind: None,
        }
    }

    pub(crate) fn residue(row: &WorthQueryReferenceConsumerDeletedResidue) -> Self {
        Self {
            kind: WorthQueryReferenceConsumerAdoptionFindingKind::DeletedResiduePresent,
            consumer: row.consumer(),
            source_path: row.source_path(),
            probe: row.probe(),
            residue_kind: Some(row.kind()),
        }
    }

    pub fn kind(&self) -> WorthQueryReferenceConsumerAdoptionFindingKind {
        self.kind
    }

    pub fn consumer(&self) -> &'static str {
        self.consumer
    }

    pub fn source_path(&self) -> &'static str {
        self.source_path
    }

    pub fn probe(&self) -> &'static str {
        self.probe
    }

    pub fn residue_kind(&self) -> Option<WorthQueryReferenceConsumerResidueKind> {
        self.residue_kind
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryReferenceConsumerAdoptionAudit {
    adopted_consumer_count: usize,
    deleted_residue_count: usize,
    before_ceremony_count: usize,
    after_ceremony_count: usize,
    findings: Vec<WorthQueryReferenceConsumerAdoptionFinding>,
}

impl WorthQueryReferenceConsumerAdoptionAudit {
    pub(crate) fn new(
        adopted_consumer_count: usize,
        deleted_residue_count: usize,
        before_ceremony_count: usize,
        after_ceremony_count: usize,
        findings: Vec<WorthQueryReferenceConsumerAdoptionFinding>,
    ) -> Self {
        Self {
            adopted_consumer_count,
            deleted_residue_count,
            before_ceremony_count,
            after_ceremony_count,
            findings,
        }
    }

    pub fn adopted_consumer_count(&self) -> usize {
        self.adopted_consumer_count
    }

    pub fn deleted_residue_count(&self) -> usize {
        self.deleted_residue_count
    }

    pub fn before_ceremony_count(&self) -> usize {
        self.before_ceremony_count
    }

    pub fn after_ceremony_count(&self) -> usize {
        self.after_ceremony_count
    }

    pub fn findings(&self) -> &[WorthQueryReferenceConsumerAdoptionFinding] {
        &self.findings
    }

    pub fn is_complete(&self) -> bool {
        self.findings.is_empty() && self.after_ceremony_count < self.before_ceremony_count
    }
}
