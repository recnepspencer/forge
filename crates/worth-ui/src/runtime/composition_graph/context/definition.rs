use super::super::WorthUiCompositionNodeId;
use crate::runtime::WorthUiPrimitiveSourceSpan;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionContextDefinition {
    scope: WorthUiCompositionContextScope,
    values: Vec<WorthUiCompositionContextValue>,
    override_policy: WorthUiCompositionContextOverridePolicy,
    source_span: Option<WorthUiPrimitiveSourceSpan>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthUiCompositionContextScope {
    Root,
    Node(WorthUiCompositionNodeId),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiCompositionContextValue {
    Theme(String),
    Density(String),
    TextDirection(WorthUiCompositionTextDirection),
    Locale(WorthUiCompositionLocalePosture),
    Disabled(bool),
    Inert(bool),
    Validation(WorthUiCompositionValidationPosture),
    FocusScope(String),
    RuntimeMode(WorthUiCompositionRuntimeMode),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiCompositionContextOverridePolicy {
    InheritOnly,
    AllowLocalOverride,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiCompositionTextDirection {
    Ltr,
    Rtl,
    Auto,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiCompositionLocalePosture {
    Ready(String),
    Limited(String),
    Unsupported(String),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiCompositionValidationPosture {
    Valid,
    Invalid,
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiCompositionRuntimeMode {
    Interactive,
    Preview,
    Diagnostic,
}

impl WorthUiCompositionContextDefinition {
    pub fn root() -> Self {
        Self::new(WorthUiCompositionContextScope::Root)
    }

    pub fn for_node(node_id: impl AsRef<str>) -> Self {
        Self::new(WorthUiCompositionContextScope::Node(
            WorthUiCompositionNodeId::new(node_id.as_ref())
                .expect("composition context node ids must not be empty"),
        ))
    }

    fn new(scope: WorthUiCompositionContextScope) -> Self {
        Self {
            scope,
            values: Vec::new(),
            override_policy: WorthUiCompositionContextOverridePolicy::InheritOnly,
            source_span: None,
        }
    }

    pub(crate) fn with_source_span(mut self, source_span: WorthUiPrimitiveSourceSpan) -> Self {
        self.source_span = Some(source_span);
        self
    }

    pub fn allow_local_override(mut self) -> Self {
        self.override_policy = WorthUiCompositionContextOverridePolicy::AllowLocalOverride;
        self
    }

    pub fn theme(mut self, theme: impl Into<String>) -> Self {
        self.values
            .push(WorthUiCompositionContextValue::Theme(theme.into()));
        self
    }

    pub fn density(mut self, density: impl Into<String>) -> Self {
        self.values
            .push(WorthUiCompositionContextValue::Density(density.into()));
        self
    }

    pub fn text_direction(mut self, direction: WorthUiCompositionTextDirection) -> Self {
        self.values
            .push(WorthUiCompositionContextValue::TextDirection(direction));
        self
    }

    pub fn locale(mut self, locale: WorthUiCompositionLocalePosture) -> Self {
        self.values
            .push(WorthUiCompositionContextValue::Locale(locale));
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.values
            .push(WorthUiCompositionContextValue::Disabled(disabled));
        self
    }

    pub fn inert(mut self, inert: bool) -> Self {
        self.values
            .push(WorthUiCompositionContextValue::Inert(inert));
        self
    }

    pub fn validation(mut self, validation: WorthUiCompositionValidationPosture) -> Self {
        self.values
            .push(WorthUiCompositionContextValue::Validation(validation));
        self
    }

    pub fn focus_scope(mut self, focus_scope: impl Into<String>) -> Self {
        self.values.push(WorthUiCompositionContextValue::FocusScope(
            focus_scope.into(),
        ));
        self
    }

    pub fn runtime_mode(mut self, runtime_mode: WorthUiCompositionRuntimeMode) -> Self {
        self.values
            .push(WorthUiCompositionContextValue::RuntimeMode(runtime_mode));
        self
    }

    pub(crate) fn scope(&self) -> &WorthUiCompositionContextScope {
        &self.scope
    }

    pub(crate) fn values(&self) -> &[WorthUiCompositionContextValue] {
        &self.values
    }

    pub(crate) fn override_policy(&self) -> WorthUiCompositionContextOverridePolicy {
        self.override_policy
    }

    pub(crate) fn source_span(&self) -> Option<WorthUiPrimitiveSourceSpan> {
        self.source_span
    }
}

impl WorthUiCompositionContextValue {
    pub(crate) fn kind_token(&self) -> &'static str {
        match self {
            Self::Theme(_) => "theme",
            Self::Density(_) => "density",
            Self::TextDirection(_) => "text_direction",
            Self::Locale(_) => "locale",
            Self::Disabled(_) => "disabled",
            Self::Inert(_) => "inert",
            Self::Validation(_) => "validation",
            Self::FocusScope(_) => "focus_scope",
            Self::RuntimeMode(_) => "runtime_mode",
        }
    }

    pub(crate) fn value_token(&self) -> String {
        match self {
            Self::Theme(value) | Self::Density(value) | Self::FocusScope(value) => value.clone(),
            Self::TextDirection(value) => value.token().to_owned(),
            Self::Locale(value) => value.token(),
            Self::Disabled(value) | Self::Inert(value) => value.to_string(),
            Self::Validation(value) => value.token().to_owned(),
            Self::RuntimeMode(value) => value.token().to_owned(),
        }
    }
}

impl WorthUiCompositionContextScope {
    pub(crate) fn identity(&self, root_id: &str) -> String {
        match self {
            Self::Root => root_id.to_owned(),
            Self::Node(node_id) => node_id.as_str().to_owned(),
        }
    }
}

impl WorthUiCompositionTextDirection {
    pub const fn token(self) -> &'static str {
        match self {
            Self::Ltr => "ltr",
            Self::Rtl => "rtl",
            Self::Auto => "auto",
        }
    }
}

impl WorthUiCompositionLocalePosture {
    pub(crate) fn token(&self) -> String {
        match self {
            Self::Ready(locale) => format!("ready:{locale}"),
            Self::Limited(locale) => format!("limited:{locale}"),
            Self::Unsupported(locale) => format!("unsupported:{locale}"),
        }
    }
}

impl WorthUiCompositionValidationPosture {
    pub const fn token(self) -> &'static str {
        match self {
            Self::Valid => "valid",
            Self::Invalid => "invalid",
            Self::Unknown => "unknown",
        }
    }
}

impl WorthUiCompositionRuntimeMode {
    pub const fn token(self) -> &'static str {
        match self {
            Self::Interactive => "interactive",
            Self::Preview => "preview",
            Self::Diagnostic => "diagnostic",
        }
    }
}
