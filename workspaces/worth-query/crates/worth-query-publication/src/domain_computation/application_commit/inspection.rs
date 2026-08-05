use worth_query_execution::facade::primary_graph::{
    WorthQueryApplicationCommitReceipt, WorthQueryApplicationCommitTerminalKind,
    WorthQueryPrimaryMutationWorkEvidence,
};

#[derive(Clone, Copy)]
pub struct WorthQueryApplicationCommitPublicationInspection<'receipt> {
    terminal: &'receipt WorthQueryApplicationCommitReceipt,
}

impl<'receipt> WorthQueryApplicationCommitPublicationInspection<'receipt> {
    pub(super) const fn new(terminal: &'receipt WorthQueryApplicationCommitReceipt) -> Self {
        Self { terminal }
    }

    pub const fn terminal(&self) -> &'receipt WorthQueryApplicationCommitReceipt {
        self.terminal
    }

    pub const fn kind(&self) -> WorthQueryApplicationCommitTerminalKind {
        self.terminal.terminal().kind()
    }

    pub fn relational_branch(&self) -> &str {
        &self.terminal.terminal().branch().0
    }

    pub const fn executed_session_identity(&self) -> Option<u64> {
        match self.terminal.terminal().execution() {
            Some(completion) => Some(completion.session_identity().as_u64()),
            None => None,
        }
    }

    pub const fn retry_session_identity(&self) -> Option<u64> {
        match self.terminal.terminal().retry_inspection() {
            Some(completion) => Some(completion.session_identity().as_u64()),
            None => None,
        }
    }

    pub const fn mutation_work(&self) -> Option<WorthQueryPrimaryMutationWorkEvidence> {
        self.terminal.mutation_work()
    }

    pub const fn changed_record_count(&self) -> usize {
        self.terminal.changed_record_count()
    }

    pub const fn emitted_effect_count(&self) -> usize {
        self.terminal.emitted_effect_count()
    }

    pub const fn publication_canonical_entries(&self) -> u32 {
        self.terminal
            .canonical_work()
            .publication()
            .canonical_entries()
    }

    pub const fn publication_sha256_compression_blocks(&self) -> usize {
        self.terminal
            .canonical_work()
            .publication()
            .sha256_compression_blocks()
    }

    pub const fn publication_identity_text_materializations(&self) -> u32 {
        self.terminal
            .canonical_work()
            .publication()
            .digest_text_materializations()
    }

    pub fn attempt_resources_released(&self) -> Option<bool> {
        self.terminal
            .terminal()
            .execution()
            .or_else(|| self.terminal.terminal().retry_inspection())
            .map(|completion| {
                let cleanup = completion.cleanup();
                completion.snapshot_released()
                    && cleanup.relational().released()
                    && cleanup.bridge().reservation_released()
                    && cleanup.provider_work().provider_retained_bytes() == 0
                    && cleanup.attempt().capacity().released_reservation_count() > 0
            })
    }
}
