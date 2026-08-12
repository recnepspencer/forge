use std::marker::PhantomData;

use crate::authentication::{
    WorthQueryExternalPrincipalIdentity, WorthQueryPrincipalMappingStatus,
};

use super::{
    ApplicationFieldRef, ApplicationFieldUnit, ApplicationRelationRef, EqualityPosture,
    EqualityPredicate, ReadOnly, ReadWrite,
};

pub struct ApplicationPrincipalBindingRequirements<Schema, Mapping, Principal, PrincipalIdentity> {
    pub mapping_identity: ApplicationPrincipalMappingIdentityRequirement<Schema, Mapping>,
    pub mapping_status: ApplicationPrincipalMappingStatusRequirement<Schema, Mapping>,
    pub target: ApplicationPrincipalTargetRequirement<Schema, Mapping, Principal>,
    pub principal_identity:
        ApplicationPrincipalIdentityRequirement<Schema, Principal, PrincipalIdentity>,
}

pub struct ApplicationPrincipalMappingIdentityRequirement<Schema, Mapping> {
    pub(super) entity: &'static str,
    pub(super) aspect: &'static str,
    pub(super) field: &'static str,
    _marker: PhantomData<fn() -> (Schema, Mapping)>,
}

impl<Schema, Mapping> ApplicationPrincipalMappingIdentityRequirement<Schema, Mapping> {
    #[doc(hidden)]
    pub fn from_field<Aspect, Field, Unit>(
        field: ApplicationFieldRef<
            Schema,
            Mapping,
            Aspect,
            Field,
            WorthQueryExternalPrincipalIdentity,
            ReadOnly,
            EqualityPredicate,
            Unit,
        >,
    ) -> Self
    where
        Unit: ApplicationFieldUnit,
    {
        Self {
            entity: field.entity(),
            aspect: field.aspect(),
            field: field.field(),
            _marker: PhantomData,
        }
    }
}

pub struct ApplicationPrincipalMappingStatusRequirement<Schema, Mapping> {
    pub(super) aspect: &'static str,
    pub(super) field: &'static str,
    _marker: PhantomData<fn() -> (Schema, Mapping)>,
}

impl<Schema, Mapping> ApplicationPrincipalMappingStatusRequirement<Schema, Mapping> {
    #[doc(hidden)]
    pub fn from_field<Aspect, Field, Equality, Unit>(
        field: ApplicationFieldRef<
            Schema,
            Mapping,
            Aspect,
            Field,
            WorthQueryPrincipalMappingStatus,
            ReadWrite,
            Equality,
            Unit,
        >,
    ) -> Self
    where
        Equality: EqualityPosture,
        Unit: ApplicationFieldUnit,
    {
        Self {
            aspect: field.aspect(),
            field: field.field(),
            _marker: PhantomData,
        }
    }
}

pub struct ApplicationPrincipalTargetRequirement<Schema, Mapping, Principal> {
    pub(super) relation: &'static str,
    pub(super) principal_entity: &'static str,
    _marker: PhantomData<fn() -> (Schema, Mapping, Principal)>,
}

impl<Schema, Mapping, Principal> ApplicationPrincipalTargetRequirement<Schema, Mapping, Principal> {
    #[doc(hidden)]
    pub fn from_relation<Relation>(
        relation: ApplicationRelationRef<Schema, Relation, Mapping, Principal>,
    ) -> Self {
        Self {
            relation: relation.name(),
            principal_entity: relation.to(),
            _marker: PhantomData,
        }
    }
}

pub struct ApplicationPrincipalIdentityRequirement<Schema, Principal, PrincipalIdentity> {
    pub(super) aspect: &'static str,
    pub(super) field: &'static str,
    _marker: PhantomData<fn() -> (Schema, Principal, PrincipalIdentity)>,
}

impl<Schema, Principal, PrincipalIdentity>
    ApplicationPrincipalIdentityRequirement<Schema, Principal, PrincipalIdentity>
where
    PrincipalIdentity: super::TypedApplicationValue,
{
    #[doc(hidden)]
    pub fn from_field<Aspect, Field, Unit>(
        field: ApplicationFieldRef<
            Schema,
            Principal,
            Aspect,
            Field,
            PrincipalIdentity,
            ReadOnly,
            EqualityPredicate,
            Unit,
        >,
    ) -> Self
    where
        Unit: ApplicationFieldUnit,
    {
        Self {
            aspect: field.aspect(),
            field: field.field(),
            _marker: PhantomData,
        }
    }
}
