use std::collections::BTreeMap;

use worth_foundational::facade::AspectValue;

use crate::application_schema::TypedApplicationValue;

/// Descriptive typed value used while comparing capability scopes.
///
/// This value carries no Query installation or execution authority.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationCapabilityValue(AspectValue);

impl ApplicationCapabilityValue {
    pub fn from_typed<Value>(value: Value) -> Self
    where
        Value: TypedApplicationValue,
    {
        Self(value.into_foundational_value())
    }

    pub const fn foundational(&self) -> &AspectValue {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationCapabilityValueSet {
    values: Vec<ApplicationCapabilityValue>,
}

impl ApplicationCapabilityValueSet {
    pub fn from_typed<Value>(
        values: impl IntoIterator<Item = Value>,
    ) -> Option<ApplicationCapabilityValueSet>
    where
        Value: TypedApplicationValue,
    {
        let mut values = values
            .into_iter()
            .map(ApplicationCapabilityValue::from_typed)
            .collect::<Vec<_>>();
        values.sort();
        values.dedup();
        (!values.is_empty()).then_some(Self { values })
    }

    pub fn values(&self) -> &[ApplicationCapabilityValue] {
        &self.values
    }

    fn is_subset_of(&self, parent: &Self) -> bool {
        self.values
            .iter()
            .all(|candidate| parent.values.binary_search(candidate).is_ok())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ApplicationCapabilityOptionalValueSet {
    NotApplicable,
    Values(ApplicationCapabilityValueSet),
}

impl ApplicationCapabilityOptionalValueSet {
    pub const fn not_applicable() -> Self {
        Self::NotApplicable
    }

    pub fn from_typed<Value>(values: impl IntoIterator<Item = Value>) -> Option<Self>
    where
        Value: TypedApplicationValue,
    {
        ApplicationCapabilityValueSet::from_typed(values).map(Self::Values)
    }

    fn is_within(&self, parent: &Self) -> bool {
        match (self, parent) {
            (Self::NotApplicable, Self::NotApplicable) => true,
            (Self::Values(child), Self::Values(parent)) => child.is_subset_of(parent),
            (Self::NotApplicable, Self::Values(_)) | (Self::Values(_), Self::NotApplicable) => {
                false
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationCapabilityTargetScope {
    action: ApplicationCapabilityValue,
    resource: ApplicationCapabilityValue,
    relation: ApplicationCapabilityOptionalValueSet,
    field: ApplicationCapabilityOptionalValueSet,
    purpose: ApplicationCapabilityValue,
}

impl ApplicationCapabilityTargetScope {
    pub fn new(
        action: ApplicationCapabilityValue,
        resource: ApplicationCapabilityValue,
        relation: ApplicationCapabilityOptionalValueSet,
        field: ApplicationCapabilityOptionalValueSet,
        purpose: ApplicationCapabilityValue,
    ) -> Self {
        Self {
            action,
            resource,
            relation,
            field,
            purpose,
        }
    }

    fn is_within(&self, parent: &Self) -> bool {
        self.action == parent.action
            && self.resource == parent.resource
            && self.relation.is_within(&parent.relation)
            && self.field.is_within(&parent.field)
            && self.purpose == parent.purpose
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationCapabilityAmountValue {
    currency: String,
    scale: u32,
    units: i128,
}

impl ApplicationCapabilityAmountValue {
    pub fn new(currency: impl Into<String>, scale: u32, units: i128) -> Option<Self> {
        let currency = currency.into();
        (!currency.is_empty()).then_some(Self {
            currency,
            scale,
            units,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ApplicationCapabilityAmountScope {
    NotApplicable,
    Ceiling(ApplicationCapabilityAmountValue),
}

impl ApplicationCapabilityAmountScope {
    pub const fn not_applicable() -> Self {
        Self::NotApplicable
    }

    pub const fn ceiling(value: ApplicationCapabilityAmountValue) -> Self {
        Self::Ceiling(value)
    }

    fn is_within(&self, parent: &Self) -> bool {
        match (self, parent) {
            (Self::NotApplicable, Self::NotApplicable) => true,
            (Self::Ceiling(child), Self::Ceiling(parent)) => {
                child.currency == parent.currency
                    && child.scale == parent.scale
                    && child.units <= parent.units
            }
            (Self::NotApplicable, Self::Ceiling(_)) | (Self::Ceiling(_), Self::NotApplicable) => {
                false
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationCapabilityValidityWindow {
    timeline: String,
    not_before: i128,
    not_after: i128,
}

impl ApplicationCapabilityValidityWindow {
    pub fn new(timeline: impl Into<String>, not_before: i128, not_after: i128) -> Option<Self> {
        let timeline = timeline.into();
        (!timeline.is_empty() && not_before <= not_after).then_some(Self {
            timeline,
            not_before,
            not_after,
        })
    }

    fn is_within(&self, parent: &Self) -> bool {
        self.timeline == parent.timeline
            && parent.not_before <= self.not_before
            && self.not_after <= parent.not_after
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationCapabilityLimitScope {
    amount: ApplicationCapabilityAmountScope,
    cardinality: u64,
    workflow_stage: ApplicationCapabilityValue,
    validity: ApplicationCapabilityValidityWindow,
}

impl ApplicationCapabilityLimitScope {
    pub fn new(
        amount: ApplicationCapabilityAmountScope,
        cardinality: u64,
        workflow_stage: ApplicationCapabilityValue,
        validity: ApplicationCapabilityValidityWindow,
    ) -> Option<Self> {
        (cardinality > 0).then_some(Self {
            amount,
            cardinality,
            workflow_stage,
            validity,
        })
    }

    fn is_within(&self, parent: &Self) -> bool {
        self.amount.is_within(&parent.amount)
            && self.cardinality <= parent.cardinality
            && self.workflow_stage == parent.workflow_stage
            && self.validity.is_within(&parent.validity)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationCapabilityDelegationScope {
    remaining: u32,
    provenance: Vec<String>,
}

impl ApplicationCapabilityDelegationScope {
    pub fn new(
        remaining: u32,
        provenance: impl IntoIterator<Item = impl Into<String>>,
    ) -> Option<Self> {
        let provenance = provenance.into_iter().map(Into::into).collect::<Vec<_>>();
        (!provenance.is_empty() && provenance.iter().all(|entry| !entry.is_empty())).then_some(
            Self {
                remaining,
                provenance,
            },
        )
    }

    fn is_within(&self, parent: &Self) -> bool {
        self.remaining < parent.remaining && self.provenance.starts_with(&parent.provenance)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApplicationCapabilityContextScope {
    constraints: BTreeMap<String, ApplicationCapabilityValue>,
}

impl ApplicationCapabilityContextScope {
    pub fn new(
        constraints: impl IntoIterator<Item = (impl Into<String>, ApplicationCapabilityValue)>,
    ) -> Option<Self> {
        let mut collected = BTreeMap::new();
        for (key, value) in constraints {
            let key = key.into();
            if key.is_empty() || collected.insert(key, value).is_some() {
                return None;
            }
        }
        Some(Self {
            constraints: collected,
        })
    }

    fn is_within(&self, parent: &Self) -> bool {
        parent.constraints.iter().all(|(key, value)| {
            self.constraints
                .get(key)
                .is_some_and(|candidate| candidate == value)
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApplicationCapabilityScope {
    target: ApplicationCapabilityTargetScope,
    limits: ApplicationCapabilityLimitScope,
    delegation: ApplicationCapabilityDelegationScope,
    context: ApplicationCapabilityContextScope,
}

impl ApplicationCapabilityScope {
    pub fn new(
        target: ApplicationCapabilityTargetScope,
        limits: ApplicationCapabilityLimitScope,
        delegation: ApplicationCapabilityDelegationScope,
        context: ApplicationCapabilityContextScope,
    ) -> Self {
        Self {
            target,
            limits,
            delegation,
            context,
        }
    }

    pub fn is_within(&self, parent: &Self) -> bool {
        self.target.is_within(&parent.target)
            && self.limits.is_within(&parent.limits)
            && self.delegation.is_within(&parent.delegation)
            && self.context.is_within(&parent.context)
    }
}
