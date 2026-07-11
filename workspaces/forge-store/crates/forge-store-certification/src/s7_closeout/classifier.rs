use super::{
    denial::shortcut_rejection, S7CloseoutCertificationInput, S7CloseoutDenial, S7CloseoutRequest,
};

#[derive(Debug)]
pub(crate) struct ClassifiedS7CloseoutRequest {
    pub(crate) input: S7CloseoutCertificationInput,
}

pub(crate) fn classify_s7_closeout_request(
    request: S7CloseoutRequest,
) -> Result<ClassifiedS7CloseoutRequest, S7CloseoutDenial> {
    match request {
        S7CloseoutRequest::Canonical(input) => {
            if !input.policy().is_counter_backed_foundational() {
                return Err(S7CloseoutDenial::CounterBackedFoundationalPolicyRequired);
            }
            Ok(ClassifiedS7CloseoutRequest { input })
        }
        S7CloseoutRequest::Shortcut(shortcut) => Err(S7CloseoutDenial::ShortcutRejected(
            shortcut_rejection(&shortcut),
        )),
    }
}
