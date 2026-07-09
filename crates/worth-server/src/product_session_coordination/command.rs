use crate::{WorthServerProductSessionCreationRequest, WorthServerProductSessionIdentity};

#[derive(Clone, Debug)]
pub enum WorthServerProductSessionCoordinationCommand {
    OpenPreview(WorthServerProductSessionCreationRequest),
    OpenMutation(WorthServerProductSessionCreationRequest),
    CloseExisting(WorthServerProductSessionIdentity),
}

impl WorthServerProductSessionCoordinationCommand {
    pub fn operation_name(&self) -> &str {
        match self {
            Self::OpenPreview(request) | Self::OpenMutation(request) => request.operation_name(),
            Self::CloseExisting(_) => "product_session.close",
        }
    }
}
