use worth_proof::NonEmpty;
use worth_query_installation::facade::{
    ApplicationSchemaBindingIdentity, WorthQueryInstalledGraphObligation,
    WorthQueryInstalledGraphObligationSet, WorthQueryInstalledGraphObligationSetIdentity,
    WorthQueryInstalledGraphObligationSubjectKind,
};

use super::WorthQueryGraphObligationSelectionCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryGraphWorkIntentKind {
    ApplicationQueryRead,
    ApplicationOperationRead,
    ApplicationOperationMutation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphWorkIntent {
    kind: WorthQueryGraphWorkIntentKind,
}

impl WorthQueryGraphWorkIntent {
    pub const fn application_query_read() -> Self {
        Self {
            kind: WorthQueryGraphWorkIntentKind::ApplicationQueryRead,
        }
    }

    pub const fn application_operation_read() -> Self {
        Self {
            kind: WorthQueryGraphWorkIntentKind::ApplicationOperationRead,
        }
    }

    pub const fn application_operation_mutation() -> Self {
        Self {
            kind: WorthQueryGraphWorkIntentKind::ApplicationOperationMutation,
        }
    }

    pub const fn kind(self) -> WorthQueryGraphWorkIntentKind {
        self.kind
    }
}

/// Sealed selection proof over one installed obligation authority.
///
/// ```compile_fail
/// use worth_query_admission::facade::graph_obligation::WorthQuerySelectedGraphObligations;
///
/// let forged = WorthQuerySelectedGraphObligations {
///     installed: todo!(),
///     rows: todo!(),
///     intent: todo!(),
///     counters: todo!(),
/// };
/// ```
#[derive(Debug)]
pub struct WorthQuerySelectedGraphObligations {
    installed: WorthQueryInstalledGraphObligationSet,
    rows: NonEmpty<WorthQueryInstalledGraphObligation>,
    intent: WorthQueryGraphWorkIntent,
    counters: WorthQueryGraphObligationSelectionCounters,
}

impl WorthQuerySelectedGraphObligations {
    pub(super) fn seal(
        installed: WorthQueryInstalledGraphObligationSet,
        rows: NonEmpty<WorthQueryInstalledGraphObligation>,
        intent: WorthQueryGraphWorkIntent,
        counters: WorthQueryGraphObligationSelectionCounters,
    ) -> Self {
        Self {
            installed,
            rows,
            intent,
            counters,
        }
    }

    pub const fn identity(&self) -> &WorthQueryInstalledGraphObligationSetIdentity {
        self.installed.identity()
    }

    pub const fn binding_identity(&self) -> &ApplicationSchemaBindingIdentity {
        self.installed.binding_identity()
    }

    pub const fn subject_kind(&self) -> WorthQueryInstalledGraphObligationSubjectKind {
        self.installed.subject_kind()
    }

    pub fn subject_name(&self) -> &str {
        self.installed.subject_name()
    }

    pub const fn intent(&self) -> WorthQueryGraphWorkIntent {
        self.intent
    }

    pub fn rows(&self) -> &[WorthQueryInstalledGraphObligation] {
        self.rows.as_slice()
    }

    pub const fn counters(&self) -> WorthQueryGraphObligationSelectionCounters {
        self.counters
    }

    pub fn inspect(&self) -> WorthQuerySelectedGraphObligationInspection<'_> {
        WorthQuerySelectedGraphObligationInspection { selected: self }
    }
}

/// Non-authoritative view of one selected installed obligation set.
///
/// The view cannot be constructed or converted back into selection authority:
///
/// ```compile_fail
/// use worth_query_admission::facade::graph_obligation::WorthQuerySelectedGraphObligationInspection;
///
/// let forged = WorthQuerySelectedGraphObligationInspection { selected: todo!() };
/// ```
#[derive(Clone, Copy, Debug)]
pub struct WorthQuerySelectedGraphObligationInspection<'a> {
    selected: &'a WorthQuerySelectedGraphObligations,
}

impl WorthQuerySelectedGraphObligationInspection<'_> {
    pub fn identity(&self) -> &WorthQueryInstalledGraphObligationSetIdentity {
        self.selected.identity()
    }

    pub fn selected_row_count(&self) -> usize {
        self.selected.rows().len()
    }

    pub const fn counters(&self) -> WorthQueryGraphObligationSelectionCounters {
        self.selected.counters()
    }
}
