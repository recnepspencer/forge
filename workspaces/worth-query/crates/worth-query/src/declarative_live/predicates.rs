use crate::authoring::{AspectFieldKey, NativeComparisonOperator, WorthQueryPredicateOperand};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarativeEqualityFilter {
    source: AspectFieldKey,
    value: WorthQueryPredicateOperand,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarativeNativeComparisonFilter {
    source: AspectFieldKey,
    operator: NativeComparisonOperator,
    value: WorthQueryPredicateOperand,
}

impl DeclarativeNativeComparisonFilter {
    pub fn greater_than(source: AspectFieldKey, value: i64) -> Self {
        Self::greater_than_native(source, value)
    }

    pub fn greater_than_native(
        source: AspectFieldKey,
        value: impl Into<WorthQueryPredicateOperand>,
    ) -> Self {
        Self {
            source,
            operator: NativeComparisonOperator::GreaterThan,
            value: value.into(),
        }
    }

    pub fn less_than(source: AspectFieldKey, value: i64) -> Self {
        Self::less_than_native(source, value)
    }

    pub fn less_than_native(
        source: AspectFieldKey,
        value: impl Into<WorthQueryPredicateOperand>,
    ) -> Self {
        Self {
            source,
            operator: NativeComparisonOperator::LessThan,
            value: value.into(),
        }
    }

    pub fn source_field_key(&self) -> &AspectFieldKey {
        &self.source
    }

    pub fn operator(&self) -> NativeComparisonOperator {
        self.operator
    }

    pub fn value(&self) -> &WorthQueryPredicateOperand {
        &self.value
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarativeStringContainsFilter {
    source: AspectFieldKey,
    value: String,
}

impl DeclarativeStringContainsFilter {
    pub fn new(source: AspectFieldKey, value: impl Into<String>) -> Self {
        Self {
            source,
            value: value.into(),
        }
    }

    pub fn source_field_key(&self) -> &AspectFieldKey {
        &self.source
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarativeSetMembershipFilter {
    source: AspectFieldKey,
    values: Vec<WorthQueryPredicateOperand>,
}

impl DeclarativeSetMembershipFilter {
    pub fn new(
        source: AspectFieldKey,
        values: impl IntoIterator<Item = WorthQueryPredicateOperand>,
    ) -> Self {
        Self {
            source,
            values: values.into_iter().collect(),
        }
    }

    pub fn source_field_key(&self) -> &AspectFieldKey {
        &self.source
    }

    pub fn values(&self) -> &[WorthQueryPredicateOperand] {
        &self.values
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeclarativePresenceFilterKind {
    IsPresent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarativePresenceFilter {
    source: AspectFieldKey,
    kind: DeclarativePresenceFilterKind,
}

impl DeclarativePresenceFilter {
    pub fn is_present(source: AspectFieldKey) -> Self {
        Self {
            source,
            kind: DeclarativePresenceFilterKind::IsPresent,
        }
    }

    pub fn source_field_key(&self) -> &AspectFieldKey {
        &self.source
    }

    pub fn kind(&self) -> DeclarativePresenceFilterKind {
        self.kind
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeclarativePredicateFilter {
    Equality(DeclarativeEqualityFilter),
    NativeComparison(DeclarativeNativeComparisonFilter),
    StringContains(DeclarativeStringContainsFilter),
    SetMembership(DeclarativeSetMembershipFilter),
    Presence(DeclarativePresenceFilter),
}

impl DeclarativePredicateFilter {
    pub fn source_field_key(&self) -> &AspectFieldKey {
        match self {
            Self::Equality(filter) => filter.source_field_key(),
            Self::NativeComparison(filter) => filter.source_field_key(),
            Self::StringContains(filter) => filter.source_field_key(),
            Self::SetMembership(filter) => filter.source_field_key(),
            Self::Presence(filter) => filter.source_field_key(),
        }
    }
}

impl DeclarativeEqualityFilter {
    pub fn new(source: AspectFieldKey, value: WorthQueryPredicateOperand) -> Self {
        Self { source, value }
    }

    pub fn source_field_key(&self) -> &AspectFieldKey {
        &self.source
    }

    pub fn value(&self) -> &WorthQueryPredicateOperand {
        &self.value
    }
}
