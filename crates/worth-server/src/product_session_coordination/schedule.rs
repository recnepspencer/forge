use crate::{
    WorthServerOperationAuthorityKind, WorthServerOperationConcurrencyClass,
    WorthServerProductSessionCoordinationTarget,
};

use super::WorthServerLoweredProductSessionCoordinationPlan;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerProductSessionSchedulerAdmission {
    scheduler_lane: String,
    concurrency_class: WorthServerOperationConcurrencyClass,
    canonical_digest: String,
}

impl WorthServerProductSessionSchedulerAdmission {
    pub(crate) fn from_plan(plan: &WorthServerLoweredProductSessionCoordinationPlan) -> Self {
        let scheduler_lane = scheduler_lane(plan);
        let canonical_digest = format!(
            "worth-server-product-session-scheduler-admission-v1|plan={}|lane={}|concurrency={}",
            plan.canonical_digest(),
            scheduler_lane,
            concurrency_label(plan.concurrency_class()),
        );
        Self {
            scheduler_lane,
            concurrency_class: plan.concurrency_class(),
            canonical_digest,
        }
    }

    pub fn scheduler_lane(&self) -> &str {
        &self.scheduler_lane
    }

    pub fn concurrency_class(&self) -> WorthServerOperationConcurrencyClass {
        self.concurrency_class.clone()
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}

fn scheduler_lane(plan: &WorthServerLoweredProductSessionCoordinationPlan) -> String {
    let authority = plan
        .operation_admission()
        .authority_footprint()
        .authority_kind();
    assert_eq!(
        authority,
        WorthServerOperationAuthorityKind::ProductSessionCoordination,
        "product session coordination plans must retain session coordination authority",
    );
    let (target, coordination_lane) = plan
        .operation_admission()
        .authority_metadata()
        .product_session_coordination_target()
        .expect("product session coordination plans must retain coordination metadata");
    match target {
        WorthServerProductSessionCoordinationTarget::ExistingSession {
            product_session_identity,
        } => format!("product-session:{product_session_identity}:{coordination_lane}"),
        WorthServerProductSessionCoordinationTarget::SessionCreation => format!(
            "product-session-create:{}:{coordination_lane}",
            plan.operation_admission()
                .authority_footprint()
                .scope()
                .canonical_digest()
        ),
    }
}

fn concurrency_label(concurrency_class: WorthServerOperationConcurrencyClass) -> &'static str {
    match concurrency_class {
        WorthServerOperationConcurrencyClass::ConcurrentSharedRead => "shared-read",
        WorthServerOperationConcurrencyClass::SerializeDeterministically => {
            "serialize-deterministically"
        }
    }
}
