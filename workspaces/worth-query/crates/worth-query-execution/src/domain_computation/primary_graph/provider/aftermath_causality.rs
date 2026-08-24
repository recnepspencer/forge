//! Relational persistence and owner reads for Query aftermath causal facts.

use std::collections::BTreeMap;

use worth_foundational::facade::{AspectValue, InternedString};
use worth_relational::facade::identity::VersionId;
use worth_relational::facade::indexes::{BoundedEntityFieldLookupRequest, BoundedIndexParityMode};
use worth_relational::facade::runtime::{ProjectionAspectRequirement, ProjectionAspectScope};
use worth_relational::facade::storage::RecordLifecycleState;
use worth_relational::facade::transactions::{
    AspectFieldPatch, CreateIntent, EntitySpec, MutationIntent, RecordRef,
};

use super::WorthQueryPrimaryGraphProvider;
use crate::domain_computation::application_aftermath::{
    WorthQueryCommittedAftermathCausality, WorthQueryPendingAftermathCausality,
};
use crate::domain_computation::primary_graph::application_attempt::WorthQueryApplicationCommitOutcomeIdentity;
use crate::domain_computation::primary_graph::schema_layout::WorthQueryAftermathCausalityLayout;

pub(super) fn aftermath_causality_create_intent(
    layout: &WorthQueryAftermathCausalityLayout,
    pending: &WorthQueryPendingAftermathCausality,
    outcome_identity: WorthQueryApplicationCommitOutcomeIdentity,
) -> MutationIntent {
    let key = pending.key();
    let fields = BTreeMap::from([
        (
            layout.key_locator.clone(),
            AspectValue::String(InternedString::from(key.clone())),
        ),
        (
            layout.role_locator.clone(),
            AspectValue::UInt64(pending.role().code()),
        ),
        (
            layout.parent_branch_locator.clone(),
            AspectValue::String(InternedString::from(pending.parent().branch_id.0.clone())),
        ),
        (
            layout.parent_commit_locator.clone(),
            AspectValue::UInt64(pending.parent().commit_id.0),
        ),
        (
            layout.outcome_identity_locator.clone(),
            AspectValue::UInt64(outcome_identity.get()),
        ),
    ]);
    MutationIntent::Create(CreateIntent::Entity(EntitySpec {
        partition_id: worth_relational::facade::identity::PartitionId::main(),
        kind_id: layout.entity_kind,
        client_key: worth_relational::facade::symbols::ClientKey::raw(format!(
            "worth-query-aftermath-causality:{key}"
        )),
        fields: AspectFieldPatch::from(fields),
    }))
}

impl WorthQueryPrimaryGraphProvider {
    fn branch_head(
        &self,
        branch: &worth_relational::facade::history::BranchId,
    ) -> Option<worth_relational::facade::history::RelationalCommitReceipt> {
        self.graph.with_runtime(|runtime| {
            crate::domain_computation::primary_graph::exact_basis_access::current_branch_head(
                runtime, branch,
            )
        })
    }

    pub(in crate::domain_computation::primary_graph) fn resolve_aftermath_causality(
        &self,
        pending: &WorthQueryPendingAftermathCausality,
        outcome_identity: Option<WorthQueryApplicationCommitOutcomeIdentity>,
    ) -> Result<Option<WorthQueryCommittedAftermathCausality>, &'static str> {
        let layout = self.graph.layout.provider_aftermath_causality().clone();
        let branch = pending.parent().branch_id.clone();
        self.graph.with_runtime_mut(|runtime| {
            self.graph
                .ensure_primary_indexes_current_for_branch(runtime, &branch)?;
            let snapshot = crate::domain_computation::primary_graph::exact_basis_access::open_current_branch_snapshot(runtime, &branch)
                .ok_or("aftermath causality branch has no current snapshot")?;
            let resolution = WorthQueryAftermathCausalityRead {
                runtime,
                snapshot: &snapshot,
                layout: &layout,
                pending,
                outcome_identity,
            }
            .resolve();
            runtime.snapshots().release_snapshot(&snapshot);
            resolution
        })
    }
}

impl<Schema> super::super::WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: worth_query_installation::facade::ApplicationSchema,
{
    pub(crate) fn relational_branch_head(
        &self,
        branch: &worth_relational::facade::history::BranchId,
    ) -> Option<worth_relational::facade::history::RelationalCommitReceipt> {
        self.primary_provider.branch_head(branch)
    }

    pub(crate) fn committed_aftermath_causality(
        &self,
        pending: &WorthQueryPendingAftermathCausality,
    ) -> Result<Option<WorthQueryCommittedAftermathCausality>, &'static str> {
        self.primary_provider
            .resolve_aftermath_causality(pending, None)
    }
}

struct WorthQueryAftermathCausalityRead<'a> {
    runtime: &'a worth_relational::facade::runtime::RelationalRuntime,
    snapshot: &'a worth_relational::facade::snapshots::SnapshotHandle,
    layout: &'a WorthQueryAftermathCausalityLayout,
    pending: &'a WorthQueryPendingAftermathCausality,
    outcome_identity: Option<WorthQueryApplicationCommitOutcomeIdentity>,
}

impl WorthQueryAftermathCausalityRead<'_> {
    fn resolve(&self) -> Result<Option<WorthQueryCommittedAftermathCausality>, &'static str> {
        let Some(entity_id) = self.lookup_entity()? else {
            return Ok(None);
        };
        let (created_at, values) = self.read_record(entity_id)?;
        if values
            .iter()
            .zip(self.expected_values(&values).iter())
            .any(|(observed, expected)| observed.as_ref() != Some(expected))
        {
            return Err("aftermath causality record drifted from admitted meaning");
        }
        let committed = self
            .runtime
            .history()
            .historical_committed_version(created_at)
            .ok_or("aftermath causality creation commit is unavailable")?;
        WorthQueryCommittedAftermathCausality::seal(
            self.pending.clone(),
            committed.commit().clone(),
            RecordRef::Entity(entity_id),
        )
        .map(Some)
        .ok_or("aftermath causality is not a linear Relational child")
    }

    fn lookup_entity(
        &self,
    ) -> Result<Option<worth_relational::facade::identity::EntityId>, &'static str> {
        let request = BoundedEntityFieldLookupRequest::new(
            self.snapshot.clone(),
            self.layout.key_index_id,
            self.layout.entity_kind,
            self.layout.key_locator.clone(),
            self.key_value(),
            2,
        )
        .map_err(|_| "aftermath causality lookup request was rejected")?;
        let lookup = self
            .runtime
            .index_access()
            .execute_bounded_entity_field_lookup(request, BoundedIndexParityMode::Production)
            .map_err(|_| "aftermath causality index lookup failed")?;
        if lookup.overflowed() || lookup.candidate_entity_ids().len() > 1 {
            return Err("aftermath causality key is not unique");
        }
        Ok(lookup.candidate_entity_ids().first().copied())
    }

    fn read_record(
        &self,
        entity_id: worth_relational::facade::identity::EntityId,
    ) -> Result<AftermathCausalityRecord, &'static str> {
        let fields = required_fields(self.layout)?;
        let aspect = self.layout.key_locator.aspect().aspect_key().clone();
        let scope =
            ProjectionAspectScope::from_requirements([ProjectionAspectRequirement::fields(
                aspect.clone(),
                fields.clone(),
            )]);
        self.runtime
            .read_truth()
            .project_snapshot(self.snapshot)
            .and_then(|view| {
                view.entity_record_with_projection_scope(entity_id, scope, |record| {
                    (record.kind_id() == self.layout.entity_kind
                        && record.lifecycle() == RecordLifecycleState::Live)
                        .then(|| {
                            let values = fields
                                .iter()
                                .map(|field| record.aspect_field_value(&aspect, field).cloned())
                                .collect();
                            (record.created_at_version(), values)
                        })
                })
            })
            .ok_or("aftermath causality record is not authoritative")
    }

    fn expected_values(&self, observed: &AftermathCausalityValues) -> [AspectValue; 5] {
        [
            self.key_value(),
            AspectValue::UInt64(self.pending.role().code()),
            AspectValue::String(InternedString::from(
                self.pending.parent().branch_id.0.clone(),
            )),
            AspectValue::UInt64(self.pending.parent().commit_id.0),
            AspectValue::UInt64(self.outcome_identity.map_or_else(
                || observed[4].as_ref().and_then(as_u64).unwrap_or(0),
                WorthQueryApplicationCommitOutcomeIdentity::get,
            )),
        ]
    }

    fn key_value(&self) -> AspectValue {
        AspectValue::String(InternedString::from(self.pending.key()))
    }
}

type AftermathCausalityValues = Vec<Option<AspectValue>>;
type AftermathCausalityRecord = (VersionId, AftermathCausalityValues);

fn required_fields(
    layout: &WorthQueryAftermathCausalityLayout,
) -> Result<Vec<worth_foundational::facade::FieldKey>, &'static str> {
    [
        &layout.key_locator,
        &layout.role_locator,
        &layout.parent_branch_locator,
        &layout.parent_commit_locator,
        &layout.outcome_identity_locator,
    ]
    .into_iter()
    .map(|locator| {
        locator
            .field_path()
            .fields()
            .first()
            .cloned()
            .ok_or("aftermath causality field locator is empty")
    })
    .collect()
}

fn as_u64(value: &AspectValue) -> Option<u64> {
    match value {
        AspectValue::UInt64(value) => Some(*value),
        _ => None,
    }
}
