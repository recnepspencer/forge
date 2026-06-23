use crate::{ForgeServerProductSessionCreationRequest, ForgeServerProductSessionIdentity};

#[derive(Clone, Debug)]
pub enum ForgeServerProductSessionCoordinationCommand {
    OpenPreview(ForgeServerProductSessionCreationRequest),
    OpenMutation(ForgeServerProductSessionCreationRequest),
    CloseExisting(ForgeServerProductSessionIdentity),
}

impl ForgeServerProductSessionCoordinationCommand {
    pub fn operation_name(&self) -> &str {
        match self {
            Self::OpenPreview(request) | Self::OpenMutation(request) => request.operation_name(),
            Self::CloseExisting(_) => "product_session.close",
        }
    }
}
