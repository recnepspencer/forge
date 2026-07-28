use std::cell::RefCell;
use std::rc::Rc;

use crate::runtime::active::WorthUiActiveRuntimeState;
use crate::runtime::allocation_frame_dispatch::UiAllocationFrameFrameworkScheduler;
use crate::runtime::allocation_receipt::UiAllocationReceiptLedger;
use crate::runtime::planning::allocation_planning::WorthUiRetainedAllocationPlanningEvidenceRegistry;

use super::preservation::WorthUiLastValidRuntimeState;

/// Canonical application framework loop. It owns runtime truth and is the only
/// production clock for allocation-source collection and frame close/pump.
#[derive(Debug)]
pub struct WorthUiRuntimeFrameworkLoop {
    pub(crate) active_application_lowering_authority:
        crate::facade::prepared_application_authority::WorthUiPreparedApplicationLoweringAuthority,
    pub(crate) active: WorthUiActiveRuntimeState,
    pub(crate) last_valid: WorthUiLastValidRuntimeState,
    pub(crate) retained_allocation_planning_evidence:
        Rc<WorthUiRetainedAllocationPlanningEvidenceRegistry>,
    pub(crate) allocation_receipt_ledger: UiAllocationReceiptLedger,
    pub(crate) allocation_invalidation_index:
        RefCell<crate::runtime::invalidation_narrowing::UiAllocationInvalidationAuthority>,
    pub(crate) allocation_frame_scheduler: UiAllocationFrameFrameworkScheduler,
    pub(crate) allocation_source_order_ledger:
        crate::runtime::stream_policy::UiAllocationSourceOrderLedger,
    pub(crate) query_binding: worth_ui_query_binding::WorthUiRuntimeQueryBinding,
    pub(crate) transient_interaction_admission:
        crate::runtime::replacement::state_inventory::WorthUiTransientInteractionAdmissionAuthority,
    pub(crate) host_measurement_source: Rc<RefCell<crate::host::UiHostMeasurementSourceAuthority>>,
    pub(crate) host_session_identity: Option<crate::facade::WorthUiHostSessionIdentity>,
    pub(crate) host_observation_generation:
        Option<worth_ui_host_contract::WorthUiHostCapabilityObservationGeneration>,
    pub(crate) host_plan_binding: crate::facade::WorthUiHostPlanBinding,
    pub(crate) durable_resize_source:
        crate::runtime::replacement::reconciliation::WorthUiDurableResizeSourceAuthority,
    pub(crate) scroll_offset_projection:
        crate::runtime::scroll_owned_allocation::UiScrollOffsetProjectionLedger,
    pub(crate) observation: crate::runtime::observation::UiObservationRuntimeState,
}

/// Established facade name for the canonical framework-loop owner.
pub type WorthUiRuntime = WorthUiRuntimeFrameworkLoop;
