use std::cell::RefCell;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WorthQueryForbiddenFallbackSeam {
    ConsumeScalarFields,
    DecodeRowPair,
    DecodeRowTriple,
    VerifyScalarAlignment,
    ReadLiveArtifactBundle,
    BindLiveArtifact,
    ReadLiveArtifactBinding,
}

impl WorthQueryForbiddenFallbackSeam {
    fn index(self) -> usize {
        match self {
            Self::ConsumeScalarFields => 0,
            Self::DecodeRowPair => 1,
            Self::DecodeRowTriple => 2,
            Self::VerifyScalarAlignment => 3,
            Self::ReadLiveArtifactBundle => 4,
            Self::BindLiveArtifact => 5,
            Self::ReadLiveArtifactBinding => 6,
        }
    }
}

thread_local! {
    static FORBIDDEN_FALLBACK_SEAM_COUNTS: RefCell<[usize; 7]> = const {
        RefCell::new([0; 7])
    };
}

pub(crate) fn record_forbidden_fallback_seam_invocation(seam: WorthQueryForbiddenFallbackSeam) {
    FORBIDDEN_FALLBACK_SEAM_COUNTS.with(|counts| {
        counts.borrow_mut()[seam.index()] += 1;
    });
}

pub(crate) fn forbidden_fallback_seam_invocation_count(
    seam: WorthQueryForbiddenFallbackSeam,
) -> usize {
    FORBIDDEN_FALLBACK_SEAM_COUNTS.with(|counts| counts.borrow()[seam.index()])
}

pub(crate) fn reset_forbidden_fallback_seam_invocations() {
    FORBIDDEN_FALLBACK_SEAM_COUNTS.with(|counts| {
        *counts.borrow_mut() = [0; 7];
    });
}
