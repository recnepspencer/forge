use crate::logic::runtime::state::subsystems::RuntimeSubsystem;
use crate::publication::data::PublicationBundle;
use crate::replay::data::RelationalReplayRecord;

#[derive(Debug, Clone, Default)]
pub(crate) struct PublicationSubsystem {
    pub(crate) diagnostics: Vec<crate::diagnostics::data::RelationalDiagnosticArtifact>,
    pub(crate) latest_bundle: Option<PublicationBundle<RelationalReplayRecord>>,
}

impl RuntimeSubsystem for PublicationSubsystem {
    type Config = ();

    fn new(_: &Self::Config) -> Self {
        Self::default()
    }

    fn fork(&self) -> Self {
        self.clone()
    }
}
