use std::collections::BTreeMap;
use std::sync::Arc;

use worth_runtime_bridge::facade::{
    BridgeCorrespondenceBasis, BridgeDeliveredCorrespondenceChange,
    BridgeGranularInvalidationDelivery, BridgeSemanticDependencyCandidate, BridgeSemanticLocality,
    RelationalBridgeRecordIdentityParts, TruthBranchIdentity, TruthCommitIdentity,
    TruthPatchIdentity, TruthSnapshotIdentity,
};

use super::{
    WorthQueryImpactAdmissionDenial, WorthQueryImpactAdmissionDenialKind, WorthQueryImpactCounters,
};

/// Query-private lookup identity for converging deliveries that may arrive on
/// different runtime workers. This key selects a convergence bucket only; it
/// is not authority to admit or publish Query work.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct WorthQueryGranularDeliveryKey {
    commit: TruthCommitIdentity,
    patch: TruthPatchIdentity,
    snapshot: TruthSnapshotIdentity,
    branch: TruthBranchIdentity,
    dependency_ordinal: usize,
    graph_participation: Arc<str>,
    graph_adapter: Arc<str>,
    source_node: Arc<str>,
    source_stage: Option<Arc<str>>,
    source_record: Option<RelationalBridgeRecordIdentityParts>,
    locality: BridgeSemanticLocality,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct WorthQueryGranularDeliveryFingerprint {
    basis: BridgeCorrespondenceBasis,
    dependency: BridgeSemanticDependencyCandidate,
    changes: Vec<BridgeDeliveredCorrespondenceChange>,
}

struct ConvergedDelivery {
    fingerprint: WorthQueryGranularDeliveryFingerprint,
    delivery: BridgeGranularInvalidationDelivery,
}

pub(super) struct WorthQueryGranularDeliveryConvergence {
    pub(super) deliveries: Vec<BridgeGranularInvalidationDelivery>,
    pub(super) duplicate_delivery_count: usize,
    pub(super) performed_promotion_count: usize,
}

pub(super) fn converge_granular_deliveries(
    deliveries: impl IntoIterator<Item = BridgeGranularInvalidationDelivery>,
) -> Result<WorthQueryGranularDeliveryConvergence, WorthQueryImpactAdmissionDenial> {
    let mut converged = BTreeMap::<WorthQueryGranularDeliveryKey, ConvergedDelivery>::new();
    let mut duplicate_count = 0;
    let mut performed_promotion_count = 0;
    for delivery in deliveries {
        let key = WorthQueryGranularDeliveryKey::from_delivery(&delivery);
        let fingerprint = WorthQueryGranularDeliveryFingerprint::from_delivery(&delivery);
        match converged.entry(key) {
            std::collections::btree_map::Entry::Vacant(entry) => {
                entry.insert(ConvergedDelivery {
                    fingerprint,
                    delivery,
                });
            }
            std::collections::btree_map::Entry::Occupied(mut entry) => {
                if entry.get().fingerprint != fingerprint {
                    return Err(WorthQueryImpactAdmissionDenial::new(
                        WorthQueryImpactAdmissionDenialKind::ConflictingDuplicateDelivery,
                        WorthQueryImpactCounters {
                            delivery_identity_checks: 1,
                            ..Default::default()
                        },
                    ));
                }
                duplicate_count += 1;
                if entry.get().delivery.performed_signal().is_none()
                    && delivery.performed_signal().is_some()
                {
                    entry.get_mut().delivery = delivery;
                    performed_promotion_count += 1;
                }
            }
        }
    }
    Ok(WorthQueryGranularDeliveryConvergence {
        deliveries: converged
            .into_values()
            .map(|converged| converged.delivery)
            .collect(),
        duplicate_delivery_count: duplicate_count,
        performed_promotion_count,
    })
}

impl WorthQueryGranularDeliveryKey {
    fn from_delivery(delivery: &BridgeGranularInvalidationDelivery) -> Self {
        let change_set = delivery.truth().change_set();
        let dependency = change_set.dependency();
        Self {
            commit: change_set.commit_identity().clone(),
            patch: change_set.patch_identity().clone(),
            snapshot: change_set.snapshot_identity().clone(),
            branch: change_set.branch_identity().clone(),
            dependency_ordinal: dependency.dependency_ordinal(),
            graph_participation: Arc::from(dependency.graph_participation_identity()),
            graph_adapter: Arc::from(dependency.graph_adapter_identity()),
            source_node: Arc::from(dependency.source_node_identity()),
            source_stage: dependency.source_stage_identity().map(Arc::from),
            source_record: dependency.source_record_identity(),
            locality: dependency.locality().clone(),
        }
    }
}

impl WorthQueryGranularDeliveryFingerprint {
    fn from_delivery(delivery: &BridgeGranularInvalidationDelivery) -> Self {
        let change_set = delivery.truth().change_set();
        Self {
            basis: change_set.basis().clone(),
            dependency: change_set.dependency().clone(),
            changes: change_set.changes().to_vec(),
        }
    }
}
