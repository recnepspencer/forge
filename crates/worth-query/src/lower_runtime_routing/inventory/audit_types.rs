use super::WorthQueryLowerRuntimeSeamKey;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryLowerRuntimeDirectImportPosture {
    RuntimeBackendBoundary,
    AllowedAdapter,
    TransitionOnlyElimination,
    DownstreamRuntimeBoundarySubtree,
}

impl WorthQueryLowerRuntimeDirectImportPosture {
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
pub struct WorthQueryLowerRuntimeDirectImportAuditRow {
    seam_key: WorthQueryLowerRuntimeSeamKey,
    module_path: &'static str,
    posture: WorthQueryLowerRuntimeDirectImportPosture,
    rationale: &'static str,
}

impl WorthQueryLowerRuntimeDirectImportAuditRow {
    pub(crate) const fn new(
        seam_key: WorthQueryLowerRuntimeSeamKey,
        module_path: &'static str,
        posture: WorthQueryLowerRuntimeDirectImportPosture,
        rationale: &'static str,
    ) -> Self {
        Self {
            seam_key,
            module_path,
            posture,
            rationale,
        }
    }

    pub fn seam_key(&self) -> WorthQueryLowerRuntimeSeamKey {
        self.seam_key
    }

    pub fn posture(&self) -> WorthQueryLowerRuntimeDirectImportPosture {
        self.posture
    }

    pub fn module_path(&self) -> &'static str {
        self.module_path
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryLowerRuntimeDirectImportAudit {
    rows: &'static [WorthQueryLowerRuntimeDirectImportAuditRow],
}

impl WorthQueryLowerRuntimeDirectImportAudit {
    pub(crate) const fn new(rows: &'static [WorthQueryLowerRuntimeDirectImportAuditRow]) -> Self {
        Self { rows }
    }

    pub fn rows(&self) -> &'static [WorthQueryLowerRuntimeDirectImportAuditRow] {
        self.rows
    }
}
