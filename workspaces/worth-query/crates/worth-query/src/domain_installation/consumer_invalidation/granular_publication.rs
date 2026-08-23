use crate::basis_lifecycle::BasisOperationLane;

use super::{
    WorthQueryConsumerInvalidationAdmissionStop, WorthQueryConsumerInvalidationAuthority,
    WorthQueryConsumerInvalidationDeltaStop, WorthQueryConsumerInvalidationDisposition,
};

pub enum WorthQueryConsumerGranularMaintenanceStop {
    Impact(crate::domain_installation::WorthQueryImpactAdmissionDenial),
    SharedOwner(
        crate::domain_installation::operation_execution::WorthQuerySharedConditionalDeliveryStop,
    ),
    Delta(WorthQueryConsumerInvalidationDeltaStop),
    Readmission(WorthQueryConsumerInvalidationAdmissionStop),
    ImpactMismatch,
}

/// Query-owned publication for one current shared consumer lease.
///
/// The shared owner may perform maintenance once for many leases, but this
/// product is minted only after the exact lease and retained invalidation
/// epoch are readmitted against the current workspace.
pub struct WorthQueryPublishedConsumerInvalidation {
    authority: WorthQueryConsumerInvalidationAuthority,
    maintenance_ordinal: u64,
    roles: Vec<crate::domain_installation::WorthQuerySemanticDependencyRole>,
    consequence_classes: Vec<crate::domain_installation::WorthQueryImpactClass>,
    disposition: WorthQueryConsumerInvalidationDisposition,
    shared_delivery_counters:
        crate::domain_installation::WorthQuerySharedProjectionDeliveryCounters,
    shared_drain_counters: crate::domain_installation::WorthQuerySharedProjectionDrainCounters,
    delivery_identity: String,
}

impl WorthQueryPublishedConsumerInvalidation {
    pub const fn authority(&self) -> &WorthQueryConsumerInvalidationAuthority {
        &self.authority
    }

    pub const fn maintenance_ordinal(&self) -> u64 {
        self.maintenance_ordinal
    }

    pub fn roles(&self) -> &[crate::domain_installation::WorthQuerySemanticDependencyRole] {
        &self.roles
    }

    pub fn consequence_classes(&self) -> &[crate::domain_installation::WorthQueryImpactClass] {
        &self.consequence_classes
    }

    pub const fn disposition(&self) -> WorthQueryConsumerInvalidationDisposition {
        self.disposition
    }

    pub const fn shared_delivery_counters(
        &self,
    ) -> crate::domain_installation::WorthQuerySharedProjectionDeliveryCounters {
        self.shared_delivery_counters
    }

    pub const fn shared_drain_counters(
        &self,
    ) -> crate::domain_installation::WorthQuerySharedProjectionDrainCounters {
        self.shared_drain_counters
    }

    pub fn delivery_identity(&self) -> &str {
        &self.delivery_identity
    }
}

impl<D: 'static, O: 'static, F: 'static, L: BasisOperationLane>
    crate::domain_installation::WorthQuerySharedLiveProjectionLease<D, O, F, L>
{
    pub fn maintain_granular_invalidation_for_consumer(
        &self,
        delivery: worth_runtime_bridge::facade::BridgeGranularInvalidationDelivery,
        workspace: &mut crate::runtime::WorthQueryWorkspace,
    ) -> Result<WorthQueryPublishedConsumerInvalidation, WorthQueryConsumerGranularMaintenanceStop>
    {
        let candidates = crate::domain_installation::select_invalidation_candidates(
            self.snapshot().semantic_aspect_dependency_closure(),
            delivery,
        )
        .map_err(WorthQueryConsumerGranularMaintenanceStop::Impact)?;
        let admitted = crate::domain_installation::admit_current_invalidation_impact(
            self.snapshot(),
            candidates,
        )
        .map_err(WorthQueryConsumerGranularMaintenanceStop::Impact)?;
        let correspondence = admitted.correspondence_receipt().clone();
        let roles = admitted.roles().to_vec();
        let consequence_classes = admitted.consequence_classes().to_vec();

        let shared = self
            .drain_conditional_owner_delivery(&correspondence, workspace)
            .map_err(WorthQueryConsumerGranularMaintenanceStop::SharedOwner)?;
        if !roles
            .iter()
            .all(|role| shared.impact().affected_roles().contains(role))
        {
            return Err(WorthQueryConsumerGranularMaintenanceStop::ImpactMismatch);
        }
        let shared_delivery_counters = shared.counters();
        let shared_drain_counters = shared.drain_counters();
        let delta = self
            .consumer_invalidation_delta(shared)
            .map_err(WorthQueryConsumerGranularMaintenanceStop::Delta)?;
        let admitted_consumer = self
            .admit_consumer_invalidation_delta(delta, workspace)
            .map_err(WorthQueryConsumerGranularMaintenanceStop::Readmission)?;
        if !admitted_consumer.remains_current(workspace) {
            return Err(WorthQueryConsumerGranularMaintenanceStop::ImpactMismatch);
        }
        let delta = admitted_consumer.into_delta();
        let authority = delta.authority().clone();
        let maintenance_ordinal = delta.maintenance_ordinal();
        let disposition = delta.disposition();
        let delivery_identity = crate::identity::hash_parts(&[
            "worth_query_published_consumer_invalidation_v1".into(),
            format!("owner:{}", authority.owner_identity().slot()),
            format!("lease:{}", authority.lease_identity().slot()),
            format!("maintenance:{maintenance_ordinal}"),
            format!("binding:{}", authority.binding_identity()),
        ]);
        Ok(WorthQueryPublishedConsumerInvalidation {
            authority,
            maintenance_ordinal,
            roles,
            consequence_classes,
            disposition,
            shared_delivery_counters,
            shared_drain_counters,
            delivery_identity,
        })
    }
}
