use std::collections::{BTreeMap, VecDeque};
use std::rc::Rc;

use worth_ui_host_contract::UiMountedFrameIdentity;

use super::{
    UiMountedFrameRetentionBudget, UiMountedRetentionClass, UiMountedRetentionClassBudget,
    UiMountedRetentionUsageSnapshot, UiPresentedFrameBasisRelation, UiRetainedPresentedFrame,
};

#[derive(Clone, Default)]
pub(super) struct UiMountedRetainedFrameState {
    pub(super) current: Option<Rc<UiRetainedPresentedFrame>>,
    pub(super) predecessors: crate::runtime::persistent_index::UiPersistentOrdMap<
        UiMountedFrameIdentity,
        Rc<UiRetainedPresentedFrame>,
    >,
    pub(super) predecessor_order: VecDeque<UiMountedFrameIdentity>,
    pub(super) predecessor_structural_bytes: usize,
    pub(super) expired:
        crate::runtime::persistent_index::UiPersistentOrdSet<UiMountedFrameIdentity>,
    pub(super) expiration_order: VecDeque<UiMountedFrameIdentity>,
    pub(super) diagnostics: crate::runtime::persistent_index::UiPersistentOrdMap<
        UiMountedFrameIdentity,
        Rc<super::UiRetainedMountedDiagnostics>,
    >,
    pub(super) diagnostic_order: VecDeque<UiMountedFrameIdentity>,
    pub(super) diagnostic_structural_bytes: usize,
}

pub(super) struct UiMountedFrameRetentionAuthority {
    pub(super) budget: UiMountedFrameRetentionBudget,
    pub(super) frames: UiMountedRetainedFrameState,
    pub(super) revision: u64,
    pub(super) reservation_active: bool,
    pub(super) in_flight_structural_bytes: usize,
    pins: BTreeMap<UiMountedFrameIdentity, UiMountedFramePinCounts>,
    inspection_usage: UiMountedRetentionUsageSnapshot,
    observation_basis_usage: UiMountedRetentionUsageSnapshot,
    diagnostic_usage: UiMountedRetentionUsageSnapshot,
}

pub(super) enum UiMountedRetainedFrameLookup<'a> {
    Found {
        evidence: &'a UiRetainedPresentedFrame,
        relation: UiPresentedFrameBasisRelation,
        frame_index_probes: usize,
    },
    Expired {
        frame_index_probes: usize,
    },
    Unknown {
        frame_index_probes: usize,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum UiMountedRetentionPinAdmissionDenial {
    CapacityExceeded {
        required_leases: usize,
        required_structural_bytes: usize,
        budget: UiMountedRetentionClassBudget,
    },
    AccountingOverflow,
}

#[derive(Clone, Copy, Default)]
struct UiMountedFramePinCounts {
    inspection: usize,
    observation_basis: usize,
    diagnostic: usize,
}

impl UiMountedFrameRetentionAuthority {
    pub(super) fn new(budget: UiMountedFrameRetentionBudget) -> Self {
        Self {
            budget,
            frames: Default::default(),
            revision: 0,
            reservation_active: false,
            in_flight_structural_bytes: 0,
            pins: BTreeMap::new(),
            inspection_usage: Default::default(),
            observation_basis_usage: Default::default(),
            diagnostic_usage: Default::default(),
        }
    }

    pub(super) fn current_frame(&self) -> UiMountedRetainedFrameLookup<'_> {
        match self.frames.current.as_deref() {
            Some(evidence) => UiMountedRetainedFrameLookup::Found {
                evidence,
                relation: UiPresentedFrameBasisRelation::Current,
                frame_index_probes: 1,
            },
            None => UiMountedRetainedFrameLookup::Unknown {
                frame_index_probes: 1,
            },
        }
    }

    pub(super) fn frame(&self, frame: UiMountedFrameIdentity) -> UiMountedRetainedFrameLookup<'_> {
        let mut probes = 1;
        if self
            .frames
            .current
            .as_ref()
            .is_some_and(|evidence| evidence.frame() == frame)
        {
            return UiMountedRetainedFrameLookup::Found {
                evidence: self
                    .frames
                    .current
                    .as_deref()
                    .expect("the current frame matched"),
                relation: UiPresentedFrameBasisRelation::Current,
                frame_index_probes: probes,
            };
        }
        let (predecessor, predecessor_probes) = self.frames.predecessors.get_with_probes(&frame);
        probes = probes
            .checked_add(predecessor_probes)
            .expect("frame index probe accounting fits usize");
        if let Some(evidence) = predecessor {
            return UiMountedRetainedFrameLookup::Found {
                evidence,
                relation: UiPresentedFrameBasisRelation::Retained,
                frame_index_probes: probes,
            };
        }
        let (expired, expired_probes) = self.frames.expired.contains_with_probes(&frame);
        probes = probes
            .checked_add(expired_probes)
            .expect("frame index probe accounting fits usize");
        if expired {
            UiMountedRetainedFrameLookup::Expired {
                frame_index_probes: probes,
            }
        } else {
            UiMountedRetainedFrameLookup::Unknown {
                frame_index_probes: probes,
            }
        }
    }

    pub(super) fn reserve_pin(
        &mut self,
        frame: UiMountedFrameIdentity,
        class: UiMountedRetentionClass,
        structural_bytes: usize,
    ) -> Result<(), UiMountedRetentionPinAdmissionDenial> {
        let existing = self.pins.get(&frame).copied().unwrap_or_default();
        let (frame_pin_count, usage, budget) = match class {
            UiMountedRetentionClass::PredecessorInspection => (
                existing.inspection,
                self.inspection_usage,
                self.budget.predecessor_inspection(),
            ),
            UiMountedRetentionClass::ObservationBasis => (
                existing.observation_basis,
                self.observation_basis_usage,
                self.budget.observation_basis(),
            ),
            UiMountedRetentionClass::Diagnostic => (
                existing.diagnostic,
                self.diagnostic_usage,
                self.budget.diagnostic(),
            ),
            _ => unreachable!("only lease-backed retention classes reserve pins"),
        };
        let next_frame_pin_count = frame_pin_count
            .checked_add(1)
            .ok_or(UiMountedRetentionPinAdmissionDenial::AccountingOverflow)?;
        let required_leases = usage
            .active_leases
            .checked_add(1)
            .ok_or(UiMountedRetentionPinAdmissionDenial::AccountingOverflow)?;
        let required_structural_bytes = usage
            .lease_charged_structural_bytes
            .checked_add(structural_bytes)
            .ok_or(UiMountedRetentionPinAdmissionDenial::AccountingOverflow)?;
        if !budget.admits(required_leases, required_structural_bytes) {
            return Err(UiMountedRetentionPinAdmissionDenial::CapacityExceeded {
                required_leases,
                required_structural_bytes,
                budget,
            });
        }

        let pins = self.pins.entry(frame).or_default();
        match class {
            UiMountedRetentionClass::PredecessorInspection => {
                pins.inspection = next_frame_pin_count
            }
            UiMountedRetentionClass::ObservationBasis => {
                pins.observation_basis = next_frame_pin_count
            }
            UiMountedRetentionClass::Diagnostic => pins.diagnostic = next_frame_pin_count,
            _ => unreachable!("only lease-backed retention classes reserve pins"),
        }
        let usage = self.pin_usage_mut(class);
        usage.active_leases = required_leases;
        usage.lease_charged_structural_bytes = required_structural_bytes;
        Ok(())
    }

    pub(super) fn release_pin(
        &mut self,
        frame: UiMountedFrameIdentity,
        class: UiMountedRetentionClass,
        structural_bytes: usize,
    ) {
        let Some(existing) = self.pins.get(&frame).copied() else {
            debug_assert!(
                false,
                "a retention lease must release an existing frame pin"
            );
            return;
        };
        let active_for_class = match class {
            UiMountedRetentionClass::PredecessorInspection => existing.inspection,
            UiMountedRetentionClass::ObservationBasis => existing.observation_basis,
            UiMountedRetentionClass::Diagnostic => existing.diagnostic,
            _ => {
                debug_assert!(false, "only lease-backed retention classes release pins");
                return;
            }
        };
        if active_for_class == 0 {
            debug_assert!(
                false,
                "a retention lease cannot release an absent class pin"
            );
            return;
        }

        let usage = self.pin_usage_mut(class);
        usage.active_leases = usage
            .active_leases
            .checked_sub(1)
            .expect("retention lease accounting includes the released lease");
        usage.lease_charged_structural_bytes = usage
            .lease_charged_structural_bytes
            .checked_sub(structural_bytes)
            .expect("retention byte accounting includes the released lease");

        let pins = self
            .pins
            .get_mut(&frame)
            .expect("the frame pin was present before accounting");
        match class {
            UiMountedRetentionClass::PredecessorInspection => pins.inspection -= 1,
            UiMountedRetentionClass::ObservationBasis => pins.observation_basis -= 1,
            UiMountedRetentionClass::Diagnostic => pins.diagnostic -= 1,
            _ => unreachable!("non-lease classes returned above"),
        }
        if pins.inspection == 0 && pins.observation_basis == 0 && pins.diagnostic == 0 {
            self.pins.remove(&frame);
        }
    }

    pub(super) fn frame_is_pinned(&self, frame: UiMountedFrameIdentity) -> bool {
        self.pins
            .get(&frame)
            .is_some_and(|pins| pins.inspection > 0 || pins.observation_basis > 0)
    }

    pub(super) fn diagnostic_is_pinned(&self, frame: UiMountedFrameIdentity) -> bool {
        self.pins
            .get(&frame)
            .is_some_and(|pins| pins.diagnostic > 0)
    }

    pub(super) fn diagnostics(
        &self,
        frame: UiMountedFrameIdentity,
    ) -> Option<Rc<super::UiRetainedMountedDiagnostics>> {
        self.frames.diagnostics.get(&frame).cloned()
    }

    pub(super) fn snapshot(&self) -> super::UiMountedFrameRetentionSnapshot {
        super::UiMountedFrameRetentionSnapshot {
            current: retained_frame_usage(self.frames.current.as_deref()),
            in_flight: UiMountedRetentionUsageSnapshot {
                retained_items: usize::from(self.reservation_active),
                retained_structural_bytes: self.in_flight_structural_bytes,
                active_leases: 0,
                lease_charged_structural_bytes: 0,
            },
            observation_basis: self.observation_basis_usage,
            predecessor_inspection: UiMountedRetentionUsageSnapshot {
                retained_items: self.frames.predecessors.len(),
                retained_structural_bytes: self.frames.predecessor_structural_bytes,
                active_leases: self.inspection_usage.active_leases,
                lease_charged_structural_bytes: self
                    .inspection_usage
                    .lease_charged_structural_bytes,
            },
            diagnostic: UiMountedRetentionUsageSnapshot {
                retained_items: self.frames.diagnostics.len(),
                retained_structural_bytes: self.frames.diagnostic_structural_bytes,
                active_leases: self.diagnostic_usage.active_leases,
                lease_charged_structural_bytes: self
                    .diagnostic_usage
                    .lease_charged_structural_bytes,
            },
            future_snapshot: Default::default(),
            budget: self.budget,
        }
    }

    fn pin_usage_mut(
        &mut self,
        class: UiMountedRetentionClass,
    ) -> &mut UiMountedRetentionUsageSnapshot {
        match class {
            UiMountedRetentionClass::PredecessorInspection => &mut self.inspection_usage,
            UiMountedRetentionClass::ObservationBasis => &mut self.observation_basis_usage,
            UiMountedRetentionClass::Diagnostic => &mut self.diagnostic_usage,
            _ => unreachable!("only lease-backed retention classes own usage"),
        }
    }
}

fn retained_frame_usage(
    evidence: Option<&UiRetainedPresentedFrame>,
) -> UiMountedRetentionUsageSnapshot {
    UiMountedRetentionUsageSnapshot {
        retained_items: usize::from(evidence.is_some()),
        retained_structural_bytes: evidence.map_or(0, UiRetainedPresentedFrame::structural_bytes),
        active_leases: 0,
        lease_charged_structural_bytes: 0,
    }
}
