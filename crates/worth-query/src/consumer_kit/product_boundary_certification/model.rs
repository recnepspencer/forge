#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WorthQueryProductBoundaryEvidenceKind {
    HostileRuntime,
    CompileBoundary,
    Sabotage,
    SemanticParity,
    Lifecycle,
    BoundedWork,
    ReferenceConsumer,
}

impl WorthQueryProductBoundaryEvidenceKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HostileRuntime => "hostile-runtime",
            Self::CompileBoundary => "compile-boundary",
            Self::Sabotage => "sabotage",
            Self::SemanticParity => "semantic-parity",
            Self::Lifecycle => "lifecycle",
            Self::BoundedWork => "bounded-work",
            Self::ReferenceConsumer => "reference-consumer",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WorthQueryProductBoundaryHostileCase {
    EquivalentDeclarationConvergence,
    CrossCapabilityOptionRejection,
    CrossBasisDenial,
    StaleContextDenial,
    OneShotLiveParity,
    HistoricalAmbiguity,
    PreviewWorkflowDenial,
    ReceiptNonPromotion,
    DiagnosticPolicyEquivalence,
}

impl WorthQueryProductBoundaryHostileCase {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EquivalentDeclarationConvergence => "equivalent-declaration-convergence",
            Self::CrossCapabilityOptionRejection => "cross-capability-option-rejection",
            Self::CrossBasisDenial => "cross-basis-denial",
            Self::StaleContextDenial => "stale-context-denial",
            Self::OneShotLiveParity => "one-shot-live-parity",
            Self::HistoricalAmbiguity => "historical-ambiguity",
            Self::PreviewWorkflowDenial => "preview-workflow-denial",
            Self::ReceiptNonPromotion => "receipt-non-promotion",
            Self::DiagnosticPolicyEquivalence => "diagnostic-policy-equivalence",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WorthQueryProductBoundarySabotageCase {
    PublicPhaseConstructor,
    DeepTransition,
    BackendSelector,
    SuccessEnvelopeBuilder,
    CompatibilityAlias,
    ConsumerLocalCoordinator,
}

impl WorthQueryProductBoundarySabotageCase {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PublicPhaseConstructor => "public-phase-constructor",
            Self::DeepTransition => "deep-transition",
            Self::BackendSelector => "backend-selector",
            Self::SuccessEnvelopeBuilder => "success-envelope-builder",
            Self::CompatibilityAlias => "compatibility-alias",
            Self::ConsumerLocalCoordinator => "consumer-local-coordinator",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryProductBoundaryEvidenceRow {
    id: &'static str,
    kind: WorthQueryProductBoundaryEvidenceKind,
    hostile_case: Option<WorthQueryProductBoundaryHostileCase>,
    sabotage_case: Option<WorthQueryProductBoundarySabotageCase>,
    source_path: &'static str,
    source_probe: &'static str,
    enforcement_layer: &'static str,
}

impl WorthQueryProductBoundaryEvidenceRow {
    pub(crate) const fn new(
        id: &'static str,
        kind: WorthQueryProductBoundaryEvidenceKind,
        hostile_case: Option<WorthQueryProductBoundaryHostileCase>,
        sabotage_case: Option<WorthQueryProductBoundarySabotageCase>,
        source_path: &'static str,
        source_probe: &'static str,
        enforcement_layer: &'static str,
    ) -> Self {
        Self {
            id,
            kind,
            hostile_case,
            sabotage_case,
            source_path,
            source_probe,
            enforcement_layer,
        }
    }

    pub fn id(&self) -> &'static str {
        self.id
    }
    pub fn kind(&self) -> WorthQueryProductBoundaryEvidenceKind {
        self.kind
    }
    pub fn hostile_case(&self) -> Option<WorthQueryProductBoundaryHostileCase> {
        self.hostile_case
    }
    pub fn sabotage_case(&self) -> Option<WorthQueryProductBoundarySabotageCase> {
        self.sabotage_case
    }
    pub fn source_path(&self) -> &'static str {
        self.source_path
    }
    pub fn source_probe(&self) -> &'static str {
        self.source_probe
    }
    pub fn enforcement_layer(&self) -> &'static str {
        self.enforcement_layer
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryProductBoundaryCertificationError {
    findings: Vec<String>,
}

impl WorthQueryProductBoundaryCertificationError {
    pub(crate) fn new(findings: Vec<String>) -> Self {
        Self { findings }
    }
    pub fn findings(&self) -> &[String] {
        &self.findings
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryProductBoundaryCertificationBundle {
    component_digests: Vec<(&'static str, String)>,
    grammar_row_count: usize,
    hostile_row_count: usize,
    sabotage_row_count: usize,
    closure_digest: String,
}

impl WorthQueryProductBoundaryCertificationBundle {
    pub(crate) fn new(
        component_digests: Vec<(&'static str, String)>,
        grammar_row_count: usize,
        hostile_row_count: usize,
        sabotage_row_count: usize,
        closure_digest: String,
    ) -> Self {
        Self {
            component_digests,
            grammar_row_count,
            hostile_row_count,
            sabotage_row_count,
            closure_digest,
        }
    }
    pub fn component_digests(&self) -> &[(&'static str, String)] {
        &self.component_digests
    }
    pub fn component_digest(&self, name: &str) -> Option<&str> {
        self.component_digests
            .iter()
            .find(|(key, _)| *key == name)
            .map(|(_, value)| value.as_str())
    }
    pub fn grammar_row_count(&self) -> usize {
        self.grammar_row_count
    }
    pub fn hostile_row_count(&self) -> usize {
        self.hostile_row_count
    }
    pub fn sabotage_row_count(&self) -> usize {
        self.sabotage_row_count
    }
    pub fn closure_digest(&self) -> &str {
        &self.closure_digest
    }
}
