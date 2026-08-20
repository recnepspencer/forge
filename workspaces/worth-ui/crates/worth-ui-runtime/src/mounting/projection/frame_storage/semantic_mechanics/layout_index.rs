use std::sync::Arc;

use worth_ui_host_contract::UiQualifiedTextLayoutIdentity;

use super::{UiMountedQualifiedSemanticText, UiMountedSemanticMechanicRows};
use crate::runtime::persistent_index::UiPersistentOrdMap;

#[derive(Clone, Default)]
pub(super) struct UiMountedQualifiedLayoutIndex {
    by_identity: UiPersistentOrdMap<[u8; 32], UiMountedQualifiedLayoutEntry>,
}

#[derive(Clone)]
struct UiMountedQualifiedLayoutEntry {
    layout: Arc<worth_ui_text::UiQualifiedTextLayout>,
    owners: usize,
}

impl UiMountedQualifiedLayoutIndex {
    pub(super) fn len(&self) -> usize {
        self.by_identity.len()
    }

    pub(super) fn insert_rows(&mut self, rows: &UiMountedSemanticMechanicRows) {
        for row in rows.iter() {
            self.insert_row(row);
        }
    }

    pub(super) fn remove_rows(&mut self, rows: &UiMountedSemanticMechanicRows) {
        for row in rows.iter() {
            self.remove_row(row);
        }
    }

    pub(super) fn replace_row(
        &mut self,
        predecessor: &UiMountedQualifiedSemanticText,
        successor: &UiMountedQualifiedSemanticText,
    ) {
        self.remove_row(predecessor);
        self.insert_row(successor);
    }

    fn insert_row(&mut self, row: &UiMountedQualifiedSemanticText) {
        let Some(layout) = row.qualified_layout() else {
            return;
        };
        let identity = row.qualified_layout_identity().digest();
        let entry = self.by_identity.get(&identity).cloned().map_or_else(
            || UiMountedQualifiedLayoutEntry {
                layout: Arc::clone(layout),
                owners: 1,
            },
            |mut entry| {
                entry.owners = entry.owners.saturating_add(1);
                entry
            },
        );
        self.by_identity.insert(identity, entry);
    }

    fn remove_row(&mut self, row: &UiMountedQualifiedSemanticText) {
        let identity = row.qualified_layout_identity().digest();
        let Some(mut entry) = self.by_identity.get(&identity).cloned() else {
            return;
        };
        if entry.owners == 1 {
            self.by_identity.remove(&identity);
        } else {
            entry.owners -= 1;
            self.by_identity.insert(identity, entry);
        }
    }

    pub(super) fn get(
        &self,
        identity: UiQualifiedTextLayoutIdentity,
    ) -> Option<&Arc<worth_ui_text::UiQualifiedTextLayout>> {
        self.by_identity
            .get(&identity.digest())
            .map(|entry| &entry.layout)
    }
}
