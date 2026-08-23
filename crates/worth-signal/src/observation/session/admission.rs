use super::request::SignalObservationRequest;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalObservationAdmissionDenial {
    EmptyRequest,
    SessionAlreadyActive,
}

impl std::fmt::Display for SignalObservationAdmissionDenial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyRequest => f.write_str("observation request selected no capture surfaces"),
            Self::SessionAlreadyActive => f.write_str("another observation session is active"),
        }
    }
}

impl std::error::Error for SignalObservationAdmissionDenial {}

pub(crate) fn admit(
    request: SignalObservationRequest,
    active_generation: u64,
) -> Result<SignalObservationRequest, SignalObservationAdmissionDenial> {
    if request.is_empty() {
        return Err(SignalObservationAdmissionDenial::EmptyRequest);
    }
    if active_generation != 0 {
        return Err(SignalObservationAdmissionDenial::SessionAlreadyActive);
    }
    Ok(request)
}
