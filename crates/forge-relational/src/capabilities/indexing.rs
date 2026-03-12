use crate::indexes::data::DerivedIndexGeneration;
use crate::logic::runtime::RelationalRuntime;
use crate::storage::data::RelationalReadView;
use crate::lineage::data::LineageEventRecord;
use crate::identity::data::VersionId;

pub(crate) trait ReplayRead {
    fn read_view_at_version(&self, version_id: VersionId) -> RelationalReadView;
    fn index_generations_at_version(&self, version_id: VersionId) -> Vec<DerivedIndexGeneration>;
}

impl ReplayRead for RelationalRuntime {
    fn read_view_at_version(&self, version_id: VersionId) -> RelationalReadView {
        self.visibility_reads().read_version(version_id)
    }

    fn index_generations_at_version(&self, version_id: VersionId) -> Vec<DerivedIndexGeneration> {
        self.index_access().generations_for_version(version_id)
    }
}

pub(crate) trait LineageRead {
    fn lineage_events(&self) -> &[LineageEventRecord];
}

impl LineageRead for RelationalRuntime {
    fn lineage_events(&self) -> &[LineageEventRecord] {
        &self.lineage.events
    }
}
