use crate::runtime::candidate::WorthUiCandidateProvenanceHandle;
use crate::source::WorthUiSourceModuleId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiReplacementCause {
    kind: WorthUiReplacementCauseKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum WorthUiReplacementCauseKind {
    FileSourceChanged {
        module_id: WorthUiSourceModuleId,
        source_revision_digest: u64,
    },
    RustAuthoredInputChanged {
        source_revision_digest: u64,
    },
    #[cfg(test)]
    ManualRefresh {
        request_digest: u64,
    },
}

impl WorthUiReplacementCause {
    pub(crate) fn file_source_change(
        module_id: WorthUiSourceModuleId,
        source_revision_digest: u64,
    ) -> Self {
        Self {
            kind: WorthUiReplacementCauseKind::FileSourceChanged {
                module_id,
                source_revision_digest,
            },
        }
    }

    pub(crate) fn rust_authored_input_change(source_revision_digest: u64) -> Self {
        Self {
            kind: WorthUiReplacementCauseKind::RustAuthoredInputChanged {
                source_revision_digest,
            },
        }
    }

    #[cfg(test)]
    pub(crate) fn manual_refresh(request_digest: u64) -> Self {
        Self {
            kind: WorthUiReplacementCauseKind::ManualRefresh { request_digest },
        }
    }

    pub fn kind_name(&self) -> &'static str {
        match self.kind {
            WorthUiReplacementCauseKind::FileSourceChanged { .. } => "file-source-changed",
            WorthUiReplacementCauseKind::RustAuthoredInputChanged { .. } => {
                "rust-authored-input-changed"
            }
            #[cfg(test)]
            WorthUiReplacementCauseKind::ManualRefresh { .. } => "manual-refresh",
        }
    }

    pub(crate) fn provenance_handle(&self) -> WorthUiCandidateProvenanceHandle {
        WorthUiCandidateProvenanceHandle::new(fold_text(&self.provenance_digest_basis()))
    }

    fn provenance_digest_basis(&self) -> String {
        match &self.kind {
            WorthUiReplacementCauseKind::FileSourceChanged {
                module_id,
                source_revision_digest,
            } => format!(
                "file-source-changed|module:{}|revision:{}",
                module_id.as_str(),
                source_revision_digest
            ),
            WorthUiReplacementCauseKind::RustAuthoredInputChanged {
                source_revision_digest,
            } => format!("rust-authored-input-changed|revision:{source_revision_digest}"),
            #[cfg(test)]
            WorthUiReplacementCauseKind::ManualRefresh { request_digest } => {
                format!("manual-refresh|request:{request_digest}")
            }
        }
    }
}

fn fold_text(text: &str) -> u64 {
    let mut digest = 0xcbf2_9ce4_8422_2325u64;
    for byte in text.as_bytes() {
        digest ^= u64::from(*byte);
        digest = digest.wrapping_mul(0x100_0000_01b3);
    }
    digest
}
