#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessPostureMatrixErrorKind {
    MissingResolvedPostureRows,
    DuplicateResolvedRequirementPosture,
    UncappedPostureFamily,
    PostureFamilyCapExceeded,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPostureMatrixError {
    kind: WorthGraphReadAccessPostureMatrixErrorKind,
    posture_family: Option<String>,
    observed_count: Option<usize>,
    cap_count: Option<usize>,
}

impl WorthGraphReadAccessPostureMatrixError {
    pub const fn new(kind: WorthGraphReadAccessPostureMatrixErrorKind) -> Self {
        Self {
            kind,
            posture_family: None,
            observed_count: None,
            cap_count: None,
        }
    }

    pub fn for_posture_family(
        kind: WorthGraphReadAccessPostureMatrixErrorKind,
        posture_family: String,
        observed_count: usize,
        cap_count: Option<usize>,
    ) -> Self {
        Self {
            kind,
            posture_family: Some(posture_family),
            observed_count: Some(observed_count),
            cap_count,
        }
    }

    pub const fn kind(&self) -> WorthGraphReadAccessPostureMatrixErrorKind {
        self.kind
    }

    pub fn posture_family(&self) -> Option<&str> {
        self.posture_family.as_deref()
    }

    pub const fn observed_count(&self) -> Option<usize> {
        self.observed_count
    }

    pub const fn cap_count(&self) -> Option<usize> {
        self.cap_count
    }
}
