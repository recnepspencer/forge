use worth_foundational::facade::AspectValue;
use worth_query_declaration::facade::application_capability::ApplicationCapabilityRequestProjection;
use worth_relational::facade::identity::EntityId;

use super::super::capability_admission::WorthQueryResolvedCapabilityRequest;
use super::super::retained_capability_request::WorthQueryRetainedCapabilityRequest;

#[derive(Clone)]
pub(in crate::domain_computation) struct WorthQueryElevationUpperBound {
    request: WorthQueryRetainedCapabilityRequest,
    grant: EntityId,
}

impl std::fmt::Debug for WorthQueryElevationUpperBound {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryElevationUpperBound")
            .field("capability_identity", &self.request.capability_identity)
            .field("requester", &self.request.principal)
            .field("resource", &self.request.resource)
            .field("grant", &self.grant)
            .field("cardinality", &self.request.cardinality)
            .finish_non_exhaustive()
    }
}

impl WorthQueryElevationUpperBound {
    pub(in crate::domain_computation::authorization) fn capture<Schema, Scope, Context>(
        capability_identity: [u8; 32],
        principal: EntityId,
        projection: &ApplicationCapabilityRequestProjection<Schema, Scope, Context>,
        resolved: &WorthQueryResolvedCapabilityRequest<Schema, Scope>,
        grant: EntityId,
    ) -> Self {
        Self {
            request: WorthQueryRetainedCapabilityRequest::capture(
                capability_identity,
                principal,
                projection,
                resolved,
            ),
            grant,
        }
    }

    pub(in crate::domain_computation) const fn capability_identity(&self) -> [u8; 32] {
        self.request.capability_identity
    }

    pub(in crate::domain_computation) const fn requester(&self) -> EntityId {
        self.request.principal
    }

    pub(in crate::domain_computation) const fn resource(&self) -> EntityId {
        self.request.resource
    }

    pub(in crate::domain_computation) const fn grant(&self) -> EntityId {
        self.grant
    }

    pub(in crate::domain_computation) const fn action(&self) -> &AspectValue {
        &self.request.action
    }

    pub(in crate::domain_computation) const fn purpose(&self) -> &AspectValue {
        &self.request.purpose
    }

    pub(in crate::domain_computation) const fn field(&self) -> Option<&AspectValue> {
        self.request.field.as_ref()
    }

    pub(in crate::domain_computation) const fn magnitude(&self) -> Option<&AspectValue> {
        self.request.magnitude.as_ref()
    }

    pub(in crate::domain_computation) const fn cardinality(&self) -> u32 {
        self.request.cardinality
    }

    pub(in crate::domain_computation) fn matches_active_request(
        &self,
        request: &WorthQueryRetainedCapabilityRequest,
        elevation: EntityId,
        grant: EntityId,
    ) -> bool {
        request.capability_identity == self.request.capability_identity
            && request.principal == self.request.principal
            && request.resource == self.request.resource
            && request.resource_entity == self.request.resource_entity
            && request.elevation == Some(elevation)
            && request.action == self.request.action
            && request.purpose == self.request.purpose
            && request.related_relation == self.request.related_relation
            && request.related == self.request.related
            && request.field == self.request.field
            && request.magnitude == self.request.magnitude
            && request.cardinality == self.request.cardinality
            && request.context_name == self.request.context_name
            && request.context_type == self.request.context_type
            && request.context == self.request.context
            && grant == self.grant
    }
}
