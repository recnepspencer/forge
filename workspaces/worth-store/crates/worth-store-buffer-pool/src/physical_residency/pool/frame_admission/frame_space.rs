use super::super::*;

impl PoolInner {
    pub(in crate::physical_residency::pool) fn reserve_frame_space(
        &self,
        state: &mut PoolState,
        scope: PhysicalOperationAllocationScope,
        bytes: u64,
    ) -> Result<(), PhysicalResidencyDenial> {
        if !state.accepting {
            return Err(Self::deny(state, PhysicalResidencyDenial::PoolClosed));
        }
        self.validate_pin_capacity(state, scope)?;
        while self.frame_space_required(state, bytes) {
            state.accounting.inspect_eviction_candidate();
            let Some(victim) = state.select_oldest_legal_victim() else {
                let demand = self.frame_space_pressure(state, scope, bytes);
                return Err(self.pressure(state, demand));
            };
            state.evict_selected_victim(victim);
        }
        Ok(())
    }

    fn validate_pin_capacity(
        &self,
        state: &mut PoolState,
        scope: PhysicalOperationAllocationScope,
    ) -> Result<(), PhysicalResidencyDenial> {
        if state.accounting.pin_leases() >= self.limits.pin_leases() {
            return Err(self.pressure(
                state,
                PhysicalResidencyPressureDemand {
                    dimension: PhysicalResidencyDimension::PinLeases,
                    scope,
                    requested: 1,
                    current: u64::from(state.accounting.pin_leases()),
                    limit: u64::from(self.limits.pin_leases()),
                },
            ));
        }
        if state.accounting.pinned_frames() >= self.limits.pinned_frames() {
            return Err(self.pressure(
                state,
                PhysicalResidencyPressureDemand {
                    dimension: PhysicalResidencyDimension::PinnedFrames,
                    scope,
                    requested: 1,
                    current: u64::from(state.accounting.pinned_frames()),
                    limit: u64::from(self.limits.pinned_frames()),
                },
            ));
        }
        Ok(())
    }

    fn frame_space_required(&self, state: &PoolState, bytes: u64) -> bool {
        state.accounting.resident_bytes().saturating_add(bytes) > self.limits.resident_bytes()
            || state.accounting.frame_entries() >= self.limits.frame_entries()
            || self.current_admitted_bytes(state).saturating_add(bytes) > self.limits.total_bytes()
    }

    fn frame_space_pressure(
        &self,
        state: &PoolState,
        scope: PhysicalOperationAllocationScope,
        bytes: u64,
    ) -> PhysicalResidencyPressureDemand {
        if state.accounting.frame_entries() >= self.limits.frame_entries() {
            PhysicalResidencyPressureDemand {
                dimension: PhysicalResidencyDimension::FrameEntries,
                scope,
                requested: 1,
                current: u64::from(state.accounting.frame_entries()),
                limit: u64::from(self.limits.frame_entries()),
            }
        } else if state.accounting.resident_bytes().saturating_add(bytes)
            > self.limits.resident_bytes()
        {
            PhysicalResidencyPressureDemand {
                dimension: PhysicalResidencyDimension::ResidentBytes,
                scope,
                requested: bytes,
                current: state.accounting.resident_bytes(),
                limit: self.limits.resident_bytes(),
            }
        } else {
            PhysicalResidencyPressureDemand {
                dimension: PhysicalResidencyDimension::TotalBytes,
                scope,
                requested: bytes,
                current: self.current_admitted_bytes(state),
                limit: self.limits.total_bytes(),
            }
        }
    }
}
