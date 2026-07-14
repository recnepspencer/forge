use std::path::{Path, PathBuf};

use crate::runtime::source_ingress::digest::fold_texts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiWatcherEvent {
    kind: WorthUiWatcherEventKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum WorthUiWatcherEventKind {
    Modified { path: PathBuf },
    Deleted { path: PathBuf },
    WriteStarted { path: PathBuf },
    WriteCompleted { path: PathBuf },
    AtomicRename { from: PathBuf, to: PathBuf },
    ProviderRevision { provider_id: String },
}

impl WorthUiWatcherEvent {
    pub fn allocation_truth_category(
        &self,
    ) -> crate::evidence::allocation::UiAllocationTruthCategory {
        crate::evidence::allocation::UiAllocationTruthCategory::EphemeralStreamEvent
    }

    pub fn modified(path: impl Into<PathBuf>) -> Self {
        Self {
            kind: WorthUiWatcherEventKind::Modified { path: path.into() },
        }
    }

    pub fn deleted(path: impl Into<PathBuf>) -> Self {
        Self {
            kind: WorthUiWatcherEventKind::Deleted { path: path.into() },
        }
    }

    pub fn write_started(path: impl Into<PathBuf>) -> Self {
        Self {
            kind: WorthUiWatcherEventKind::WriteStarted { path: path.into() },
        }
    }

    pub fn write_completed(path: impl Into<PathBuf>) -> Self {
        Self {
            kind: WorthUiWatcherEventKind::WriteCompleted { path: path.into() },
        }
    }

    pub fn atomic_rename(from: impl Into<PathBuf>, to: impl Into<PathBuf>) -> Self {
        Self {
            kind: WorthUiWatcherEventKind::AtomicRename {
                from: from.into(),
                to: to.into(),
            },
        }
    }

    pub fn provider_revision(provider_id: impl Into<String>) -> Self {
        Self {
            kind: WorthUiWatcherEventKind::ProviderRevision {
                provider_id: provider_id.into(),
            },
        }
    }

    pub(crate) fn is_partial_without_completion(&self) -> bool {
        matches!(self.kind, WorthUiWatcherEventKind::WriteStarted { .. })
    }

    pub(crate) fn burst_digest_basis(&self) -> String {
        match &self.kind {
            WorthUiWatcherEventKind::Modified { path } => {
                format!("modified|{}", normalize_path(path))
            }
            WorthUiWatcherEventKind::Deleted { path } => {
                format!("deleted|{}", normalize_path(path))
            }
            WorthUiWatcherEventKind::WriteStarted { path } => {
                format!("write-started|{}", normalize_path(path))
            }
            WorthUiWatcherEventKind::WriteCompleted { path } => {
                format!("write-completed|{}", normalize_path(path))
            }
            WorthUiWatcherEventKind::AtomicRename { from, to } => {
                format!(
                    "atomic-rename|{}|{}",
                    normalize_path(from),
                    normalize_path(to)
                )
            }
            WorthUiWatcherEventKind::ProviderRevision { provider_id } => {
                format!("provider-revision|{provider_id}")
            }
        }
    }
}

pub(crate) fn event_burst_digest(events: &[WorthUiWatcherEvent]) -> u64 {
    let mut basis = events
        .iter()
        .map(WorthUiWatcherEvent::burst_digest_basis)
        .collect::<Vec<_>>();
    basis.sort();
    fold_texts(basis)
}

fn normalize_path(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
        .replace('\\', "/")
}
