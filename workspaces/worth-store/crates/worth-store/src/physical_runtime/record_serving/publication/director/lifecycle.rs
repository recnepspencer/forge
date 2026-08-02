use std::sync::Arc;

use super::{RecordPublicationDirector, RecordPublicationTerminalState};

impl RecordPublicationDirector {
    pub(in crate::physical_runtime) fn stop_and_extract(
        director: Arc<Self>,
    ) -> RecordPublicationTerminalState {
        let mutations = director.mutations.stop_and_drain();
        let director = Arc::try_unwrap(director)
            .unwrap_or_else(|_| unreachable!("submission capabilities retain only weak authority"));
        RecordPublicationTerminalState {
            residue: director.residue,
            mutations,
        }
    }
}
