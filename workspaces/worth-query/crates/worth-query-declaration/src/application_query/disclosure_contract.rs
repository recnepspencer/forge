use std::collections::BTreeSet;

use worth_foundational::facade::{
    AspectMask, AspectValue, CanonicalFieldPath, DiagnosticMask, FieldKey, ProjectionMask,
};

use super::{
    ApplicationQueryCardinality, ApplicationQueryResultFieldRef,
    ApplicationQueryResultRelationCardinality, ApplicationQueryResultRelationRef,
    ApplicationQueryResultSlotKey, ApplicationQueryResultTraversal,
    ApplicationQueryResultTraversalDirection,
};
use crate::application_capability::ApplicationCapabilityRef;
use crate::application_schema::{
    ApplicationFieldCurrency, ApplicationFieldRef, TypedApplicationValue,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ApplicationQueryDisclosurePosture {
    Public,
    InstalledPolicyRequired,
    Governed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ApplicationQueryObservableInfluence {
    RowPresence,
    Ordering,
    Pagination,
    Count,
    Aggregate,
    Explanation,
    HistoricalMembership,
    Preview,
    LiveMembership,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationQueryInfluenceContract {
    permitted: BTreeSet<ApplicationQueryObservableInfluence>,
}

impl ApplicationQueryInfluenceContract {
    pub const fn forbid_all() -> Self {
        Self {
            permitted: BTreeSet::new(),
        }
    }

    pub fn permit(
        surfaces: impl IntoIterator<Item = ApplicationQueryObservableInfluence>,
    ) -> Self {
        Self {
            permitted: surfaces.into_iter().collect(),
        }
    }

    pub fn permit_all() -> Self {
        Self::permit([
            ApplicationQueryObservableInfluence::RowPresence,
            ApplicationQueryObservableInfluence::Ordering,
            ApplicationQueryObservableInfluence::Pagination,
            ApplicationQueryObservableInfluence::Count,
            ApplicationQueryObservableInfluence::Aggregate,
            ApplicationQueryObservableInfluence::Explanation,
            ApplicationQueryObservableInfluence::HistoricalMembership,
            ApplicationQueryObservableInfluence::Preview,
            ApplicationQueryObservableInfluence::LiveMembership,
        ])
    }

    pub fn permits(&self, surface: ApplicationQueryObservableInfluence) -> bool {
        self.permitted.contains(&surface)
    }

    pub fn permitted(&self) -> impl ExactSizeIterator<Item = ApplicationQueryObservableInfluence> + '_ {
        self.permitted.iter().copied()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ApplicationQueryDisclosureSelector {
    InternalField {
        entity: &'static str,
        aspect: &'static str,
        field: &'static str,
        projection_mask: AspectMask<ProjectionMask>,
        diagnostic_mask: AspectMask<DiagnosticMask>,
    },
    Field {
        slot_key: ApplicationQueryResultSlotKey,
        query_type: &'static str,
        slot_type: &'static str,
        entity: &'static str,
        aspect: &'static str,
        field: &'static str,
        output_name: &'static str,
        projection_mask: AspectMask<ProjectionMask>,
        diagnostic_mask: AspectMask<DiagnosticMask>,
    },
    Relation {
        slot_key: ApplicationQueryResultSlotKey,
        query_type: &'static str,
        slot_type: &'static str,
        relation: &'static str,
        from: &'static str,
        to: &'static str,
        direction: ApplicationQueryResultTraversalDirection,
        cardinality: ApplicationQueryCardinality,
        output_name: &'static str,
    },
}

impl ApplicationQueryDisclosureSelector {
    pub const fn result_slot_key(&self) -> Option<ApplicationQueryResultSlotKey> {
        match self {
            Self::Field { slot_key, .. } | Self::Relation { slot_key, .. } => Some(*slot_key),
            Self::InternalField { .. } => None,
        }
    }

    pub const fn is_internal_computation(&self) -> bool {
        matches!(self, Self::InternalField { .. })
    }

    pub const fn slot_type(&self) -> &'static str {
        match self {
            Self::Field { slot_type, .. } | Self::Relation { slot_type, .. } => slot_type,
            Self::InternalField { .. } => "internal-computation",
        }
    }

    pub const fn query_type(&self) -> &'static str {
        match self {
            Self::Field { query_type, .. } | Self::Relation { query_type, .. } => query_type,
            Self::InternalField { .. } => "internal-computation",
        }
    }

    pub const fn output_name(&self) -> &'static str {
        match self {
            Self::Field { output_name, .. } | Self::Relation { output_name, .. } => output_name,
            Self::InternalField { .. } => "internal-computation",
        }
    }

    pub const fn field_contract(&self) -> Option<(&'static str, &'static str, &'static str)> {
        match self {
            Self::InternalField {
                entity,
                aspect,
                field,
                ..
            }
            | Self::Field {
                entity,
                aspect,
                field,
                ..
            } => Some((entity, aspect, field)),
            Self::Relation { .. } => None,
        }
    }

    pub const fn relation_contract(
        &self,
    ) -> Option<(
        &'static str,
        &'static str,
        &'static str,
        ApplicationQueryResultTraversalDirection,
        ApplicationQueryCardinality,
    )> {
        match self {
            Self::Relation {
                relation,
                from,
                to,
                direction,
                cardinality,
                ..
            } => Some((relation, from, to, *direction, *cardinality)),
            Self::Field { .. } | Self::InternalField { .. } => None,
        }
    }

    pub const fn projection_mask(&self) -> Option<&AspectMask<ProjectionMask>> {
        match self {
            Self::InternalField {
                projection_mask, ..
            }
            | Self::Field {
                projection_mask, ..
            } => Some(projection_mask),
            Self::Relation { .. } => None,
        }
    }

    pub const fn diagnostic_mask(&self) -> Option<&AspectMask<DiagnosticMask>> {
        match self {
            Self::InternalField {
                diagnostic_mask, ..
            }
            | Self::Field {
                diagnostic_mask, ..
            } => Some(diagnostic_mask),
            Self::Relation { .. } => None,
        }
    }
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
        field: ApplicationFieldRef<
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
        let field_key = FieldKey::new(field.field())
            .expect("typed application fields are valid Foundational keys");
        self.rules.push(ApplicationQueryDisclosureRule {
            selector: ApplicationQueryDisclosureSelector::InternalField {
                entity: field.entity(),
                aspect: field.aspect(),
                field: field.field(),
                projection_mask: AspectMask::new([CanonicalFieldPath::single(
                    field_key.clone(),
                )]),
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
