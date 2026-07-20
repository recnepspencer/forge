use notify::event::{ModifyKind, RenameMode};
use notify::{Event, EventKind};

use crate::runtime::source_ingress::WorthUiWatcherEvent;

pub(super) fn translate_filesystem_event(
    event: Event,
    provider_id: &str,
) -> Vec<WorthUiWatcherEvent> {
    match event.kind {
        EventKind::Access(_) => Vec::new(),
        EventKind::Remove(_) => event
            .paths
            .into_iter()
            .map(WorthUiWatcherEvent::deleted)
            .collect(),
        EventKind::Modify(ModifyKind::Name(RenameMode::Both)) if event.paths.len() >= 2 => {
            vec![WorthUiWatcherEvent::atomic_rename(
                event.paths[0].clone(),
                event.paths[event.paths.len() - 1].clone(),
            )]
        }
        EventKind::Create(_) | EventKind::Modify(_) => event
            .paths
            .into_iter()
            .map(WorthUiWatcherEvent::modified)
            .collect(),
        EventKind::Any | EventKind::Other => {
            vec![WorthUiWatcherEvent::provider_revision(provider_id)]
        }
    }
}
