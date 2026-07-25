use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::{
    WorthQueryBoundCapabilityGeneration, WorthQueryOperationCollectionContract,
    WorthQueryOperationGroupingContract, WorthQuerySettledDomainProjection,
};
use worth_proof::TransitionOutcome;

use super::super::row_indexing::collection_rows;
use super::super::{
    WorthQueryCollectionCapabilityCounters, WorthQueryCollectionCapabilityDenial,
    WorthQueryCollectionCapabilityDenialKind,
};
use super::{WorthQueryBoundCollection, WorthQueryCollectionCapabilityOutcome};

impl<D, O, F, L: BasisOperationLane> WorthQuerySettledDomainProjection<D, O, F, L> {
    pub fn into_bound_collection(self) -> WorthQueryCollectionCapabilityOutcome<D, O, F, L> {
        let mut counters = WorthQueryCollectionCapabilityCounters::default();
        counters.current_generation_checks += 1;
        if !self.bound_operation().installation_is_current() {
            return TransitionOutcome::Stale(super::WorthQueryCollectionCapabilityStop::new(
                self,
                WorthQueryCollectionCapabilityDenial::new(
                    WorthQueryCollectionCapabilityDenialKind::StaleInstallationGeneration,
                    counters,
                ),
            ));
        }
        let prepared = match prepare_collection_binding(&self, &mut counters) {
            Ok(prepared) => prepared,
            Err(kind) => return capability_denied(self, kind, counters),
        };
        TransitionOutcome::Success(WorthQueryBoundCollection {
            basis_identity: self.consumer_contract().basis_identity().to_string(),
            source_identity: self.identity().to_string(),
            binding_identity: self.bound_operation().binding_identity().to_string(),
            result_shape_identity: prepared.result_shape_identity,
            collection_delivery_contract_identity: prepared.delivery_contract_identity,
            projection: self,
            rows: prepared.rows,
            capability_identity: prepared.capability_identity,
            capability_generation: prepared.capability_generation,
            ordering_identity: prepared.ordering_identity,
            window_policy: prepared.window_policy,
            continuation_posture: prepared.continuation_posture,
            maintenance_index: prepared.maintenance_index,
            counters,
        })
    }
}

pub(crate) struct WorthQueryPreparedCollectionBinding {
    pub(crate) capability_identity: u64,
    pub(crate) capability_generation: WorthQueryBoundCapabilityGeneration,
    pub(crate) rows: Vec<crate::domain_installation::WorthQueryCollectionRowHandle>,
    pub(crate) maintenance_index:
        crate::domain_installation::collection_delivery::WorthQueryCollectionMaintenanceIndex,
    pub(crate) window_policy: crate::domain_installation::WorthQueryOperationWindowPolicy,
    pub(crate) continuation_posture:
        crate::domain_installation::WorthQueryOperationContinuationPosture,
    pub(crate) result_shape_identity: String,
    pub(crate) ordering_identity: String,
    pub(crate) delivery_contract_identity: String,
}

pub(crate) fn prepare_collection_binding<D, O, F, L: BasisOperationLane>(
    projection: &WorthQuerySettledDomainProjection<D, O, F, L>,
    counters: &mut WorthQueryCollectionCapabilityCounters,
) -> Result<WorthQueryPreparedCollectionBinding, WorthQueryCollectionCapabilityDenialKind> {
    let (window_policy, continuation_posture) = collection_posture(projection, counters)?;
    counters.native_layout_checks += 1;
    let native_layout = projection
        .native_access_layout()
        .ok_or(WorthQueryCollectionCapabilityDenialKind::NativeAccessNotBound)?;
    let maintenance_request = projection
        .collection_declarative_request()
        .cloned()
        .ok_or(WorthQueryCollectionCapabilityDenialKind::NativeAccessNotBound)?;
    let capability_identity = projection.bound_operation().capability_identity();
    let capability_generation = WorthQueryBoundCapabilityGeneration::mint();
    let rows = collection_rows(
        projection.authority(),
        capability_identity,
        capability_generation,
        counters,
    )?;
    let canonical = projection.consumer_contract().canonical_projection();
    let ordering_parts = canonical
        .query()
        .ordering()
        .iter()
        .map(|entry| entry.digest_part())
        .collect::<Vec<_>>();
    counters.ordering_terms_retained = ordering_parts.len();
    let maintenance_index =
        crate::domain_installation::collection_delivery::WorthQueryCollectionMaintenanceIndex::build(
            crate::domain_installation::collection_delivery::WorthQueryCollectionMaintenanceInputs {
                request: maintenance_request,
                window_policy,
                continuation_posture,
                delivery_supported: collection_delivery_supported(projection),
                entities: projection.collection_execution_rows().to_vec(),
                handles: &rows,
                native_keys: native_layout.selected_keys(),
            },
            counters,
        );
    Ok(WorthQueryPreparedCollectionBinding {
        capability_identity,
        capability_generation,
        rows,
        maintenance_index,
        window_policy,
        continuation_posture,
        result_shape_identity: canonical.result_shape().digest().as_str().to_string(),
        ordering_identity: crate::identity::hash_parts(&ordering_parts),
        delivery_contract_identity: projection
            .collection_delivery_contract_identity()
            .expect("native layout was validated above"),
    })
}

fn collection_posture<D, O, F, L: BasisOperationLane>(
    projection: &WorthQuerySettledDomainProjection<D, O, F, L>,
    counters: &mut WorthQueryCollectionCapabilityCounters,
) -> Result<
    (
        crate::domain_installation::WorthQueryOperationWindowPolicy,
        crate::domain_installation::WorthQueryOperationContinuationPosture,
    ),
    WorthQueryCollectionCapabilityDenialKind,
> {
    counters.collection_contract_checks += 1;
    match projection.consumer_contract().collection() {
        WorthQueryOperationCollectionContract::NotCollection => {
            Err(WorthQueryCollectionCapabilityDenialKind::NotCollection)
        }
        WorthQueryOperationCollectionContract::Collection {
            grouping: WorthQueryOperationGroupingContract::Grouped { .. },
            ..
        } => Err(WorthQueryCollectionCapabilityDenialKind::UnsupportedGrouping),
        WorthQueryOperationCollectionContract::Collection {
            window,
            continuation,
            ..
        } => Ok((*window, *continuation)),
    }
}

fn collection_delivery_supported<D, O, F, L: BasisOperationLane>(
    projection: &WorthQuerySettledDomainProjection<D, O, F, L>,
) -> bool {
    projection.consumer_contract().support_posture(
        crate::domain_installation::WorthQueryConsumerSupportDimension::CollectionDelivery,
    ) == crate::domain_installation::WorthQueryConsumerSupportPosture::Supported
}

fn capability_denied<D, O, F, L: BasisOperationLane>(
    projection: WorthQuerySettledDomainProjection<D, O, F, L>,
    kind: WorthQueryCollectionCapabilityDenialKind,
    counters: WorthQueryCollectionCapabilityCounters,
) -> WorthQueryCollectionCapabilityOutcome<D, O, F, L> {
    TransitionOutcome::Denied(super::WorthQueryCollectionCapabilityStop::new(
        projection,
        WorthQueryCollectionCapabilityDenial::new(kind, counters),
    ))
}
