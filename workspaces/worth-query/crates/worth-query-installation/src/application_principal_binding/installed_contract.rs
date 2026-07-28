use std::marker::PhantomData;

use sha2::{Digest, Sha256};
use worth_foundational::facade::ScalarAspectType;
use worth_query_declaration::facade::application_schema::{
    ApplicationSchema, ApplicationSchemaBindingIdentity, ApplicationSchemaMember,
};

use crate::application_schema::WorthQueryInstalledApplicationSchema;
use crate::installed_index::WorthQueryInstalledPackageAuthority;

/// Opaque installation authority for one schema-declared principal binding.
///
/// The type is intentionally neither cloneable nor serializable. Its
/// constructor consumes meaning retained only by an installed schema handle.
pub struct WorthQueryInstalledPrincipalBinding<
    Schema,
    Binding,
    Mapping,
    Principal,
    PrincipalIdentity,
> {
    binding_identity: ApplicationSchemaBindingIdentity,
    owner: String,
    schema_name: String,
    binding: String,
    mapping_entity: String,
    identity_aspect: String,
    identity_field: String,
    status_aspect: String,
    status_field: String,
    target_relation: String,
    principal_entity: String,
    principal_identity_aspect: String,
    principal_identity_field: String,
    principal_identity_scalar_family: ScalarAspectType,
    principal_identity_value_type: String,
    authority_identity: String,
    _marker: PhantomData<fn() -> (Schema, Binding, Mapping, Principal, PrincipalIdentity)>,
}

impl<Schema, Binding, Mapping, Principal, PrincipalIdentity>
    WorthQueryInstalledPrincipalBinding<Schema, Binding, Mapping, Principal, PrincipalIdentity>
{
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_installed_schema(
        schema: &WorthQueryInstalledApplicationSchema<Schema>,
        binding: &str,
        mapping_entity: &str,
        identity_aspect: &str,
        identity_field: &str,
        status_aspect: &str,
        status_field: &str,
        target_relation: &str,
        principal_entity: &str,
        principal_identity_aspect: &str,
        principal_identity_field: &str,
        principal_identity_scalar_family: ScalarAspectType,
        principal_identity_value_type: &str,
    ) -> Self
    where
        Schema: ApplicationSchema,
    {
        let binding_identity = schema.binding_identity();
        let authority_identity = authority_identity(
            &schema.package_authority.authority_nonce,
            &binding_identity,
            binding,
            mapping_entity,
            identity_aspect,
            identity_field,
            status_aspect,
            status_field,
            target_relation,
            principal_entity,
            principal_identity_aspect,
            principal_identity_field,
            principal_identity_scalar_family,
            principal_identity_value_type,
        );
        Self {
            binding_identity,
            owner: schema.owner().to_string(),
            schema_name: schema.schema_name().to_string(),
            binding: binding.to_string(),
            mapping_entity: mapping_entity.to_string(),
            identity_aspect: identity_aspect.to_string(),
            identity_field: identity_field.to_string(),
            status_aspect: status_aspect.to_string(),
            status_field: status_field.to_string(),
            target_relation: target_relation.to_string(),
            principal_entity: principal_entity.to_string(),
            principal_identity_aspect: principal_identity_aspect.to_string(),
            principal_identity_field: principal_identity_field.to_string(),
            principal_identity_scalar_family,
            principal_identity_value_type: principal_identity_value_type.to_string(),
            authority_identity,
            _marker: PhantomData,
        }
    }

    pub fn binding_identity(&self) -> &ApplicationSchemaBindingIdentity {
        &self.binding_identity
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    pub fn binding(&self) -> &str {
        &self.binding
    }

    pub fn mapping_entity(&self) -> &str {
        &self.mapping_entity
    }

    pub fn identity_aspect(&self) -> &str {
        &self.identity_aspect
    }

    pub fn identity_field(&self) -> &str {
        &self.identity_field
    }

    pub fn status_aspect(&self) -> &str {
        &self.status_aspect
    }

    pub fn status_field(&self) -> &str {
        &self.status_field
    }

    pub fn target_relation(&self) -> &str {
        &self.target_relation
    }

    pub fn principal_entity(&self) -> &str {
        &self.principal_entity
    }

    pub fn principal_identity_aspect(&self) -> &str {
        &self.principal_identity_aspect
    }

    pub fn principal_identity_field(&self) -> &str {
        &self.principal_identity_field
    }

    pub const fn principal_identity_scalar_family(&self) -> ScalarAspectType {
        self.principal_identity_scalar_family
    }

    pub fn principal_identity_value_type(&self) -> &str {
        &self.principal_identity_value_type
    }

    pub fn authority_identity(&self) -> &str {
        &self.authority_identity
    }

    pub(crate) fn meaning_matches(&self, member: &ApplicationSchemaMember) -> bool {
        matches!(
            member,
            ApplicationSchemaMember::PrincipalBinding {
                binding,
                mapping_entity,
                identity_aspect,
                identity_field,
                status_aspect,
                status_field,
                target_relation,
                principal_entity,
                principal_identity_aspect,
                principal_identity_field,
                principal_identity_scalar_family,
                principal_identity_value_type,
            } if binding == &self.binding
                && mapping_entity == &self.mapping_entity
                && identity_aspect == &self.identity_aspect
                && identity_field == &self.identity_field
                && status_aspect == &self.status_aspect
                && status_field == &self.status_field
                && target_relation == &self.target_relation
                && principal_entity == &self.principal_entity
                && principal_identity_aspect == &self.principal_identity_aspect
                && principal_identity_field == &self.principal_identity_field
                && principal_identity_scalar_family == &self.principal_identity_scalar_family
                && principal_identity_value_type == &self.principal_identity_value_type
        )
    }

    pub(crate) fn authority_matches(&self, package: &WorthQueryInstalledPackageAuthority) -> bool {
        self.authority_identity
            == authority_identity(
                &package.authority_nonce,
                &self.binding_identity,
                &self.binding,
                &self.mapping_entity,
                &self.identity_aspect,
                &self.identity_field,
                &self.status_aspect,
                &self.status_field,
                &self.target_relation,
                &self.principal_entity,
                &self.principal_identity_aspect,
                &self.principal_identity_field,
                self.principal_identity_scalar_family,
                &self.principal_identity_value_type,
            )
    }
}

impl<Schema, Binding, Mapping, Principal, PrincipalIdentity> std::fmt::Debug
    for WorthQueryInstalledPrincipalBinding<Schema, Binding, Mapping, Principal, PrincipalIdentity>
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryInstalledPrincipalBinding")
            .field("binding", &self.binding)
            .field("binding_identity", &self.binding_identity)
            .field("authority_identity", &self.authority_identity)
            .finish_non_exhaustive()
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn authority_identity(
    nonce: &[u8; 32],
    identity: &ApplicationSchemaBindingIdentity,
    binding: &str,
    mapping_entity: &str,
    identity_aspect: &str,
    identity_field: &str,
    status_aspect: &str,
    status_field: &str,
    target_relation: &str,
    principal_entity: &str,
    principal_identity_aspect: &str,
    principal_identity_field: &str,
    principal_identity_scalar_family: ScalarAspectType,
    principal_identity_value_type: &str,
) -> String {
    let mut hash = Sha256::new();
    hash.update(b"worth-query-installed-principal-binding-v1");
    hash.update(nonce);
    for value in [
        identity.package_identity(),
        identity.schema_identity().as_str(),
        binding,
        mapping_entity,
        identity_aspect,
        identity_field,
        status_aspect,
        status_field,
        target_relation,
        principal_entity,
        principal_identity_aspect,
        principal_identity_field,
        principal_identity_scalar_family.canonical_name(),
        principal_identity_value_type,
    ] {
        hash.update(value.len().to_le_bytes());
        hash.update(value.as_bytes());
    }
    format!("{:x}", hash.finalize())
}
