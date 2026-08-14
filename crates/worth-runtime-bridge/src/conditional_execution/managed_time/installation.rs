use std::{num::NonZeroUsize, sync::Arc};

use super::{
    contract::{
        validate_identity, BridgeManagedClockLease, BridgeManagedTemporalDenial,
        BridgeManagedTemporalDenialKind,
    },
    BridgeManagedClockBinding, BridgeManagedClockInstallationParts, BridgeManagedClockLane,
};
use crate::conditional_execution::BridgeOwnedSignalRuntime;

impl BridgeOwnedSignalRuntime {
    pub fn install_managed_clock(
        &mut self,
        parts: BridgeManagedClockInstallationParts<'_>,
    ) -> Result<BridgeManagedClockBinding, BridgeManagedTemporalDenial> {
        if !self
            .conditional_lowerings
            .get(&parts.lowering.signal_node())
            .is_some_and(|installed| Arc::ptr_eq(installed, parts.lowering))
        {
            return Err(BridgeManagedTemporalDenial::new(
                BridgeManagedTemporalDenialKind::ForeignClockBinding,
                "managed clock requires its exact live conditional lowering",
            ));
        }
        validate_identity(&parts.binding_identity, "managed clock binding")?;
        validate_identity(&parts.source_identity, "managed clock source")?;
        validate_identity(&parts.timeline_identity, "managed clock timeline")?;
        let maximum_due_wakes_per_observation =
            NonZeroUsize::new(parts.maximum_due_wakes_per_observation).ok_or_else(|| {
                BridgeManagedTemporalDenial::new(
                    BridgeManagedTemporalDenialKind::InvalidContract,
                    "managed clock due-wake bound must be non-zero",
                )
            })?;
        if parts.maximum_active_intents == 0 {
            return Err(BridgeManagedTemporalDenial::new(
                BridgeManagedTemporalDenialKind::InvalidContract,
                "managed clock active-intent capacity must be non-zero",
            ));
        }
        if self
            .managed_clock_lanes
            .contains_key(&parts.binding_identity)
        {
            return Err(BridgeManagedTemporalDenial::new(
                BridgeManagedTemporalDenialKind::DuplicateClockBinding,
                "managed clock binding identity is already installed",
            ));
        }

        let lease = Arc::new(BridgeManagedClockLease::issue());
        let binding = BridgeManagedClockBinding {
            bridge_runtime_key: self.bridge.signal_runtime_key,
            binding_identity: Arc::clone(&parts.binding_identity),
            source_identity: Arc::clone(&parts.source_identity),
            timeline_identity: Arc::clone(&parts.timeline_identity),
            lease: Arc::clone(&lease),
        };
        self.managed_clock_lanes.insert(
            parts.binding_identity,
            BridgeManagedClockLane::new(
                Arc::clone(parts.lowering),
                parts.source_identity,
                parts.timeline_identity,
                lease,
                parts.maximum_active_intents,
                maximum_due_wakes_per_observation,
            ),
        );
        Ok(binding)
    }
}
