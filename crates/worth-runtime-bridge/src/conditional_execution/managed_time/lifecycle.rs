use std::sync::Arc;

use super::{
    BridgeManagedClockBinding, BridgeManagedClockClosure, BridgeManagedClockLane,
    BridgeManagedTemporalDenial, BridgeManagedTemporalDenialKind,
};
use crate::conditional_execution::BridgeOwnedSignalRuntime;

impl BridgeOwnedSignalRuntime {
    pub(super) fn managed_clock_lane_mut(
        &mut self,
        binding: &BridgeManagedClockBinding,
    ) -> Result<&mut BridgeManagedClockLane, BridgeManagedTemporalDenial> {
        if binding.bridge_runtime_key != self.bridge.signal_runtime_key {
            return Err(foreign_binding_denial());
        }
        let Some(lane) = self.managed_clock_lanes.get_mut(&binding.binding_identity) else {
            return Err(if binding.is_live() {
                foreign_binding_denial()
            } else {
                BridgeManagedTemporalDenial::new(
                    BridgeManagedTemporalDenialKind::ClosedClockBinding,
                    "managed clock binding is no longer installed",
                )
            });
        };
        if !Arc::ptr_eq(&lane.lease, &binding.lease) {
            return Err(foreign_binding_denial());
        }
        if !binding.is_live() {
            return Err(BridgeManagedTemporalDenial::new(
                BridgeManagedTemporalDenialKind::ClosedClockBinding,
                "managed clock binding has been closed",
            ));
        }
        Ok(lane)
    }

    pub fn close_managed_clock(
        &mut self,
        binding: BridgeManagedClockBinding,
    ) -> Result<BridgeManagedClockClosure, BridgeManagedTemporalDenial> {
        self.managed_clock_lane_mut(&binding)?;
        let lane = self
            .managed_clock_lanes
            .remove(&binding.binding_identity)
            .expect("validated managed clock lane remains installed until removal");
        let (active_intents, scheduled_wakes, ready_wakes) = lane.closure_counts();
        lane.lease.revoke();
        Ok(BridgeManagedClockClosure::new(
            active_intents,
            scheduled_wakes,
            ready_wakes,
        ))
    }

    pub(in crate::conditional_execution) fn revoke_managed_clock_liveness(&mut self) {
        for lane in self.managed_clock_lanes.values() {
            lane.lease.revoke();
        }
    }

    pub fn managed_clock_count(&self) -> usize {
        self.managed_clock_lanes.len()
    }
}

fn foreign_binding_denial() -> BridgeManagedTemporalDenial {
    BridgeManagedTemporalDenial::new(
        BridgeManagedTemporalDenialKind::ForeignClockBinding,
        "managed clock binding belongs to another Bridge runtime or installation",
    )
}
