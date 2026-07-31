use crate::application_schema::{
    ApplicationFieldCurrency, ApplicationFieldRef, ApplicationRelationRef, TypedApplicationValue,
    WritePosture,
};
use worth_foundational::facade::AspectValue;

use super::{
    ApplicationCapabilityContextRef, ApplicationCapabilityCurrentnessDefinition,
    ApplicationCapabilityProvenanceRef,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationCapabilityFieldBinding {
    entity: String,
    aspect: String,
    field: String,
    value_type: String,
}

impl ApplicationCapabilityFieldBinding {
    pub fn from_reference<Schema, Entity, Aspect, Field, Value, Write, Equality, Currency>(
        field: ApplicationFieldRef<Schema, Entity, Aspect, Field, Value, Write, Equality, Currency>,
    ) -> Self
    where
        Value: TypedApplicationValue,
        Write: WritePosture,
        Currency: ApplicationFieldCurrency,
    {
        Self {
            entity: field.entity().to_string(),
            aspect: field.aspect().to_string(),
            field: field.field().to_string(),
            value_type: field.value_type_name().to_string(),
        }
    }

    pub fn entity(&self) -> &str {
        &self.entity
    }

    pub fn aspect(&self) -> &str {
        &self.aspect
    }

    pub fn field(&self) -> &str {
        &self.field
    }

    pub fn value_type(&self) -> &str {
        &self.value_type
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationCapabilityValueBinding {
    field: ApplicationCapabilityFieldBinding,
    value: AspectValue,
}

impl ApplicationCapabilityValueBinding {
    pub fn new<Schema, Entity, Aspect, Field, Value, Write, Equality, Currency>(
        field: ApplicationFieldRef<Schema, Entity, Aspect, Field, Value, Write, Equality, Currency>,
        value: Value,
    ) -> Self
    where
        Value: TypedApplicationValue,
        Write: WritePosture,
        Currency: ApplicationFieldCurrency,
    {
        Self {
            field: ApplicationCapabilityFieldBinding::from_reference(field),
            value: value.into_foundational_value(),
        }
    }

    pub const fn field(&self) -> &ApplicationCapabilityFieldBinding {
        &self.field
    }

    pub const fn value(&self) -> &AspectValue {
        &self.value
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationCapabilityRelationBinding {
    relation: String,
    from: String,
    to: String,
}

impl ApplicationCapabilityRelationBinding {
    pub fn from_reference<Schema, Relation, From, To>(
        relation: ApplicationRelationRef<Schema, Relation, From, To>,
    ) -> Self {
        Self {
            relation: relation.name().to_string(),
            from: relation.from().to_string(),
            to: relation.to().to_string(),
        }
    }

    pub fn relation(&self) -> &str {
        &self.relation
    }

    pub fn from(&self) -> &str {
        &self.from
    }

    pub fn to(&self) -> &str {
        &self.to
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ApplicationCapabilityFieldDimension {
    NotApplicable,
    Bound(ApplicationCapabilityFieldBinding),
}

impl ApplicationCapabilityFieldDimension {
    pub const fn not_applicable() -> Self {
        Self::NotApplicable
    }

    pub fn bound<Schema, Entity, Aspect, Field, Value, Write, Equality, Currency>(
        field: ApplicationFieldRef<Schema, Entity, Aspect, Field, Value, Write, Equality, Currency>,
    ) -> Self
    where
        Value: TypedApplicationValue,
        Write: WritePosture,
        Currency: ApplicationFieldCurrency,
    {
        Self::Bound(ApplicationCapabilityFieldBinding::from_reference(field))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ApplicationCapabilityRelationDimension {
    NotApplicable,
    Bound(ApplicationCapabilityRelationBinding),
}

impl ApplicationCapabilityRelationDimension {
    pub const fn not_applicable() -> Self {
        Self::NotApplicable
    }

    pub fn bound<Schema, Relation, From, To>(
        relation: ApplicationRelationRef<Schema, Relation, From, To>,
    ) -> Self {
        Self::Bound(ApplicationCapabilityRelationBinding::from_reference(
            relation,
        ))
    }
}

pub type ApplicationCapabilityAmountDimension = ApplicationCapabilityFieldDimension;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ApplicationCapabilityCardinalityDimension {
    One,
    Many,
    Bounded(u32),
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationCapabilityTargetDefinition {
    action: ApplicationCapabilityValueBinding,
    resource: ApplicationCapabilityRelationBinding,
    relation: ApplicationCapabilityRelationDimension,
    field: ApplicationCapabilityFieldDimension,
    purpose: ApplicationCapabilityValueBinding,
}

impl ApplicationCapabilityTargetDefinition {
    pub fn new(
        action: ApplicationCapabilityValueBinding,
        resource: ApplicationCapabilityRelationBinding,
        relation: ApplicationCapabilityRelationDimension,
        field: ApplicationCapabilityFieldDimension,
        purpose: ApplicationCapabilityValueBinding,
    ) -> Self {
        Self {
            action,
            resource,
            relation,
            field,
            purpose,
        }
    }

    pub fn action(&self) -> &ApplicationCapabilityValueBinding {
        &self.action
    }

    pub fn resource(&self) -> &ApplicationCapabilityRelationBinding {
        &self.resource
    }

    pub const fn relation(&self) -> &ApplicationCapabilityRelationDimension {
        &self.relation
    }

    pub const fn field(&self) -> &ApplicationCapabilityFieldDimension {
        &self.field
    }

    pub fn purpose(&self) -> &ApplicationCapabilityValueBinding {
        &self.purpose
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationCapabilityConstraintDefinition {
    amount: ApplicationCapabilityAmountDimension,
    cardinality: ApplicationCapabilityCardinalityDimension,
    currentness: ApplicationCapabilityCurrentnessDefinition,
    context: String,
    context_type: String,
}

impl ApplicationCapabilityConstraintDefinition {
    pub fn new<Schema, Context>(
        amount: ApplicationCapabilityAmountDimension,
        cardinality: ApplicationCapabilityCardinalityDimension,
        currentness: ApplicationCapabilityCurrentnessDefinition,
        context: ApplicationCapabilityContextRef<Schema, Context>,
    ) -> Self {
        Self {
            amount,
            cardinality,
            currentness,
            context: context.name().to_string(),
            context_type: context.marker_type().to_string(),
        }
    }

    pub const fn amount(&self) -> &ApplicationCapabilityAmountDimension {
        &self.amount
    }

    pub const fn cardinality(&self) -> ApplicationCapabilityCardinalityDimension {
        self.cardinality
    }

    pub const fn currentness(&self) -> &ApplicationCapabilityCurrentnessDefinition {
        &self.currentness
    }

    pub fn context(&self) -> &str {
        &self.context
    }

    pub fn context_type(&self) -> &str {
        &self.context_type
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationCapabilityDelegationDefinition {
    parent: ApplicationCapabilityRelationBinding,
    grantor: ApplicationCapabilityRelationBinding,
    grantee: ApplicationCapabilityRelationBinding,
    limit: ApplicationCapabilityFieldBinding,
    provenance: String,
    provenance_type: String,
}

impl ApplicationCapabilityDelegationDefinition {
    pub fn new<Schema, Provenance>(
        parent: ApplicationCapabilityRelationBinding,
        grantor: ApplicationCapabilityRelationBinding,
        grantee: ApplicationCapabilityRelationBinding,
        limit: ApplicationCapabilityFieldBinding,
        provenance: ApplicationCapabilityProvenanceRef<Schema, Provenance>,
    ) -> Self {
        Self {
            parent,
            grantor,
            grantee,
            limit,
            provenance: provenance.name().to_string(),
            provenance_type: provenance.marker_type().to_string(),
        }
    }

    pub fn parent(&self) -> &ApplicationCapabilityRelationBinding {
        &self.parent
    }

    pub fn grantor(&self) -> &ApplicationCapabilityRelationBinding {
        &self.grantor
    }

    pub fn grantee(&self) -> &ApplicationCapabilityRelationBinding {
        &self.grantee
    }

    pub fn limit(&self) -> &ApplicationCapabilityFieldBinding {
        &self.limit
    }

    pub fn provenance(&self) -> &str {
        &self.provenance
    }

    pub fn provenance_type(&self) -> &str {
        &self.provenance_type
    }
}
