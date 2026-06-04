use crate::source::{SourceFailureRecord, SourceMaterializationRecord};

use super::*;

impl BridgeDiagnosticsFacade {
    pub fn source_materialization_records(&self) -> Vec<SourceMaterializationRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .source_materialization_records()
    }

    pub fn source_failure_records(&self) -> Vec<SourceFailureRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .source_failure_records()
    }

    pub fn stream_checkpoints(&self) -> Vec<ConsumerCheckpointToken> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .stream_checkpoints()
    }

    pub fn stream_replay_records(&self) -> Vec<CanonicalStreamReplayRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .stream_replay_records()
    }

    pub fn last_source_materialization_record(&self) -> Option<SourceMaterializationRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .last_source_materialization_record()
    }

    pub fn last_source_failure_record(&self) -> Option<SourceFailureRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .last_source_failure_record()
    }

    pub fn last_stream_checkpoint(&self) -> Option<ConsumerCheckpointToken> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .last_stream_checkpoint()
    }

    pub fn last_stream_replay_record(&self) -> Option<CanonicalStreamReplayRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .last_stream_replay_record()
    }

    pub fn source_materialization_record_for_identity(
        &self,
        record_identity: &str,
    ) -> Option<SourceMaterializationRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .source_materialization_record_for_identity(record_identity)
    }

    pub fn source_failure_for_declaration_identity(
        &self,
        declaration_identity: &str,
    ) -> Option<SourceFailureRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .source_failure_for_declaration_identity(declaration_identity)
    }

    pub fn source_failure_record_for_identity(
        &self,
        failure_identity: &str,
    ) -> Option<SourceFailureRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .source_failure_record_for_identity(failure_identity)
    }

    pub fn stream_checkpoint_for_identity(
        &self,
        checkpoint_identity: &str,
    ) -> Option<ConsumerCheckpointToken> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .stream_checkpoint_for_identity(checkpoint_identity)
    }

    pub fn stream_replay_record_for_identity(
        &self,
        replay_record_identity: &str,
    ) -> Option<CanonicalStreamReplayRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .stream_replay_record_for_identity(replay_record_identity)
    }

    pub fn stream_replay_record_for_checkpoint_identity(
        &self,
        checkpoint_identity: &str,
    ) -> Option<CanonicalStreamReplayRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .stream_replay_record_for_checkpoint_identity(checkpoint_identity)
    }
}
