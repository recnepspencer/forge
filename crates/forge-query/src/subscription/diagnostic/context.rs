use crate::evidence_identity::ForgeQueryEvidenceIdentity;

use super::super::error::QuerySubscriptionFamilySelectionError;
use super::super::evidence_identities::{
    diagnostic_selection_context_denied_identity, diagnostic_selection_context_selected_identity,
};
use super::super::input::LiveQueryAdmissionArtifact;
use super::super::selection::QuerySubscriptionFamilySelection;

#[derive(Clone, Debug, Eq, PartialEq)]
enum QuerySubscriptionDiagnosticSelectionContextKind {
    Selected {
        selection: QuerySubscriptionFamilySelection,
        context_identity: ForgeQueryEvidenceIdentity,
    },
    Denied {
        source_identity: ForgeQueryEvidenceIdentity,
        query_family_label: String,
        declaration_family_label: String,
        basis_posture_label: String,
        context_identity: ForgeQueryEvidenceIdentity,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionDiagnosticSelectionContext {
    kind: QuerySubscriptionDiagnosticSelectionContextKind,
}

impl QuerySubscriptionDiagnosticSelectionContext {
    pub fn from_selection(selection: &QuerySubscriptionFamilySelection) -> Self {
        let context_identity = diagnostic_selection_context_selected_identity(
            selection.equivalence_basis().evidence_identity(),
        );
        Self {
            kind: QuerySubscriptionDiagnosticSelectionContextKind::Selected {
                selection: selection.clone(),
                context_identity,
            },
        }
    }

    pub fn from_selection_denial(
        live: &LiveQueryAdmissionArtifact,
        error: &QuerySubscriptionFamilySelectionError,
    ) -> Self {
        let query_family_label = match live.view_family() {
            Some(view_family) => format!(
                "selection_unresolved:{}:{}",
                live.live_family().as_str(),
                view_family.as_str()
            ),
            None => format!("selection_unresolved:{}:none", live.live_family().as_str()),
        };
        let declaration_family_label = format!("not_declared:{query_family_label}");
        let source_identity = error.diagnostic().source_identity().clone();
        let context_identity = diagnostic_selection_context_denied_identity(
            &source_identity,
            &query_family_label,
            &declaration_family_label,
            live.basis_posture().as_str(),
        );
        Self {
            kind: QuerySubscriptionDiagnosticSelectionContextKind::Denied {
                source_identity,
                query_family_label,
                declaration_family_label,
                basis_posture_label: live.basis_posture().as_str().to_string(),
                context_identity,
            },
        }
    }

    pub fn query_family_label(&self) -> &str {
        match &self.kind {
            QuerySubscriptionDiagnosticSelectionContextKind::Selected { selection, .. } => {
                selection.family().as_str()
            }
            QuerySubscriptionDiagnosticSelectionContextKind::Denied {
                query_family_label, ..
            } => query_family_label,
        }
    }

    pub fn declaration_family_label(&self) -> &str {
        match &self.kind {
            QuerySubscriptionDiagnosticSelectionContextKind::Selected { selection, .. } => {
                selection.family().as_str()
            }
            QuerySubscriptionDiagnosticSelectionContextKind::Denied {
                declaration_family_label,
                ..
            } => declaration_family_label,
        }
    }

    pub fn basis_posture_label(&self) -> &str {
        match &self.kind {
            QuerySubscriptionDiagnosticSelectionContextKind::Selected { selection, .. } => {
                selection.basis_posture().as_str()
            }
            QuerySubscriptionDiagnosticSelectionContextKind::Denied {
                basis_posture_label,
                ..
            } => basis_posture_label,
        }
    }

    pub fn context_identity(&self) -> &ForgeQueryEvidenceIdentity {
        match &self.kind {
            QuerySubscriptionDiagnosticSelectionContextKind::Selected {
                context_identity, ..
            }
            | QuerySubscriptionDiagnosticSelectionContextKind::Denied {
                context_identity, ..
            } => context_identity,
        }
    }

    pub(crate) fn selection(&self) -> Option<&QuerySubscriptionFamilySelection> {
        match &self.kind {
            QuerySubscriptionDiagnosticSelectionContextKind::Selected { selection, .. } => {
                Some(selection)
            }
            QuerySubscriptionDiagnosticSelectionContextKind::Denied { .. } => None,
        }
    }

    pub(crate) fn source_identity(&self) -> ForgeQueryEvidenceIdentity {
        match &self.kind {
            QuerySubscriptionDiagnosticSelectionContextKind::Selected { selection, .. } => {
                selection.equivalence_basis().evidence_identity().clone()
            }
            QuerySubscriptionDiagnosticSelectionContextKind::Denied {
                source_identity, ..
            } => source_identity.clone(),
        }
    }

    pub(crate) fn is_selection_denied(&self) -> bool {
        matches!(
            self.kind,
            QuerySubscriptionDiagnosticSelectionContextKind::Denied { .. }
        )
    }
}
