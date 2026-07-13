use worth_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use worth_query::facade::foundation::{
    AuthorizedProjectionFieldPath, ProjectionAuthorityContract,
    ProjectionConsumptionBindingContext, ProjectionFactFieldPath,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerDirectProjectionRequest {
    authorized_projection_identity: String,
    narrowed_result_shape_digest: String,
    policy_digest: String,
    tenant_schema_basis_digest: String,
    visible_fields: Vec<AuthorizedProjectionFieldPath>,
    authority_contract: ProjectionAuthorityContract,
}

impl WorthServerDirectProjectionRequest {
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
            visible_fields: visible_fields
                .into_iter()
                .map(Into::into)
                .map(|field| {
                    admit_authorized_projection_field_path(&field).unwrap_or_else(|error| {
                        panic!("visible field `{field}` must be a foundational projection field path: {error}")
                    })
                })
                .collect(),
            authority_contract: ProjectionAuthorityContract::declare()
                .require_settled_consumption()
                .require_source_authority(),
        }
    }

    pub fn entity_identities(mut self) -> Self {
        self.authority_contract = self.authority_contract.require_entity_identities();
        self
    }

    pub fn view_local_identities(mut self) -> Self {
        self.authority_contract = self.authority_contract.require_view_local_identities();
        self
    }

    pub fn target_identity(mut self) -> Self {
        self.authority_contract = self.authority_contract.require_target_identity();
        self
    }

    pub fn source_references(mut self) -> Self {
        self.authority_contract = self.authority_contract.require_source_references();
        self
    }

    pub fn effect_continuity_facts(mut self) -> Self {
        self.authority_contract = self.authority_contract.require_effect_continuity_facts();
        self
    }

    pub fn memberships(mut self) -> Self {
        self.authority_contract = self.authority_contract.require_memberships();
        self
    }

    pub fn relation_endpoints(mut self) -> Self {
        self.authority_contract = self.authority_contract.require_relation_endpoints();
        self
    }

    pub fn display_field(mut self, field: impl Into<String>) -> Self {
        let field = field.into();
        let field_path = admit_projection_fact_field_path(&field).unwrap_or_else(|error| {
            panic!("display field `{field}` must be a foundational projection field path: {error}")
        });
        self.authority_contract = self.authority_contract.require_display_field(field_path);
        self
    }

    pub fn derived_scalar_field(mut self, field: impl Into<String>) -> Self {
        let field = field.into();
        let field_path = admit_projection_fact_field_path(&field).unwrap_or_else(|error| {
            panic!(
                "derived scalar field `{field}` must be a foundational projection field path: {error}"
            )
        });
        self.authority_contract = self
            .authority_contract
            .require_derived_scalar_field(field_path);
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

    pub fn visible_fields(&self) -> &[AuthorizedProjectionFieldPath] {
        &self.visible_fields
    }

    pub fn authority_contract(&self) -> &ProjectionAuthorityContract {
        &self.authority_contract
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

    pub(crate) fn authority_contract_owned(&self) -> ProjectionAuthorityContract {
        self.authority_contract.clone()
    }
}

fn admit_authorized_projection_field_path(
    field: &str,
) -> Result<AuthorizedProjectionFieldPath, String> {
    let Some((aspect, terminal_field)) = field.split_once('.') else {
        return Err("field must use `aspect.field` form".to_string());
    };
    let aspect_key = AspectKey::new(aspect.to_string())
        .ok_or_else(|| "aspect is not foundational".to_string())?;
    let field_key = FieldKey::new(terminal_field.to_string())
        .ok_or_else(|| "field is not foundational".to_string())?;
    Ok(AuthorizedProjectionFieldPath::from_native_keys(
        aspect_key, field_key,
    ))
}

fn admit_projection_fact_field_path(field: &str) -> Result<ProjectionFactFieldPath, String> {
    let segments = field
        .split('.')
        .map(|segment| {
            FieldKey::new(segment.to_string())
                .ok_or_else(|| format!("`{segment}` is not a foundational field segment"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let path = CanonicalFieldPath::new(segments)
        .ok_or_else(|| "projection field path must contain at least one segment".to_string())?;
    Ok(ProjectionFactFieldPath::from_canonical_field_path(path))
}
