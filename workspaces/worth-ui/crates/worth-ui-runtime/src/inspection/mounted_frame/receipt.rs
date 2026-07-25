use worth_ui_host_contract::{UiMountedFrameIdentity, UiMountedNodeReceiptIdentity};

pub enum UiMountedInspectionReceipt {
    Available(Box<UiMountedInspectedFrame>),
    Omitted(UiMountedInspectionOmission),
}

pub struct UiMountedInspectedFrame {
    frame: UiMountedFrameIdentity,
    relation: UiMountedInspectionRelation,
    presented_binding_count: usize,
    mounted_instance_count: usize,
    selected_node_receipt: Option<UiMountedNodeReceiptIdentity>,
    mount_cost: crate::mounting::UiMountCostReport,
    retained_structural_bytes: usize,
    frame_index_probes: usize,
    instance_index_probes: usize,
    lease: crate::mounting::UiMountedRetentionLease,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMountedInspectionRelation {
    Current,
    RetainedPredecessor,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMountedInspectionOmission {
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
        budget: crate::mounting::UiMountedRetentionClassBudget,
    },
    AccountingOverflow,
}

impl UiMountedInspectionReceipt {
    pub(crate) fn available(basis: crate::mounting::UiMountedFrameInspectionBasis) -> Self {
        Self::Available(Box::new(UiMountedInspectedFrame {
            frame: basis.frame,
            relation: match basis.relation {
                crate::mounting::UiPresentedFrameBasisRelation::Current => {
                    UiMountedInspectionRelation::Current
                }
                crate::mounting::UiPresentedFrameBasisRelation::Retained => {
                    UiMountedInspectionRelation::RetainedPredecessor
                }
            },
            presented_binding_count: basis.presented_binding_count,
            mounted_instance_count: basis.mounted_instance_count,
            selected_node_receipt: basis.selected_node_receipt,
            mount_cost: basis.mount_cost,
            retained_structural_bytes: basis.retained_structural_bytes,
            frame_index_probes: basis.frame_index_probes,
            instance_index_probes: basis.instance_index_probes,
            lease: basis.lease,
        }))
    }

    pub(crate) fn omitted(denial: crate::mounting::UiMountedFrameInspectionDenial) -> Self {
        Self::Omitted(match denial {
            crate::mounting::UiMountedFrameInspectionDenial::FrameTransitionInFlight => {
                UiMountedInspectionOmission::FrameTransitionInFlight
            }
            crate::mounting::UiMountedFrameInspectionDenial::NoCurrentFrame => {
                UiMountedInspectionOmission::NoCurrentFrame
            }
            crate::mounting::UiMountedFrameInspectionDenial::UnknownFrame {
                frame_index_probes,
            } => UiMountedInspectionOmission::UnknownFrame { frame_index_probes },
            crate::mounting::UiMountedFrameInspectionDenial::ExpiredFrame {
                frame_index_probes,
            } => UiMountedInspectionOmission::ExpiredFrame { frame_index_probes },
            crate::mounting::UiMountedFrameInspectionDenial::InstanceNotPresented {
                frame_index_probes,
                instance_index_probes,
            } => UiMountedInspectionOmission::InstanceNotPresented {
                frame_index_probes,
                instance_index_probes,
            },
            crate::mounting::UiMountedFrameInspectionDenial::CapacityExceeded {
                required_leases,
                required_structural_bytes,
                budget,
            } => UiMountedInspectionOmission::CapacityExceeded {
                required_leases,
                required_structural_bytes,
                budget,
            },
            crate::mounting::UiMountedFrameInspectionDenial::AccountingOverflow => {
                UiMountedInspectionOmission::AccountingOverflow
            }
        })
    }
}

impl UiMountedInspectedFrame {
    pub const fn frame(&self) -> UiMountedFrameIdentity {
        self.frame
    }

    pub const fn relation(&self) -> UiMountedInspectionRelation {
        self.relation
    }

    pub const fn presented_binding_count(&self) -> usize {
        self.presented_binding_count
    }

    pub const fn mounted_instance_count(&self) -> usize {
        self.mounted_instance_count
    }

    pub const fn selected_node_receipt(&self) -> Option<UiMountedNodeReceiptIdentity> {
        self.selected_node_receipt
    }

    pub const fn mount_cost(&self) -> crate::mounting::UiMountCostReport {
        self.mount_cost
    }

    pub const fn retained_structural_bytes(&self) -> usize {
        self.retained_structural_bytes
    }

    pub const fn frame_index_probes(&self) -> usize {
        self.frame_index_probes
    }

    pub const fn instance_index_probes(&self) -> usize {
        self.instance_index_probes
    }

    pub fn retention_lease(&self) -> &crate::mounting::UiMountedRetentionLease {
        &self.lease
    }

    pub fn into_retention_lease(self) -> crate::mounting::UiMountedRetentionLease {
        self.lease
    }
}

impl std::fmt::Debug for UiMountedInspectionReceipt {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Available(frame) => formatter.debug_tuple("Available").field(frame).finish(),
            Self::Omitted(omission) => formatter.debug_tuple("Omitted").field(omission).finish(),
        }
    }
}

impl std::fmt::Debug for UiMountedInspectedFrame {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("UiMountedInspectedFrame")
            .field("frame", &self.frame)
            .field("relation", &self.relation)
            .field("presented_binding_count", &self.presented_binding_count)
            .field("mounted_instance_count", &self.mounted_instance_count)
            .field("selected_node_receipt", &self.selected_node_receipt)
            .field("mount_cost", &self.mount_cost)
            .field("retained_structural_bytes", &self.retained_structural_bytes)
            .field("frame_index_probes", &self.frame_index_probes)
            .field("instance_index_probes", &self.instance_index_probes)
            .finish_non_exhaustive()
    }
}
