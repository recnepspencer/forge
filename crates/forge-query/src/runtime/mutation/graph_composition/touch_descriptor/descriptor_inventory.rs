use std::collections::BTreeSet;

use crate::runtime::ForgeQueryMutationFamily;
use forge_relational::facade::identity::KindId;

use super::touch_rows::ForgeQueryGraphTouchDescriptorRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ForgeQueryGraphTouchDescriptorInventory {
    declared_collections: BTreeSet<String>,
    relation_kind_ids: BTreeSet<KindId>,
    declared_aspect_paths: BTreeSet<String>,
    declared_aspect_operations: BTreeSet<String>,
    touched_aspect_paths: BTreeSet<String>,
    insert_command_count: usize,
    update_command_count: usize,
    assertion_command_count: usize,
    delete_command_count: usize,
}

impl ForgeQueryGraphTouchDescriptorInventory {
    pub(super) fn from_rows(rows: &[ForgeQueryGraphTouchDescriptorRow]) -> Self {
        let declared_collections = collect_declared_collections(rows);
        let relation_kind_ids = collect_relation_kind_ids(rows);
        let declared_aspect_operations = collect_declared_aspect_operations(rows);
        let declared_aspect_paths = collect_declared_aspect_paths(&declared_aspect_operations);
        let touched_aspect_paths = collect_touched_aspect_paths(rows);
        let insert_command_count = count_command_family(rows, ForgeQueryMutationFamily::Insert);
        let update_command_count = count_command_family(rows, ForgeQueryMutationFamily::Update);
        let assertion_command_count =
            count_command_family(rows, ForgeQueryMutationFamily::Assertion);
        let delete_command_count = count_command_family(rows, ForgeQueryMutationFamily::Delete);

        Self {
            declared_collections,
            relation_kind_ids,
            declared_aspect_paths,
            declared_aspect_operations,
            touched_aspect_paths,
            insert_command_count,
            update_command_count,
            assertion_command_count,
            delete_command_count,
        }
    }

    pub(super) fn insert_command_count(&self) -> usize {
        self.insert_command_count
    }

    pub(super) fn update_command_count(&self) -> usize {
        self.update_command_count
    }

    pub(super) fn assertion_command_count(&self) -> usize {
        self.assertion_command_count
    }

    pub(super) fn delete_command_count(&self) -> usize {
        self.delete_command_count
    }

    pub(super) fn declared_collection_count(&self) -> usize {
        self.declared_collections.len()
    }

    pub(super) fn relation_kind_count(&self) -> usize {
        self.relation_kind_ids.len()
    }

    pub(super) fn declared_aspect_path_count(&self) -> usize {
        self.declared_aspect_paths.len()
    }

    pub(super) fn declared_aspect_operation_count(&self) -> usize {
        self.declared_aspect_operations.len()
    }

    pub(super) fn touched_aspect_count(&self) -> usize {
        self.touched_aspect_paths.len()
    }
}

pub(super) fn declared_operation_path(operation: &str) -> Option<&str> {
    operation.split_once(':').map(|(_, path)| path)
}

fn collect_declared_collections(rows: &[ForgeQueryGraphTouchDescriptorRow]) -> BTreeSet<String> {
    rows.iter()
        .filter_map(ForgeQueryGraphTouchDescriptorRow::declared_collection)
        .map(str::to_string)
        .collect()
}

fn collect_relation_kind_ids(rows: &[ForgeQueryGraphTouchDescriptorRow]) -> BTreeSet<KindId> {
    rows.iter()
        .filter_map(ForgeQueryGraphTouchDescriptorRow::relation_kind_id)
        .collect()
}

fn collect_declared_aspect_operations(
    rows: &[ForgeQueryGraphTouchDescriptorRow],
) -> BTreeSet<String> {
    rows.iter()
        .flat_map(ForgeQueryGraphTouchDescriptorRow::declared_aspect_operations)
        .cloned()
        .collect()
}

fn collect_declared_aspect_paths(
    declared_aspect_operations: &BTreeSet<String>,
) -> BTreeSet<String> {
    declared_aspect_operations
        .iter()
        .filter_map(|operation| declared_operation_path(operation).map(str::to_string))
        .collect()
}

fn collect_touched_aspect_paths(rows: &[ForgeQueryGraphTouchDescriptorRow]) -> BTreeSet<String> {
    rows.iter()
        .flat_map(ForgeQueryGraphTouchDescriptorRow::touched_aspect_paths)
        .cloned()
        .collect()
}

fn count_command_family(
    rows: &[ForgeQueryGraphTouchDescriptorRow],
    family: ForgeQueryMutationFamily,
) -> usize {
    rows.iter()
        .filter(|row| row.mutation_family() == family)
        .count()
}
