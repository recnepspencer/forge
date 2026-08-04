use super::super::identity::{LiveProgressBasis, LiveStartBasis, LiveSubscriptionDigest};
use super::plan::LiveQueryPlan;
use crate::basis::{BasisPreflightError, ExecutionPreflightBundle};
use crate::execution::{execute_preflight_bundle, ExecutionError};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LivePromotionError {
    UnsupportedPreflightRoute,
    UnsupportedLiveCollectionFamily,
    PlanDescriptorMismatch,
    BasisPreflight(BasisPreflightError),
    Execution(ExecutionError),
}

impl From<BasisPreflightError> for LivePromotionError {
    fn from(value: BasisPreflightError) -> Self {
        Self::BasisPreflight(value)
    }
}

impl From<ExecutionError> for LivePromotionError {
    fn from(value: ExecutionError) -> Self {
        Self::Execution(value)
    }
}

pub fn promote_preflight_bundle_to_live(
    preflight: &ExecutionPreflightBundle,
) -> Result<LiveQueryPlan, LivePromotionError> {
    let route = preflight.plan().query().route();
    match route {
        crate::planning::PlannedExecutionRoute::RuntimeSnapshotRead
        | crate::planning::PlannedExecutionRoute::RuntimeExpandedSnapshotRead => {}
        crate::planning::PlannedExecutionRoute::StoreSnapshotRead => {
            return Err(LivePromotionError::UnsupportedPreflightRoute);
        }
    }

    let descriptor = preflight.plan().live_promotion().clone();
    let plan_digest = preflight.plan().query().plan_digest();
    if descriptor.plan_digest() != plan_digest {
        return Err(LivePromotionError::PlanDescriptorMismatch);
    }
    if preflight.plan().collection().is_some_and(|collection| {
        collection.planning_context().result_family()
            != &crate::collection::CollectionResultFamily::OrdinaryCollection
    }) {
        return Err(LivePromotionError::UnsupportedLiveCollectionFamily);
    }

    let start_basis = LiveStartBasis::new(preflight.basis().clone());
    let mut parts = vec![
        format!("plan:{}", descriptor.plan_digest().as_str()),
        format!("family:{}", descriptor.family().as_str()),
        format!("basis:{}", preflight.basis().proof().digest().as_str()),
        format!(
            "incremental:{}",
            descriptor
                .incremental_eligibility()
                .maintenance_class()
                .as_str()
        ),
    ];
    parts.extend(descriptor.performance_report().digest_parts());
    if let Some(collection_digest) = descriptor.collection_digest() {
        parts.push(format!("collection:{}", collection_digest.as_str()));
    }
    let subscription_digest = LiveSubscriptionDigest::from_parts(&parts);
    let progress_basis = LiveProgressBasis::initial(&subscription_digest, &start_basis);
    let baseline_result_digest = execute_preflight_bundle(preflight)?
        .report()
        .result_digest()
        .as_str()
        .to_string();

    Ok(LiveQueryPlan {
        descriptor,
        start_basis,
        progress_basis,
        subscription_digest,
        baseline_result_digest,
    })
}
