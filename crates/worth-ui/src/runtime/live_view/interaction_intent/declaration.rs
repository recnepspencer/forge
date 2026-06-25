#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiLiveViewInteractionIntentKind {
    Submit,
    Unsupported(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewInteractionIntentDeclaration {
    interaction_id: String,
    kind: WorthUiLiveViewInteractionIntentKind,
    effect: String,
    readiness_id: String,
    payload_id: String,
    label: String,
    primitive_props: Vec<WorthUiLiveViewInteractionPrimitiveProp>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewInteractionPrimitiveProp {
    key: String,
    value: String,
    source_span: Option<crate::runtime::WorthUiPrimitiveSourceSpan>,
}

impl WorthUiLiveViewInteractionIntentDeclaration {
    pub fn new(interaction_id: impl Into<String>) -> Self {
        let interaction_id = interaction_id.into();
        Self {
            label: interaction_id.clone(),
            interaction_id,
            kind: WorthUiLiveViewInteractionIntentKind::Unsupported(String::new()),
            effect: String::new(),
            readiness_id: String::new(),
            payload_id: String::new(),
            primitive_props: Vec::new(),
        }
    }

    pub fn with_kind(mut self, kind: WorthUiLiveViewInteractionIntentKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn with_effect(mut self, effect: impl Into<String>) -> Self {
        self.effect = effect.into();
        self
    }

    pub fn with_readiness(mut self, readiness_id: impl Into<String>) -> Self {
        self.readiness_id = readiness_id.into();
        self
    }

    pub fn with_payload(mut self, payload_id: impl Into<String>) -> Self {
        self.payload_id = payload_id.into();
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn with_primitive_props(
        mut self,
        props: Vec<WorthUiLiveViewInteractionPrimitiveProp>,
    ) -> Self {
        self.primitive_props = props;
        self
    }

    pub fn interaction_id(&self) -> &str {
        &self.interaction_id
    }

    pub fn kind(&self) -> &WorthUiLiveViewInteractionIntentKind {
        &self.kind
    }

    pub fn effect(&self) -> &str {
        &self.effect
    }

    pub fn readiness_id(&self) -> &str {
        &self.readiness_id
    }

    pub fn payload_id(&self) -> &str {
        &self.payload_id
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn primitive_props(&self) -> &[WorthUiLiveViewInteractionPrimitiveProp] {
        &self.primitive_props
    }
}

impl WorthUiLiveViewInteractionIntentKind {
    pub fn token(&self) -> &str {
        match self {
            Self::Submit => "submit",
            Self::Unsupported(value) => value.as_str(),
        }
    }

    pub(crate) fn is_supported(&self) -> bool {
        matches!(self, Self::Submit)
    }
}

impl WorthUiLiveViewInteractionPrimitiveProp {
    pub fn new(
        key: impl Into<String>,
        value: impl Into<String>,
        source_span: Option<crate::runtime::WorthUiPrimitiveSourceSpan>,
    ) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
            source_span,
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn source_span(&self) -> Option<crate::runtime::WorthUiPrimitiveSourceSpan> {
        self.source_span
    }
}
