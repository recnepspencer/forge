#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessAdmissionPostureErrorKind {
    MissingRequirementRecord,
    CapabilityGapCapExceeded,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessAdmissionPostureError {
    kind: WorthGraphReadAccessAdmissionPostureErrorKind,
}

impl WorthGraphReadAccessAdmissionPostureError {
    pub(crate) const fn new(kind: WorthGraphReadAccessAdmissionPostureErrorKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> WorthGraphReadAccessAdmissionPostureErrorKind {
        self.kind
    }
}
