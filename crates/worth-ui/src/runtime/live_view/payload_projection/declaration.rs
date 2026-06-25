#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiLiveViewPayloadShape {
    PayloadValues,
    DataPayloadValues,
    Unsupported(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewPayloadProjectionDeclaration {
    payload_id: String,
    shape: WorthUiLiveViewPayloadShape,
}

impl WorthUiLiveViewPayloadProjectionDeclaration {
    pub fn new(payload_id: impl Into<String>, shape: WorthUiLiveViewPayloadShape) -> Self {
        Self {
            payload_id: payload_id.into(),
            shape,
        }
    }

    pub fn payload_id(&self) -> &str {
        &self.payload_id
    }

    pub fn shape(&self) -> &WorthUiLiveViewPayloadShape {
        &self.shape
    }
}

impl WorthUiLiveViewPayloadShape {
    pub fn token(&self) -> &str {
        match self {
            Self::PayloadValues => "payload_values",
            Self::DataPayloadValues => "data_payload_values",
            Self::Unsupported(value) => value.as_str(),
        }
    }

    pub(crate) fn is_supported(&self) -> bool {
        !matches!(self, Self::Unsupported(_))
    }
}
