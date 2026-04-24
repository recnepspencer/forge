use crate::identity::hash_parts;

use super::super::error::QuerySubscriptionFamilySelectionError;
use super::super::input::LiveQueryAdmissionArtifact;
use super::super::selection::QuerySubscriptionFamilySelection;

#[derive(Clone, Debug, Eq, PartialEq)]
enum QuerySubscriptionDiagnosticSelectionContextKind {
    Selected(QuerySubscriptionFamilySelection),
    Denied {
        source_digest: String,
        query_family_label: String,
        declaration_family_label: String,
        basis_posture_label: String,
        digest: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionDiagnosticSelectionContext {
    kind: QuerySubscriptionDiagnosticSelectionContextKind,
}

impl QuerySubscriptionDiagnosticSelectionContext {
    pub fn from_selection(selection: &QuerySubscriptionFamilySelection) -> Self {
        Self {
            kind: QuerySubscriptionDiagnosticSelectionContextKind::Selected(selection.clone()),
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
        let source_digest = error.diagnostic().source_digest().to_string();
        let digest = hash_parts(&[
            "query_subscription_diagnostic_selection_context_v1".to_string(),
            "selection_denied".to_string(),
            format!("source:{source_digest}"),
            format!("query_family:{query_family_label}"),
            format!("declaration_family:{declaration_family_label}"),
            format!("basis_posture:{}", live.basis_posture().as_str()),
        ]);
        Self {
            kind: QuerySubscriptionDiagnosticSelectionContextKind::Denied {
                source_digest,
                query_family_label,
                declaration_family_label,
                basis_posture_label: live.basis_posture().as_str().to_string(),
                digest,
            },
        }
    }

    pub fn query_family_label(&self) -> &str {
        match &self.kind {
            QuerySubscriptionDiagnosticSelectionContextKind::Selected(selection) => {
                selection.family().as_str()
            }
            QuerySubscriptionDiagnosticSelectionContextKind::Denied {
                query_family_label, ..
            } => query_family_label,
        }
    }

    pub fn declaration_family_label(&self) -> &str {
        match &self.kind {
            QuerySubscriptionDiagnosticSelectionContextKind::Selected(selection) => {
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
            QuerySubscriptionDiagnosticSelectionContextKind::Selected(selection) => {
                selection.basis_posture().as_str()
            }
            QuerySubscriptionDiagnosticSelectionContextKind::Denied {
                basis_posture_label,
                ..
            } => basis_posture_label,
        }
    }

    pub fn digest(&self) -> &str {
        match &self.kind {
            QuerySubscriptionDiagnosticSelectionContextKind::Selected(selection) => {
                selection.equivalence_basis().digest().as_str()
            }
            QuerySubscriptionDiagnosticSelectionContextKind::Denied { digest, .. } => digest,
        }
    }

    pub(crate) fn selection(&self) -> Option<&QuerySubscriptionFamilySelection> {
        match &self.kind {
            QuerySubscriptionDiagnosticSelectionContextKind::Selected(selection) => Some(selection),
            QuerySubscriptionDiagnosticSelectionContextKind::Denied { .. } => None,
        }
    }

    pub(crate) fn source_digest(&self) -> &str {
        match &self.kind {
            QuerySubscriptionDiagnosticSelectionContextKind::Selected(selection) => {
                selection.equivalence_basis().digest().as_str()
            }
            QuerySubscriptionDiagnosticSelectionContextKind::Denied { source_digest, .. } => {
                source_digest
            }
        }
    }

    pub(crate) fn is_selection_denied(&self) -> bool {
        matches!(
            self.kind,
            QuerySubscriptionDiagnosticSelectionContextKind::Denied { .. }
        )
    }
}
