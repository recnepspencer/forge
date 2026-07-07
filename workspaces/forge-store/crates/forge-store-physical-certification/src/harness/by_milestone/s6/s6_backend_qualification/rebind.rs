use forge_store_physical_backend::{BackendCapabilitySupportPosture, BackendRebindTriggers};

use super::BackendQualificationRow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QualificationRebindEvaluation {
    Current,
    Stale,
    RebindRequired(BackendRebindTriggers),
}

impl QualificationRebindEvaluation {
    pub const fn requires_rebind(self) -> bool {
        matches!(self, Self::RebindRequired(_))
    }
}

pub fn evaluate_row_rebind(row: &BackendQualificationRow) -> QualificationRebindEvaluation {
    match row.support_posture() {
        BackendCapabilitySupportPosture::Stale => QualificationRebindEvaluation::Stale,
        BackendCapabilitySupportPosture::RebindRequired => {
            QualificationRebindEvaluation::RebindRequired(row.rebind_triggers())
        }
        BackendCapabilitySupportPosture::Supported
        | BackendCapabilitySupportPosture::Unsupported
        | BackendCapabilitySupportPosture::Unavailable
        | BackendCapabilitySupportPosture::Unknown => QualificationRebindEvaluation::Current,
    }
}
