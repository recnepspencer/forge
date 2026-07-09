use crate::{
    authority::{
        AuthoritativeBranchHeadRecord, CanonicalizedCommitEnvelope, PersistedAuthoritativeCommit,
        VerifiedAuthoritativeAppend,
    },
    compatibility::CompatibilityFamilyKind,
    delta::{
        SharedBaseBranchCreationReceipt, SharedBaseBranchCreationRequest,
        SharedBaseBranchCreationWitness,
    },
    failure::{StoreError, StoreErrorKind},
};
use worth_relational::facade::history::BranchId;

use super::{core::verify_durable_barrier, StateBackedStoreBackend, StatePersistence};
use crate::authority::FetchedAuthoritativeCommit;
use worth_relational::facade::history::CommitId;

impl<P: StatePersistence> StateBackedStoreBackend<P> {
    pub fn create_branch(
        &mut self,
        new_branch: BranchId,
        from_branch: Option<&BranchId>,
    ) -> Result<AuthoritativeBranchHeadRecord, StoreError> {
        self.admit_runtime_write_compatibility(
            CompatibilityFamilyKind::BranchVersionDagRecord,
            "create_branch",
        )?;
        let created_branch_id = new_branch.clone();
        let applied = self
            .state
            .apply_branch_creation_in_place(new_branch, from_branch)?;
        if let Err(error) = self.state.verify_applied_branch_creation(&applied) {
            self.state.rollback_branch_creation(applied);
            return Err(error);
        }
        let report = match self.persistence.persist_state(&self.state) {
            Ok(report) => report,
            Err(error) => {
                self.state.rollback_branch_creation(applied);
                return Err(error);
            }
        };
        if let Err(error) = verify_durable_barrier(&mut self.counters, &report) {
            self.state.rollback_branch_creation(applied);
            return Err(error);
        }
        self.counters.record_state_delta_apply(2, 2);
        self.counters.record_branch_create();
        self.fetch_branch_head(&created_branch_id)
    }

    pub fn create_shared_base_branch(
        &mut self,
        request: SharedBaseBranchCreationRequest,
    ) -> Result<SharedBaseBranchCreationReceipt, StoreError> {
        self.admit_runtime_write_compatibility(
            CompatibilityFamilyKind::BranchVersionDagRecord,
            "create_shared_base_branch",
        )?;
        let (applied, receipt) = self
            .state
            .apply_shared_base_branch_creation_in_place(request)?;
        if let Err(error) = self
            .state
            .verify_applied_shared_base_branch_creation(&applied)
        {
            self.state.rollback_shared_base_branch_creation(applied);
            return Err(error);
        }
        let report = match self.persistence.persist_state(&self.state) {
            Ok(report) => report,
            Err(error) => {
                self.state.rollback_shared_base_branch_creation(applied);
                return Err(error);
            }
        };
        if let Err(error) = verify_durable_barrier(&mut self.counters, &report) {
            self.state.rollback_shared_base_branch_creation(applied);
            return Err(error);
        }
        self.counters.record_state_delta_apply(3, 3);
        self.counters.record_branch_create();
        self.counters.record_branch_base_reuse();
        Ok(receipt)
    }

    pub fn admit_shared_base_branch_creation(
        &self,
        request: SharedBaseBranchCreationRequest,
    ) -> Result<SharedBaseBranchCreationWitness, StoreError> {
        self.state.admit_shared_base_branch_creation(request)
    }

    pub fn verify_append(
        &self,
        append: CanonicalizedCommitEnvelope,
    ) -> Result<VerifiedAuthoritativeAppend, StoreError> {
        self.state.verify_authoritative_append(append)
    }

    pub fn append(
        &mut self,
        verified: VerifiedAuthoritativeAppend,
    ) -> Result<PersistedAuthoritativeCommit, StoreError> {
        self.admit_runtime_write_compatibility(CompatibilityFamilyKind::CommitEnvelope, "append")?;
        let commit_id = verified.envelope().commit.commit_id;
        if let Some(existing) = self.state.commit_record(commit_id) {
            return Ok(existing.clone().into_persisted());
        }

        let branch_already_exists = self
            .state
            .branch_exists(&verified.envelope().branch_context);
        let emits_schema_support = verified.envelope().schema_transition.is_some()
            || verified.envelope().schema_continuation_descriptor.is_some()
            || verified
                .envelope()
                .schema_reconciliation_descriptor
                .is_some();
        let emits_lineage_support = !verified.envelope().lineage_event_ids().is_empty()
            || !verified.envelope().lineage_events().is_empty();
        let support_family_writes =
            1 + u64::from(emits_schema_support) + u64::from(emits_lineage_support);
        let digest_writes = verified.envelope().commit.parents.len() as u64
            + support_family_writes
            + if branch_already_exists { 2 } else { 4 };
        let branch_head_writes = if branch_already_exists { 1 } else { 2 };
        let touched_families = if branch_already_exists { 3 } else { 4 }
            + usize::from(emits_schema_support)
            + usize::from(emits_lineage_support)
            + 1;
        let touched_records = verified.envelope().commit.parents.len()
            + support_family_writes as usize
            + if branch_already_exists { 2 } else { 3 };
        let applied = self.state.apply_verified_append_in_place(&verified)?;
        if let Err(error) = self.state.verify_applied_authoritative_append(&applied) {
            self.state.rollback_verified_append(applied);
            return Err(error);
        }
        let report = match self.persistence.persist_state(&self.state) {
            Ok(report) => report,
            Err(error) => {
                self.state.rollback_verified_append(applied);
                return Err(error);
            }
        };
        if let Err(error) = verify_durable_barrier(&mut self.counters, &report) {
            self.state.rollback_verified_append(applied);
            return Err(error);
        }
        self.counters
            .record_state_delta_apply(touched_families as u64, touched_records as u64);
        self.counters.record_append(
            verified.envelope().commit.parents.len(),
            digest_writes,
            branch_head_writes,
        );
        self.counters.record_commit_support_summary_build();
        self.counters.record_commit_support_publication();
        self.state
            .commit_envelopes
            .get(&commit_id.0)
            .cloned()
            .map(super::super::records::StoredCommitEnvelope::into_persisted)
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::AuthoritativeAppendAtomicityViolation,
                    format!("commit {} missing after successful append", commit_id.0),
                )
            })
    }

    pub fn fetch_commit(
        &self,
        commit_id: CommitId,
    ) -> Result<FetchedAuthoritativeCommit, StoreError> {
        self.admit_runtime_read_compatibility(
            CompatibilityFamilyKind::CommitEnvelope,
            "fetch_commit",
        )?;
        let stored = self.state.commit_record(commit_id).ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::CommitNotFound,
                format!("commit {} not found", commit_id.0),
            )
        })?;
        let verification = self.state.verify_commit_record(stored);
        self.counters
            .record_fetch_verification(verification.is_ok());
        verification?;
        Ok(stored.clone().into_fetched())
    }

    pub fn fetch_branch_head(
        &self,
        branch_id: &BranchId,
    ) -> Result<AuthoritativeBranchHeadRecord, StoreError> {
        self.admit_runtime_read_compatibility(
            CompatibilityFamilyKind::BranchVersionDagRecord,
            "fetch_branch_head",
        )?;
        let record = self
            .state
            .branch_head_records
            .get(&super::super::integrity::branch_key(branch_id))
            .ok_or_else(|| StoreError::unknown_branch(branch_id))?;
        let head = match record.head_commit_id {
            Some(head_commit_id) => {
                Some(self.fetch_commit(head_commit_id)?.envelope().commit.clone())
            }
            None => None,
        };
        Ok(AuthoritativeBranchHeadRecord::new(
            record.branch_id.clone(),
            head,
            record.head_update_sequence,
        ))
    }
}
