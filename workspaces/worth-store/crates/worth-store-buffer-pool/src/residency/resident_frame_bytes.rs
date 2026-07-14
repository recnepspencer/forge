use crate::{ResidentFrameDenial, ResidentFrameDenialKind, ResidentFrameLoadRequest};
use worth_store_physical_format::PhysicalPayloadViewAdmission;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResidentFrameBytes {
    payload: Vec<u8>,
}

impl ResidentFrameBytes {
    pub fn from_physical_format_payload_admission(
        request: ResidentFrameLoadRequest,
        admission: PhysicalPayloadViewAdmission<'_>,
    ) -> Result<Self, ResidentFrameDenial> {
        let view = admission.view();
        if view.witness() != request.header() {
            return Err(ResidentFrameDenial::new(
                ResidentFrameDenialKind::ResidentPayloadWitnessMismatch,
            ));
        }
        if view.as_bytes().len() != request.header().payload_length() as usize {
            return Err(ResidentFrameDenial::new(
                ResidentFrameDenialKind::ResidentPayloadLengthMismatch,
            ));
        }
        Ok(Self {
            payload: view.as_bytes().to_vec(),
        })
    }

    pub const fn as_bytes(&self) -> &[u8] {
        self.payload.as_slice()
    }
}
