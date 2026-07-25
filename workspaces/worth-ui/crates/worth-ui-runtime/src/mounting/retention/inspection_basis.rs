use worth_ui_host_contract::{UiMountedFrameIdentity, UiMountedNodeReceiptIdentity};

#[derive(Clone, Copy)]
pub(crate) struct UiMountedFrameInspectionSelection {
    pub(crate) target: UiMountedFrameInspectionTarget,
    pub(crate) instance: Option<worth_ui_host_contract::UiMountedInstanceIdentity>,
    pub(crate) diagnostics: bool,
}

#[derive(Clone, Copy)]
pub(crate) enum UiMountedFrameInspectionTarget {
    Current,
    Frame(UiMountedFrameIdentity),
}

pub(crate) struct UiMountedFrameInspectionBasis {
    pub(crate) frame: UiMountedFrameIdentity,
    pub(crate) relation: super::UiPresentedFrameBasisRelation,
    pub(crate) presented_binding_count: usize,
    pub(crate) mounted_instance_count: usize,
    pub(crate) selected_node_receipt: Option<UiMountedNodeReceiptIdentity>,
    pub(crate) mount_cost: super::super::UiMountCostReport,
    pub(crate) retained_structural_bytes: usize,
    pub(crate) frame_index_probes: usize,
    pub(crate) instance_index_probes: usize,
    pub(crate) diagnostics: UiMountedDiagnosticInspectionBasis,
    pub(crate) lease: super::UiMountedRetentionLease,
}

pub(crate) enum UiMountedDiagnosticInspectionBasis {
    NotRequested,
    Available {
        evidence: std::rc::Rc<super::UiRetainedMountedDiagnostics>,
        lease: super::UiMountedDiagnosticRetentionLease,
    },
    Omitted(UiMountedDiagnosticInspectionDenial),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiMountedDiagnosticInspectionDenial {
    NotRetained,
    CapacityExceeded {
        required_leases: usize,
        required_structural_bytes: usize,
        budget: super::UiMountedRetentionClassBudget,
    },
    AccountingOverflow,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiMountedFrameInspectionDenial {
    FrameTransitionInFlight,
    NoCurrentFrame,
    UnknownFrame {
        frame_index_probes: usize,
    },
    ExpiredFrame {
        frame_index_probes: usize,
    },
    InstanceNotPresented {
        frame_index_probes: usize,
        instance_index_probes: usize,
    },
    CapacityExceeded {
        required_leases: usize,
        required_structural_bytes: usize,
        budget: super::UiMountedRetentionClassBudget,
    },
    AccountingOverflow,
}
