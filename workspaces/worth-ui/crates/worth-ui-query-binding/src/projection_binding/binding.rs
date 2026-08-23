use crate::declaration::{UiCollectionSchemaRequirement, UiScalarSchemaRequirement};

pub(super) struct UiProjectionBindingAuthority<R> {
    query_world_identity: worth_query::facade::runtime::WorthQueryEvidenceIdentity,
    runtime_provenance: worth_query::facade::runtime::WorthQueryRuntimeProvenance,
    reference: R,
}

impl<R> UiProjectionBindingAuthority<R> {
    pub(super) fn query_issued(
        query_world_identity: worth_query::facade::runtime::WorthQueryEvidenceIdentity,
        runtime_provenance: worth_query::facade::runtime::WorthQueryRuntimeProvenance,
        reference: R,
    ) -> Self {
        Self {
            query_world_identity,
            runtime_provenance,
            reference,
        }
    }

    fn into_parts(
        self,
    ) -> (
        worth_query::facade::runtime::WorthQueryEvidenceIdentity,
        worth_query::facade::runtime::WorthQueryRuntimeProvenance,
        R,
    ) {
        (
            self.query_world_identity,
            self.runtime_provenance,
            self.reference,
        )
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct UiProjectionBinding {
    query_binding_reporting_projection: crate::UiQueryIdentityReportingProjection,
}

impl UiProjectionBinding {
    pub fn query_binding_reporting_projection(&self) -> &crate::UiQueryIdentityReportingProjection {
        &self.query_binding_reporting_projection
    }

    pub(crate) fn query_issued(binding_identity_for_reporting: &str) -> Self {
        Self {
            query_binding_reporting_projection:
                crate::UiQueryIdentityReportingProjection::from_query_reporting_text(
                    binding_identity_for_reporting,
                ),
        }
    }

    pub(crate) fn retained_query_binding_reporting_projection(
        &self,
    ) -> crate::UiQueryIdentityReportingProjection {
        self.query_binding_reporting_projection.clone()
    }
}

pub struct UiScalarProjectionBinding {
    core: UiProjectionBinding,
    requirement: UiScalarSchemaRequirement,
    view_identity: crate::WorthUiQueryViewIdentity,
    query_world_identity: worth_query::facade::runtime::WorthQueryEvidenceIdentity,
    runtime_provenance: worth_query::facade::runtime::WorthQueryRuntimeProvenance,
    async_binding_identity: Option<worth_query::facade::runtime::WorthQueryEvidenceIdentity>,
    next_observation_order: std::cell::Cell<u64>,
    reference: crate::application_binding::WorthUiInstalledScalarTextOperationReference,
    prepared: Option<crate::application_binding::WorthUiPreparedScalarTextConsumer>,
}

impl UiScalarProjectionBinding {
    pub fn core(&self) -> &UiProjectionBinding {
        &self.core
    }

    pub fn requirement(&self) -> &UiScalarSchemaRequirement {
        &self.requirement
    }

    pub fn view_identity(&self) -> &crate::WorthUiQueryViewIdentity {
        &self.view_identity
    }

    pub(super) fn admitted(
        requirement: UiScalarSchemaRequirement,
        view_identity: crate::WorthUiQueryViewIdentity,
        authority: UiProjectionBindingAuthority<
            crate::application_binding::WorthUiInstalledScalarTextOperationReference,
        >,
        prepared: crate::application_binding::WorthUiPreparedScalarTextConsumer,
    ) -> Self {
        let (query_world_identity, runtime_provenance, reference) = authority.into_parts();
        let core = UiProjectionBinding::query_issued(prepared.binding_identity_for_reporting());
        Self {
            core,
            requirement,
            view_identity,
            query_world_identity,
            runtime_provenance,
            async_binding_identity: None,
            next_observation_order: std::cell::Cell::new(1),
            reference,
            prepared: Some(prepared),
        }
    }

    pub(crate) fn runtime_provenance(
        &self,
    ) -> worth_query::facade::runtime::WorthQueryRuntimeProvenance {
        self.runtime_provenance
    }

    pub(crate) fn query_world_identity(
        &self,
    ) -> &worth_query::facade::runtime::WorthQueryEvidenceIdentity {
        &self.query_world_identity
    }

    pub(crate) fn async_binding_identity(
        &self,
    ) -> Option<&worth_query::facade::runtime::WorthQueryEvidenceIdentity> {
        self.async_binding_identity.as_ref()
    }

    pub(crate) fn retain_async_binding_identity(
        &mut self,
        identity: worth_query::facade::runtime::WorthQueryEvidenceIdentity,
    ) {
        self.async_binding_identity = Some(identity);
    }

    pub(crate) fn issue_observation_order(&self) -> u64 {
        let order = self.next_observation_order.get();
        self.next_observation_order.set(
            order
                .checked_add(1)
                .expect("projection observation order exhausted"),
        );
        order
    }

    pub(crate) fn take_prepared(
        &mut self,
    ) -> Option<crate::application_binding::WorthUiPreparedScalarTextConsumer> {
        self.prepared.take()
    }

    pub(crate) fn discard_prepared_after_pending(&mut self) {
        self.prepared = None;
    }

    pub(crate) fn reference(
        &self,
    ) -> &crate::application_binding::WorthUiInstalledScalarTextOperationReference {
        &self.reference
    }

    pub(crate) fn replacement_attempt_identity(
        &self,
    ) -> worth_query::facade::runtime::WorthQueryEvidenceIdentity {
        self.query_world_identity.clone()
    }

    pub(crate) fn inherit_compatible_identity_from(
        mut self,
        predecessor: UiScalarProjectionBinding,
    ) -> Self {
        self.core = predecessor.core;
        self.async_binding_identity = predecessor.async_binding_identity;
        self.next_observation_order = predecessor.next_observation_order;
        self
    }
}

impl std::fmt::Debug for UiScalarProjectionBinding {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("UiScalarProjectionBinding")
            .field("core", &self.core)
            .field("requirement", &self.requirement)
            .field("view_identity", &self.view_identity)
            .field("async_binding_identity", &self.async_binding_identity)
            .field("prepared", &"sealed Query consumer")
            .finish()
    }
}

pub struct UiCollectionProjectionBinding {
    core: UiProjectionBinding,
    requirement: UiCollectionSchemaRequirement,
    view_identity: crate::WorthUiQueryViewIdentity,
    query_world_identity: worth_query::facade::runtime::WorthQueryEvidenceIdentity,
    runtime_provenance: worth_query::facade::runtime::WorthQueryRuntimeProvenance,
    next_observation_order: std::cell::Cell<u64>,
    reference: crate::application_binding::WorthUiInstalledCollectionTextOperationReference,
    prepared: Option<crate::application_binding::WorthUiPreparedCollectionTextConsumer>,
}

impl UiCollectionProjectionBinding {
    pub fn core(&self) -> &UiProjectionBinding {
        &self.core
    }

    pub fn requirement(&self) -> &UiCollectionSchemaRequirement {
        &self.requirement
    }

    pub fn view_identity(&self) -> &crate::WorthUiQueryViewIdentity {
        &self.view_identity
    }

    pub(super) fn admitted(
        requirement: UiCollectionSchemaRequirement,
        view_identity: crate::WorthUiQueryViewIdentity,
        authority: UiProjectionBindingAuthority<
            crate::application_binding::WorthUiInstalledCollectionTextOperationReference,
        >,
        prepared: crate::application_binding::WorthUiPreparedCollectionTextConsumer,
    ) -> Self {
        let (query_world_identity, runtime_provenance, reference) = authority.into_parts();
        let core = UiProjectionBinding::query_issued(prepared.binding_identity_for_reporting());
        Self {
            core,
            requirement,
            view_identity,
            query_world_identity,
            runtime_provenance,
            next_observation_order: std::cell::Cell::new(1),
            reference,
            prepared: Some(prepared),
        }
    }

    pub(crate) fn query_world_identity(
        &self,
    ) -> &worth_query::facade::runtime::WorthQueryEvidenceIdentity {
        &self.query_world_identity
    }

    pub(crate) fn runtime_provenance(
        &self,
    ) -> worth_query::facade::runtime::WorthQueryRuntimeProvenance {
        self.runtime_provenance
    }

    pub(crate) fn reference(
        &self,
    ) -> &crate::application_binding::WorthUiInstalledCollectionTextOperationReference {
        &self.reference
    }

    pub(crate) fn issue_observation_order(&self) -> u64 {
        let order = self.next_observation_order.get();
        self.next_observation_order.set(
            order
                .checked_add(1)
                .expect("projection observation order exhausted"),
        );
        order
    }

    pub(crate) fn replacement_attempt_identity(
        &self,
    ) -> worth_query::facade::runtime::WorthQueryEvidenceIdentity {
        self.query_world_identity.clone()
    }

    pub(crate) fn inherit_compatible_identity_from(
        mut self,
        predecessor: UiCollectionProjectionBinding,
    ) -> Self {
        self.core = predecessor.core;
        self.next_observation_order = predecessor.next_observation_order;
        self
    }

    pub(crate) fn take_prepared(
        &mut self,
    ) -> Option<crate::application_binding::WorthUiPreparedCollectionTextConsumer> {
        self.prepared.take()
    }
}

impl std::fmt::Debug for UiCollectionProjectionBinding {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("UiCollectionProjectionBinding")
            .field("core", &self.core)
            .field("requirement", &self.requirement)
            .field("view_identity", &self.view_identity)
            .field("reference", &"sealed Query collection operation")
            .field("prepared", &"sealed Query collection consumer")
            .finish()
    }
}
