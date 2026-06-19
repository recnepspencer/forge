use crate::{
    ForgeServerOperationAuthorityKind, ForgeServerOperationConcurrencyClass,
    ForgeServerProductOperationSurfaceDenial, ForgeServerProductOperationSurfaceDenialCode,
};

use super::ForgeServerLoweredProductOperationPlan;

#[derive(Clone, Debug)]
pub struct ForgeServerProductSchedulerAdmission {
    scheduler_lane: String,
    concurrency_class: ForgeServerOperationConcurrencyClass,
    canonical_digest: String,
}

impl ForgeServerProductSchedulerAdmission {
    fn new(
        scheduler_lane: String,
        concurrency_class: ForgeServerOperationConcurrencyClass,
        canonical_digest: String,
    ) -> Self {
        Self {
            scheduler_lane,
            concurrency_class,
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

#[derive(Clone, Debug)]
pub struct ForgeServerScheduledProductOperation {
    plan: ForgeServerLoweredProductOperationPlan,
    scheduler_admission: ForgeServerProductSchedulerAdmission,
    canonical_digest: String,
}

impl ForgeServerScheduledProductOperation {
    pub(crate) fn admit(
        plan: ForgeServerLoweredProductOperationPlan,
    ) -> Result<Self, ForgeServerProductOperationSurfaceDenial> {
        let scheduler_lane = derive_scheduler_lane(&plan)?;
        validate_product_scheduler_admission(&plan, &scheduler_lane)?;
        let canonical_digest = format!(
            "forge-server-scheduled-product-operation-v1|plan={}|lane={}|concurrency={}",
            plan.canonical_digest(),
            scheduler_lane,
            concurrency_label(plan.concurrency_class()),
        );
        let scheduler_admission = ForgeServerProductSchedulerAdmission::new(
            scheduler_lane,
            plan.concurrency_class(),
            canonical_digest.clone(),
        );
        Ok(Self {
            plan,
            scheduler_admission,
            canonical_digest,
        })
    }

    pub fn plan(&self) -> &ForgeServerLoweredProductOperationPlan {
        &self.plan
    }

    pub fn scheduler_admission(&self) -> &ForgeServerProductSchedulerAdmission {
        &self.scheduler_admission
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}

fn derive_scheduler_lane(
    plan: &ForgeServerLoweredProductOperationPlan,
) -> Result<String, ForgeServerProductOperationSurfaceDenial> {
    let authority_metadata = plan.operation_admission().authority_metadata();
    match plan
        .operation_admission()
        .authority_footprint()
        .authority_kind()
    {
        ForgeServerOperationAuthorityKind::SharedReadOnly
        | ForgeServerOperationAuthorityKind::DiagnosticsOnly => Ok("shared-read".to_string()),
        ForgeServerOperationAuthorityKind::ProductDraftMutation => {
            let (product_session_identity, draft_scope) =
                authority_metadata.product_draft_scope().ok_or_else(|| {
                    invalid_product_scheduler_contract(
                        "product draft mutations require scheduler-visible draft scope metadata",
                    )
                })?;
            Ok(format!(
                "product-draft:{product_session_identity}:{draft_scope}"
            ))
        }
        ForgeServerOperationAuthorityKind::ProductSessionCoordination => {
            let (target, coordination_lane) = authority_metadata
                .product_session_coordination_target()
                .ok_or_else(|| {
                    invalid_product_scheduler_contract(
                        "product session coordination requires scheduler-visible coordination metadata",
                    )
                })?;
            match target {
                crate::ForgeServerProductSessionCoordinationTarget::ExistingSession {
                    product_session_identity,
                } => Ok(format!(
                    "product-session:{product_session_identity}:{coordination_lane}"
                )),
                crate::ForgeServerProductSessionCoordinationTarget::SessionCreation => Ok(format!(
                    "product-session-create:{}:{coordination_lane}",
                    plan.operation_admission()
                        .authority_footprint()
                        .scope()
                        .canonical_digest()
                )),
            }
        }
        unsupported => Err(invalid_product_scheduler_contract(&format!(
            "product adapter scheduling does not admit authority kind `{}`",
            unsupported.as_str()
        ))),
    }
}

fn validate_product_scheduler_admission(
    plan: &ForgeServerLoweredProductOperationPlan,
    scheduler_lane: &str,
) -> Result<(), ForgeServerProductOperationSurfaceDenial> {
    let concurrency_class = plan.concurrency_class();
    let expected = if scheduler_lane == "shared-read" {
        ForgeServerOperationConcurrencyClass::ConcurrentSharedRead
    } else {
        ForgeServerOperationConcurrencyClass::SerializeDeterministically
    };
    if concurrency_class != expected {
        return Err(invalid_product_scheduler_contract(&format!(
            "product scheduler lane `{scheduler_lane}` requires `{}` concurrency, got `{}`",
            concurrency_label(expected),
            concurrency_label(concurrency_class),
        )));
    }
    Ok(())
}

fn concurrency_label(concurrency_class: ForgeServerOperationConcurrencyClass) -> &'static str {
    match concurrency_class {
        ForgeServerOperationConcurrencyClass::ConcurrentSharedRead => "shared-read",
        ForgeServerOperationConcurrencyClass::SerializeDeterministically => {
            "serialize-deterministically"
        }
    }
}

fn invalid_product_scheduler_contract(detail: &str) -> ForgeServerProductOperationSurfaceDenial {
    ForgeServerProductOperationSurfaceDenial::new(
        ForgeServerProductOperationSurfaceDenialCode::InvalidDeclaration,
        detail.to_string(),
    )
}
