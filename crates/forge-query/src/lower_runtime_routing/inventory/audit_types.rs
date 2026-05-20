use super::ForgeQueryLowerRuntimeSeamKey;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryLowerRuntimeDirectImportPosture {
    RuntimeBackendBoundary,
    AllowedAdapter,
    TransitionOnlyElimination,
    DownstreamRuntimeBoundarySubtree,
}

impl ForgeQueryLowerRuntimeDirectImportPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeBackendBoundary => "runtime-backend-boundary",
            Self::AllowedAdapter => "allowed-adapter",
            Self::TransitionOnlyElimination => "transition-only-elimination",
            Self::DownstreamRuntimeBoundarySubtree => "downstream-runtime-boundary-subtree",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeDirectImportAuditRow {
    seam_key: ForgeQueryLowerRuntimeSeamKey,
    module_path: &'static str,
    posture: ForgeQueryLowerRuntimeDirectImportPosture,
    rationale: &'static str,
}

impl ForgeQueryLowerRuntimeDirectImportAuditRow {
    pub(crate) const fn new(
        seam_key: ForgeQueryLowerRuntimeSeamKey,
        module_path: &'static str,
        posture: ForgeQueryLowerRuntimeDirectImportPosture,
        rationale: &'static str,
    ) -> Self {
        Self {
            seam_key,
            module_path,
            posture,
            rationale,
        }
    }

    pub fn seam_key(&self) -> ForgeQueryLowerRuntimeSeamKey {
        self.seam_key
    }

    pub fn posture(&self) -> ForgeQueryLowerRuntimeDirectImportPosture {
        self.posture
    }

    pub fn module_path(&self) -> &'static str {
        self.module_path
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeDirectImportAudit {
    rows: &'static [ForgeQueryLowerRuntimeDirectImportAuditRow],
}

impl ForgeQueryLowerRuntimeDirectImportAudit {
    pub(crate) const fn new(rows: &'static [ForgeQueryLowerRuntimeDirectImportAuditRow]) -> Self {
        Self { rows }
    }

    pub fn rows(&self) -> &'static [ForgeQueryLowerRuntimeDirectImportAuditRow] {
        self.rows
    }
}
