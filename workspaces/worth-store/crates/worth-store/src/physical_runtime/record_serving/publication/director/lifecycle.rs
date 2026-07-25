use std::sync::Arc;

use super::{RecordPublicationCall, RecordPublicationDirector, RecordPublicationTerminalState};
use crate::physical_runtime::record_serving::{RecordAppendDenial, RecordAppendError};

impl RecordPublicationDirector {
    pub(super) fn begin(director: &Arc<Self>) -> Result<RecordPublicationCall, RecordAppendError> {
        let mut gate = director
            .gate
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if !gate.accepting {
            return Err(RecordAppendError::Denied(
                RecordAppendDenial::PublicationAdmissionStopped,
            ));
        }
        gate.active = gate.active.saturating_add(1);
        drop(gate);
        Ok(RecordPublicationCall {
            director: Arc::clone(director),
        })
    }

    pub(in crate::physical_runtime) fn stop_and_extract(
        director: Arc<Self>,
    ) -> RecordPublicationTerminalState {
        {
            let mut gate = director
                .gate
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            gate.accepting = false;
            while gate.active != 0 {
                gate = director
                    .changed
                    .wait(gate)
                    .unwrap_or_else(|poisoned| poisoned.into_inner());
            }
        }
        let director = Arc::try_unwrap(director)
            .unwrap_or_else(|_| unreachable!("submission capabilities retain only weak authority"));
        let state = director
            .state
            .into_inner()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        RecordPublicationTerminalState {
            residue: state.residue,
        }
    }
}

impl Drop for RecordPublicationCall {
    fn drop(&mut self) {
        let mut gate = self
            .director
            .gate
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        gate.active = gate.active.saturating_sub(1);
        self.director.changed.notify_all();
    }
}
