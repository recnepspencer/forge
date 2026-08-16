use std::collections::{BTreeMap, BTreeSet};

use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::WorthQuerySharedLiveProjectionLease;
use crate::runtime::{
    WorthQuerySharedExecutionOwnerIdentity, WorthQuerySharedProjectionLeaseIdentity,
};

use super::super::consumer_delivery::current_shared_consumer_delivery_authority;
use super::super::WorthQuerySharedPrimaryGranularMaintenanceDenial;

pub(super) fn admitted_consumer_set<D: 'static, O: 'static, F: 'static, L: BasisOperationLane>(
    consumers: &[&WorthQuerySharedLiveProjectionLease<D, O, F, L>],
    owner: WorthQuerySharedExecutionOwnerIdentity,
    workspace: &mut crate::runtime::WorthQueryWorkspace,
) -> Result<
    BTreeMap<
        WorthQuerySharedProjectionLeaseIdentity,
        super::super::WorthQuerySharedConsumerDeliveryAuthority,
    >,
    WorthQuerySharedPrimaryGranularMaintenanceDenial,
> {
    let Some(first) = consumers.first().copied() else {
        return Err(WorthQuerySharedPrimaryGranularMaintenanceDenial::EmptyConsumerSet);
    };
    let current = workspace
        .current_shared_primary_leases(first.workspace_capability(), first.readmission())
        .map_err(WorthQuerySharedPrimaryGranularMaintenanceDenial::Runtime)?;
    let supplied: BTreeSet<_> = consumers
        .iter()
        .map(|consumer| consumer.lease_identity())
        .collect();
    let all_readmit = consumers.iter().all(|consumer| {
        consumer.owner_identity() == owner
            && workspace.readmits_shared_primary_lease(
                consumer.workspace_capability(),
                consumer.readmission(),
            )
    });
    if supplied.len() != consumers.len() || supplied != current || !all_readmit {
        return Err(WorthQuerySharedPrimaryGranularMaintenanceDenial::ConsumerSetMismatch);
    }
    consumers
        .iter()
        .map(|consumer| {
            Ok((
                consumer.lease_identity(),
                current_shared_consumer_delivery_authority(*consumer, workspace).ok_or(
                    WorthQuerySharedPrimaryGranularMaintenanceDenial::ConsumerDeliveryPolicyRequired,
                )?,
            ))
        })
        .collect()
}
