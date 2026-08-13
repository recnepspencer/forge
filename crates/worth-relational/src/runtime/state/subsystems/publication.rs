use crate::publication::bundle::PublicationBundle;
use crate::replay::data::RelationalReplayRecord;
use crate::runtime::state::subsystems::RuntimeSubsystem;
use std::collections::VecDeque;

const RETIRED_BUNDLE_BACKLOG_LIMIT: usize = 8;

#[derive(Debug, Clone)]
pub(crate) struct PublicationSubsystem {
    pub(crate) diagnostics: Vec<crate::diagnostics::data::RelationalDiagnosticArtifact>,
    pub(crate) latest_bundle: Option<PublicationBundle<RelationalReplayRecord>>,
    retired_bundles: VecDeque<PublicationBundle<RelationalReplayRecord>>,
    pub(crate) post_commit_consumer: std::sync::Arc<dyn crate::publication::PostCommitConsumer>,
}

impl Default for PublicationSubsystem {
    fn default() -> Self {
        Self {
            diagnostics: Vec::new(),
            latest_bundle: None,
            retired_bundles: VecDeque::new(),
            post_commit_consumer: crate::publication::production_post_commit_consumer(),
        }
    }
}

impl PublicationSubsystem {
    pub(crate) fn replace_latest_bundle(
        &mut self,
        bundle: PublicationBundle<RelationalReplayRecord>,
    ) {
        if let Some(previous) = self.latest_bundle.replace(bundle) {
            self.retired_bundles.push_back(previous);
            if self.retired_bundles.len() > RETIRED_BUNDLE_BACKLOG_LIMIT {
                let _ = self.retired_bundles.pop_front();
            }
        }
    }
}

impl RuntimeSubsystem for PublicationSubsystem {
    type Config = ();

    fn new(_: &Self::Config) -> Self {
        Self::default()
    }

    fn fork(&self) -> Self {
        Self {
            diagnostics: self.diagnostics.clone(),
            latest_bundle: self.latest_bundle.clone(),
            retired_bundles: VecDeque::new(),
            post_commit_consumer: std::sync::Arc::clone(&self.post_commit_consumer),
        }
    }
}
