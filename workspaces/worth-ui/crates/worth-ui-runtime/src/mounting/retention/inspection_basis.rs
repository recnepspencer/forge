use worth_ui_host_contract::{UiMountedFrameIdentity, UiMountedNodeReceiptIdentity};

pub(crate) struct UiMountedFrameInspectionBasis {
    pub(crate) frame: UiMountedFrameIdentity,
    pub(crate) relation: super::UiPresentedFrameBasisRelation,
    pub(crate) presented_binding_count: usize,
    pub(crate) mounted_instance_count: usize,
    pub(crate) selected_node_receipt: Option<UiMountedNodeReceiptIdentity>,
    pub(crate) mount_cost: super::super::UiMountCostReport,
    pub(crate) frame_index_probes: usize,
    pub(crate) instance_index_probes: usize,
    pub(crate) lease: super::UiMountedRetentionLease,
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
