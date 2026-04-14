use super::AuthoringError;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ScalarPredicateValue {
    String(String),
    Integer(i64),
    Boolean(bool),
}

impl ScalarPredicateValue {
    pub fn kind_name(&self) -> &'static str {
        match self {
            Self::String(_) => "String",
            Self::Integer(_) => "Integer",
            Self::Boolean(_) => "Boolean",
        }
    }
}

fn validate_predicate_target(
    aspect: impl Into<String>,
    field: impl Into<String>,
) -> Result<(String, String), AuthoringError> {
    let aspect = aspect.into();
    let field = field.into();
    if aspect.trim().is_empty() || field.trim().is_empty() {
        return Err(AuthoringError::EmptyProjectionSelector);
    }
    Ok((aspect, field))
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EqualityPredicate {
    aspect: String,
    field: String,
    value: ScalarPredicateValue,
}

impl EqualityPredicate {
    pub fn new(
        aspect: impl Into<String>,
        field: impl Into<String>,
        value: ScalarPredicateValue,
    ) -> Result<Self, AuthoringError> {
        let (aspect, field) = validate_predicate_target(aspect, field)?;
        Ok(Self {
            aspect,
            field,
            value,
        })
    }

    pub fn aspect(&self) -> &str {
        &self.aspect
    }

    pub fn field(&self) -> &str {
        &self.field
    }

    pub fn value(&self) -> &ScalarPredicateValue {
        &self.value
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum IntegerComparisonOperator {
    GreaterThan,
    LessThan,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IntegerComparisonPredicate {
    aspect: String,
    field: String,
    operator: IntegerComparisonOperator,
    value: i64,
}

impl IntegerComparisonPredicate {
    pub fn greater_than(
        aspect: impl Into<String>,
        field: impl Into<String>,
        value: i64,
    ) -> Result<Self, AuthoringError> {
        let (aspect, field) = validate_predicate_target(aspect, field)?;
        Ok(Self {
            aspect,
            field,
            operator: IntegerComparisonOperator::GreaterThan,
            value,
        })
    }

    pub fn less_than(
        aspect: impl Into<String>,
        field: impl Into<String>,
        value: i64,
    ) -> Result<Self, AuthoringError> {
        let (aspect, field) = validate_predicate_target(aspect, field)?;
        Ok(Self {
            aspect,
            field,
            operator: IntegerComparisonOperator::LessThan,
            value,
        })
    }

    pub fn aspect(&self) -> &str {
        &self.aspect
    }

    pub fn field(&self) -> &str {
        &self.field
    }

    pub fn operator(&self) -> IntegerComparisonOperator {
        self.operator
    }

    pub fn value(&self) -> i64 {
        self.value
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct StringContainsPredicate {
    aspect: String,
    field: String,
    value: String,
}

impl StringContainsPredicate {
    pub fn new(
        aspect: impl Into<String>,
        field: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, AuthoringError> {
        let (aspect, field) = validate_predicate_target(aspect, field)?;
        let value = value.into();
        Ok(Self {
            aspect,
            field,
            value,
        })
    }

    pub fn aspect(&self) -> &str {
        &self.aspect
    }

    pub fn field(&self) -> &str {
        &self.field
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SetMembershipPredicate {
    aspect: String,
    field: String,
    values: Vec<ScalarPredicateValue>,
}

impl SetMembershipPredicate {
    pub fn new(
        aspect: impl Into<String>,
        field: impl Into<String>,
        values: impl IntoIterator<Item = ScalarPredicateValue>,
    ) -> Result<Self, AuthoringError> {
        let (aspect, field) = validate_predicate_target(aspect, field)?;
        let values: Vec<_> = values.into_iter().collect();
        if values.is_empty() {
            return Err(AuthoringError::EmptyProjectionSet);
        }
        Ok(Self {
            aspect,
            field,
            values,
        })
    }

    pub fn aspect(&self) -> &str {
        &self.aspect
    }

    pub fn field(&self) -> &str {
        &self.field
    }

    pub fn values(&self) -> &[ScalarPredicateValue] {
        &self.values
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PresencePredicateKind {
    IsPresent,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PresencePredicate {
    aspect: String,
    field: String,
    kind: PresencePredicateKind,
}

impl PresencePredicate {
    pub fn is_present(
        aspect: impl Into<String>,
        field: impl Into<String>,
    ) -> Result<Self, AuthoringError> {
        let (aspect, field) = validate_predicate_target(aspect, field)?;
        Ok(Self {
            aspect,
            field,
            kind: PresencePredicateKind::IsPresent,
        })
    }

    pub fn aspect(&self) -> &str {
        &self.aspect
    }

    pub fn field(&self) -> &str {
        &self.field
    }

    pub fn kind(&self) -> PresencePredicateKind {
        self.kind
    }
}

impl PresencePredicateKind {
    pub(crate) fn digest_key(self) -> &'static str {
        match self {
            Self::IsPresent => "is-present",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PredicateSelector {
    Equality(EqualityPredicate),
    IntegerComparison(IntegerComparisonPredicate),
    StringContains(StringContainsPredicate),
    SetMembership(SetMembershipPredicate),
    Presence(PresencePredicate),
}

impl PredicateSelector {
    pub fn aspect(&self) -> &str {
        match self {
            Self::Equality(predicate) => predicate.aspect(),
            Self::IntegerComparison(predicate) => predicate.aspect(),
            Self::StringContains(predicate) => predicate.aspect(),
            Self::SetMembership(predicate) => predicate.aspect(),
            Self::Presence(predicate) => predicate.aspect(),
        }
    }

    pub fn field(&self) -> &str {
        match self {
            Self::Equality(predicate) => predicate.field(),
            Self::IntegerComparison(predicate) => predicate.field(),
            Self::StringContains(predicate) => predicate.field(),
            Self::SetMembership(predicate) => predicate.field(),
            Self::Presence(predicate) => predicate.field(),
        }
    }
}
