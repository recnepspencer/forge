use crate::{ResidentFrameDenial, ResidentFrameDenialKind, ResidentFrameLoadRequest};
use worth_store_physical_format::PhysicalPayloadViewAdmission;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResidentFrameBytes {
    payload: Vec<u8>,
}

impl ResidentFrameBytes {
    pub(crate) fn validate_physical_format_payload_admission(
        request: ResidentFrameLoadRequest,
        admission: PhysicalPayloadViewAdmission<'_>,
    ) -> Result<(), ResidentFrameDenial> {
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
        Ok(())
    }

    pub fn from_physical_format_payload_admission(
        request: ResidentFrameLoadRequest,
        admission: PhysicalPayloadViewAdmission<'_>,
    ) -> Result<Self, ResidentFrameDenial> {
        Self::validate_physical_format_payload_admission(request, admission)?;
        let bytes = admission.view().as_bytes();
        let mut payload = Vec::new();
        payload.try_reserve_exact(bytes.len()).map_err(|_| {
            ResidentFrameDenial::new(ResidentFrameDenialKind::ResidentByteAllocationFailed)
        })?;
        payload.extend_from_slice(bytes);
        Ok(Self { payload })
    }

    pub const fn as_bytes(&self) -> &[u8] {
        self.payload.as_slice()
    }
}
