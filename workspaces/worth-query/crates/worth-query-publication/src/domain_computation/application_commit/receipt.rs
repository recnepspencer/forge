use worth_query_execution::facade::primary_graph::WorthQueryApplicationCommitReceipt;

use super::WorthQueryApplicationCommitPublicationInspection;

/// Publication receipt derived from one execution-owned commit terminal.
///
/// ```compile_fail
/// use worth_query_execution::facade::primary_graph::WorthQueryApplicationCommitReceipt;
/// use worth_query_publication::facade::domain_computation::WorthQueryApplicationCommitPublicationReceipt;
///
/// fn counterfeit(
///     terminal: WorthQueryApplicationCommitReceipt,
/// ) -> WorthQueryApplicationCommitPublicationReceipt {
///     WorthQueryApplicationCommitPublicationReceipt { terminal }
/// }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryApplicationCommitPublicationReceipt {
    terminal: WorthQueryApplicationCommitReceipt,
}

impl std::ops::Deref for WorthQueryApplicationCommitPublicationReceipt {
    type Target = WorthQueryApplicationCommitReceipt;

    fn deref(&self) -> &Self::Target {
        &self.terminal
    }
}

impl WorthQueryApplicationCommitPublicationReceipt {
    pub(super) const fn from_terminal(terminal: WorthQueryApplicationCommitReceipt) -> Self {
        Self { terminal }
    }

    pub const fn inspect(&self) -> WorthQueryApplicationCommitPublicationInspection<'_> {
        WorthQueryApplicationCommitPublicationInspection::new(&self.terminal)
    }

    pub const fn terminal(&self) -> &WorthQueryApplicationCommitReceipt {
        &self.terminal
    }
}
