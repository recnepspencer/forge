//! Fresh Relational owner reads for committed dispatch-outbox rows.

use worth_foundational::facade::{AspectValue, InternedString};
use worth_relational::facade::history::CommitReference;
use worth_relational::facade::indexes::{BoundedEntityFieldLookupRequest, BoundedIndexParityMode};
use worth_relational::facade::runtime::{ProjectionAspectRequirement, ProjectionAspectScope};
use worth_relational::facade::storage::RecordLifecycleState;
use worth_relational::facade::transactions::RecordRef;

use super::WorthQueryPrimaryGraphProvider;
use crate::domain_computation::application_aftermath::{
    WorthQueryDispatchOutboxLayout, WorthQueryDispatchOutboxRecord,
};
use crate::domain_computation::primary_graph::WorthQueryApplicationCommitReceipt;

mod restoration;
mod work;

use restoration::{hex_bytes, required_fields, restore_record};
pub use work::WorthQueryCommittedDispatchOutboxReadWork;

/// Fresh Query-provider observation of one authoritative Relational outbox row.
///
/// This is read-only owner evidence, not publication authority. Only this
/// provider owner module can seal production observations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryCommittedDispatchOutboxObservation {
    record: WorthQueryDispatchOutboxRecord,
    commit: CommitReference,
    record_ref: RecordRef,
    relational_runtime_instance_id: u64,
    work: WorthQueryCommittedDispatchOutboxReadWork,
}

/// Why Query could not establish an authoritative committed outbox row.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryCommittedDispatchOutboxReadDenial {
    ForeignRuntime,
    IndexUnavailable,
    Missing,
    Ambiguous,
    NotAuthoritative,
    ExactCommitUnavailable,
    Malformed,
    CommitMismatch,
    RecordMismatch,
}

use WorthQueryCommittedDispatchOutboxReadDenial as Denial;

impl WorthQueryCommittedDispatchOutboxObservation {
    const fn seal(
        record: WorthQueryDispatchOutboxRecord,
        commit: CommitReference,
        record_ref: RecordRef,
        relational_runtime_instance_id: u64,
        work: WorthQueryCommittedDispatchOutboxReadWork,
    ) -> Self {
        Self {
            record,
            commit,
            record_ref,
            relational_runtime_instance_id,
            work,
        }
    }

    #[cfg(test)]
    pub(crate) const fn fixture(
        record: WorthQueryDispatchOutboxRecord,
        commit: CommitReference,
        record_ref: RecordRef,
    ) -> Self {
        Self::seal(
            record,
            commit,
            record_ref,
            1,
            WorthQueryCommittedDispatchOutboxReadWork::exact_read(1),
        )
    }

    pub const fn record(&self) -> &WorthQueryDispatchOutboxRecord {
        &self.record
    }

    pub const fn commit_reference(&self) -> &CommitReference {
        &self.commit
    }

    pub const fn record_ref(&self) -> &RecordRef {
        &self.record_ref
    }

    pub const fn relational_runtime_instance_id(&self) -> u64 {
        self.relational_runtime_instance_id
    }

    pub const fn work(&self) -> WorthQueryCommittedDispatchOutboxReadWork {
        self.work
    }
}

impl WorthQueryPrimaryGraphProvider {
    pub(in crate::domain_computation::primary_graph) fn committed_dispatch_outbox(
        &self,
        receipt: &WorthQueryApplicationCommitReceipt,
    ) -> Result<Option<WorthQueryCommittedDispatchOutboxObservation>, Denial> {
        let Some(expected) = receipt.dispatch_outbox() else {
            return Ok(None);
        };
        self.observe_expected(
            expected,
            receipt.commit_reference(),
            receipt.provider_runtime_instance_id(),
        )
        .map(Some)
    }

    pub(in crate::domain_computation::primary_graph) fn committed_dispatch_outbox_for_binding(
        &self,
        binding: &crate::domain_computation::application_aftermath::WorthQueryRecoveryHandleBinding,
    ) -> Result<WorthQueryCommittedDispatchOutboxObservation, Denial> {
        let expected = binding.dispatch_outbox().ok_or(Denial::Missing)?;
        self.observe_expected(
            expected,
            binding.commit_reference(),
            binding.runtime_instance_id(),
        )
    }

    fn observe_expected(
        &self,
        expected: &WorthQueryDispatchOutboxRecord,
        expected_commit: &worth_relational::facade::history::CommitReference,
        expected_runtime: u64,
    ) -> Result<WorthQueryCommittedDispatchOutboxObservation, Denial> {
        let layout = self.graph.layout.provider_dispatch_outbox().clone();
        let expected_commit = expected_commit.clone();
        self.graph.with_runtime_mut(|runtime| {
            let retained = runtime
                .snapshots()
                .retained_snapshot_for_commit(expected_runtime, &expected_commit)
                .map_err(map_retained_snapshot_denial)?;
            CommittedOutboxRead {
                runtime,
                snapshot: retained.snapshot_handle(),
                layout: &layout,
                expected,
                expected_commit: retained.commit(),
            }
            .resolve()
        })
    }
}

fn map_retained_snapshot_denial(
    denial: worth_relational::facade::runtime::RelationalRetainedCommitSnapshotDenial,
) -> Denial {
    use worth_relational::facade::runtime::RelationalRetainedCommitSnapshotDenialKind as Kind;
    match denial.kind() {
        Kind::ForeignRuntime => Denial::ForeignRuntime,
        Kind::VersionUnavailable | Kind::SnapshotNotRetained => Denial::ExactCommitUnavailable,
        Kind::BranchMismatch | Kind::CommitMismatch | Kind::SnapshotBindingMismatch => {
            Denial::CommitMismatch
        }
    }
}

impl<Schema> super::super::WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: worth_query_installation::facade::ApplicationSchema,
{
    /// Reads this receipt's outbox from a fresh provider-owned Relational view.
    pub fn observe_committed_dispatch_outbox(
        &self,
        receipt: &WorthQueryApplicationCommitReceipt,
    ) -> Result<Option<WorthQueryCommittedDispatchOutboxObservation>, Denial> {
        self.primary_provider.committed_dispatch_outbox(receipt)
    }
}

struct CommittedOutboxRead<'a> {
    runtime: &'a worth_relational::facade::runtime::RelationalRuntime,
    snapshot: &'a worth_relational::facade::snapshots::SnapshotHandle,
    layout: &'a WorthQueryDispatchOutboxLayout,
    expected: &'a WorthQueryDispatchOutboxRecord,
    expected_commit: &'a worth_relational::facade::history::CommitReference,
}

impl CommittedOutboxRead<'_> {
    fn resolve(&self) -> Result<WorthQueryCommittedDispatchOutboxObservation, Denial> {
        let (entity_id, examined_entries) = self.lookup_entity()?;
        let (created_at, values) = self.read_record(entity_id)?;
        let record = restore_record(values)?;
        if &record != self.expected {
            return Err(Denial::RecordMismatch);
        }
        let committed = self
            .runtime
            .history()
            .committed_version(created_at)
            .ok_or(Denial::NotAuthoritative)?;
        if committed.commit() != self.expected_commit {
            return Err(Denial::CommitMismatch);
        }
        Ok(WorthQueryCommittedDispatchOutboxObservation::seal(
            record,
            committed.commit().clone(),
            RecordRef::Entity(entity_id),
            self.snapshot.runtime_instance_id,
            WorthQueryCommittedDispatchOutboxReadWork::exact_read(examined_entries),
        ))
    }

    fn lookup_entity(
        &self,
    ) -> Result<(worth_relational::facade::identity::EntityId, usize), Denial> {
        let request = BoundedEntityFieldLookupRequest::new(
            self.snapshot.clone(),
            self.layout.correlation_index_id,
            self.layout.entity_kind,
            self.layout.correlation_locator.clone(),
            AspectValue::String(InternedString::from(hex_bytes(
                self.expected.correlation().bytes(),
            ))),
            2,
        )
        .map_err(|_| Denial::IndexUnavailable)?;
        let lookup = self
            .runtime
            .index_access()
            .execute_bounded_entity_field_lookup(request, BoundedIndexParityMode::Production)
            .map_err(|_| Denial::IndexUnavailable)?;
        if lookup.overflowed() || lookup.candidate_entity_ids().len() > 1 {
            return Err(Denial::Ambiguous);
        }
        let entity = lookup
            .candidate_entity_ids()
            .first()
            .copied()
            .ok_or(Denial::Missing)?;
        Ok((entity, lookup.examined_entry_count()))
    }

    fn read_record(
        &self,
        entity_id: worth_relational::facade::identity::EntityId,
    ) -> Result<
        (
            worth_relational::facade::identity::VersionId,
            Vec<AspectValue>,
        ),
        Denial,
    > {
        let fields = required_fields(self.layout)?;
        let aspect = self
            .layout
            .correlation_locator
            .aspect()
            .aspect_key()
            .clone();
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
                            fields
                                .iter()
                                .map(|field| record.aspect_field_value(&aspect, field).cloned())
                                .collect::<Option<Vec<_>>>()
                                .map(|values| (record.created_at_version(), values))
                        })
                        .flatten()
                })
            })
            .ok_or(Denial::NotAuthoritative)
    }
}

#[cfg(test)]
#[path = "committed_dispatch_outbox_tests.rs"]
mod tests;
