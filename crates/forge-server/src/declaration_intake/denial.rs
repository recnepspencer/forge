use forge_foundational::DiagnosticRichnessProfile;

use super::ForgeServerDirectSupportSnapshot;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerDirectDeclarationDenial {
    InvalidDeclarationIdentity {
        diagnostics_profile: DiagnosticRichnessProfile,
        detail: String,
    },
    WorkspaceBindingFailed {
        diagnostics_profile: DiagnosticRichnessProfile,
        detail: String,
    },
    SourceNotAdmitted {
        diagnostics_profile: DiagnosticRichnessProfile,
        detail: String,
        support_snapshot: ForgeServerDirectSupportSnapshot,
    },
    QueryFacadeFamilyNotAdmitted {
        diagnostics_profile: DiagnosticRichnessProfile,
        detail: String,
        support_snapshot: ForgeServerDirectSupportSnapshot,
    },
}

impl ForgeServerDirectDeclarationDenial {
    pub(crate) fn invalid_declaration_identity(
        diagnostics_profile: DiagnosticRichnessProfile,
        detail: impl Into<String>,
    ) -> Self {
        Self::InvalidDeclarationIdentity {
            diagnostics_profile,
            detail: detail.into(),
        }
    }

    pub(crate) fn workspace_binding_failed(
        diagnostics_profile: DiagnosticRichnessProfile,
        detail: impl Into<String>,
    ) -> Self {
        Self::WorkspaceBindingFailed {
            diagnostics_profile,
            detail: detail.into(),
        }
    }

    pub(crate) fn source_not_admitted(
        diagnostics_profile: DiagnosticRichnessProfile,
        detail: impl Into<String>,
        support_snapshot: ForgeServerDirectSupportSnapshot,
    ) -> Self {
        Self::SourceNotAdmitted {
            diagnostics_profile,
            detail: detail.into(),
            support_snapshot,
        }
    }

    pub(crate) fn query_facade_family_not_admitted(
        diagnostics_profile: DiagnosticRichnessProfile,
        detail: impl Into<String>,
        support_snapshot: ForgeServerDirectSupportSnapshot,
    ) -> Self {
        Self::QueryFacadeFamilyNotAdmitted {
            diagnostics_profile,
            detail: detail.into(),
            support_snapshot,
        }
    }

    pub fn code(&self) -> ForgeServerDirectDeclarationDenialCode {
        match self {
            Self::InvalidDeclarationIdentity { .. } => {
                ForgeServerDirectDeclarationDenialCode::InvalidDeclarationIdentity
            }
            Self::WorkspaceBindingFailed { .. } => {
                ForgeServerDirectDeclarationDenialCode::WorkspaceBindingFailed
            }
            Self::SourceNotAdmitted { .. } => {
                ForgeServerDirectDeclarationDenialCode::SourceNotAdmitted
            }
            Self::QueryFacadeFamilyNotAdmitted { .. } => {
                ForgeServerDirectDeclarationDenialCode::QueryFacadeFamilyNotAdmitted
            }
        }
    }

    pub fn diagnostics_profile(&self) -> DiagnosticRichnessProfile {
        match self {
            Self::InvalidDeclarationIdentity {
                diagnostics_profile,
                ..
            }
            | Self::WorkspaceBindingFailed {
                diagnostics_profile,
                ..
            }
            | Self::SourceNotAdmitted {
                diagnostics_profile,
                ..
            }
            | Self::QueryFacadeFamilyNotAdmitted {
                diagnostics_profile,
                ..
            } => *diagnostics_profile,
        }
    }

    pub fn detail(&self) -> &str {
        match self {
            Self::InvalidDeclarationIdentity { detail, .. }
            | Self::WorkspaceBindingFailed { detail, .. }
            | Self::SourceNotAdmitted { detail, .. }
            | Self::QueryFacadeFamilyNotAdmitted { detail, .. } => detail,
        }
    }

    pub fn support_snapshot(&self) -> Option<&ForgeServerDirectSupportSnapshot> {
        match self {
            Self::SourceNotAdmitted {
                support_snapshot, ..
            }
            | Self::QueryFacadeFamilyNotAdmitted {
                support_snapshot, ..
            } => Some(support_snapshot),
            Self::InvalidDeclarationIdentity { .. } | Self::WorkspaceBindingFailed { .. } => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerDirectDeclarationDenialCode {
    InvalidDeclarationIdentity,
    WorkspaceBindingFailed,
    SourceNotAdmitted,
    QueryFacadeFamilyNotAdmitted,
}
