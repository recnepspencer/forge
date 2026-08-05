use worth_foundational::facade::{AspectMask, AspectValue, CanonicalFieldPath, FieldKey};

use super::{
    ApplicationQueryOptionalResultFieldRef, ApplicationQueryResultFieldRef,
    ApplicationQueryResultRelationCardinality, ApplicationQueryResultRelationRef,
    ApplicationQueryResultTraversal,
};
use crate::application_capability::ApplicationCapabilityRef;
use crate::application_schema::{
    ApplicationFieldCurrency, ApplicationFieldRef, OptionalApplicationFieldValue,
    TypedApplicationValue,
};

mod influence;
mod selector;
pub use influence::{ApplicationQueryInfluenceContract, ApplicationQueryObservableInfluence};
pub use selector::ApplicationQueryDisclosureSelector;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ApplicationQueryDisclosurePosture {
    Public,
    InstalledPolicyRequired,
    Governed,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationQueryDisclosureRule {
    selector: ApplicationQueryDisclosureSelector,
    disclosure_value: AspectValue,
    influence: ApplicationQueryInfluenceContract,
}

impl ApplicationQueryDisclosureRule {
    pub const fn selector(&self) -> &ApplicationQueryDisclosureSelector {
        &self.selector
    }

    pub const fn disclosure_value(&self) -> &AspectValue {
        &self.disclosure_value
    }

    pub const fn influence(&self) -> &ApplicationQueryInfluenceContract {
        &self.influence
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationQueryDisclosureContract {
    posture: ApplicationQueryDisclosurePosture,
    classification: &'static str,
    capability_name: Option<&'static str>,
    capability_type: Option<&'static str>,
    rules: Vec<ApplicationQueryDisclosureRule>,
}

impl ApplicationQueryDisclosureContract {
    pub const fn public() -> Self {
        Self {
            posture: ApplicationQueryDisclosurePosture::Public,
            classification: "public",
            capability_name: None,
            capability_type: None,
            rules: Vec::new(),
        }
    }

    pub const fn installed_policy(classification: &'static str) -> Self {
        Self {
            posture: ApplicationQueryDisclosurePosture::InstalledPolicyRequired,
            classification,
            capability_name: None,
            capability_type: None,
            rules: Vec::new(),
        }
    }

    pub fn governed_by<Schema, Capability>(
        classification: &'static str,
        capability: ApplicationCapabilityRef<Schema, Capability>,
    ) -> Self {
        Self {
            posture: ApplicationQueryDisclosurePosture::Governed,
            classification,
            capability_name: Some(capability.name()),
            capability_type: Some(capability.marker_type()),
            rules: Vec::new(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn use_field_by<
        Schema,
        Entity,
        Aspect,
        Field,
        Value,
        Write,
        Equality,
        Currency,
        DisclosureValue,
    >(
        mut self,
        field: ApplicationFieldRef<Schema, Entity, Aspect, Field, Value, Write, Equality, Currency>,
        disclosure_value: DisclosureValue,
        influence: ApplicationQueryInfluenceContract,
    ) -> Self
    where
        Value: TypedApplicationValue,
        Currency: ApplicationFieldCurrency,
        DisclosureValue: TypedApplicationValue,
    {
        let field_key = FieldKey::new(field.field())
            .expect("typed application fields are valid Foundational keys");
        self.rules.push(ApplicationQueryDisclosureRule {
            selector: ApplicationQueryDisclosureSelector::InternalField {
                entity: field.entity(),
                aspect: field.aspect(),
                field: field.field(),
                projection_mask: AspectMask::new([CanonicalFieldPath::single(field_key.clone())]),
                diagnostic_mask: AspectMask::new([CanonicalFieldPath::single(field_key)]),
            },
            disclosure_value: disclosure_value.into_foundational_value(),
            influence,
        });
        self.rules.sort();
        self
    }

    #[allow(clippy::too_many_arguments)]
    pub fn disclose_field_by<
        Query: 'static,
        Slot: 'static,
        Schema,
        Entity,
        Aspect,
        Field,
        Value,
        Write,
        Equality,
        Currency,
        DisclosureValue,
    >(
        mut self,
        selector: ApplicationQueryResultFieldRef<
            Query,
            Slot,
            Schema,
            Entity,
            Aspect,
            Field,
            Value,
            Write,
            Equality,
            Currency,
        >,
        disclosure_value: DisclosureValue,
        influence: ApplicationQueryInfluenceContract,
    ) -> Self
    where
        Value: TypedApplicationValue,
        Currency: ApplicationFieldCurrency,
        DisclosureValue: TypedApplicationValue,
    {
        let field = FieldKey::new(selector.field())
            .expect("typed application-query fields are valid Foundational keys");
        self.rules.push(ApplicationQueryDisclosureRule {
            selector: ApplicationQueryDisclosureSelector::Field {
                slot_key: selector.slot_key(),
                query_type: selector.query_type(),
                slot_type: selector.slot_type(),
                entity: selector.entity(),
                aspect: selector.aspect(),
                field: selector.field(),
                output_name: selector.output_name(),
                projection_mask: AspectMask::new([CanonicalFieldPath::single(field.clone())]),
                diagnostic_mask: AspectMask::new([CanonicalFieldPath::single(field)]),
            },
            disclosure_value: disclosure_value.into_foundational_value(),
            influence,
        });
        self.rules.sort();
        self
    }

    #[allow(clippy::too_many_arguments)]
    pub fn disclose_optional_field_by<
        Query: 'static,
        Slot: 'static,
        Schema,
        Entity,
        Aspect,
        Field,
        Value,
        Write,
        Equality,
        Currency,
        DisclosureValue,
    >(
        mut self,
        selector: ApplicationQueryOptionalResultFieldRef<
            Query,
            Slot,
            Schema,
            Entity,
            Aspect,
            Field,
            Value,
            Write,
            Equality,
            Currency,
        >,
        disclosure_value: DisclosureValue,
        influence: ApplicationQueryInfluenceContract,
    ) -> Self
    where
        Field: OptionalApplicationFieldValue<Value = Value>,
        Value: TypedApplicationValue,
        Currency: ApplicationFieldCurrency,
        DisclosureValue: TypedApplicationValue,
    {
        let field = FieldKey::new(selector.field())
            .expect("typed application-query fields are valid Foundational keys");
        self.rules.push(ApplicationQueryDisclosureRule {
            selector: ApplicationQueryDisclosureSelector::Field {
                slot_key: selector.slot_key(),
                query_type: selector.query_type(),
                slot_type: selector.slot_type(),
                entity: selector.entity(),
                aspect: selector.aspect(),
                field: selector.field(),
                output_name: selector.output_name(),
                projection_mask: AspectMask::new([CanonicalFieldPath::single(field.clone())]),
                diagnostic_mask: AspectMask::new([CanonicalFieldPath::single(field)]),
            },
            disclosure_value: disclosure_value.into_foundational_value(),
            influence,
        });
        self.rules.sort();
        self
    }

    #[allow(clippy::too_many_arguments)]
    pub fn disclose_relation_by<
        Query: 'static,
        Slot: 'static,
        Schema,
        Relation,
        From,
        To,
        Direction,
        Cardinality,
        DisclosureValue,
    >(
        mut self,
        selector: ApplicationQueryResultRelationRef<
            Query,
            Slot,
            Schema,
            Relation,
            From,
            To,
            Direction,
            Cardinality,
        >,
        disclosure_value: DisclosureValue,
        influence: ApplicationQueryInfluenceContract,
    ) -> Self
    where
        Direction: ApplicationQueryResultTraversal,
        Cardinality: ApplicationQueryResultRelationCardinality,
        DisclosureValue: TypedApplicationValue,
    {
        self.rules.push(ApplicationQueryDisclosureRule {
            selector: ApplicationQueryDisclosureSelector::Relation {
                slot_key: selector.slot_key(),
                query_type: selector.query_type(),
                slot_type: selector.slot_type(),
                relation: selector.relation(),
                from: selector.from(),
                to: selector.to(),
                direction: selector.direction(),
                cardinality: selector.cardinality(),
                output_name: selector.output_name(),
            },
            disclosure_value: disclosure_value.into_foundational_value(),
            influence,
        });
        self.rules.sort();
        self
    }

    pub const fn posture(&self) -> ApplicationQueryDisclosurePosture {
        self.posture
    }

    pub const fn classification(&self) -> &'static str {
        self.classification
    }

    pub const fn capability_name(&self) -> Option<&'static str> {
        self.capability_name
    }

    pub const fn capability_type(&self) -> Option<&'static str> {
        self.capability_type
    }

    pub fn rules(&self) -> &[ApplicationQueryDisclosureRule] {
        &self.rules
    }
}
