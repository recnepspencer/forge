use worth_query_execution::facade::primary_graph::WorthQueryApplicationQueryAccessReceipt;

use super::WorthQueryApplicationQueryPublicationInspection;

/// Publication-owned evidence derived from one completed application query.
///
/// This receipt retains the execution terminal for inspection. It cannot be
/// constructed by consumers and is not accepted by an execution transition.
///
/// ```compile_fail
/// use worth_query_execution::facade::primary_graph::WorthQueryApplicationQueryAccessReceipt;
/// use worth_query_publication::facade::domain_computation::WorthQueryApplicationQueryPublicationReceipt;
///
/// fn counterfeit(
///     terminal: WorthQueryApplicationQueryAccessReceipt,
/// ) -> WorthQueryApplicationQueryPublicationReceipt {
///     WorthQueryApplicationQueryPublicationReceipt { terminal }
/// }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryApplicationQueryPublicationReceipt {
    terminal: WorthQueryApplicationQueryAccessReceipt,
}

impl WorthQueryApplicationQueryPublicationReceipt {
    pub(super) fn from_terminal(terminal: &WorthQueryApplicationQueryAccessReceipt) -> Self {
        Self {
            terminal: terminal.clone(),
        }
    }

    pub const fn inspect(&self) -> WorthQueryApplicationQueryPublicationInspection<'_> {
        WorthQueryApplicationQueryPublicationInspection::new(&self.terminal)
    }
}

impl std::ops::Deref for WorthQueryApplicationQueryPublicationReceipt {
    type Target = WorthQueryApplicationQueryAccessReceipt;

    fn deref(&self) -> &Self::Target {
        &self.terminal
    }
}
