use crate::WorthServerResponseEnvelope;

#[derive(Clone, Debug)]
pub enum WorthServerEvidenceInput {
    ResponseEnvelope(WorthServerResponseEnvelope),
}

impl WorthServerEvidenceInput {
    pub fn response_envelope(response: WorthServerResponseEnvelope) -> Self {
        Self::ResponseEnvelope(response)
    }
}
