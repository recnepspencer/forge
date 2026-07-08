use std::rc::Rc;

use crate::runtime::active::WorthUiActiveRuntimeState;
use crate::runtime::allocation_planning::WorthUiRetainedAllocationPlanningEvidenceRegistry;

use super::preservation::WorthUiLastValidRuntimeState;

/// Runtime host that owns active Worth UI runtime truth.
#[derive(Debug)]
pub struct WorthUiRuntimeHost {
    pub(crate) active: WorthUiActiveRuntimeState,
    pub(crate) last_valid: WorthUiLastValidRuntimeState,
    pub(crate) retained_allocation_planning_evidence:
        Rc<WorthUiRetainedAllocationPlanningEvidenceRegistry>,
}