use crate::{
    backend::records::StoreState,
    bulk::{compute_checkpoint_digest, BULK_FAMILY_VERSION},
    failure::{StoreError, StoreErrorKind},
};

use super::identity::{
    bulk_checkpoint_artifact_id, bulk_plan_artifact_id, bulk_program_artifact_id,
    bulk_witness_artifact_id, bulk_witness_index_artifact_id, frozen_bulk_manifest_artifact_id,
    frozen_transform_basis_artifact_id, frozen_transform_partition_artifact_id,
};

impl StoreState {
    pub fn verify_bulk_record_family(&self) -> Result<(), StoreError> {
        for (stored_key, record) in &self.bulk_program_identity_records {
            let expected = bulk_program_artifact_id(&record.program_id);
            if stored_key != &expected || record.artifact_id != expected {
                return Err(StoreError::backend_integrity(format!(
                    "bulk program identity key `{stored_key}` did not match expected artifact id `{expected}`"
                )));
            }
            if record.family_version != BULK_FAMILY_VERSION {
                return Err(StoreError::new(
                    StoreErrorKind::BulkProgramVersionUnsupported,
                    format!(
                        "bulk program `{}` used unsupported family version {}",
                        record.program_id, record.family_version
                    ),
                ));
            }
        }

        for (stored_key, record) in &self.frozen_bulk_manifest_records {
            let expected = frozen_bulk_manifest_artifact_id(
                &record.program_id,
                record.manifest.manifest_digest(),
            );
            if stored_key != &expected || record.artifact_id != expected {
                return Err(StoreError::backend_integrity(format!(
                    "bulk manifest key `{stored_key}` did not match expected artifact id `{expected}`"
                )));
            }
            if record.family_version != BULK_FAMILY_VERSION
                || record.manifest.family_version() != BULK_FAMILY_VERSION
            {
                return Err(StoreError::new(
                    StoreErrorKind::BulkProgramVersionUnsupported,
                    format!(
                        "bulk manifest `{}` used unsupported family version",
                        record.program_id
                    ),
                ));
            }
            if record.program_id != record.manifest.program_id() {
                return Err(StoreError::backend_integrity(format!(
                    "bulk manifest `{}` did not preserve program linkage",
                    record.artifact_id
                )));
            }
            if !record.manifest.has_valid_digest()? {
                return Err(StoreError::backend_integrity(format!(
                    "bulk manifest `{}` digest no longer matched its payload",
                    record.artifact_id
                )));
            }
            let program_id = self
                .bulk_program_identity_records
                .get(&bulk_program_artifact_id(&record.program_id))
                .ok_or_else(|| {
                    StoreError::backend_integrity(format!(
                        "bulk manifest `{}` referenced missing program identity",
                        record.artifact_id
                    ))
                })?;
            if program_id.source_identity != record.manifest.source_identity() {
                return Err(StoreError::backend_integrity(format!(
                    "bulk manifest `{}` source identity drifted from program identity",
                    record.artifact_id
                )));
            }
        }

        for (stored_key, record) in &self.frozen_transform_basis_records {
            let expected =
                frozen_transform_basis_artifact_id(&record.program_id, record.basis.basis_digest());
            if stored_key != &expected || record.artifact_id != expected {
                return Err(StoreError::backend_integrity(format!(
                    "bulk transform basis key `{stored_key}` did not match expected artifact id `{expected}`"
                )));
            }
            if record.family_version != BULK_FAMILY_VERSION
                || record.basis.family_version() != BULK_FAMILY_VERSION
            {
                return Err(StoreError::new(
                    StoreErrorKind::BulkProgramVersionUnsupported,
                    format!(
                        "bulk transform basis `{}` used unsupported family version",
                        record.program_id
                    ),
                ));
            }
            if record.program_id != record.basis.program_id() {
                return Err(StoreError::backend_integrity(format!(
                    "bulk transform basis `{}` did not preserve program linkage",
                    record.artifact_id
                )));
            }
            if !record.basis.has_valid_digest()? {
                return Err(StoreError::backend_integrity(format!(
                    "bulk transform basis `{}` digest no longer matched its payload",
                    record.artifact_id
                )));
            }
        }

        for (stored_key, record) in &self.frozen_transform_partition_records {
            let expected = frozen_transform_partition_artifact_id(
                &record.program_id,
                record.partition.partition_digest(),
            );
            if stored_key != &expected || record.artifact_id != expected {
                return Err(StoreError::backend_integrity(format!(
                    "bulk transform partition key `{stored_key}` did not match expected artifact id `{expected}`"
                )));
            }
            if record.family_version != BULK_FAMILY_VERSION
                || record.partition.family_version() != BULK_FAMILY_VERSION
            {
                return Err(StoreError::new(
                    StoreErrorKind::BulkProgramVersionUnsupported,
                    format!(
                        "bulk transform partition `{}` used unsupported family version",
                        record.program_id
                    ),
                ));
            }
            if !record.partition.has_valid_digest()? {
                return Err(StoreError::backend_integrity(format!(
                    "bulk transform partition `{}` digest no longer matched its payload",
                    record.artifact_id
                )));
            }
            let basis = self
                .frozen_transform_basis_records
                .values()
                .find(|basis| {
                    basis.program_id == record.program_id
                        && basis.basis.transform_identity() == record.partition.transform_identity()
                        && basis.basis.target_branch_scope()
                            == record.partition.target_branch_scope()
                        && basis.basis.basis_commit_id() == record.partition.basis_commit_id()
                })
                .ok_or_else(|| {
                    StoreError::backend_integrity(format!(
                        "bulk transform partition `{}` referenced missing frozen basis",
                        record.artifact_id
                    ))
                })?;
            if basis.program_id != record.program_id {
                return Err(StoreError::backend_integrity(format!(
                    "bulk transform partition `{}` drifted from basis program linkage",
                    record.artifact_id
                )));
            }
        }

        for (stored_key, record) in &self.bulk_deterministic_plan_records {
            let expected = bulk_plan_artifact_id(&record.program_id, record.plan.plan_id());
            if stored_key != &expected || record.artifact_id != expected {
                return Err(StoreError::backend_integrity(format!(
                    "bulk plan key `{stored_key}` did not match expected artifact id `{expected}`"
                )));
            }
            if record.family_version != BULK_FAMILY_VERSION
                || record.plan.family_version() != BULK_FAMILY_VERSION
            {
                return Err(StoreError::new(
                    StoreErrorKind::BulkProgramVersionUnsupported,
                    format!(
                        "bulk plan `{}` used unsupported family version",
                        record.artifact_id
                    ),
                ));
            }
            if record.program_id != record.plan.program_id() {
                return Err(StoreError::backend_integrity(format!(
                    "bulk plan `{}` did not preserve program linkage",
                    record.artifact_id
                )));
            }
            if !record.plan.has_valid_plan_id()? {
                return Err(StoreError::backend_integrity(format!(
                    "bulk plan `{}` id no longer matched its payload",
                    record.artifact_id
                )));
            }
            self.bulk_program_identity_records
                .get(&bulk_program_artifact_id(&record.program_id))
                .ok_or_else(|| {
                    StoreError::backend_integrity(format!(
                        "bulk plan `{}` referenced missing program identity",
                        record.artifact_id
                    ))
                })?;
        }

        for (stored_key, record) in &self.bulk_chunk_witness_records {
            let expected = bulk_witness_artifact_id(
                &record.program_id,
                &record.plan_id,
                record.witness.chunk_ordinal().value(),
            );
            if stored_key != &expected || record.artifact_id != expected {
                return Err(StoreError::backend_integrity(format!(
                    "bulk witness key `{stored_key}` did not match expected artifact id `{expected}`"
                )));
            }
            if record.family_version != BULK_FAMILY_VERSION {
                return Err(StoreError::new(
                    StoreErrorKind::BulkProgramVersionUnsupported,
                    format!(
                        "bulk witness `{}` used unsupported family version",
                        record.artifact_id
                    ),
                ));
            }
            let plan = self
                .bulk_deterministic_plan_records
                .get(&bulk_plan_artifact_id(&record.program_id, &record.plan_id))
                .ok_or_else(|| {
                    StoreError::backend_integrity(format!(
                        "bulk witness `{}` referenced missing plan",
                        record.artifact_id
                    ))
                })?;
            if plan.plan.plan_id() != record.witness.plan_id()
                || plan.plan.program_id() != record.witness.program_id()
            {
                return Err(StoreError::backend_integrity(format!(
                    "bulk witness `{}` drifted from its plan linkage",
                    record.artifact_id
                )));
            }
            if plan.plan.target_branch_scope() != record.witness.target_branch_scope() {
                return Err(StoreError::backend_integrity(format!(
                    "bulk witness `{}` drifted from its plan branch scope",
                    record.artifact_id
                )));
            }
            if !self.has_commit(record.witness.canonical_commit_id()) {
                return Err(StoreError::backend_integrity(format!(
                    "bulk witness `{}` referenced missing canonical commit {}",
                    record.artifact_id,
                    record.witness.canonical_commit_id().0
                )));
            }
        }

        for (stored_key, record) in &self.bulk_progress_checkpoint_records {
            let expected = bulk_checkpoint_artifact_id(
                &record.program_id,
                &record.plan_id,
                record.checkpoint.checkpoint_sequence(),
            );
            if stored_key != &expected || record.artifact_id != expected {
                return Err(StoreError::backend_integrity(format!(
                    "bulk checkpoint key `{stored_key}` did not match expected artifact id `{expected}`"
                )));
            }
            if record.family_version != BULK_FAMILY_VERSION {
                return Err(StoreError::new(
                    StoreErrorKind::BulkProgramVersionUnsupported,
                    format!(
                        "bulk checkpoint `{}` used unsupported family version",
                        record.artifact_id
                    ),
                ));
            }
            let recomputed = compute_checkpoint_digest(
                record.checkpoint.program_id(),
                record.checkpoint.plan_id(),
                record.checkpoint.checkpoint_sequence(),
                record.checkpoint.completed_chunk_ordinal(),
                record.checkpoint.next_chunk_ordinal(),
                record.checkpoint.last_committed_chunk_witness_artifact_id(),
            );
            if recomputed != record.checkpoint.checkpoint_digest() {
                return Err(StoreError::new(
                    StoreErrorKind::BulkCheckpointDigestMismatch,
                    format!("bulk checkpoint `{}` digest mismatch", record.artifact_id),
                ));
            }
            let witness = self
                .bulk_chunk_witness_records
                .get(record.checkpoint.last_committed_chunk_witness_artifact_id());
            let Some(witness) = witness else {
                return Err(StoreError::backend_integrity(format!(
                    "bulk checkpoint `{}` referenced missing chunk witness `{}`",
                    record.artifact_id,
                    record.checkpoint.last_committed_chunk_witness_artifact_id()
                )));
            };
            if witness.program_id != record.program_id
                || witness.plan_id != record.plan_id
                || witness.witness.program_id() != record.checkpoint.program_id()
                || witness.witness.plan_id() != record.checkpoint.plan_id()
            {
                return Err(StoreError::backend_integrity(format!(
                    "bulk checkpoint `{}` drifted from its witness program or plan linkage",
                    record.artifact_id
                )));
            }
            if witness.witness.chunk_ordinal() != record.checkpoint.completed_chunk_ordinal() {
                return Err(StoreError::backend_integrity(format!(
                    "bulk checkpoint `{}` completed chunk ordinal did not match its witness ordinal",
                    record.artifact_id
                )));
            }
            if record.checkpoint.next_chunk_ordinal().value()
                != record.checkpoint.completed_chunk_ordinal().value() + 1
            {
                return Err(StoreError::backend_integrity(format!(
                    "bulk checkpoint `{}` next chunk ordinal was not the successor of its completed chunk",
                    record.artifact_id
                )));
            }
        }

        let checkpoint_families = self.bulk_progress_checkpoint_records.values().fold(
            std::collections::BTreeMap::<(&str, &str), Vec<_>>::new(),
            |mut acc, record| {
                acc.entry((&record.program_id, &record.plan_id))
                    .or_default()
                    .push(record);
                acc
            },
        );
        for ((program_id, plan_id), mut checkpoints) in checkpoint_families {
            checkpoints.sort_by_key(|record| record.checkpoint.checkpoint_sequence());
            let mut expected_sequence = 1;
            let mut minimum_completed_chunk_ordinal = 0;
            for checkpoint in checkpoints {
                if checkpoint.checkpoint.checkpoint_sequence() != expected_sequence {
                    return Err(StoreError::backend_integrity(format!(
                        "bulk checkpoint family `{program_id}:{plan_id}` contained non-contiguous sequence {}; expected {}",
                        checkpoint.checkpoint.checkpoint_sequence(),
                        expected_sequence
                    )));
                }
                if checkpoint.checkpoint.completed_chunk_ordinal().value()
                    < minimum_completed_chunk_ordinal
                {
                    return Err(StoreError::backend_integrity(format!(
                        "bulk checkpoint family `{program_id}:{plan_id}` regressed completed chunk ordinal {} below required boundary {}",
                        checkpoint.checkpoint.completed_chunk_ordinal().value(),
                        minimum_completed_chunk_ordinal
                    )));
                }
                minimum_completed_chunk_ordinal =
                    checkpoint.checkpoint.next_chunk_ordinal().value();
                expected_sequence += 1;
            }
        }

        for (stored_key, record) in &self.program_chunk_witness_index_records {
            let expected = bulk_witness_index_artifact_id(&record.program_id, &record.plan_id);
            if stored_key != &expected || record.artifact_id != expected {
                return Err(StoreError::backend_integrity(format!(
                    "bulk witness index key `{stored_key}` did not match expected artifact id `{expected}`"
                )));
            }
            if record.family_version != BULK_FAMILY_VERSION {
                return Err(StoreError::new(
                    StoreErrorKind::BulkProgramVersionUnsupported,
                    format!(
                        "bulk witness index `{}` used unsupported family version",
                        record.artifact_id
                    ),
                ));
            }
            let witnesses: Vec<_> = self
                .bulk_chunk_witness_records
                .values()
                .filter(|witness| {
                    witness.program_id == record.program_id && witness.plan_id == record.plan_id
                })
                .collect();
            if witnesses.is_empty() {
                return Err(StoreError::backend_integrity(format!(
                    "bulk witness index `{}` existed without any witness records",
                    record.artifact_id
                )));
            }
            let highest = witnesses
                .iter()
                .max_by_key(|witness| witness.witness.chunk_ordinal().value())
                .expect("non-empty witnesses");
            if highest.witness.chunk_ordinal() != record.index.highest_committed_chunk_ordinal()
                || highest.witness.canonical_commit_id()
                    != record.index.highest_committed_commit_id()
                || witnesses.len() as u64 != record.index.witness_count()
            {
                return Err(StoreError::backend_integrity(format!(
                    "bulk witness index `{}` did not match persisted witness set",
                    record.artifact_id
                )));
            }
            if let Some(checkpoint_sequence) = record.index.latest_checkpoint_sequence() {
                let checkpoint_artifact_id = bulk_checkpoint_artifact_id(
                    &record.program_id,
                    &record.plan_id,
                    checkpoint_sequence,
                );
                let checkpoint = self
                    .bulk_progress_checkpoint_records
                    .get(&checkpoint_artifact_id)
                    .ok_or_else(|| {
                        StoreError::backend_integrity(format!(
                            "bulk witness index `{}` referenced missing checkpoint `{checkpoint_artifact_id}`",
                            record.artifact_id
                        ))
                    })?;
                if checkpoint.checkpoint.completed_chunk_ordinal().value()
                    >= record.index.witness_count()
                {
                    return Err(StoreError::backend_integrity(format!(
                        "bulk witness index `{}` referenced checkpoint `{}` beyond persisted witness count",
                        record.artifact_id, checkpoint.artifact_id
                    )));
                }
            }
        }

        Ok(())
    }
}
