//! Fresh Relational owner reads for committed dispatch-outbox rows.

use worth_foundational::facade::AspectValue;
use worth_relational::facade::history::CommitReference;
use worth_relational::facade::runtime::{ProjectionAspectRequirement, ProjectionAspectScope};
use worth_relational::facade::transactions::RecordRef;

use super::WorthQueryPrimaryGraphProvider;
use crate::domain_computation::application_aftermath::{
    WorthQueryDispatchOutboxLayout, WorthQueryDispatchOutboxRecord,
};
use crate::domain_computation::primary_graph::WorthQueryApplicationCommitReceipt;

#[cfg(test)]
mod layout_tests;
#[cfg(test)]
mod owner_test_support;
mod restoration;
#[cfg(test)]
mod test_support;
mod work;

use restoration::{required_fields, restore_record};
#[cfg(test)]
pub(in crate::domain_computation::primary_graph) use test_support::commit_and_observe_fixture;
#[cfg(test)]
pub(in crate::domain_computation) use test_support::{
    commit_distinct_records_and_admit_fixture, commit_observe_and_admit_fixture,
    commit_observe_and_admit_twice_fixture,
};
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
    Missing,
    WrongRecordKind,
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
        let Some(binding) = receipt.committed_dispatch_outbox() else {
            return Ok(None);
        };
        self.observe_expected(
            binding,
            receipt.commit_reference(),
            receipt.provider_runtime_instance_id(),
        )
        .map(Some)
    }

    pub(in crate::domain_computation::primary_graph) fn committed_dispatch_outbox_for_binding(
        &self,
        binding: &crate::domain_computation::application_aftermath::WorthQueryRecoveryHandleBinding,
    ) -> Result<WorthQueryCommittedDispatchOutboxObservation, Denial> {
        let committed = binding.committed_dispatch_outbox().ok_or(Denial::Missing)?;
        self.observe_expected(
            committed,
            binding.commit_reference(),
            binding.runtime_instance_id(),
        )
    }

    fn observe_expected(
        &self,
        binding: &super::WorthQueryCommittedDispatchOutboxBinding,
        expected_commit: &worth_relational::facade::history::CommitReference,
        expected_runtime: u64,
    ) -> Result<WorthQueryCommittedDispatchOutboxObservation, Denial> {
        let layout = self.graph.layout.provider_dispatch_outbox().clone();
        let expected_commit = expected_commit.clone();
        self.graph.with_runtime_mut(|runtime| {
            CommittedOutboxRead {
                runtime,
                layout: &layout,
                binding,
                expected_commit: &expected_commit,
                expected_runtime,
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
        Kind::EntityKindMismatch => Denial::WrongRecordKind,
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
    runtime: &'a mut worth_relational::facade::runtime::RelationalRuntime,
    layout: &'a WorthQueryDispatchOutboxLayout,
    binding: &'a super::WorthQueryCommittedDispatchOutboxBinding,
    expected_commit: &'a worth_relational::facade::history::CommitReference,
    expected_runtime: u64,
}

impl CommittedOutboxRead<'_> {
    fn resolve(&mut self) -> Result<WorthQueryCommittedDispatchOutboxObservation, Denial> {
        let RecordRef::Entity(entity_id) = self.binding.record_ref() else {
            return Err(Denial::NotAuthoritative);
        };
        let entity_id = *entity_id;
        let (created_at, values, owner_work) = self.read_record(entity_id)?;
        let record = restore_record(values)?;
        if &record != self.binding.record() {
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
            self.expected_runtime,
            WorthQueryCommittedDispatchOutboxReadWork::from_owner(owner_work),
        ))
    }

    fn read_record(
        &mut self,
        entity_id: worth_relational::facade::identity::EntityId,
    ) -> Result<
        (
            worth_relational::facade::identity::VersionId,
            Vec<AspectValue>,
            worth_relational::facade::runtime::RelationalRetainedCommitProjectionWork,
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
        let projection = self
            .runtime
            .snapshots()
            .project_retained_entity_for_commit(
                self.expected_runtime,
                self.expected_commit,
                entity_id,
                self.layout.entity_kind,
                scope,
                |record| {
                    Some(
                        fields
                            .iter()
                            .map(|field| record.aspect_field_value(&aspect, field).cloned())
                            .collect::<Option<Vec<_>>>()
                            .map(|values| (record.created_at_version(), values))
                            .ok_or(Denial::NotAuthoritative),
                    )
                },
            )
            .map_err(map_retained_snapshot_denial)?;
        let (projected, work) = projection.into_parts();
        let (created_at, values) = projected.ok_or(Denial::Missing)??;
        Ok((created_at, values, work))
    }
}

#[cfg(test)]
#[path = "committed_dispatch_outbox_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "committed_dispatch_outbox/restoration_tests.rs"]
mod restoration_tests;

#[cfg(test)]
#[path = "committed_dispatch_outbox/corruption_tests.rs"]
mod corruption_tests;
