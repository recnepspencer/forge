use crate::runtime::WorthUiLiveViewStateValue;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewPayloadField {
    name: String,
    value: WorthUiLiveViewStateValue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiLiveViewEmittedPayload {
    Payload {
        fields: Vec<WorthUiLiveViewPayloadField>,
    },
    DataPayload {
        fields: Vec<WorthUiLiveViewPayloadField>,
    },
}

impl WorthUiLiveViewPayloadField {
    pub(crate) fn new(name: impl Into<String>, value: WorthUiLiveViewStateValue) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &WorthUiLiveViewStateValue {
        &self.value
    }
}

impl WorthUiLiveViewEmittedPayload {
    pub(crate) fn payload(fields: Vec<WorthUiLiveViewPayloadField>) -> Self {
        Self::Payload { fields }
    }

    pub(crate) fn data_payload(fields: Vec<WorthUiLiveViewPayloadField>) -> Self {
        Self::DataPayload { fields }
    }

    pub fn fields(&self) -> &[WorthUiLiveViewPayloadField] {
        match self {
            Self::Payload { fields } | Self::DataPayload { fields } => fields,
        }
    }

    pub fn shape_token(&self) -> &'static str {
        match self {
            Self::Payload { .. } => "payload",
            Self::DataPayload { .. } => "data_payload",
        }
    }

    pub fn display_shape(&self) -> String {
        let body = self
            .fields()
            .iter()
            .map(|field| format!("{}={}", field.name(), field.value().as_display_text()))
            .collect::<Vec<_>>()
            .join(",");
        match self {
            Self::Payload { .. } => format!("payload{{{body}}}"),
            Self::DataPayload { .. } => format!("data.payload{{{body}}}"),
        }
    }
}
