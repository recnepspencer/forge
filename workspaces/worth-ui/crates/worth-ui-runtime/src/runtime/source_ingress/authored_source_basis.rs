use super::{
    WorthUiCandidateCompositionBasis, WorthUiCandidateOrderingReceipt, WorthUiSourcePackageRevision,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiAuthoredSourceBasis {
    origin: WorthUiAuthoredSourceOrigin,
    composition: WorthUiCandidateCompositionBasis,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum WorthUiAuthoredSourceOrigin {
    RustAuthored {
        source_revision_digest: u64,
    },
    Watched {
        revision: WorthUiSourcePackageRevision,
        ordering: WorthUiCandidateOrderingReceipt,
    },
}

impl WorthUiAuthoredSourceBasis {
    pub(crate) fn rust_authored(
        source_revision_digest: u64,
        composition: WorthUiCandidateCompositionBasis,
    ) -> Self {
        Self {
            origin: WorthUiAuthoredSourceOrigin::RustAuthored {
                source_revision_digest,
            },
            composition,
        }
    }

    pub(crate) fn watched(
        revision: WorthUiSourcePackageRevision,
        ordering: WorthUiCandidateOrderingReceipt,
        composition: WorthUiCandidateCompositionBasis,
    ) -> Self {
        Self {
            origin: WorthUiAuthoredSourceOrigin::Watched { revision, ordering },
            composition,
        }
    }
}
