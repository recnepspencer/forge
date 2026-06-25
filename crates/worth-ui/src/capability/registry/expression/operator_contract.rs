#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiExpressionArity {
    min: usize,
    max: Option<usize>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiExpressionDependencyContract {
    NoRuntimeFacts,
    BindingReference,
    BindingSet,
    NestedBooleanExpressions,
    BindingReferenceAndLiteral,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiExpressionCostPosture {
    Constant,
    SingleBindingLookup,
    BindingSetLinear,
    NestedExpressionLinear,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiExpressionDiagnosticsPosture {
    SchemaReferenced,
    DependencyReferenced,
}

impl WorthUiExpressionArity {
    pub const fn exact(count: usize) -> Self {
        Self {
            min: count,
            max: Some(count),
        }
    }

    pub const fn at_least(count: usize) -> Self {
        Self {
            min: count,
            max: None,
        }
    }

    pub fn admits(self, actual: usize) -> bool {
        actual >= self.min && self.max.map(|max| actual <= max).unwrap_or(true)
    }

    pub fn min(self) -> usize {
        self.min
    }

    pub fn max(self) -> Option<usize> {
        self.max
    }

    pub fn token(self) -> String {
        match self.max {
            Some(max) if max == self.min => format!("exact:{}", self.min),
            Some(max) => format!("range:{}..={max}", self.min),
            None => format!("at_least:{}", self.min),
        }
    }
}

impl WorthUiExpressionDependencyContract {
    pub const fn token(self) -> &'static str {
        match self {
            Self::NoRuntimeFacts => "no_runtime_facts",
            Self::BindingReference => "binding_reference",
            Self::BindingSet => "binding_set",
            Self::NestedBooleanExpressions => "nested_boolean_expressions",
            Self::BindingReferenceAndLiteral => "binding_reference_and_literal",
        }
    }
}
