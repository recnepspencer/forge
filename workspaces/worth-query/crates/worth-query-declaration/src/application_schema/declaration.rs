use std::marker::PhantomData;

use super::authorization_policy::ApplicationAuthorizationPath;
use super::canonical_identity::{canonical_identity, ApplicationSchemaCanonicalHeader};
use super::capabilities::{ApplicationFieldCurrency, EqualityPosture, WritePosture};
use super::identifier_validation::{validate_member_identifiers, validate_schema_header};
use super::member_closure::validate_member_closure;
use super::principal_binding_reference::ApplicationPrincipalBindingRef;
use super::references::{
    ApplicationAbilityRef, ApplicationAspectRef, ApplicationCurrencyRef, ApplicationEffectRef,
    ApplicationEntityRef, ApplicationFieldRef, ApplicationOperationRef, ApplicationPolicyRef,
    ApplicationRelationRef,
};
use super::schema_member::ApplicationSchemaMember;
use super::values::TypedApplicationValue;

pub trait ApplicationSchema: Sized + 'static {
    const OWNER: &'static str;
    const NAME: &'static str;
    const MAJOR: u32;
    const MINOR: u32;

    fn declaration(
    ) -> Result<ApplicationSchemaDeclaration<Self>, ApplicationSchemaDeclarationDenial>;
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationSchemaIdentity(String);

impl ApplicationSchemaIdentity {
    pub(super) fn from_canonical_hash(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ErasedApplicationSchemaDeclaration {
    owner: String,
    name: String,
    major: u32,
    minor: u32,
    identity: ApplicationSchemaIdentity,
    members: Vec<ApplicationSchemaMember>,
}

impl ErasedApplicationSchemaDeclaration {
    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn major(&self) -> u32 {
        self.major
    }

    pub const fn minor(&self) -> u32 {
        self.minor
    }

    pub fn identity(&self) -> &ApplicationSchemaIdentity {
        &self.identity
    }

    pub fn members(&self) -> &[ApplicationSchemaMember] {
        &self.members
    }
}

pub struct ApplicationSchemaDeclaration<Schema> {
    erased: ErasedApplicationSchemaDeclaration,
    _schema: PhantomData<fn() -> Schema>,
}

impl<Schema> Clone for ApplicationSchemaDeclaration<Schema> {
    fn clone(&self) -> Self {
        Self {
            erased: self.erased.clone(),
            _schema: PhantomData,
        }
    }
}

impl<Schema> std::fmt::Debug for ApplicationSchemaDeclaration<Schema> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ApplicationSchemaDeclaration")
            .field("erased", &self.erased)
            .finish_non_exhaustive()
    }
}

impl<Schema> PartialEq for ApplicationSchemaDeclaration<Schema> {
    fn eq(&self, other: &Self) -> bool {
        self.erased == other.erased
    }
}

impl<Schema> Eq for ApplicationSchemaDeclaration<Schema> {}

impl<Schema> ApplicationSchemaDeclaration<Schema> {
    pub fn identity(&self) -> &ApplicationSchemaIdentity {
        self.erased.identity()
    }

    pub fn erased(&self) -> &ErasedApplicationSchemaDeclaration {
        &self.erased
    }

    pub fn into_erased(self) -> ErasedApplicationSchemaDeclaration {
        self.erased
    }
}

#[derive(Clone, Debug)]
pub struct ApplicationSchemaDeclarationBuilder<Schema> {
    owner: &'static str,
    name: &'static str,
    major: u32,
    minor: u32,
    members: Vec<ApplicationSchemaMember>,
    _schema: PhantomData<fn() -> Schema>,
}

impl<Schema> ApplicationSchemaDeclarationBuilder<Schema> {
    pub(super) fn push_member(mut self, member: ApplicationSchemaMember) -> Self {
        self.members.push(member);
        self
    }

    pub fn for_schema() -> Self
    where
        Schema: ApplicationSchema,
    {
        ApplicationSchemaDeclarationBuilder {
            owner: Schema::OWNER,
            name: Schema::NAME,
            major: Schema::MAJOR,
            minor: Schema::MINOR,
            members: Vec::new(),
            _schema: PhantomData,
        }
    }

    pub fn entity<Entity>(mut self, reference: ApplicationEntityRef<Schema, Entity>) -> Self {
        self.members.push(ApplicationSchemaMember::Entity {
            entity: reference.name().to_string(),
        });
        self
    }

    pub fn aspect<Entity, Aspect>(
        mut self,
        entity: ApplicationEntityRef<Schema, Entity>,
        aspect: ApplicationAspectRef<Schema, Entity, Aspect>,
    ) -> Self {
        self.members.push(ApplicationSchemaMember::Aspect {
            entity: entity.name().to_string(),
            aspect: aspect.name().to_string(),
        });
        self
    }

    pub fn field<Entity, Aspect, Field, Value, Write, Equality, Currency>(
        mut self,
        entity: ApplicationEntityRef<Schema, Entity>,
        field: ApplicationFieldRef<Schema, Entity, Aspect, Field, Value, Write, Equality, Currency>,
    ) -> Self
    where
        Value: TypedApplicationValue,
        Write: WritePosture,
        Equality: EqualityPosture,
        Currency: ApplicationFieldCurrency,
    {
        self.members.push(ApplicationSchemaMember::Field {
            entity: entity.name().to_string(),
            aspect: field.aspect().to_string(),
            field: field.field().to_string(),
            scalar_family: field.scalar_family(),
            value_type: field.value_type_name().to_string(),
            currency: field.currency().map(str::to_string),
            writable: Write::WRITABLE,
            equality_queryable: Equality::QUERYABLE,
        });
        self
    }

    pub fn relation<Relation, From, To>(
        mut self,
        relation: ApplicationRelationRef<Schema, Relation, From, To>,
        from: ApplicationEntityRef<Schema, From>,
        to: ApplicationEntityRef<Schema, To>,
    ) -> Self {
        self.members.push(ApplicationSchemaMember::Relation {
            relation: relation.name().to_string(),
            from: from.name().to_string(),
            to: to.name().to_string(),
        });
        self
    }

    pub fn principal_binding<Binding, Mapping, Principal, PrincipalIdentity>(
        mut self,
        binding: ApplicationPrincipalBindingRef<
            Schema,
            Binding,
            Mapping,
            Principal,
            PrincipalIdentity,
        >,
    ) -> Self
    where
        PrincipalIdentity: TypedApplicationValue,
    {
        self.members
            .push(ApplicationSchemaMember::PrincipalBinding {
                binding: binding.name().to_string(),
                mapping_entity: binding.mapping_entity().to_string(),
                identity_aspect: binding.identity_aspect().to_string(),
                identity_field: binding.identity_field().to_string(),
                status_aspect: binding.status_aspect().to_string(),
                status_field: binding.status_field().to_string(),
                target_relation: binding.target_relation().to_string(),
                principal_entity: binding.principal_entity().to_string(),
                principal_identity_aspect: binding.principal_identity_aspect().to_string(),
                principal_identity_field: binding.principal_identity_field().to_string(),
                principal_identity_scalar_family: binding.principal_identity_scalar_family(),
                principal_identity_value_type: binding.principal_identity_value_type().to_string(),
            });
        self
    }

    pub fn operation<Operation, Input>(
        mut self,
        operation: ApplicationOperationRef<Schema, Operation, Input>,
    ) -> Self {
        self.members.push(ApplicationSchemaMember::Operation {
            operation: operation.name().to_string(),
            input_type: std::any::type_name::<Input>().to_string(),
        });
        self
    }

    pub fn policy<Policy>(mut self, policy: ApplicationPolicyRef<Schema, Policy>) -> Self {
        self.members.push(ApplicationSchemaMember::Policy {
            policy: policy.name().to_string(),
        });
        self
    }

    pub fn ability<Ability, Scope>(
        mut self,
        ability: ApplicationAbilityRef<Schema, Ability, Scope>,
    ) -> Self {
        self.members.push(ApplicationSchemaMember::Ability {
            ability: ability.name().to_string(),
            scope_entity: ability.scope().to_string(),
        });
        self
    }

    pub fn operation_requires_ability<Operation, Input, Ability, Scope>(
        mut self,
        operation: ApplicationOperationRef<Schema, Operation, Input>,
        ability: ApplicationAbilityRef<Schema, Ability, Scope>,
    ) -> Self
    where
        Ability: super::capabilities::OperationRequiresAbility<Operation>,
    {
        self.members
            .push(ApplicationSchemaMember::OperationAbility {
                operation: operation.name().to_string(),
                ability: ability.name().to_string(),
                scope_entity: ability.scope().to_string(),
            });
        self
    }

    pub fn ability_policy<Ability, Scope, Policy>(
        mut self,
        ability: ApplicationAbilityRef<Schema, Ability, Scope>,
        policy: ApplicationPolicyRef<Schema, Policy>,
        paths: impl IntoIterator<Item = ApplicationAuthorizationPath>,
    ) -> Self {
        let mut paths = paths.into_iter().collect::<Vec<_>>();
        paths.sort();
        paths.dedup();
        self.members.push(ApplicationSchemaMember::AbilityPolicy {
            ability: ability.name().to_string(),
            scope_entity: ability.scope().to_string(),
            policy: policy.name().to_string(),
            paths,
        });
        self
    }

    pub fn currency<Currency>(
        mut self,
        currency: ApplicationCurrencyRef<Schema, Currency>,
    ) -> Self {
        self.members.push(ApplicationSchemaMember::Currency {
            currency: currency.name().to_string(),
        });
        self
    }

    pub fn effect<Effect, Payload>(
        mut self,
        effect: ApplicationEffectRef<Schema, Effect, Payload>,
    ) -> Self {
        self.members.push(ApplicationSchemaMember::Effect {
            effect: effect.name().to_string(),
            payload_type: std::any::type_name::<Payload>().to_string(),
        });
        self
    }

    pub fn build(
        mut self,
    ) -> Result<ApplicationSchemaDeclaration<Schema>, ApplicationSchemaDeclarationDenial> {
        validate_schema_header(self.owner, self.name)?;
        validate_member_identifiers(&self.members)?;
        self.members.sort();
        if self.members.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err(ApplicationSchemaDeclarationDenial::DuplicateMember);
        }
        validate_member_closure(&self.members)?;
        let identity = canonical_identity(
            ApplicationSchemaCanonicalHeader {
                owner: self.owner,
                name: self.name,
                major: self.major,
                minor: self.minor,
            },
            &self.members,
        );
        Ok(ApplicationSchemaDeclaration {
            erased: ErasedApplicationSchemaDeclaration {
                owner: self.owner.to_string(),
                name: self.name.to_string(),
                major: self.major,
                minor: self.minor,
                identity,
                members: self.members,
            },
            _schema: PhantomData,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApplicationSchemaDeclarationDenial {
    InvalidIdentifier,
    DuplicateMember,
    MissingEntity,
    MissingAspect,
    MissingCurrency,
    MissingPrincipalBindingDependency,
    MissingOperationProgramDependency,
    MissingOperationDecisionReadDependency,
    InvalidOperationDecisionFactBudget,
    InvalidOperationProjectionWorkBudget,
    MissingAbilityDependency,
    MissingAbilityPolicyDependency,
    InvalidAbilityPolicy,
}

impl std::fmt::Display for ApplicationSchemaDeclarationDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "application schema declaration denied: {self:?}")
    }
}

impl std::error::Error for ApplicationSchemaDeclarationDenial {}
