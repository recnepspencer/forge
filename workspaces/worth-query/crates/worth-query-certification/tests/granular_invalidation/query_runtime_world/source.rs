use std::collections::BTreeMap;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use worth_foundational::facade::{CanonicalFieldPath, ContractValidatedAspectValueView};
use worth_query::facade::runtime::WorthQueryPrimaryGraphSourceProjection;
use worth_query::facade::{domain, foundation, runtime};

#[derive(Default)]
pub struct SourceObservations {
    exact_record_reads: AtomicUsize,
    full_target_reads: AtomicUsize,
}

impl SourceObservations {
    pub fn exact_record_reads(&self) -> usize {
        self.exact_record_reads.load(Ordering::SeqCst)
    }

    pub fn full_target_reads(&self) -> usize {
        self.full_target_reads.load(Ordering::SeqCst)
    }
}

pub struct IntentSourceProjection {
    record: worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts,
    observations: Arc<SourceObservations>,
}

impl IntentSourceProjection {
    pub fn new(
        record: worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts,
        observations: Arc<SourceObservations>,
    ) -> Self {
        Self {
            record,
            observations,
        }
    }
}

impl WorthQueryPrimaryGraphSourceProjection for IntentSourceProjection {
    fn project_live_target(
        &self,
        graph: &worth_query_execution::facade::integration::WorthQueryPrimaryGraphIntegrationHandle,
        _target: &runtime::WorthQueryLiveArtifactTarget,
    ) -> Vec<foundation::WorthQueryEntity> {
        self.observations
            .full_target_reads
            .fetch_add(1, Ordering::SeqCst);
        project_record(graph, self.record).into_iter().collect()
    }

    fn project_granular_scope(
        &self,
        graph: &worth_query_execution::facade::integration::WorthQueryPrimaryGraphIntegrationHandle,
        _target: &runtime::WorthQueryLiveArtifactTarget,
        scope: &domain::WorthQueryMaintenanceScope,
    ) -> Result<Vec<foundation::WorthQueryEntity>, foundation::WorthQueryWorkspaceError> {
        let domain::WorthQueryMaintenanceScope::ExactSourceRecord {
            partition_id,
            local_slot,
            generation,
        } = scope
        else {
            return Err(foundation::WorthQueryWorkspaceError::new(
                "the temporal certification source admits exact records only",
            ));
        };
        let requested = worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts::entity(
            *partition_id,
            *local_slot,
            *generation,
        );
        if requested != self.record {
            return Ok(Vec::new());
        }
        self.observations
            .exact_record_reads
            .fetch_add(1, Ordering::SeqCst);
        Ok(project_record(graph, requested).into_iter().collect())
    }
}

fn project_record(
    graph: &worth_query_execution::facade::integration::WorthQueryPrimaryGraphIntegrationHandle,
    record: worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts,
) -> Option<foundation::WorthQueryEntity> {
    graph.with_runtime(|runtime| {
        let version = runtime
            .history()
            .branch_head(&worth_relational::facade::history::BranchId("main".into()))?
            .version_id;
        let entity = worth_relational::facade::identity::EntityId::new(
            worth_relational::facade::identity::PartitionId::new(record.partition_id()),
            record.local_slot(),
            record.generation(),
        );
        let authoritative = runtime
            .read_truth()
            .visible_entity_at_version(entity, version)?;
        let mut fields = BTreeMap::new();
        for (aspect, value) in authoritative
            .authoritative_aspect_state?
            .aspects()
            .entries()
        {
            let ContractValidatedAspectValueView::Struct(value) = value.view() else {
                continue;
            };
            let aspect = worth_foundational::facade::FieldKey::new(aspect.as_str().to_owned())?;
            fields.extend(value.fields().filter_map(|(field, value)| {
                CanonicalFieldPath::new(vec![aspect.clone(), field.clone()])
                    .map(|path| (path, value.clone()))
            }));
        }
        Some(foundation::WorthQueryEntity::from_native_field_values(
            foundation::WorthQueryEntityIdentity::from_bridge_record_projection(record),
            fields,
        ))
    })
}
