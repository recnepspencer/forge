use crate::capability::ComponentId;
use crate::runtime::WorthUiPrimitiveSourceSpan;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewControlProjectionDeclaration {
    control_id: String,
    binding_id: String,
    kind: WorthUiLiveViewControlProjectionKind,
    label: String,
    options: Option<WorthUiLiveViewControlOptionsSource>,
    primitive_props: Vec<WorthUiLiveViewControlPrimitiveProp>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiLiveViewControlProjectionKind {
    TextInput,
    Select,
    Unsupported(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiLiveViewControlOptionsSource {
    Static {
        source_id: String,
        options: Vec<WorthUiLiveViewControlOptionDeclaration>,
    },
    Unsupported(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewControlOptionDeclaration {
    value: String,
    label: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewControlPrimitiveProp {
    key: String,
    value: String,
    source_span: Option<WorthUiPrimitiveSourceSpan>,
}

impl WorthUiLiveViewControlProjectionDeclaration {
    pub fn new(
        control_id: impl Into<String>,
        binding_id: impl Into<String>,
        kind: WorthUiLiveViewControlProjectionKind,
        label: impl Into<String>,
    ) -> Self {
        Self {
            control_id: control_id.into(),
            binding_id: binding_id.into(),
            kind,
            label: label.into(),
            options: None,
            primitive_props: Vec::new(),
        }
    }

    pub fn with_options(mut self, options: WorthUiLiveViewControlOptionsSource) -> Self {
        self.options = Some(options);
        self
    }

    pub fn with_primitive_props(mut self, props: Vec<WorthUiLiveViewControlPrimitiveProp>) -> Self {
        self.primitive_props = props;
        self
    }

    pub fn control_id(&self) -> &str {
        &self.control_id
    }

    pub fn binding_id(&self) -> &str {
        &self.binding_id
    }

    pub fn kind(&self) -> &WorthUiLiveViewControlProjectionKind {
        &self.kind
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn options(&self) -> Option<&WorthUiLiveViewControlOptionsSource> {
        self.options.as_ref()
    }

    pub fn primitive_props(&self) -> &[WorthUiLiveViewControlPrimitiveProp] {
        &self.primitive_props
    }
}

impl WorthUiLiveViewControlProjectionKind {
    pub fn component_id(&self) -> Option<ComponentId> {
        match self {
            Self::TextInput => ComponentId::new("worth.component.text_input").ok(),
            Self::Select => ComponentId::new("worth.component.dropdown_input").ok(),
            Self::Unsupported(_) => None,
        }
    }

    pub fn token(&self) -> &str {
        match self {
            Self::TextInput => "text_input",
            Self::Select => "select",
            Self::Unsupported(value) => value.as_str(),
        }
    }

    pub(crate) fn is_supported(&self) -> bool {
        !matches!(self, Self::Unsupported(_))
    }
}

impl WorthUiLiveViewControlOptionsSource {
    pub fn static_options(
        source_id: impl Into<String>,
        options: Vec<WorthUiLiveViewControlOptionDeclaration>,
    ) -> Self {
        Self::Static {
            source_id: source_id.into(),
            options,
        }
    }

    pub fn source_id(&self) -> &str {
        match self {
            Self::Static { source_id, .. } => source_id,
            Self::Unsupported(value) => value,
        }
    }

    pub fn options(&self) -> &[WorthUiLiveViewControlOptionDeclaration] {
        match self {
            Self::Static { options, .. } => options,
            Self::Unsupported(_) => &[],
        }
    }
}

impl WorthUiLiveViewControlOptionDeclaration {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn label(&self) -> &str {
        &self.label
    }
}

impl WorthUiLiveViewControlPrimitiveProp {
    pub fn new(
        key: impl Into<String>,
        value: impl Into<String>,
        source_span: Option<WorthUiPrimitiveSourceSpan>,
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

    pub fn source_span(&self) -> Option<WorthUiPrimitiveSourceSpan> {
        self.source_span
    }
}
