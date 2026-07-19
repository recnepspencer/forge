use super::{AspectFieldKey, AspectName, AuthoringError, FieldName, WorthQueryPredicateOperand};

fn validate_predicate_target(
    aspect: impl Into<String>,
    field: impl Into<String>,
) -> Result<AspectFieldKey, AuthoringError> {
    AspectFieldKey::from_authoring_parts(aspect, field)
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EqualityPredicate {
    target: AspectFieldKey,
    value: WorthQueryPredicateOperand,
}

impl EqualityPredicate {
    pub fn new(
        aspect: impl Into<String>,
        field: impl Into<String>,
        value: impl Into<WorthQueryPredicateOperand>,
    ) -> Result<Self, AuthoringError> {
        let target = validate_predicate_target(aspect, field)?;
        Ok(Self {
            target,
            value: value.into(),
        })
    }

    pub fn from_target_field_key(
        target: AspectFieldKey,
        value: impl Into<WorthQueryPredicateOperand>,
    ) -> Self {
        Self {
            target,
            value: value.into(),
        }
    }

    pub fn target_field_key(&self) -> &AspectFieldKey {
        &self.target
    }

    pub fn aspect(&self) -> &str {
        self.target.aspect().as_str()
    }

    pub fn field(&self) -> &str {
        self.target.field().as_str()
    }

    pub fn aspect_name(&self) -> &AspectName {
        self.target.aspect()
    }

    pub fn field_name(&self) -> &FieldName {
        self.target.field()
    }

    pub fn value(&self) -> &WorthQueryPredicateOperand {
        &self.value
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum NativeComparisonOperator {
    GreaterThan,
    LessThan,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NativeComparisonPredicate {
    target: AspectFieldKey,
    operator: NativeComparisonOperator,
    value: WorthQueryPredicateOperand,
}

impl NativeComparisonPredicate {
    pub fn greater_than(
        aspect: impl Into<String>,
        field: impl Into<String>,
        value: i64,
    ) -> Result<Self, AuthoringError> {
        Self::greater_than_native(aspect, field, value)
    }

    pub fn greater_than_native(
        aspect: impl Into<String>,
        field: impl Into<String>,
        value: impl Into<WorthQueryPredicateOperand>,
    ) -> Result<Self, AuthoringError> {
        let target = validate_predicate_target(aspect, field)?;
        Ok(Self {
            target,
            operator: NativeComparisonOperator::GreaterThan,
            value: value.into(),
        })
    }

    pub fn less_than(
        aspect: impl Into<String>,
        field: impl Into<String>,
        value: i64,
    ) -> Result<Self, AuthoringError> {
        Self::less_than_native(aspect, field, value)
    }

    pub fn less_than_native(
        aspect: impl Into<String>,
        field: impl Into<String>,
        value: impl Into<WorthQueryPredicateOperand>,
    ) -> Result<Self, AuthoringError> {
        let target = validate_predicate_target(aspect, field)?;
        Ok(Self {
            target,
            operator: NativeComparisonOperator::LessThan,
            value: value.into(),
        })
    }

    pub fn greater_than_target_field_key(target: AspectFieldKey, value: i64) -> Self {
        Self::greater_than_native_target_field_key(target, value)
    }

    pub fn greater_than_native_target_field_key(
        target: AspectFieldKey,
        value: impl Into<WorthQueryPredicateOperand>,
    ) -> Self {
        Self {
            target,
            operator: NativeComparisonOperator::GreaterThan,
            value: value.into(),
        }
    }

    pub fn less_than_target_field_key(target: AspectFieldKey, value: i64) -> Self {
        Self::less_than_native_target_field_key(target, value)
    }

    pub fn less_than_native_target_field_key(
        target: AspectFieldKey,
        value: impl Into<WorthQueryPredicateOperand>,
    ) -> Self {
        Self {
            target,
            operator: NativeComparisonOperator::LessThan,
            value: value.into(),
        }
    }

    pub fn target_field_key(&self) -> &AspectFieldKey {
        &self.target
    }

    pub fn aspect(&self) -> &str {
        self.target.aspect().as_str()
    }

    pub fn field(&self) -> &str {
        self.target.field().as_str()
    }

    pub fn aspect_name(&self) -> &AspectName {
        self.target.aspect()
    }

    pub fn field_name(&self) -> &FieldName {
        self.target.field()
    }

    pub fn operator(&self) -> NativeComparisonOperator {
        self.operator
    }

    pub fn value(&self) -> &WorthQueryPredicateOperand {
        &self.value
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct StringContainsPredicate {
    target: AspectFieldKey,
    value: String,
}

impl StringContainsPredicate {
    pub fn new(
        aspect: impl Into<String>,
        field: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, AuthoringError> {
        let target = validate_predicate_target(aspect, field)?;
        let value = value.into();
        Ok(Self { target, value })
    }

    pub fn from_target_field_key(target: AspectFieldKey, value: impl Into<String>) -> Self {
        Self {
            target,
            value: value.into(),
        }
    }

    pub fn target_field_key(&self) -> &AspectFieldKey {
        &self.target
    }

    pub fn aspect(&self) -> &str {
        self.target.aspect().as_str()
    }

    pub fn field(&self) -> &str {
        self.target.field().as_str()
    }

    pub fn aspect_name(&self) -> &AspectName {
        self.target.aspect()
    }

    pub fn field_name(&self) -> &FieldName {
        self.target.field()
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SetMembershipPredicate {
    target: AspectFieldKey,
    values: Vec<WorthQueryPredicateOperand>,
}

impl SetMembershipPredicate {
    pub fn new<Value>(
        aspect: impl Into<String>,
        field: impl Into<String>,
        values: impl IntoIterator<Item = Value>,
    ) -> Result<Self, AuthoringError>
    where
        Value: Into<WorthQueryPredicateOperand>,
    {
        let target = validate_predicate_target(aspect, field)?;
        let values: Vec<_> = values.into_iter().map(Into::into).collect();
        if values.is_empty() {
            return Err(AuthoringError::EmptyProjectionSet);
        }
        Ok(Self { target, values })
    }

    pub fn from_target_field_key<Value>(
        target: AspectFieldKey,
        values: impl IntoIterator<Item = Value>,
    ) -> Result<Self, AuthoringError>
    where
        Value: Into<WorthQueryPredicateOperand>,
    {
        let values: Vec<_> = values.into_iter().map(Into::into).collect();
        if values.is_empty() {
            return Err(AuthoringError::EmptyProjectionSet);
        }
        Ok(Self { target, values })
    }

    pub fn target_field_key(&self) -> &AspectFieldKey {
        &self.target
    }

    pub fn aspect(&self) -> &str {
        self.target.aspect().as_str()
    }

    pub fn field(&self) -> &str {
        self.target.field().as_str()
    }

    pub fn aspect_name(&self) -> &AspectName {
        self.target.aspect()
    }

    pub fn field_name(&self) -> &FieldName {
        self.target.field()
    }

    pub fn values(&self) -> &[WorthQueryPredicateOperand] {
        &self.values
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PresencePredicateKind {
    IsPresent,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PresencePredicate {
    target: AspectFieldKey,
    kind: PresencePredicateKind,
}

impl PresencePredicate {
    pub fn is_present(
        aspect: impl Into<String>,
        field: impl Into<String>,
    ) -> Result<Self, AuthoringError> {
        let target = validate_predicate_target(aspect, field)?;
        Ok(Self {
            target,
            kind: PresencePredicateKind::IsPresent,
        })
    }

    pub fn is_present_target_field_key(target: AspectFieldKey) -> Self {
        Self {
            target,
            kind: PresencePredicateKind::IsPresent,
        }
    }

    pub fn target_field_key(&self) -> &AspectFieldKey {
        &self.target
    }

    pub fn aspect(&self) -> &str {
        self.target.aspect().as_str()
    }

    pub fn field(&self) -> &str {
        self.target.field().as_str()
    }

    pub fn aspect_name(&self) -> &AspectName {
        self.target.aspect()
    }

    pub fn field_name(&self) -> &FieldName {
        self.target.field()
    }

    pub fn kind(&self) -> PresencePredicateKind {
        self.kind
    }
}

impl PresencePredicateKind {
    pub fn digest_key(self) -> &'static str {
        match self {
            Self::IsPresent => "is-present",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PredicateSelector {
    Equality(EqualityPredicate),
    NativeComparison(NativeComparisonPredicate),
    StringContains(StringContainsPredicate),
    SetMembership(SetMembershipPredicate),
    Presence(PresencePredicate),
}

impl PredicateSelector {
    pub fn target_field_key(&self) -> &AspectFieldKey {
        match self {
            Self::Equality(predicate) => predicate.target_field_key(),
            Self::NativeComparison(predicate) => predicate.target_field_key(),
            Self::StringContains(predicate) => predicate.target_field_key(),
            Self::SetMembership(predicate) => predicate.target_field_key(),
            Self::Presence(predicate) => predicate.target_field_key(),
        }
    }
}
