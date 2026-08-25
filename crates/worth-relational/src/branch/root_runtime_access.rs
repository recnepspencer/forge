use super::root::RelationalBranchRootState;

impl RelationalBranchRootState {
    pub(crate) fn version_id(&self) -> crate::identity::data::VersionId {
        self.root
            .axes()
            .map(|axes| crate::identity::data::VersionId(axes.storage_version))
            .unwrap_or(crate::identity::data::VersionId(0))
    }
}
