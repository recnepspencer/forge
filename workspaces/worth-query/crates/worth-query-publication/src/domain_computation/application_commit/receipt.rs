use worth_query_execution::facade::primary_graph::WorthQueryApplicationCommitReceipt;

use super::WorthQueryApplicationCommitPublicationInspection;
use crate::application_aftermath::{
    publish_application_aftermath, WorthQueryPublishedApplicationAftermath,
};

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
///
/// The publication receipt does not dereference back into execution:
///
/// ```compile_fail
/// use worth_query_execution::facade::primary_graph::WorthQueryApplicationCommitReceipt;
/// use worth_query_publication::facade::domain_computation::WorthQueryApplicationCommitPublicationReceipt;
///
/// fn escape(
///     published: &WorthQueryApplicationCommitPublicationReceipt,
/// ) -> &WorthQueryApplicationCommitReceipt {
///     published
/// }
/// ```
///
/// Nor does it expose a terminal accessor:
///
/// ```compile_fail
/// use worth_query_publication::facade::domain_computation::WorthQueryApplicationCommitPublicationReceipt;
///
/// fn escape(published: &WorthQueryApplicationCommitPublicationReceipt) {
///     let _ = published.terminal();
/// }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryApplicationCommitPublicationReceipt {
    terminal: WorthQueryApplicationCommitReceipt,
    aftermath: WorthQueryPublishedApplicationAftermath,
}

impl WorthQueryApplicationCommitPublicationReceipt {
    pub(crate) const fn from_terminal(terminal: WorthQueryApplicationCommitReceipt) -> Self {
        let aftermath = publish_application_aftermath(&terminal);
        Self {
            terminal,
            aftermath,
        }
    }

    pub const fn inspect(&self) -> WorthQueryApplicationCommitPublicationInspection<'_> {
        WorthQueryApplicationCommitPublicationInspection::new(&self.terminal)
    }

    pub const fn aftermath(&self) -> &WorthQueryPublishedApplicationAftermath {
        &self.aftermath
    }
}
