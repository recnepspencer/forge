use crate::basis_lifecycle::BasisOperationLane;

use super::WorthQuerySharedLiveProjectionLease;

pub enum WorthQuerySharedConditionalDeliveryStop {
    Runtime {
        error: crate::runtime::WorthQueryRuntimeError,
        counters: super::WorthQuerySharedProjectionDrainCounters,
    },
    Refresh {
        error: super::super::WorthQueryLiveProjectionRefreshError,
        counters: super::WorthQuerySharedProjectionDrainCounters,
    },
}

impl WorthQuerySharedConditionalDeliveryStop {
    pub fn owner_delivery_retained(&self) -> bool {
        match self {
            Self::Runtime { .. } => true,
            Self::Refresh { error, .. } => error.owner_delivery_retained(),
        }
    }

    pub fn runtime_error(&self) -> Option<&crate::runtime::WorthQueryRuntimeError> {
        match self {
            Self::Runtime { error, .. } => Some(error),
            Self::Refresh { .. } => None,
        }
    }

    pub fn refresh_error(&self) -> Option<&super::super::WorthQueryLiveProjectionRefreshError> {
        match self {
            Self::Runtime { .. } => None,
            Self::Refresh { error, .. } => Some(error),
        }
    }

    pub const fn counters(&self) -> super::WorthQuerySharedProjectionDrainCounters {
        match self {
            Self::Runtime { counters, .. } | Self::Refresh { counters, .. } => *counters,
        }
    }
}

impl<D: 'static, O: 'static, F: 'static, L: BasisOperationLane>
    WorthQuerySharedLiveProjectionLease<D, O, F, L>
{
    pub fn drain_conditional_owner_delivery(
        &self,
        delivery: &worth_runtime_bridge::facade::BridgeCorrespondenceDeliveryReceipt,
        workspace: &mut crate::runtime::WorthQueryWorkspace,
    ) -> Result<super::WorthQuerySharedProjectionDelivery, WorthQuerySharedConditionalDeliveryStop>
    {
        workspace.drain_shared_conditional_owner_delivery(
            self.workspace_capability(),
            self.readmission(),
            self.snapshot(),
            delivery,
        )
    }
}

impl crate::runtime::WorthQueryWorkspace {
    fn drain_shared_conditional_owner_delivery<D: 'static, O: 'static, F: 'static, L>(
        &mut self,
        capability: &std::sync::Arc<crate::runtime::WorthQueryManagedLiveWorkspaceCapability>,
        readmission: super::WorthQuerySharedProjectionLeaseReadmission<'_>,
        source: &super::super::WorthQuerySettledDomainProjection<D, O, F, L>,
        delivery: &worth_runtime_bridge::facade::BridgeCorrespondenceDeliveryReceipt,
    ) -> Result<super::WorthQuerySharedProjectionDelivery, WorthQuerySharedConditionalDeliveryStop>
    where
        L: BasisOperationLane,
    {
        let mut counters = super::WorthQuerySharedProjectionDrainCounters {
            workspace_capability_checks: 1,
            ..super::WorthQuerySharedProjectionDrainCounters::default()
        };
        let reaped = self
            .admit_shared_conditional_delivery_capability(capability, readmission.owner)
            .map_err(|error| runtime_stop(error, counters))?;
        counters.abandoned_owner_index_lookups = 1;
        counters.abandoned_leases_reaped = reaped;
        let lease_identity = readmission.lease;
        let (owner_identity, owner) = self
            .take_shared_owner_for_conditional_delivery(readmission, &mut counters)
            .map_err(|error| runtime_stop(error, counters))?;
        let refreshed = super::super::projection_lifecycle::refresh_owner_delivery::<D, O, F, L, _>(
            source,
            owner.handle(),
            self,
            super::super::projection_lifecycle::WorthQueryPendingOwnerImpact::new(
                delivery,
                owner.closure(),
            ),
        );
        let classified = match refreshed {
            Ok(classified) => classified,
            Err(error) => {
                self.restore_shared_owner_after_conditional_stop(owner_identity, owner);
                return Err(WorthQuerySharedConditionalDeliveryStop::Refresh { error, counters });
            }
        };
        self.finish_shared_conditional_delivery(
            crate::runtime::WorthQuerySharedConditionalDeliveryCompletion::new(
                owner_identity,
                lease_identity,
                owner,
                classified,
                counters,
            ),
        )
        .map_err(|error| runtime_stop(error, counters))
    }
}

fn runtime_stop(
    error: crate::runtime::WorthQueryRuntimeError,
    counters: super::WorthQuerySharedProjectionDrainCounters,
) -> WorthQuerySharedConditionalDeliveryStop {
    WorthQuerySharedConditionalDeliveryStop::Runtime { error, counters }
}
