use std::collections::BTreeSet;
use std::sync::Arc;

use crate::domain_installation::{
    WorthQuerySharedProjectionDelivery, WorthQuerySharedProjectionDeliveryCounters,
    WorthQuerySharedProjectionDrainCounters, WorthQuerySharedProjectionLeaseReadmission,
};

use super::delivery::WorthQuerySharedProjectionEpoch;
use super::registry::WorthQuerySharedProjectionOwner;
use super::{WorthQuerySharedExecutionOwnerIdentity, WorthQuerySharedProjectionLeaseIdentity};

pub(crate) struct WorthQuerySharedConditionalDeliveryCompletion {
    owner_identity: WorthQuerySharedExecutionOwnerIdentity,
    lease_identity: WorthQuerySharedProjectionLeaseIdentity,
    owner: WorthQuerySharedProjectionOwner,
    classified: crate::domain_installation::WorthQueryClassifiedOwnerDeliveryCompletion,
    counters: WorthQuerySharedProjectionDrainCounters,
}

impl WorthQuerySharedConditionalDeliveryCompletion {
    pub(crate) fn new(
        owner_identity: WorthQuerySharedExecutionOwnerIdentity,
        lease_identity: WorthQuerySharedProjectionLeaseIdentity,
        owner: WorthQuerySharedProjectionOwner,
        classified: crate::domain_installation::WorthQueryClassifiedOwnerDeliveryCompletion,
        counters: WorthQuerySharedProjectionDrainCounters,
    ) -> Self {
        Self {
            owner_identity,
            lease_identity,
            owner,
            classified,
            counters,
        }
    }
}

impl super::super::WorthQueryRuntime {
    pub(crate) fn take_shared_owner_for_conditional_delivery(
        &mut self,
        readmission: WorthQuerySharedProjectionLeaseReadmission<'_>,
        counters: &mut WorthQuerySharedProjectionDrainCounters,
    ) -> Result<
        (
            WorthQuerySharedExecutionOwnerIdentity,
            WorthQuerySharedProjectionOwner,
        ),
        super::super::WorthQueryRuntimeError,
    > {
        let owner_identity = readmission.owner;
        counters.runtime_affinity_checks = 1;
        if owner_identity.runtime_authority() != self.authority_identity.as_u64() {
            return Err(delivery_error(
                "shared execution owner belongs to a foreign runtime",
            ));
        }
        counters.owner_index_lookups = 1;
        let Some(owner) = self.shared_projection_owners.owners.remove(&owner_identity) else {
            return Err(delivery_error("shared execution owner is not active"));
        };
        counters.lease_index_lookups = 1;
        counters.sharing_readmission_checks = 1;
        let exact = owner.leases.get(&readmission.lease).is_some_and(|record| {
            record.source_identity == readmission.source_identity
                && record.affinity.binding_identity == readmission.binding_identity
                && record.affinity.capability_identity == readmission.capability_identity
                && owner.admission.readmits_lease(
                    readmission.source_identity,
                    &record.affinity,
                    readmission.closure,
                )
        });
        counters.retained_epoch_checks = 1;
        counters.retained_epoch_pending_lookups = usize::from(owner.epoch.is_some());
        let epoch_ready = owner
            .epoch
            .as_ref()
            .is_none_or(|epoch| epoch.pending.is_empty());
        if !exact || !epoch_ready {
            self.shared_projection_owners
                .owners
                .insert(owner_identity, owner);
            return Err(delivery_error(if exact {
                "every indexed lease must observe the current epoch before owner delivery"
            } else {
                "shared projection lease no longer readmits its exact source and closure"
            }));
        }
        Ok((owner_identity, owner))
    }

    pub(crate) fn restore_shared_owner_after_conditional_stop(
        &mut self,
        owner_identity: WorthQuerySharedExecutionOwnerIdentity,
        owner: WorthQuerySharedProjectionOwner,
    ) {
        let replaced = self
            .shared_projection_owners
            .owners
            .insert(owner_identity, owner);
        debug_assert!(replaced.is_none());
    }

    pub(crate) fn finish_shared_conditional_delivery(
        &mut self,
        completion: WorthQuerySharedConditionalDeliveryCompletion,
    ) -> Result<WorthQuerySharedProjectionDelivery, super::super::WorthQueryRuntimeError> {
        let WorthQuerySharedConditionalDeliveryCompletion {
            owner_identity,
            lease_identity,
            mut owner,
            classified,
            mut counters,
        } = completion;
        counters.lease_index_lookups += 1;
        let Some(lease_evidence) =
            owner
                .leases
                .get(&lease_identity)
                .map(|record| SharedConditionalLeaseEvidence {
                    source_identity: record.source_identity.clone(),
                    affinity: record.affinity.clone(),
                })
        else {
            self.restore_shared_owner_after_conditional_stop(owner_identity, owner);
            return Err(delivery_error("shared projection lease is not active"));
        };
        let view = SharedConditionalEpochCompilation {
            owner_identity,
            lease_identity,
            lease_evidence,
            classified,
            counters,
        }
        .compile(&mut owner);
        self.restore_shared_owner_after_conditional_stop(owner_identity, owner);
        Ok(view)
    }
}

struct SharedConditionalLeaseEvidence {
    source_identity: String,
    affinity: crate::domain_installation::WorthQueryOperationAuthorityBasis,
}

struct SharedConditionalEpochCompilation {
    owner_identity: WorthQuerySharedExecutionOwnerIdentity,
    lease_identity: WorthQuerySharedProjectionLeaseIdentity,
    lease_evidence: SharedConditionalLeaseEvidence,
    classified: crate::domain_installation::WorthQueryClassifiedOwnerDeliveryCompletion,
    counters: WorthQuerySharedProjectionDrainCounters,
}

impl SharedConditionalEpochCompilation {
    fn compile(
        self,
        owner: &mut WorthQuerySharedProjectionOwner,
    ) -> WorthQuerySharedProjectionDelivery {
        let (delivery, impact, conditional, owner_delivery_receipt, work) =
            self.classified.into_parts();
        let delivery = Arc::new(delivery);
        let maintenance_passes = delivery
            .batches()
            .iter()
            .filter(|batch| batch.maintenance_work().is_some())
            .count();
        let semantic_delivery = usize::from(
            !delivery.is_empty()
                && impact.class()
                    != crate::domain_installation::WorthQueryImpactClass::UnaffectedOrSuppressed,
        );
        owner.next_maintenance_ordinal += 1;
        let lease_count = owner.leases.len();
        let mut pending = owner.leases.keys().copied().collect::<BTreeSet<_>>();
        pending.remove(&self.lease_identity);
        let invalidation_seed = Arc::new(
            crate::domain_installation::WorthQuerySharedInvalidationSeed::compile(
                &delivery,
                lease_count,
            ),
        );
        owner.epoch = Some(WorthQuerySharedProjectionEpoch {
            ordinal: owner.next_maintenance_ordinal,
            delivery,
            impact,
            impact_closure: Arc::clone(&owner.closure),
            conditional_provenance: Arc::clone(&owner.conditional_provenance),
            conditional_decision: Some(conditional),
            owner_delivery_receipt: Some(owner_delivery_receipt),
            invalidation_seed,
            admission: Arc::clone(&owner.admission),
            pending,
            counters: WorthQuerySharedProjectionDeliveryCounters {
                owner_drain_calls: 1,
                underlying_maintenance_passes: maintenance_passes,
                lease_index_visits: lease_count,
                fanout_targets: lease_count,
                this_lease_view: 1,
                this_lease_semantic_delivery: semantic_delivery,
                conditional_compute_contacts: work.conditional_compute_contacts(),
                impact_classifications: work.impact_classifications(),
                unrelated_owner_scans: 0,
                unrelated_lease_scans: 0,
            },
        });
        let mut counters = self.counters;
        counters.epoch_compilations = 1;
        owner
            .epoch
            .as_ref()
            .expect("classified conditional epoch is retained")
            .view(
                self.owner_identity,
                self.lease_identity,
                self.lease_evidence.source_identity,
                self.lease_evidence.affinity,
                counters,
            )
    }
}

impl super::super::WorthQueryWorkspace {
    pub(crate) fn admit_shared_conditional_delivery_capability(
        &mut self,
        capability: &Arc<super::super::WorthQueryManagedLiveWorkspaceCapability>,
        owner: WorthQuerySharedExecutionOwnerIdentity,
    ) -> Result<usize, super::super::WorthQueryRuntimeError> {
        self.runtime
            .admit_managed_live_capability(capability, "shared-projection-owner")?;
        self.runtime
            .reap_abandoned_shared_projection_leases_for_owner(owner)
    }

    pub(crate) fn take_shared_owner_for_conditional_delivery(
        &mut self,
        readmission: WorthQuerySharedProjectionLeaseReadmission<'_>,
        counters: &mut WorthQuerySharedProjectionDrainCounters,
    ) -> Result<
        (
            WorthQuerySharedExecutionOwnerIdentity,
            WorthQuerySharedProjectionOwner,
        ),
        super::super::WorthQueryRuntimeError,
    > {
        self.runtime
            .take_shared_owner_for_conditional_delivery(readmission, counters)
    }

    pub(crate) fn restore_shared_owner_after_conditional_stop(
        &mut self,
        owner_identity: WorthQuerySharedExecutionOwnerIdentity,
        owner: WorthQuerySharedProjectionOwner,
    ) {
        self.runtime
            .restore_shared_owner_after_conditional_stop(owner_identity, owner);
    }

    pub(crate) fn finish_shared_conditional_delivery(
        &mut self,
        completion: WorthQuerySharedConditionalDeliveryCompletion,
    ) -> Result<WorthQuerySharedProjectionDelivery, super::super::WorthQueryRuntimeError> {
        self.runtime.finish_shared_conditional_delivery(completion)
    }
}

fn delivery_error(detail: &str) -> super::super::WorthQueryRuntimeError {
    super::super::WorthQueryRuntimeError::LiveSubscriptionInstallation {
        view_name: "shared-projection-owner".into(),
        stage: "shared-projection-delivery",
        message: detail.into(),
    }
}
