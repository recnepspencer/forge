use crate::identity::data::VersionId;
use crate::indexes::data::DerivedIndexGeneration;
use crate::lineage::data::LineageEventRecord;
use crate::logic::runtime::RelationalRuntime;
use crate::replay::data::ReplaySnapshotSurface;

pub(crate) trait ReplayRead {
    fn replay_snapshot_surface_at_version(&self, version_id: VersionId) -> ReplaySnapshotSurface;
    fn index_generations_at_version(&self, version_id: VersionId) -> Vec<DerivedIndexGeneration>;
}

impl ReplayRead for RelationalRuntime {
    fn replay_snapshot_surface_at_version(&self, version_id: VersionId) -> ReplaySnapshotSurface {
        let projection = self.visibility_reads().project_version(version_id);
        ReplaySnapshotSurface {
            version_id,
            entities: projection.all_entity_records(),
            relations: projection.all_relation_records(),
        }
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
