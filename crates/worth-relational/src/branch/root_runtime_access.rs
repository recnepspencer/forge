use super::root::RelationalBranchRootState;

impl RelationalBranchRootState {
    pub(crate) fn version_id(&self) -> crate::identity::data::VersionId {
        self.root
            .axes()
            .map(|axes| crate::identity::data::VersionId(axes.storage_version))
            .unwrap_or(crate::identity::data::VersionId(0))
    }

    pub(crate) fn entity_slot_count(&self) -> usize {
        self.root.entity_slot_count()
    }

    pub(crate) fn relation_slot_count(&self) -> usize {
        self.root.relation_slot_count()
    }
}
