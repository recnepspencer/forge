#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionGraphAdmissionDenial {
    code: WorthUiCompositionGraphDenialCode,
    subject: String,
    message: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiCompositionGraphDenialCode {
    DuplicateNodeIdentity,
    MissingParent,
    MissingChild,
    DuplicateChildOrder,
    MultipleParents,
    UnsupportedParentKind,
    Cycle,
    UnmountedNode,
    MissingPolicyNode,
    DuplicatePolicyAttachment,
    UnsupportedPolicyNodeKind,
}

impl WorthUiCompositionGraphAdmissionDenial {
    pub(crate) fn new(
        code: WorthUiCompositionGraphDenialCode,
        subject: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            subject: subject.into(),
            message: message.into(),
        }
    }

    pub fn code(&self) -> WorthUiCompositionGraphDenialCode {
        self.code
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}
