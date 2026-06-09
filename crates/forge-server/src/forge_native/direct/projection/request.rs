use forge_query::facade::{ProjectMaterializedFacts, ProjectionConsumptionBindingContext};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerDirectProjectionRequest {
    authorized_projection_identity: String,
    narrowed_result_shape_digest: String,
    policy_digest: String,
    tenant_schema_basis_digest: String,
    visible_fields: Vec<String>,
    requested_facts: ProjectMaterializedFacts,
}

impl ForgeServerDirectProjectionRequest {
    pub fn new(
        authorized_projection_identity: impl Into<String>,
        narrowed_result_shape_digest: impl Into<String>,
        policy_digest: impl Into<String>,
        tenant_schema_basis_digest: impl Into<String>,
        visible_fields: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            authorized_projection_identity: authorized_projection_identity.into(),
            narrowed_result_shape_digest: narrowed_result_shape_digest.into(),
            policy_digest: policy_digest.into(),
            tenant_schema_basis_digest: tenant_schema_basis_digest.into(),
            visible_fields: visible_fields.into_iter().map(Into::into).collect(),
            requested_facts: ProjectMaterializedFacts::declare(),
        }
    }

    pub fn entity_identities(mut self) -> Self {
        self.requested_facts = self.requested_facts.entity_identities();
        self
    }

    pub fn view_local_identities(mut self) -> Self {
        self.requested_facts = self.requested_facts.view_local_identities();
        self
    }

    pub fn target_identity(mut self) -> Self {
        self.requested_facts = self.requested_facts.target_identity();
        self
    }

    pub fn source_references(mut self) -> Self {
        self.requested_facts = self.requested_facts.source_references();
        self
    }

    pub fn effect_continuity_facts(mut self) -> Self {
        self.requested_facts = self.requested_facts.effect_continuity_facts();
        self
    }

    pub fn memberships(mut self) -> Self {
        self.requested_facts = self.requested_facts.memberships();
        self
    }

    pub fn relation_endpoints(mut self) -> Self {
        self.requested_facts = self.requested_facts.relation_endpoints();
        self
    }

    pub fn display_field(mut self, field: impl Into<String>) -> Self {
        self.requested_facts = self.requested_facts.display_field(field);
        self
    }

    pub fn derived_scalar_field(mut self, field: impl Into<String>) -> Self {
        self.requested_facts = self.requested_facts.derived_scalar_field(field);
        self
    }

    pub fn authorized_projection_identity(&self) -> &str {
        &self.authorized_projection_identity
    }

    pub fn narrowed_result_shape_digest(&self) -> &str {
        &self.narrowed_result_shape_digest
    }

    pub fn policy_digest(&self) -> &str {
        &self.policy_digest
    }

    pub fn tenant_schema_basis_digest(&self) -> &str {
        &self.tenant_schema_basis_digest
    }

    pub fn visible_fields(&self) -> &[String] {
        &self.visible_fields
    }

    pub fn requested_facts(&self) -> &ProjectMaterializedFacts {
        &self.requested_facts
    }

    pub(crate) fn binding_context(
        &self,
        projection_query_digest: &str,
        result_shape_digest: &str,
    ) -> ProjectionConsumptionBindingContext {
        ProjectionConsumptionBindingContext::from_projection_metadata(
            result_shape_digest.to_string(),
            projection_query_digest.to_string(),
            result_shape_digest.to_string(),
            self.authorized_projection_identity.clone(),
            self.narrowed_result_shape_digest.clone(),
            self.policy_digest.clone(),
            self.tenant_schema_basis_digest.clone(),
            self.visible_fields.clone(),
        )
    }

    pub(crate) fn requested_facts_owned(&self) -> ProjectMaterializedFacts {
        self.requested_facts.clone()
    }
}
