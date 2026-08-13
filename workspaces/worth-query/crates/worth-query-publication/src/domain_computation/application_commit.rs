use worth_query_execution::facade::primary_graph::{
    WorthQueryApplicationCommitPublicationSource, WorthQueryApplicationCommitReceipt,
};

mod inspection;
mod receipt;
mod terminal_release;

pub use inspection::WorthQueryApplicationCommitPublicationInspection;
pub use receipt::WorthQueryApplicationCommitPublicationReceipt;
pub use terminal_release::WorthQueryPublishedApplicationCommitAttemptReleasePosture;

/// Publication-owned application-commit result. It is an inspection product,
/// not a commit or retry authority.
pub struct WorthQueryPublishedApplicationCommit {
    receipt: WorthQueryApplicationCommitPublicationReceipt,
}

pub fn publish_application_commit(
    terminal: WorthQueryApplicationCommitReceipt,
) -> WorthQueryPublishedApplicationCommit {
    WorthQueryPublishedApplicationCommit {
        receipt: WorthQueryApplicationCommitPublicationReceipt::from_terminal(terminal),
    }
}

pub(crate) fn publish_application_commit_source(
    source: &WorthQueryApplicationCommitPublicationSource,
) -> WorthQueryPublishedApplicationCommit {
    WorthQueryPublishedApplicationCommit {
        receipt: WorthQueryApplicationCommitPublicationReceipt::from_publication_source(source),
    }
}

impl WorthQueryPublishedApplicationCommit {
    pub const fn receipt(&self) -> &WorthQueryApplicationCommitPublicationReceipt {
        &self.receipt
    }

    pub fn into_receipt(self) -> WorthQueryApplicationCommitPublicationReceipt {
        self.receipt
    }
}
