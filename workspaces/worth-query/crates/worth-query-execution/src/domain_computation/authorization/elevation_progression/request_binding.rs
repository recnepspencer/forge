use std::sync::Arc;

use worth_foundational::facade::{AspectFieldLocator, AspectValue};
use worth_relational::facade::identity::{EntityId, KindId};

pub(in crate::domain_computation) struct WorthQueryElevationRequestBinding {
    pub(in crate::domain_computation) capability_identity: [u8; 32],
    pub(in crate::domain_computation) capability_authority_identity: Arc<str>,
    pub(in crate::domain_computation) requester: EntityId,
    pub(in crate::domain_computation) resource: EntityId,
    pub(in crate::domain_computation) grant: EntityId,
    pub(in crate::domain_computation) elevation_kind: KindId,
    pub(in crate::domain_computation) review_kind: KindId,
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
    pub(in crate::domain_computation) review_identity_field: AspectFieldLocator,
    pub(in crate::domain_computation) review_identity: AspectValue,
    pub(in crate::domain_computation) review_status_field: AspectFieldLocator,
    pub(in crate::domain_computation) review_required_status: AspectValue,
    pub(in crate::domain_computation) requester_relation: KindId,
    pub(in crate::domain_computation) grant_relation: KindId,
    pub(in crate::domain_computation) review_relation: KindId,
}
