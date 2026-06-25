#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiLiveViewPayloadProjectionDenial {
    InvalidPayloadId { payload_id: String },
    UnsupportedPayloadShape { payload_id: String, shape: String },
}

impl WorthUiLiveViewPayloadProjectionDenial {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidPayloadId { .. } => "live_view_payload.invalid_id",
            Self::UnsupportedPayloadShape { .. } => "live_view_payload.unsupported_shape",
        }
    }
}
