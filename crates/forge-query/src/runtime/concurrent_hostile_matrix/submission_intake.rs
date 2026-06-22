use std::sync::{Arc, Mutex};

use super::super::ForgeQueryWriteCommand;

#[derive(Clone, Debug)]
pub struct ForgeQueryConcurrentSubmissionIntake {
    records: Arc<Mutex<Vec<ForgeQueryConcurrentSubmissionRecord>>>,
}

impl ForgeQueryConcurrentSubmissionIntake {
    pub fn new() -> Self {
        Self {
            records: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn lane(&self, submitter_thread_ordinal: usize) -> ForgeQueryConcurrentSubmissionLane {
        ForgeQueryConcurrentSubmissionLane {
            submitter_thread_ordinal,
            records: Arc::clone(&self.records),
        }
    }

    pub fn drain_ordered(self) -> Vec<ForgeQueryConcurrentSubmissionRecord> {
        let mut records = Arc::try_unwrap(self.records)
            .expect("all concurrent submission lanes should be dropped before drain")
            .into_inner()
            .expect("concurrent submission intake lock should not be poisoned");
        records.sort_by_key(|record| record.submission_ordinal);
        records
    }
}

impl Default for ForgeQueryConcurrentSubmissionIntake {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct ForgeQueryConcurrentSubmissionLane {
    submitter_thread_ordinal: usize,
    records: Arc<Mutex<Vec<ForgeQueryConcurrentSubmissionRecord>>>,
}

impl ForgeQueryConcurrentSubmissionLane {
    pub fn submit(&self, submission_ordinal: usize, command: ForgeQueryWriteCommand) {
        let mut records = self
            .records
            .lock()
            .expect("concurrent submission intake lock should not be poisoned");
        records.push(ForgeQueryConcurrentSubmissionRecord {
            submitter_thread_ordinal: self.submitter_thread_ordinal,
            submission_ordinal,
            command,
        });
    }
}

#[derive(Clone, Debug)]
pub struct ForgeQueryConcurrentSubmissionRecord {
    submitter_thread_ordinal: usize,
    submission_ordinal: usize,
    command: ForgeQueryWriteCommand,
}

impl ForgeQueryConcurrentSubmissionRecord {
    pub fn submitter_thread_ordinal(&self) -> usize {
        self.submitter_thread_ordinal
    }

    pub fn submission_ordinal(&self) -> usize {
        self.submission_ordinal
    }

    pub fn into_command(self) -> ForgeQueryWriteCommand {
        self.command
    }
}
