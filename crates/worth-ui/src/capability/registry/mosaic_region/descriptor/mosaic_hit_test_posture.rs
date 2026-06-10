/// Hit-test posture for a mosaic region kind.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MosaicHitTestPosture {
    Participates,
    PassThrough,
    ModalCapture,
    MissingForDiagnostics,
}

impl MosaicHitTestPosture {
    pub fn participates() -> Self {
        Self::Participates
    }

    pub fn pass_through() -> Self {
        Self::PassThrough
    }

    pub fn modal_capture() -> Self {
        Self::ModalCapture
    }

    pub fn missing_for_diagnostics() -> Self {
        Self::MissingForDiagnostics
    }

    pub(crate) fn is_missing(&self) -> bool {
        matches!(self, Self::MissingForDiagnostics)
    }

    pub(crate) fn digest_basis(&self) -> &'static str {
        match self {
            Self::Participates => "participates",
            Self::PassThrough => "pass_through",
            Self::ModalCapture => "modal_capture",
            Self::MissingForDiagnostics => "missing",
        }
    }
}
