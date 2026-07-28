use std::marker::PhantomData;

use worth_foundational::facade::ScalarAspectType;

use super::TypedApplicationValue;

pub struct ApplicationPrincipalBindingRef<Schema, Binding, Mapping, Principal, PrincipalIdentity> {
    name: &'static str,
    mapping_entity: &'static str,
    identity_aspect: &'static str,
    identity_field: &'static str,
    status_aspect: &'static str,
    status_field: &'static str,
    target_relation: &'static str,
    principal_entity: &'static str,
    principal_identity_aspect: &'static str,
    principal_identity_field: &'static str,
    principal_identity_scalar_family: ScalarAspectType,
    principal_identity_value_type: &'static str,
    _marker: PhantomData<fn() -> (Schema, Binding, Mapping, Principal, PrincipalIdentity)>,
}

impl<Schema, Binding, Mapping, Principal, PrincipalIdentity> Copy
    for ApplicationPrincipalBindingRef<Schema, Binding, Mapping, Principal, PrincipalIdentity>
{
}

impl<Schema, Binding, Mapping, Principal, PrincipalIdentity> Clone
    for ApplicationPrincipalBindingRef<Schema, Binding, Mapping, Principal, PrincipalIdentity>
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<Schema, Binding, Mapping, Principal, PrincipalIdentity>
    ApplicationPrincipalBindingRef<Schema, Binding, Mapping, Principal, PrincipalIdentity>
where
    PrincipalIdentity: TypedApplicationValue,
{
    #[doc(hidden)]
    #[allow(clippy::too_many_arguments)]
    pub fn from_schema_identifiers(
        name: &'static str,
        mapping_entity: &'static str,
        identity_aspect: &'static str,
        identity_field: &'static str,
        status_aspect: &'static str,
        status_field: &'static str,
        target_relation: &'static str,
        principal_entity: &'static str,
        principal_identity_aspect: &'static str,
        principal_identity_field: &'static str,
    ) -> Self {
        Self {
            name,
            mapping_entity,
            identity_aspect,
            identity_field,
            status_aspect,
            status_field,
            target_relation,
            principal_entity,
            principal_identity_aspect,
            principal_identity_field,
            principal_identity_scalar_family: PrincipalIdentity::SCALAR_FAMILY,
            principal_identity_value_type: std::any::type_name::<PrincipalIdentity>(),
            _marker: PhantomData,
        }
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn mapping_entity(&self) -> &'static str {
        self.mapping_entity
    }

    pub const fn identity_aspect(&self) -> &'static str {
        self.identity_aspect
    }

    pub const fn identity_field(&self) -> &'static str {
        self.identity_field
    }

    pub const fn status_aspect(&self) -> &'static str {
        self.status_aspect
    }

    pub const fn status_field(&self) -> &'static str {
        self.status_field
    }

    pub const fn target_relation(&self) -> &'static str {
        self.target_relation
    }

    pub const fn principal_entity(&self) -> &'static str {
        self.principal_entity
    }

    pub const fn principal_identity_aspect(&self) -> &'static str {
        self.principal_identity_aspect
    }

    pub const fn principal_identity_field(&self) -> &'static str {
        self.principal_identity_field
    }

    pub const fn principal_identity_scalar_family(&self) -> ScalarAspectType {
        self.principal_identity_scalar_family
    }

    pub const fn principal_identity_value_type(&self) -> &'static str {
        self.principal_identity_value_type
    }
}

impl<Schema, Binding, Mapping, Principal, PrincipalIdentity> std::fmt::Debug
    for ApplicationPrincipalBindingRef<Schema, Binding, Mapping, Principal, PrincipalIdentity>
where
    PrincipalIdentity: TypedApplicationValue,
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ApplicationPrincipalBindingRef")
            .field("name", &self.name)
            .field("mapping_entity", &self.mapping_entity)
            .field("principal_entity", &self.principal_entity)
            .field("principal_identity_field", &self.principal_identity_field)
            .finish_non_exhaustive()
    }
}
