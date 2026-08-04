use worth_query_execution::facade::primary_graph::WorthQueryApplicationQueryAccessReceipt;

/// Borrowed, non-authoritative view of actual application-query terminal work.
#[derive(Clone, Copy)]
pub struct WorthQueryApplicationQueryPublicationInspection<'receipt> {
    terminal: &'receipt WorthQueryApplicationQueryAccessReceipt,
}

impl<'receipt> WorthQueryApplicationQueryPublicationInspection<'receipt> {
    pub(super) const fn new(terminal: &'receipt WorthQueryApplicationQueryAccessReceipt) -> Self {
        Self { terminal }
    }

    pub const fn terminal(&self) -> &'receipt WorthQueryApplicationQueryAccessReceipt {
        self.terminal
    }

    pub const fn session_identity(&self) -> u64 {
        self.terminal.read_completion().session_identity().as_u64()
    }

    pub const fn managed_run_identity(&self) -> u64 {
        self.terminal
            .read_completion()
            .managed_run_identity()
            .as_u64()
    }

    pub const fn admitted_plan_identity(&self) -> u64 {
        self.terminal.read_completion().plan_identity().as_u64()
    }

    pub fn relational_branch(&self) -> &str {
        &self.terminal.basis_identity().branch_id().0
    }

    pub const fn result_count(&self) -> usize {
        self.terminal.result_count()
    }

    pub const fn ordinary_work_units(&self) -> usize {
        self.terminal.total_work_units()
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

    pub fn terminal_resources_released(&self) -> bool {
        let completion = self.terminal.read_completion();
        self.terminal.basis_released()
            && completion.basis_release().released()
            && completion.release().released_reservation_count() == 1
    }
}
