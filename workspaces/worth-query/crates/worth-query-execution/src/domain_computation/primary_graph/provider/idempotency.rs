use std::collections::BTreeMap;

use worth_foundational::facade::{AspectValue, InternedString};
use worth_relational::facade::indexes::{BoundedEntityFieldLookupRequest, BoundedIndexParityMode};
use worth_relational::facade::runtime::{ProjectionAspectRequirement, ProjectionAspectScope};
use worth_relational::facade::storage::RecordLifecycleState;
use worth_relational::facade::transactions::{
    AspectFieldPatch, CreateIntent, EntitySpec, MutationIntent,
};

use super::WorthQueryPrimaryGraphProvider;
use crate::domain_computation::primary_graph::application_attempt::WorthQueryApplicationIdempotencyBinding;
use crate::domain_computation::primary_graph::provider::WorthQueryPrimaryGraphCommittedApplication;
use crate::domain_computation::primary_graph::schema_layout::WorthQueryProviderIdempotencyLayout;

pub(in crate::domain_computation::primary_graph) enum WorthQueryProviderIdempotencyResolution {
    Absent,
    Equivalent(WorthQueryPrimaryGraphCommittedApplication),
    Drift,
}

pub(super) fn idempotency_create_intent(
    layout: &WorthQueryProviderIdempotencyLayout,
    binding: WorthQueryApplicationIdempotencyBinding,
    emitted_effect_count: u64,
) -> MutationIntent {
    let key = binding.key_text();
    let fields = BTreeMap::from([
        (
            layout.key_locator.clone(),
            AspectValue::String(InternedString::from(key.clone())),
        ),
        (
            layout.intent_locator.clone(),
            AspectValue::String(InternedString::from(binding.intent_text())),
        ),
        (
            layout.emitted_effect_count_locator.clone(),
            AspectValue::UInt64(emitted_effect_count),
        ),
    ]);
    MutationIntent::Create(CreateIntent::Entity(EntitySpec {
        partition_id: worth_relational::facade::identity::PartitionId::main(),
        kind_id: layout.entity_kind,
        client_key: worth_relational::facade::symbols::ClientKey::raw(format!(
            "worth-query-idempotency:{key}"
        )),
        fields: AspectFieldPatch::from(fields),
    }))
}

impl WorthQueryPrimaryGraphProvider {
    pub(in crate::domain_computation::primary_graph) fn resolve_idempotency_binding(
        &self,
        binding: WorthQueryApplicationIdempotencyBinding,
        branch_id: &worth_relational::facade::history::BranchId,
    ) -> Result<WorthQueryProviderIdempotencyResolution, &'static str> {
        let layout = self.graph.layout.provider_idempotency().clone();
        self.graph.with_runtime_mut(|runtime| {
            self.graph
                .ensure_primary_indexes_current(runtime, branch_id)?;
            let snapshot = runtime.snapshots().snapshot();
            let resolution = if &snapshot.branch_id == branch_id {
                resolve_at_snapshot(runtime, &snapshot, &layout, binding)
            } else {
                Err("provider idempotency snapshot belongs to another branch")
            };
            runtime.snapshots().release_snapshot(&snapshot);
            resolution
        })
    }

    pub(in crate::domain_computation::primary_graph) fn resolve_application_idempotency(
        &self,
        session_identity: &str,
    ) -> Result<WorthQueryProviderIdempotencyResolution, &'static str> {
        let (binding, branch_id) = self
            .sessions
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .application_attempts
            .get(session_identity)
            .map(|attempt| (attempt.idempotency, attempt.branch_id.clone()))
            .ok_or("provider session lost its idempotency binding")?;
        self.resolve_idempotency_binding(binding, &branch_id)
    }
}

fn resolve_at_snapshot(
    runtime: &worth_relational::facade::runtime::RelationalRuntime,
    snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
    layout: &WorthQueryProviderIdempotencyLayout,
    binding: WorthQueryApplicationIdempotencyBinding,
) -> Result<WorthQueryProviderIdempotencyResolution, &'static str> {
    let key = AspectValue::String(InternedString::from(binding.key_text()));
    let request = BoundedEntityFieldLookupRequest::new(
        snapshot.clone(),
        layout.key_index_id,
        layout.entity_kind,
        layout.key_locator.clone(),
        key.clone(),
        2,
    )
    .map_err(|_| "provider idempotency lookup request was rejected")?;
    let lookup = runtime
        .index_access()
        .execute_bounded_entity_field_lookup(request, BoundedIndexParityMode::Production)
        .map_err(|_| "provider idempotency index lookup failed")?;
    if lookup.overflowed() || lookup.candidate_entity_ids().len() > 1 {
        return Err("provider idempotency key is not unique");
    }
    let Some(entity_id) = lookup.candidate_entity_ids().first().copied() else {
        return Ok(WorthQueryProviderIdempotencyResolution::Absent);
    };
    let key_field = layout
        .key_locator
        .field_path()
        .fields()
        .first()
        .cloned()
        .ok_or("provider idempotency key locator is empty")?;
    let intent_field = layout
        .intent_locator
        .field_path()
        .fields()
        .first()
        .cloned()
        .ok_or("provider idempotency intent locator is empty")?;
    let emitted_effect_count_field = layout
        .emitted_effect_count_locator
        .field_path()
        .fields()
        .first()
        .cloned()
        .ok_or("provider idempotency emitted-effect-count locator is empty")?;
    let scope = ProjectionAspectScope::from_requirements([ProjectionAspectRequirement::fields(
        layout.key_locator.aspect().aspect_key().clone(),
        [
            key_field.clone(),
            intent_field.clone(),
            emitted_effect_count_field.clone(),
        ],
    )]);
    let record = runtime
        .read_truth()
        .project_snapshot(snapshot)
        .and_then(|view| {
            view.entity_record_with_projection_scope(entity_id, scope, |record| {
                (record.kind_id() == layout.entity_kind
                    && record.lifecycle() == RecordLifecycleState::Live
                    && record
                        .aspect_field_value(layout.key_locator.aspect().aspect_key(), &key_field)
                        == Some(&key))
                .then(|| {
                    (
                        record.created_at_version(),
                        record
                            .aspect_field_value(
                                layout.intent_locator.aspect().aspect_key(),
                                &intent_field,
                            )
                            .cloned(),
                        record
                            .aspect_field_value(
                                layout.emitted_effect_count_locator.aspect().aspect_key(),
                                &emitted_effect_count_field,
                            )
                            .cloned(),
                    )
                })
            })
        })
        .ok_or("provider idempotency record is not authoritative")?;
    let expected_intent = AspectValue::String(InternedString::from(binding.intent_text()));
    if record.1.as_ref() != Some(&expected_intent) {
        return Ok(WorthQueryProviderIdempotencyResolution::Drift);
    }
    let committed = runtime
        .history()
        .committed_version(record.0)
        .ok_or("provider idempotency creation commit is unavailable")?;
    let Some(AspectValue::UInt64(emitted)) = record.2 else {
        return Err("provider idempotency emitted-effect count is unavailable");
    };
    let emitted = usize::try_from(emitted)
        .map_err(|_| "provider idempotency emitted-effect count exceeds host representation")?;
    Ok(WorthQueryProviderIdempotencyResolution::Equivalent(
        WorthQueryPrimaryGraphCommittedApplication::new(
            snapshot.runtime_instance_id,
            committed.commit().branch_id.clone(),
            committed.commit().commit_id,
            committed.changed_record_count(),
            emitted,
        ),
    ))
}
