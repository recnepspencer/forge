use crate::{
    ForgeServerOperationAuthorityKind, ForgeServerOperationConcurrencyClass,
    ForgeServerProductSessionCoordinationTarget,
};

use super::ForgeServerLoweredProductSessionCoordinationPlan;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerProductSessionSchedulerAdmission {
    scheduler_lane: String,
    concurrency_class: ForgeServerOperationConcurrencyClass,
    canonical_digest: String,
}

impl ForgeServerProductSessionSchedulerAdmission {
    pub(crate) fn from_plan(plan: &ForgeServerLoweredProductSessionCoordinationPlan) -> Self {
        let scheduler_lane = scheduler_lane(plan);
        let canonical_digest = format!(
            "forge-server-product-session-scheduler-admission-v1|plan={}|lane={}|concurrency={}",
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

    pub fn concurrency_class(&self) -> ForgeServerOperationConcurrencyClass {
        self.concurrency_class.clone()
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}

fn scheduler_lane(plan: &ForgeServerLoweredProductSessionCoordinationPlan) -> String {
    let authority = plan
        .operation_admission()
        .authority_footprint()
        .authority_kind();
    assert_eq!(
        authority,
        ForgeServerOperationAuthorityKind::ProductSessionCoordination,
        "product session coordination plans must retain session coordination authority",
    );
    let (target, coordination_lane) = plan
        .operation_admission()
        .authority_metadata()
        .product_session_coordination_target()
        .expect("product session coordination plans must retain coordination metadata");
    match target {
        ForgeServerProductSessionCoordinationTarget::ExistingSession {
            product_session_identity,
        } => format!("product-session:{product_session_identity}:{coordination_lane}"),
        ForgeServerProductSessionCoordinationTarget::SessionCreation => format!(
            "product-session-create:{}:{coordination_lane}",
            plan.operation_admission()
                .authority_footprint()
                .scope()
                .canonical_digest()
        ),
    }
}

fn concurrency_label(concurrency_class: ForgeServerOperationConcurrencyClass) -> &'static str {
    match concurrency_class {
        ForgeServerOperationConcurrencyClass::ConcurrentSharedRead => "shared-read",
        ForgeServerOperationConcurrencyClass::SerializeDeterministically => {
            "serialize-deterministically"
        }
    }
}
