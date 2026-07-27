use std::marker::PhantomData;

use worth_foundational::facade::ScalarAspectType;

use super::canonical_identity::{canonical_identity, ApplicationSchemaCanonicalHeader};
use super::capabilities::{ApplicationFieldCurrency, EqualityPosture, WritePosture};
use super::member_closure::validate_member_closure;
use super::references::{
    ApplicationAspectRef, ApplicationCurrencyRef, ApplicationEffectRef, ApplicationEntityRef,
    ApplicationFieldRef, ApplicationOperationRef, ApplicationPolicyRef, ApplicationRelationRef,
};
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
pub enum ApplicationOperationProgramTarget {
    Create {
        entity: String,
    },
    Delete {
        entity: String,
    },
    Write {
        entity: String,
        aspect: String,
        field: String,
    },
    Link {
        relation: String,
        from: String,
        to: String,
    },
    Unlink {
        relation: String,
        from: String,
        to: String,
    },
    Emit {
        effect: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ApplicationSchemaMember {
    Entity {
        entity: String,
    },
    Aspect {
        entity: String,
        aspect: String,
    },
    Field {
        entity: String,
        aspect: String,
        field: String,
        scalar_family: ScalarAspectType,
        value_type: String,
        currency: Option<String>,
        writable: bool,
        equality_queryable: bool,
    },
    Relation {
        relation: String,
        from: String,
        to: String,
    },
    Operation {
        operation: String,
        input_type: String,
    },
    OperationProgram {
        operation: String,
        target: ApplicationOperationProgramTarget,
    },
    Policy {
        policy: String,
    },
    Currency {
        currency: String,
    },
    Effect {
        effect: String,
        payload_type: String,
    },
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
        validate_identifier(self.owner, "owner")?;
        validate_identifier(self.name, "schema")?;
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
    MissingOperationProgramDependency,
}

impl std::fmt::Display for ApplicationSchemaDeclarationDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "application schema declaration denied: {self:?}")
    }
}

impl std::error::Error for ApplicationSchemaDeclarationDenial {}

fn validate_identifier(value: &str, _kind: &str) -> Result<(), ApplicationSchemaDeclarationDenial> {
    if value.is_empty()
        || value.trim() != value
        || value.chars().any(char::is_whitespace)
        || value.contains('.')
    {
        return Err(ApplicationSchemaDeclarationDenial::InvalidIdentifier);
    }
    Ok(())
}
