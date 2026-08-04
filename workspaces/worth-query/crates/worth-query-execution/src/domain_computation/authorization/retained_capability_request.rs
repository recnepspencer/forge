use std::collections::BTreeMap;
use std::sync::Arc;

use worth_foundational::facade::AspectValue;
use worth_query_declaration::facade::application_capability::{
    ApplicationCapabilityRelationBinding, ApplicationCapabilityRequestProjection,
};

use super::capability_request_resolution::{
    WorthQueryCapabilityContextKey, WorthQueryResolvedCapabilityRequest,
};

#[derive(Clone)]
pub(in crate::domain_computation) struct WorthQueryRetainedCapabilityRequest {
    pub(super) capability_identity: [u8; 32],
    pub(super) principal: worth_relational::facade::identity::EntityId,
    pub(super) resource: worth_relational::facade::identity::EntityId,
    pub(super) resource_entity: Arc<str>,
    pub(super) action: AspectValue,
    pub(super) purpose: AspectValue,
    pub(super) related_relation: Option<ApplicationCapabilityRelationBinding>,
    pub(super) related: Option<worth_relational::facade::identity::EntityId>,
    pub(super) field: Option<AspectValue>,
    pub(super) amount: Option<AspectValue>,
    pub(super) cardinality: u32,
    pub(super) context_name: Arc<str>,
    pub(super) context_type: Arc<str>,
    pub(super) context:
        BTreeMap<WorthQueryCapabilityContextKey, worth_relational::facade::identity::EntityId>,
}

impl WorthQueryRetainedCapabilityRequest {
    pub(super) fn for_principal(
        &self,
        principal: worth_relational::facade::identity::EntityId,
    ) -> Self {
        let mut request = self.clone();
        request.principal = principal;
        request
    }

    pub(super) fn capture<Schema, Scope, Context>(
        capability_identity: [u8; 32],
        principal: worth_relational::facade::identity::EntityId,
        projection: &ApplicationCapabilityRequestProjection<Schema, Scope, Context>,
        resolved: &WorthQueryResolvedCapabilityRequest<Schema, Scope>,
    ) -> Self {
        Self {
            capability_identity,
            principal,
            resource: resolved.resource.entity_id(),
            resource_entity: Arc::from(projection.resource().entity()),
            action: projection.action().clone(),
            purpose: projection.purpose().clone(),
            related_relation: projection
                .related()
                .map(|related| related.relation().clone()),
            related: resolved.related,
            field: projection.field_value().cloned(),
            amount: projection.amount_value().cloned(),
            cardinality: projection.cardinality_value(),
            context_name: Arc::from(projection.context_value().context()),
            context_type: Arc::from(projection.context_value().context_type()),
            context: resolved.context.clone(),
        }
    }
}
