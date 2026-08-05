use std::sync::Arc;

use worth_foundational::facade::{AspectFieldLocator, AspectValue};
use worth_query_installation::facade::ApplicationOperationProgramTarget;
use worth_relational::facade::identity::KindId;

use super::WorthQueryElevationUpperBound;
use crate::domain_computation::authorization::WorthQueryRetainedCapabilitySupport;

#[derive(Debug)]
pub(in crate::domain_computation) struct WorthQueryElevationRequestBinding {
    pub(in crate::domain_computation) runtime_authority:
        crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity,
    pub(in crate::domain_computation) branch: worth_relational::facade::history::BranchId,
    pub(in crate::domain_computation) capability_identity: [u8; 32],
    pub(in crate::domain_computation) capability_authority_identity: Arc<str>,
    pub(in crate::domain_computation) upper_bound: WorthQueryElevationUpperBound,
    pub(in crate::domain_computation) supporting: WorthQueryRetainedCapabilitySupport,
    pub(in crate::domain_computation) elevation_kind: KindId,
    pub(in crate::domain_computation) review_kind: KindId,
    pub(in crate::domain_computation) elevation_key: String,
    pub(in crate::domain_computation) elevation_identity_field: AspectFieldLocator,
    pub(in crate::domain_computation) elevation_identity: AspectValue,
    pub(in crate::domain_computation) reason_field: AspectFieldLocator,
    pub(in crate::domain_computation) reason: AspectValue,
    pub(in crate::domain_computation) status_field: AspectFieldLocator,
    pub(in crate::domain_computation) requested_status: AspectValue,
    pub(in crate::domain_computation) not_before_field: AspectFieldLocator,
    pub(in crate::domain_computation) issued_at: AspectValue,
    pub(in crate::domain_computation) not_after_field: AspectFieldLocator,
    pub(in crate::domain_computation) expires_at: AspectValue,
    pub(in crate::domain_computation) review_key: String,
    pub(in crate::domain_computation) review_identity_field: AspectFieldLocator,
    pub(in crate::domain_computation) review_identity: AspectValue,
    pub(in crate::domain_computation) review_type_field: AspectFieldLocator,
    pub(in crate::domain_computation) review_type: AspectValue,
    pub(in crate::domain_computation) review_status_field: AspectFieldLocator,
    pub(in crate::domain_computation) review_required_status: AspectValue,
    pub(in crate::domain_computation) requester_relation: KindId,
    pub(in crate::domain_computation) grant_relation: KindId,
    pub(in crate::domain_computation) resource_relation: Option<KindId>,
    pub(in crate::domain_computation) review_relation: KindId,
    pub(in crate::domain_computation) review_scope_relation: KindId,
    pub(in crate::domain_computation) required_program_targets:
        Vec<ApplicationOperationProgramTarget>,
    pub(in crate::domain_computation) lifecycle_effect:
        Option<worth_query_declaration::lifecycle_effect_derivation_authority::DerivedApplicationCapabilityLifecycleEffect>,
}

impl WorthQueryElevationRequestBinding {
    pub(in crate::domain_computation) const fn requester(
        &self,
    ) -> worth_relational::facade::identity::EntityId {
        self.upper_bound.requester()
    }

    pub(in crate::domain_computation) const fn resource(
        &self,
    ) -> worth_relational::facade::identity::EntityId {
        self.upper_bound.resource()
    }

    pub(in crate::domain_computation) const fn grant(
        &self,
    ) -> worth_relational::facade::identity::EntityId {
        self.upper_bound.grant()
    }
}
