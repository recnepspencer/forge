use worth_query_execution::facade::primary_graph::WorthQueryApplicationQueryAccessReceipt;

use super::WorthQueryApplicationQueryPublicationInspection;
use super::WorthQueryPublishedApplicationDisclosure;
use super::WorthQueryPublishedApplicationQueryTerminalRelease;
use crate::application_aftermath::WorthQueryPublishedCanonicalWork;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryPublishedApplicationQueryOmissionPosture {
    NoOmission,
    GovernedOmission,
}

/// Publication-owned evidence derived from one completed application query.
///
/// This receipt stores only a closed publication projection. It cannot be
/// constructed by consumers and is not accepted by an execution transition.
///
/// ```compile_fail
/// use worth_query_execution::facade::primary_graph::WorthQueryApplicationQueryAccessReceipt;
/// use worth_query_publication::facade::domain_computation::WorthQueryApplicationQueryPublicationReceipt;
///
/// fn counterfeit(
///     terminal: WorthQueryApplicationQueryAccessReceipt,
/// ) -> WorthQueryApplicationQueryPublicationReceipt {
///     let _ = terminal;
///     WorthQueryApplicationQueryPublicationReceipt {
///         result_count: todo!(),
///         ordinary_work_units: todo!(),
///         disclosure: todo!(),
///         publication_work: todo!(),
///         terminal_release: todo!(),
///     }
/// }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryApplicationQueryPublicationReceipt {
    result_count: usize,
    ordinary_work_units: usize,
    disclosure: WorthQueryPublishedApplicationDisclosure,
    publication_work: WorthQueryPublishedCanonicalWork,
    terminal_release: WorthQueryPublishedApplicationQueryTerminalRelease,
}

impl WorthQueryApplicationQueryPublicationReceipt {
    pub(super) fn from_terminal(terminal: &WorthQueryApplicationQueryAccessReceipt) -> Self {
        let disclosure = WorthQueryPublishedApplicationDisclosure::capture(terminal.disclosure());
        Self {
            result_count: terminal.result_count(),
            ordinary_work_units: terminal.total_work_units(),
            disclosure,
            publication_work: WorthQueryPublishedCanonicalWork::from_owner(
                terminal.canonical_work().publication(),
            ),
            terminal_release: WorthQueryPublishedApplicationQueryTerminalRelease::capture(terminal),
        }
    }

    pub const fn inspect(&self) -> WorthQueryApplicationQueryPublicationInspection<'_> {
        WorthQueryApplicationQueryPublicationInspection::new(self)
    }

    pub(super) const fn result_count(&self) -> usize {
        self.result_count
    }
    pub(super) const fn ordinary_work_units(&self) -> usize {
        self.ordinary_work_units
    }
    pub(super) const fn omission_posture(
        &self,
    ) -> WorthQueryPublishedApplicationQueryOmissionPosture {
        if self.disclosure.has_omissions() {
            WorthQueryPublishedApplicationQueryOmissionPosture::GovernedOmission
        } else {
            WorthQueryPublishedApplicationQueryOmissionPosture::NoOmission
        }
    }
    pub const fn disclosure(&self) -> &WorthQueryPublishedApplicationDisclosure {
        &self.disclosure
    }
    pub(super) const fn publication_work(&self) -> WorthQueryPublishedCanonicalWork {
        self.publication_work
    }
    pub(super) const fn terminal_release(
        &self,
    ) -> WorthQueryPublishedApplicationQueryTerminalRelease {
        self.terminal_release
    }
}
