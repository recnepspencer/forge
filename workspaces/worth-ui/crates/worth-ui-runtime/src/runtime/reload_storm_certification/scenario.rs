use crate::runtime::{WorthUiSourceProvider, WorthUiSourceProviderKind, WorthUiWatcherEvent};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiReloadStormScenario {
    name: String,
    steps: Vec<WorthUiReloadStormCandidateStep>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiReloadStormCandidateStep {
    label: String,
    provider: WorthUiSourceProvider,
    kind: WorthUiReloadStormCandidateStepKind,
    events: Vec<WorthUiWatcherEvent>,
    reuse_previous_receipt_probe: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiReloadStormCandidateStepKind {
    FileAuthored,
    RustAuthored,
}

impl WorthUiReloadStormScenario {
    pub fn named(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            steps: Vec::new(),
        }
    }

    pub fn with_file_candidate(
        mut self,
        label: impl Into<String>,
        provider: WorthUiSourceProvider,
    ) -> Self {
        self.steps
            .push(WorthUiReloadStormCandidateStep::file(label, provider));
        self
    }

    pub fn with_rust_candidate(
        mut self,
        label: impl Into<String>,
        provider: WorthUiSourceProvider,
    ) -> Self {
        self.steps
            .push(WorthUiReloadStormCandidateStep::rust(label, provider));
        self
    }

    pub fn with_file_candidate_events(
        mut self,
        label: impl Into<String>,
        provider: WorthUiSourceProvider,
        events: impl IntoIterator<Item = WorthUiWatcherEvent>,
    ) -> Self {
        self.steps
            .push(WorthUiReloadStormCandidateStep::file(label, provider).with_events(events));
        self
    }

    pub fn with_rust_candidate_events(
        mut self,
        label: impl Into<String>,
        provider: WorthUiSourceProvider,
        events: impl IntoIterator<Item = WorthUiWatcherEvent>,
    ) -> Self {
        self.steps
            .push(WorthUiReloadStormCandidateStep::rust(label, provider).with_events(events));
        self
    }

    #[cfg(test)]
    pub(crate) fn with_forged_receipt_reuse_probe(
        mut self,
        label: impl Into<String>,
        provider: WorthUiSourceProvider,
    ) -> Self {
        self.steps.push(
            WorthUiReloadStormCandidateStep::rust(label, provider)
                .with_reuse_previous_receipt_probe(),
        );
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn steps(&self) -> &[WorthUiReloadStormCandidateStep] {
        &self.steps
    }

    pub(crate) fn consumes_file_and_rust_candidates(&self) -> bool {
        let has_file = self
            .steps
            .iter()
            .any(|step| step.kind == WorthUiReloadStormCandidateStepKind::FileAuthored);
        let has_rust = self
            .steps
            .iter()
            .any(|step| step.kind == WorthUiReloadStormCandidateStepKind::RustAuthored);
        has_file && has_rust
    }

    pub(crate) fn scenario_digest(&self) -> u64 {
        let mut entries = vec![format!("scenario:{}", self.name)];
        for step in &self.steps {
            entries.push(format!(
                "{}|{:?}|{}|{}|{}",
                step.label,
                step.kind,
                step.provider.id(),
                step.provider.final_package_digest(),
                step.event_burst_digest()
            ));
        }
        super::digest::fold_texts(entries)
    }
}

impl WorthUiReloadStormCandidateStep {
    pub fn file(label: impl Into<String>, provider: WorthUiSourceProvider) -> Self {
        Self::new(
            label,
            provider,
            WorthUiReloadStormCandidateStepKind::FileAuthored,
        )
    }

    pub fn rust(label: impl Into<String>, provider: WorthUiSourceProvider) -> Self {
        Self::new(
            label,
            provider,
            WorthUiReloadStormCandidateStepKind::RustAuthored,
        )
    }

    fn new(
        label: impl Into<String>,
        provider: WorthUiSourceProvider,
        kind: WorthUiReloadStormCandidateStepKind,
    ) -> Self {
        Self {
            label: label.into(),
            provider,
            kind,
            events: Vec::new(),
            reuse_previous_receipt_probe: false,
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn provider(&self) -> &WorthUiSourceProvider {
        &self.provider
    }

    pub fn kind(&self) -> WorthUiReloadStormCandidateStepKind {
        self.kind
    }

    pub(crate) fn events(&self) -> Vec<WorthUiWatcherEvent> {
        if self.events.is_empty() {
            return vec![WorthUiWatcherEvent::provider_revision(
                self.provider.id().to_owned(),
            )];
        }
        self.events.clone()
    }

    pub(crate) fn event_burst_digest(&self) -> u64 {
        let events = self.events();
        let mut basis = events
            .iter()
            .map(WorthUiWatcherEvent::burst_digest_basis)
            .collect::<Vec<_>>();
        basis.sort();
        super::digest::fold_texts(basis)
    }

    pub(crate) fn expected_provider_kind_matches(&self) -> bool {
        matches!(
            (self.kind, self.provider.kind()),
            (
                WorthUiReloadStormCandidateStepKind::FileAuthored,
                WorthUiSourceProviderKind::Filesystem,
            ) | (
                WorthUiReloadStormCandidateStepKind::FileAuthored,
                WorthUiSourceProviderKind::EditorBuffer,
            ) | (
                WorthUiReloadStormCandidateStepKind::FileAuthored,
                WorthUiSourceProviderKind::Generated,
            ) | (
                WorthUiReloadStormCandidateStepKind::FileAuthored,
                WorthUiSourceProviderKind::InMemory,
            ) | (
                WorthUiReloadStormCandidateStepKind::RustAuthored,
                WorthUiSourceProviderKind::RustAuthoredArtifact,
            )
        )
    }

    pub(crate) fn reuse_previous_receipt_probe(&self) -> bool {
        self.reuse_previous_receipt_probe
    }

    #[cfg(test)]
    fn with_reuse_previous_receipt_probe(mut self) -> Self {
        self.reuse_previous_receipt_probe = true;
        self
    }

    fn with_events(mut self, events: impl IntoIterator<Item = WorthUiWatcherEvent>) -> Self {
        self.events = events.into_iter().collect();
        self
    }
}
