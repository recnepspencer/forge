use std::collections::{BTreeMap, BTreeSet, VecDeque};

use worth_ui_host_contract::{
    UiMountedFrameIdentity, UiMountedInstanceIdentity, UiMountedNodeReceiptIdentity,
    UiSurfaceBindingGeneration,
};

const RETAINED_PRESENTED_FRAME_LIMIT: usize = 8;
const EXPIRED_PRESENTED_FRAME_IDENTITY_LIMIT: usize = 64;

#[derive(Clone)]
struct UiPresentedFrameBasis {
    frame: UiMountedFrameIdentity,
    bindings: BTreeSet<UiSurfaceBindingGeneration>,
    receipts: BTreeMap<UiMountedInstanceIdentity, UiMountedNodeReceiptIdentity>,
}

pub(crate) struct UiPreparedPresentedFrameBasis(UiPresentedFrameBasis);

#[derive(Clone)]
pub(crate) struct UiMountedPresentedFrameRetention {
    current: Option<UiPresentedFrameBasis>,
    retained: VecDeque<UiPresentedFrameBasis>,
    expiration_order: VecDeque<UiMountedFrameIdentity>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiPresentedFrameBasisRelation {
    Current,
    Retained,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiPresentedFrameBasisDenial {
    Expired,
    Unknown,
    BindingNotPresented,
    InstanceNotPresented,
    NodeReceiptMismatch,
}

impl UiMountedPresentedFrameRetention {
    pub(crate) fn prepare(
        frame: UiMountedFrameIdentity,
        bindings: &[UiSurfaceBindingGeneration],
        receipts: impl Iterator<Item = (UiMountedInstanceIdentity, UiMountedNodeReceiptIdentity)>,
    ) -> UiPreparedPresentedFrameBasis {
        UiPreparedPresentedFrameBasis(UiPresentedFrameBasis {
            frame,
            bindings: bindings.iter().copied().collect(),
            receipts: receipts.collect(),
        })
    }

    pub(crate) fn publish(&mut self, prepared: UiPreparedPresentedFrameBasis) {
        if let Some(predecessor) = self.current.replace(prepared.0) {
            self.retained.push_back(predecessor);
        }
        self.enforce_bound();
    }

    pub(crate) fn reconcile_current(&mut self, prepared: UiPreparedPresentedFrameBasis) {
        debug_assert_eq!(
            self.current.as_ref().map(|basis| basis.frame),
            Some(prepared.0.frame),
            "reconciliation may replace only the current frame's presentation basis"
        );
        self.current = Some(prepared.0);
    }

    pub(crate) fn inherited_by_replacement(&self) -> Self {
        let mut inherited = self.clone();
        if let Some(current) = inherited.current.take() {
            inherited.retained.push_back(current);
        }
        inherited.enforce_bound();
        inherited
    }

    pub(crate) fn classify(
        &self,
        frame: UiMountedFrameIdentity,
        binding: UiSurfaceBindingGeneration,
        mounted_instance: Option<UiMountedInstanceIdentity>,
        node_receipt: Option<UiMountedNodeReceiptIdentity>,
    ) -> Result<UiPresentedFrameBasisRelation, UiPresentedFrameBasisDenial> {
        let (basis, relation) = if self
            .current
            .as_ref()
            .is_some_and(|basis| basis.frame == frame)
        {
            (
                self.current.as_ref().expect("current frame matched"),
                UiPresentedFrameBasisRelation::Current,
            )
        } else if let Some(basis) = self.retained.iter().find(|basis| basis.frame == frame) {
            (basis, UiPresentedFrameBasisRelation::Retained)
        } else if self.expiration_order.contains(&frame) {
            return Err(UiPresentedFrameBasisDenial::Expired);
        } else {
            return Err(UiPresentedFrameBasisDenial::Unknown);
        };
        if !basis.bindings.contains(&binding) {
            return Err(UiPresentedFrameBasisDenial::BindingNotPresented);
        }
        match (mounted_instance, node_receipt) {
            (None, None) => Ok(relation),
            (Some(instance), Some(receipt)) => {
                let expected = basis
                    .receipts
                    .get(&instance)
                    .ok_or(UiPresentedFrameBasisDenial::InstanceNotPresented)?;
                (*expected == receipt)
                    .then_some(relation)
                    .ok_or(UiPresentedFrameBasisDenial::NodeReceiptMismatch)
            }
            _ => Err(UiPresentedFrameBasisDenial::InstanceNotPresented),
        }
    }

    fn enforce_bound(&mut self) {
        while self.retained.len() > RETAINED_PRESENTED_FRAME_LIMIT {
            let expired = self
                .retained
                .pop_front()
                .expect("over-limit retained frame queue is non-empty")
                .frame;
            if !self.expiration_order.contains(&expired) {
                self.expiration_order.push_back(expired);
            }
        }
        while self.expiration_order.len() > EXPIRED_PRESENTED_FRAME_IDENTITY_LIMIT {
            let forgotten = self
                .expiration_order
                .pop_front()
                .expect("over-limit expiration queue is non-empty");
            let _ = forgotten;
        }
    }
}

impl Default for UiMountedPresentedFrameRetention {
    fn default() -> Self {
        Self {
            current: None,
            retained: VecDeque::with_capacity(RETAINED_PRESENTED_FRAME_LIMIT),
            expiration_order: VecDeque::with_capacity(EXPIRED_PRESENTED_FRAME_IDENTITY_LIMIT),
        }
    }
}
