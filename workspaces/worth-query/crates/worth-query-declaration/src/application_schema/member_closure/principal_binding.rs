use worth_foundational::facade::ScalarAspectType;

use super::super::ApplicationSchemaMember;
use super::ClosureIndex;

pub(super) enum PrincipalBindingWritePosture {
    ReadOnly,
    Writable,
}

impl PrincipalBindingWritePosture {
    const fn matches(&self, writable: bool) -> bool {
        match self {
            Self::ReadOnly => !writable,
            Self::Writable => writable,
        }
    }
}

pub(super) enum PrincipalBindingEqualityPosture {
    Required,
    Unconstrained,
}

impl PrincipalBindingEqualityPosture {
    const fn admits(&self, equality_queryable: bool) -> bool {
        match self {
            Self::Required => equality_queryable,
            Self::Unconstrained => true,
        }
    }
}

pub(super) struct PrincipalBindingFieldRequirement<'a> {
    pub(super) entity: &'a str,
    pub(super) aspect: &'a str,
    pub(super) field: &'a str,
    pub(super) scalar_family: ScalarAspectType,
    pub(super) value_type: &'a str,
    pub(super) write: PrincipalBindingWritePosture,
    pub(super) equality: PrincipalBindingEqualityPosture,
}

pub(super) struct PrincipalBindingRelationRequirement<'a> {
    pub(super) relation: &'a str,
    pub(super) from: &'a str,
    pub(super) to: &'a str,
}

pub(super) struct PrincipalBindingClosureRequirements<'a> {
    pub(super) mapping_identity: PrincipalBindingFieldRequirement<'a>,
    pub(super) mapping_status: PrincipalBindingFieldRequirement<'a>,
    pub(super) target: PrincipalBindingRelationRequirement<'a>,
    pub(super) principal_identity: PrincipalBindingFieldRequirement<'a>,
}

impl ClosureIndex<'_> {
    pub(super) fn principal_binding_dependencies_exist(
        &self,
        requirements: PrincipalBindingClosureRequirements<'_>,
    ) -> bool {
        self.entities.contains(requirements.mapping_identity.entity)
            && self
                .entities
                .contains(requirements.principal_identity.entity)
            && self.principal_binding_field_matches(&requirements.mapping_identity)
            && self.principal_binding_field_matches(&requirements.mapping_status)
            && self.relations.contains(&(
                requirements.target.relation,
                requirements.target.from,
                requirements.target.to,
            ))
            && self.principal_binding_field_matches(&requirements.principal_identity)
    }

    fn principal_binding_field_matches(
        &self,
        requirement: &PrincipalBindingFieldRequirement<'_>,
    ) -> bool {
        self.members.iter().any(|member| {
            matches!(
                member,
                ApplicationSchemaMember::Field {
                    entity,
                    aspect,
                    field,
                    scalar_family,
                    value_type,
                    writable,
                    equality_queryable,
                    ..
                } if entity == requirement.entity
                    && aspect == requirement.aspect
                    && field == requirement.field
                    && *scalar_family == requirement.scalar_family
                    && value_type == requirement.value_type
                    && requirement.write.matches(*writable)
                    && requirement.equality.admits(*equality_queryable)
            )
        })
    }
}
