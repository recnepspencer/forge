use crate::admission::{
    UiAdmissionHostCapability, UiAdmissionQueryBasis, UiAdmissionSelectionBudget,
    UiAdmissionStaleEvidence,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiObligationDispatchStopPosture {
    None,
    Unsupported,
    Deferred,
    DiagnosticOnly,
    WrongWorld,
    WrongQueryBasis {
        required: UiAdmissionQueryBasis,
        observed: UiAdmissionQueryBasis,
    },
    WrongHostCapability {
        required: UiAdmissionHostCapability,
        observed: UiAdmissionHostCapability,
    },
    Stale {
        required: UiAdmissionQueryBasis,
        observed: UiAdmissionQueryBasis,
        evidence: UiAdmissionStaleEvidence,
    },
    Ambiguous {
        required_query_basis: Option<UiAdmissionQueryBasis>,
        observed_query_basis: Option<UiAdmissionQueryBasis>,
        required_host_capability: Option<UiAdmissionHostCapability>,
        observed_host_capability: Option<UiAdmissionHostCapability>,
    },
    BudgetExceeded {
        budget: UiAdmissionSelectionBudget,
        attempted_lane_cost: u8,
    },
}
