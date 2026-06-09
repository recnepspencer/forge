use crate::ForgeServerResponseEnvelope;

#[derive(Clone, Debug)]
pub enum ForgeServerEvidenceInput {
    ResponseEnvelope(ForgeServerResponseEnvelope),
}

impl ForgeServerEvidenceInput {
    pub fn response_envelope(response: ForgeServerResponseEnvelope) -> Self {
        Self::ResponseEnvelope(response)
    }
}
