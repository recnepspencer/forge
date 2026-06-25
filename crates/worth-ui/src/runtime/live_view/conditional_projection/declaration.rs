#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewConditionalProjectionDeclaration {
    control_id: String,
    condition: WorthUiLiveViewConditionExpression,
    when_true: WorthUiLiveViewParticipationPosture,
    when_false: WorthUiLiveViewParticipationPosture,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiLiveViewConditionExpression {
    BindingEqualsLiteral { binding_id: String, literal: String },
    Unsupported(String),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiLiveViewParticipationPosture {
    Present,
    AbsentRetainingState,
    Unsupported,
}

impl WorthUiLiveViewConditionalProjectionDeclaration {
    pub fn new(
        control_id: impl Into<String>,
        condition: WorthUiLiveViewConditionExpression,
        when_true: WorthUiLiveViewParticipationPosture,
        when_false: WorthUiLiveViewParticipationPosture,
    ) -> Self {
        Self {
            control_id: control_id.into(),
            condition,
            when_true,
            when_false,
        }
    }

    pub fn control_id(&self) -> &str {
        &self.control_id
    }

    pub fn condition(&self) -> &WorthUiLiveViewConditionExpression {
        &self.condition
    }

    pub fn when_true(&self) -> WorthUiLiveViewParticipationPosture {
        self.when_true
    }

    pub fn when_false(&self) -> WorthUiLiveViewParticipationPosture {
        self.when_false
    }
}

impl WorthUiLiveViewConditionExpression {
    pub fn binding_equals_literal(
        binding_id: impl Into<String>,
        literal: impl Into<String>,
    ) -> Self {
        Self::BindingEqualsLiteral {
            binding_id: binding_id.into(),
            literal: literal.into(),
        }
    }

    pub fn consumed_binding_id(&self) -> Option<&str> {
        match self {
            Self::BindingEqualsLiteral { binding_id, .. } => Some(binding_id),
            Self::Unsupported(_) => None,
        }
    }

    pub fn token(&self) -> &str {
        match self {
            Self::BindingEqualsLiteral { .. } => "binding_equals_literal",
            Self::Unsupported(value) => value.as_str(),
        }
    }
}

impl WorthUiLiveViewParticipationPosture {
    pub fn token(self) -> &'static str {
        match self {
            Self::Present => "present",
            Self::AbsentRetainingState => "absent_retaining_state",
            Self::Unsupported => "unsupported",
        }
    }

    pub(crate) fn is_supported(self) -> bool {
        !matches!(self, Self::Unsupported)
    }
}
