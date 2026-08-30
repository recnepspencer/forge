use std::collections::BTreeMap;

use worth_foundational::facade::{AspectValue, FieldKey, InternedString};
use worth_relational::facade::indexes::{BoundedEntityFieldLookupRequest, BoundedIndexParityMode};
use worth_relational::facade::runtime::{ProjectionAspectRequirement, ProjectionAspectScope};
use worth_relational::facade::storage::RecordLifecycleState;
use worth_relational::facade::transactions::{
    AspectFieldLocator, AspectFieldPatch, CreateIntent, EntitySpec, MutationIntent,
};

use super::WorthQueryPrimaryGraphProvider;
use crate::domain_computation::primary_graph::application_attempt::WorthQueryApplicationCommitOutcomeIdentity;
use crate::domain_computation::primary_graph::application_attempt::WorthQueryApplicationIdempotencyBinding;
use crate::domain_computation::primary_graph::provider::WorthQueryPrimaryGraphCommittedApplication;
use crate::domain_computation::primary_graph::schema_layout::WorthQueryProviderIdempotencyLayout;

#[derive(Debug)]
pub(in crate::domain_computation::primary_graph) enum WorthQueryProviderIdempotencyResolution {
    Absent,
    Equivalent(WorthQueryPrimaryGraphCommittedApplication),
    Drift,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::domain_computation::primary_graph) enum WorthQueryProviderIdempotencyResolutionDenial
{
    ActiveSnapshotCapacityExhausted { maximum_active_snapshots: usize },
    RetentionCapacityExhausted,
    RetentionIdentityExhausted,
    SnapshotIdentityExhausted,
    Unavailable,
}

impl From<&'static str> for WorthQueryProviderIdempotencyResolutionDenial {
    fn from(_: &'static str) -> Self {
        Self::Unavailable
    }
}

pub(super) fn idempotency_create_intent(
    layout: &WorthQueryProviderIdempotencyLayout,
    binding: WorthQueryApplicationIdempotencyBinding,
    outcome_identity: WorthQueryApplicationCommitOutcomeIdentity,
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
            layout.outcome_identity_locator.clone(),
            AspectValue::UInt64(outcome_identity.get()),
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
        branch: &worth_relational::facade::history::BranchId,
    ) -> Result<
        WorthQueryProviderIdempotencyResolution,
        WorthQueryProviderIdempotencyResolutionDenial,
    > {
        self.application_attempt_work.observe_retained_resolution();
        let layout = self.graph.layout.provider_idempotency().clone();
        self.graph.with_runtime_mut(|runtime| {
            self.resume_pending_application_publication(runtime)
                .map_err(pending_publication_denial)?;
            self.graph
                .ensure_primary_indexes_current_for_branch(runtime, branch)
                .map_err(idempotency_index_currency_denial)?;
            let snapshot = crate::domain_computation::primary_graph::exact_basis_access::open_current_branch_snapshot(runtime, branch)
                .map_err(idempotency_snapshot_denial)?;
            let resolution = resolve_at_snapshot(self, runtime, &snapshot, &layout, binding);
            crate::relational_snapshot_release::release_query_snapshot(runtime, &snapshot);
            let resolution = resolution.map_err(WorthQueryProviderIdempotencyResolutionDenial::from)?;
            if let WorthQueryProviderIdempotencyResolution::Equivalent(committed) = &resolution {
                self.repair_equivalent_publication_settlement(runtime, committed)?;
            }
            Ok(resolution)
        })
    }

    pub(in crate::domain_computation::primary_graph) fn resolve_application_idempotency(
        &self,
        provider_session: &crate::domain_computation::provider_session::WorthQueryProviderSessionTerminalBinding,
    ) -> Result<
        WorthQueryProviderIdempotencyResolution,
        WorthQueryProviderIdempotencyResolutionDenial,
    > {
        let basis = self
            .attempts
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .idempotency_basis(provider_session)
            .ok_or(WorthQueryProviderIdempotencyResolutionDenial::Unavailable)?;
        basis.resolve(self)
    }
}

fn pending_publication_denial(
    failure: crate::domain_computation::WorthQueryProviderSessionFailure,
) -> WorthQueryProviderIdempotencyResolutionDenial {
    match failure.kind() {
        crate::domain_computation::WorthQueryProviderSessionDenialKind::ActiveSnapshotCapacityExhausted {
            maximum_active_snapshots,
        } => WorthQueryProviderIdempotencyResolutionDenial::ActiveSnapshotCapacityExhausted {
            maximum_active_snapshots,
        },
        crate::domain_computation::WorthQueryProviderSessionDenialKind::RetentionCapacityExhausted => {
            WorthQueryProviderIdempotencyResolutionDenial::RetentionCapacityExhausted
        }
        crate::domain_computation::WorthQueryProviderSessionDenialKind::RetentionIdentityExhausted => {
            WorthQueryProviderIdempotencyResolutionDenial::RetentionIdentityExhausted
        }
        crate::domain_computation::WorthQueryProviderSessionDenialKind::SnapshotIdentityExhausted => {
            WorthQueryProviderIdempotencyResolutionDenial::SnapshotIdentityExhausted
        }
        _ => WorthQueryProviderIdempotencyResolutionDenial::Unavailable,
    }
}

fn idempotency_index_currency_denial(
    denial: crate::domain_computation::primary_graph::index_currency::WorthQueryPrimaryIndexCurrencyDenial,
) -> WorthQueryProviderIdempotencyResolutionDenial {
    match denial {
        crate::domain_computation::primary_graph::index_currency::WorthQueryPrimaryIndexCurrencyDenial::Basis(
            crate::domain_computation::primary_graph::WorthQueryExactBasisSnapshotDenial::RetentionCapacityExhausted,
        ) => WorthQueryProviderIdempotencyResolutionDenial::RetentionCapacityExhausted,
        crate::domain_computation::primary_graph::index_currency::WorthQueryPrimaryIndexCurrencyDenial::Basis(
            crate::domain_computation::primary_graph::WorthQueryExactBasisSnapshotDenial::RetentionIdentityExhausted,
        ) => WorthQueryProviderIdempotencyResolutionDenial::RetentionIdentityExhausted,
        crate::domain_computation::primary_graph::index_currency::WorthQueryPrimaryIndexCurrencyDenial::Basis(
            crate::domain_computation::primary_graph::WorthQueryExactBasisSnapshotDenial::SnapshotIdentityExhausted,
        ) => WorthQueryProviderIdempotencyResolutionDenial::SnapshotIdentityExhausted,
        _ => WorthQueryProviderIdempotencyResolutionDenial::Unavailable,
    }
}

fn idempotency_snapshot_denial(
    denial: crate::domain_computation::primary_graph::WorthQueryExactBasisSnapshotDenial,
) -> WorthQueryProviderIdempotencyResolutionDenial {
    match denial {
        crate::domain_computation::primary_graph::WorthQueryExactBasisSnapshotDenial::ActiveSnapshotCapacityExhausted {
            maximum_active_snapshots,
        } => WorthQueryProviderIdempotencyResolutionDenial::ActiveSnapshotCapacityExhausted {
            maximum_active_snapshots,
        },
        crate::domain_computation::primary_graph::WorthQueryExactBasisSnapshotDenial::RetentionCapacityExhausted => {
            WorthQueryProviderIdempotencyResolutionDenial::RetentionCapacityExhausted
        }
        crate::domain_computation::primary_graph::WorthQueryExactBasisSnapshotDenial::RetentionIdentityExhausted => {
            WorthQueryProviderIdempotencyResolutionDenial::RetentionIdentityExhausted
        }
        crate::domain_computation::primary_graph::WorthQueryExactBasisSnapshotDenial::SnapshotIdentityExhausted => {
            WorthQueryProviderIdempotencyResolutionDenial::SnapshotIdentityExhausted
        }
        _ => WorthQueryProviderIdempotencyResolutionDenial::Unavailable,
    }
}

fn resolve_at_snapshot(
    provider: &WorthQueryPrimaryGraphProvider,
    runtime: &worth_relational::facade::runtime::RelationalRuntime,
    snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
    layout: &WorthQueryProviderIdempotencyLayout,
    binding: WorthQueryApplicationIdempotencyBinding,
) -> Result<WorthQueryProviderIdempotencyResolution, &'static str> {
    let key = AspectValue::String(InternedString::from(binding.key_text()));
    let context = WorthQueryIdempotencySnapshotContext {
        provider,
        runtime,
        snapshot,
        layout,
    };
    let Some(entity_id) = locate_idempotency_entity(&context, &key)? else {
        return Ok(WorthQueryProviderIdempotencyResolution::Absent);
    };
    let record = read_idempotency_record(&context, entity_id, &key)?;
    resolve_projected_idempotency(&context, binding, record)
}

struct WorthQueryIdempotencySnapshotContext<'a> {
    provider: &'a WorthQueryPrimaryGraphProvider,
    runtime: &'a worth_relational::facade::runtime::RelationalRuntime,
    snapshot: &'a worth_relational::facade::snapshots::SnapshotHandle,
    layout: &'a WorthQueryProviderIdempotencyLayout,
}

struct WorthQueryIdempotencyProjectionFields {
    key: FieldKey,
    intent: FieldKey,
    outcome_identity: FieldKey,
    emitted_effect_count: FieldKey,
}

struct WorthQueryProjectedIdempotencyRecord {
    created_at_version: worth_relational::facade::identity::VersionId,
    intent: Option<AspectValue>,
    outcome_identity: Option<AspectValue>,
    emitted_effect_count: Option<AspectValue>,
}

fn locate_idempotency_entity(
    context: &WorthQueryIdempotencySnapshotContext<'_>,
    key: &AspectValue,
) -> Result<Option<worth_relational::facade::identity::EntityId>, &'static str> {
    let request = BoundedEntityFieldLookupRequest::new(
        context.snapshot.clone(),
        context.layout.key_index_id,
        context.layout.entity_kind,
        context.layout.key_locator.clone(),
        key.clone(),
        2,
    )
    .map_err(|_| "provider idempotency lookup request was rejected")?;
    let lookup = context
        .runtime
        .index_access()
        .execute_bounded_entity_field_lookup(request, BoundedIndexParityMode::Production)
        .map_err(|_| "provider idempotency index lookup failed")?;
    if lookup.overflowed() || lookup.candidate_entity_ids().len() > 1 {
        return Err("provider idempotency key is not unique");
    }
    Ok(lookup.candidate_entity_ids().first().copied())
}

fn read_idempotency_record(
    context: &WorthQueryIdempotencySnapshotContext<'_>,
    entity_id: worth_relational::facade::identity::EntityId,
    key: &AspectValue,
) -> Result<WorthQueryProjectedIdempotencyRecord, &'static str> {
    let fields = idempotency_projection_fields(context.layout)?;
    let scope = ProjectionAspectScope::from_requirements([ProjectionAspectRequirement::fields(
        context.layout.key_locator.aspect().aspect_key().clone(),
        [
            fields.key.clone(),
            fields.intent.clone(),
            fields.outcome_identity.clone(),
            fields.emitted_effect_count.clone(),
        ],
    )]);
    context
        .runtime
        .read_truth()
        .project_snapshot(context.snapshot)
        .and_then(|view| {
            view.entity_record_with_projection_scope(entity_id, scope, |record| {
                (record.kind_id() == context.layout.entity_kind
                    && record.lifecycle() == RecordLifecycleState::Live
                    && record.aspect_field_value(
                        context.layout.key_locator.aspect().aspect_key(),
                        &fields.key,
                    ) == Some(key))
                .then(|| WorthQueryProjectedIdempotencyRecord {
                    created_at_version: record.created_at_version(),
                    intent: record
                        .aspect_field_value(
                            context.layout.intent_locator.aspect().aspect_key(),
                            &fields.intent,
                        )
                        .cloned(),
                    outcome_identity: record
                        .aspect_field_value(
                            context
                                .layout
                                .outcome_identity_locator
                                .aspect()
                                .aspect_key(),
                            &fields.outcome_identity,
                        )
                        .cloned(),
                    emitted_effect_count: record
                        .aspect_field_value(
                            context
                                .layout
                                .emitted_effect_count_locator
                                .aspect()
                                .aspect_key(),
                            &fields.emitted_effect_count,
                        )
                        .cloned(),
                })
            })
        })
        .ok_or("provider idempotency record is not authoritative")
}

fn idempotency_projection_fields(
    layout: &WorthQueryProviderIdempotencyLayout,
) -> Result<WorthQueryIdempotencyProjectionFields, &'static str> {
    Ok(WorthQueryIdempotencyProjectionFields {
        key: required_locator_field(
            &layout.key_locator,
            "provider idempotency key locator is empty",
        )?,
        intent: required_locator_field(
            &layout.intent_locator,
            "provider idempotency intent locator is empty",
        )?,
        outcome_identity: required_locator_field(
            &layout.outcome_identity_locator,
            "provider idempotency outcome-identity locator is empty",
        )?,
        emitted_effect_count: required_locator_field(
            &layout.emitted_effect_count_locator,
            "provider idempotency emitted-effect-count locator is empty",
        )?,
    })
}

fn required_locator_field(
    locator: &AspectFieldLocator,
    denial: &'static str,
) -> Result<FieldKey, &'static str> {
    locator.field_path().fields().first().cloned().ok_or(denial)
}

#[cfg(test)]
mod denial_mapping_tests {
    use super::*;

    #[test]
    fn retention_identity_exhaustion_survives_idempotency_snapshot_mapping() {
        assert_eq!(
            idempotency_snapshot_denial(
                crate::domain_computation::primary_graph::WorthQueryExactBasisSnapshotDenial::RetentionIdentityExhausted,
            ),
            WorthQueryProviderIdempotencyResolutionDenial::RetentionIdentityExhausted,
        );
    }
}

fn resolve_projected_idempotency(
    context: &WorthQueryIdempotencySnapshotContext<'_>,
    binding: WorthQueryApplicationIdempotencyBinding,
    record: WorthQueryProjectedIdempotencyRecord,
) -> Result<WorthQueryProviderIdempotencyResolution, &'static str> {
    let expected_intent = AspectValue::String(InternedString::from(binding.intent_text()));
    if record.intent.as_ref() != Some(&expected_intent) {
        return Ok(WorthQueryProviderIdempotencyResolution::Drift);
    }
    let committed = context
        .runtime
        .history()
        .historical_committed_version(record.created_at_version)
        .ok_or("provider idempotency creation commit is unavailable")?;
    let Some(AspectValue::UInt64(outcome_identity)) = record.outcome_identity else {
        return Err("provider idempotency outcome identity is unavailable");
    };
    let outcome_identity = WorthQueryApplicationCommitOutcomeIdentity::restore(outcome_identity)
        .ok_or("provider idempotency outcome identity is invalid")?;
    let Some(AspectValue::UInt64(emitted)) = record.emitted_effect_count else {
        return Err("provider idempotency emitted-effect count is unavailable");
    };
    let emitted = usize::try_from(emitted)
        .map_err(|_| "provider idempotency emitted-effect count exceeds host representation")?;
    let commit = committed.commit().clone();
    let committed = context
        .provider
        .observe_completed_application(&commit)
        .ok_or("provider idempotency commit evidence is unavailable")?;
    if committed.application_outcome_identity() != Some(outcome_identity)
        || committed.runtime_instance_id() != context.snapshot.runtime_instance_id()
        || committed.emitted_effect_count() != emitted
    {
        return Err("provider idempotency commit evidence has foreign affinity");
    }
    Ok(WorthQueryProviderIdempotencyResolution::Equivalent(
        committed,
    ))
}
