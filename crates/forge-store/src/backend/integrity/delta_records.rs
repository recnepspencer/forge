use crate::{
    delta::{
        stable_branch_delta_layer_authority_digest, stable_shared_base_authority_digest,
        BRANCH_DELTA_FAMILY_VERSION,
    },
    failure::{StoreError, StoreErrorKind},
};
use std::collections::BTreeSet;

use crate::backend::records::{
    BranchDeltaLayerArtifacts, BranchDeltaLayerRecord, BranchDeltaReplacementProofEntry,
    BranchSharedBaseRecord, CommitParentRecord, StoreState,
};

impl StoreState {
    fn verify_branch_delta_layer_artifacts(
        &self,
        record: &BranchDeltaLayerRecord,
    ) -> Result<(), StoreError> {
        let artifacts = &record.artifacts;
        let artifact_commit_ids = artifacts
            .commit_envelopes
            .iter()
            .map(|entry| entry.envelope.commit.commit_id)
            .collect::<Vec<_>>();
        if artifact_commit_ids != record.commit_ids {
            return Err(StoreError::new(
                StoreErrorKind::BranchDeltaPublicationGap,
                format!(
                    "branch delta layer {} artifact commit envelopes drifted from the declared segment",
                    record.branch_delta_layer_id.0
                ),
            ));
        }
        for commit_record in &artifacts.commit_envelopes {
            if commit_record.envelope.branch_context != record.branch_id {
                return Err(StoreError::new(
                    StoreErrorKind::BranchDeltaPublicationGap,
                    format!(
                        "branch delta layer {} artifact commit {} drifted onto branch `{}`",
                        record.branch_delta_layer_id.0,
                        commit_record.envelope.commit.commit_id.0,
                        commit_record.envelope.branch_context.0
                    ),
                ));
            }
            let authoritative = self
                .commit_record(commit_record.envelope.commit.commit_id)
                .ok_or_else(|| {
                    StoreError::new(
                        StoreErrorKind::BranchDeltaPublicationGap,
                        format!(
                            "branch delta layer {} artifact commit {} is missing from authority",
                            record.branch_delta_layer_id.0,
                            commit_record.envelope.commit.commit_id.0
                        ),
                    )
                })?;
            if authoritative != commit_record {
                return Err(StoreError::new(
                    StoreErrorKind::BranchDeltaDigestMismatch,
                    format!(
                        "branch delta layer {} artifact commit {} drifted from authoritative commit storage",
                        record.branch_delta_layer_id.0,
                        commit_record.envelope.commit.commit_id.0
                    ),
                ));
            }
        }

        let expected_parent_records = artifacts
            .commit_envelopes
            .iter()
            .flat_map(|commit_record| {
                commit_record
                    .envelope
                    .commit
                    .parents
                    .iter()
                    .copied()
                    .enumerate()
                    .map(|(parent_position, parent_commit_id)| CommitParentRecord {
                        commit_id: commit_record.envelope.commit.commit_id,
                        parent_position,
                        parent_commit_id,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let mut expected_parent_artifacts = BranchDeltaLayerArtifacts {
            commit_envelopes: Vec::new(),
            commit_parent_records: expected_parent_records,
            commit_support_summaries: Vec::new(),
            schema_support_records: Vec::new(),
            lineage_support_records: Vec::new(),
        };
        expected_parent_artifacts.canonicalize_order();
        if artifacts.commit_parent_records != expected_parent_artifacts.commit_parent_records {
            return Err(StoreError::new(
                StoreErrorKind::BranchDeltaPublicationGap,
                format!(
                    "branch delta layer {} artifact parent records drifted from the admitted commit ancestry",
                    record.branch_delta_layer_id.0
                ),
            ));
        }
        for parent_record in &artifacts.commit_parent_records {
            let key =
                super::parent_artifact_id(parent_record.commit_id, parent_record.parent_position);
            let authoritative = self.commit_parent_records.get(&key).ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BranchDeltaPublicationGap,
                    format!(
                        "branch delta layer {} artifact parent {}:{} is missing from authority",
                        record.branch_delta_layer_id.0,
                        parent_record.commit_id.0,
                        parent_record.parent_position
                    ),
                )
            })?;
            if authoritative != parent_record {
                return Err(StoreError::new(
                    StoreErrorKind::BranchDeltaDigestMismatch,
                    format!(
                        "branch delta layer {} artifact parent {}:{} drifted from authoritative parent storage",
                        record.branch_delta_layer_id.0,
                        parent_record.commit_id.0,
                        parent_record.parent_position
                    ),
                ));
            }
        }

        let mut seen_summary_commits = BTreeSet::new();
        for summary in &artifacts.commit_support_summaries {
            if summary.branch_id != record.branch_id {
                return Err(StoreError::new(
                    StoreErrorKind::BranchDeltaPublicationGap,
                    format!(
                        "branch delta layer {} support summary for commit {} drifted onto branch `{}`",
                        record.branch_delta_layer_id.0, summary.commit_id.0, summary.branch_id.0
                    ),
                ));
            }
            if !record.commit_ids.contains(&summary.commit_id)
                || !seen_summary_commits.insert(summary.commit_id)
            {
                return Err(StoreError::new(
                    StoreErrorKind::BranchDeltaPublicationGap,
                    format!(
                        "branch delta layer {} support summaries are not a one-per-commit subset of the declared segment",
                        record.branch_delta_layer_id.0
                    ),
                ));
            }
            let authoritative = self.commit_support_summaries.get(&summary.commit_id.0);
            if let Some(authoritative) = authoritative {
                if authoritative != summary {
                    return Err(StoreError::new(
                        StoreErrorKind::BranchDeltaDigestMismatch,
                        format!(
                            "branch delta layer {} support summary for commit {} drifted from authoritative support storage",
                            record.branch_delta_layer_id.0, summary.commit_id.0
                        ),
                    ));
                }
            }
        }

        let mut seen_schema_commits = BTreeSet::new();
        for schema_record in &artifacts.schema_support_records {
            if schema_record.branch_id != record.branch_id {
                return Err(StoreError::new(
                    StoreErrorKind::BranchDeltaPublicationGap,
                    format!(
                        "branch delta layer {} schema support for commit {} drifted onto branch `{}`",
                        record.branch_delta_layer_id.0, schema_record.commit_id.0, schema_record.branch_id.0
                    ),
                ));
            }
            if !record.commit_ids.contains(&schema_record.commit_id)
                || !seen_schema_commits.insert(schema_record.commit_id)
            {
                return Err(StoreError::new(
                    StoreErrorKind::BranchDeltaPublicationGap,
                    format!(
                        "branch delta layer {} schema support rows are not a one-per-commit subset of the declared segment",
                        record.branch_delta_layer_id.0
                    ),
                ));
            }
            let authoritative = self.schema_support_records.get(&schema_record.artifact_id);
            if let Some(authoritative) = authoritative {
                if authoritative != schema_record {
                    return Err(StoreError::new(
                        StoreErrorKind::BranchDeltaDigestMismatch,
                        format!(
                            "branch delta layer {} schema support for commit {} drifted from authoritative support storage",
                            record.branch_delta_layer_id.0, schema_record.commit_id.0
                        ),
                    ));
                }
            }
        }

        let mut seen_lineage_commits = BTreeSet::new();
        for lineage_record in &artifacts.lineage_support_records {
            if lineage_record.branch_id != record.branch_id {
                return Err(StoreError::new(
                    StoreErrorKind::BranchDeltaPublicationGap,
                    format!(
                        "branch delta layer {} lineage support for commit {} drifted onto branch `{}`",
                        record.branch_delta_layer_id.0, lineage_record.commit_id.0, lineage_record.branch_id.0
                    ),
                ));
            }
            if !record.commit_ids.contains(&lineage_record.commit_id)
                || !seen_lineage_commits.insert(lineage_record.commit_id)
            {
                return Err(StoreError::new(
                    StoreErrorKind::BranchDeltaPublicationGap,
                    format!(
                        "branch delta layer {} lineage support rows are not a one-per-commit subset of the declared segment",
                        record.branch_delta_layer_id.0
                    ),
                ));
            }
            let authoritative = self
                .lineage_support_records
                .get(&lineage_record.artifact_id);
            if let Some(authoritative) = authoritative {
                if authoritative != lineage_record {
                    return Err(StoreError::new(
                        StoreErrorKind::BranchDeltaDigestMismatch,
                        format!(
                            "branch delta layer {} lineage support for commit {} drifted from authoritative support storage",
                            record.branch_delta_layer_id.0, lineage_record.commit_id.0
                        ),
                    ));
                }
            }
        }
        Ok(())
    }

    fn verify_branch_delta_replacement_proof_entry(
        &self,
        replacement_layer_id: crate::delta::BranchDeltaLayerId,
        replacement_branch_id: &forge_relational::facade::history::BranchId,
        entry: &BranchDeltaReplacementProofEntry,
    ) -> Result<(), StoreError> {
        if entry.delta_family_version != BRANCH_DELTA_FAMILY_VERSION {
            return Err(StoreError::new(
                StoreErrorKind::BranchDeltaFamilyVersionUnsupported,
                format!(
                    "branch delta replacement proof for layer {} used unsupported family version {}",
                    replacement_layer_id.0, entry.delta_family_version
                ),
            ));
        }
        if entry.branch_id != *replacement_branch_id {
            return Err(StoreError::new(
                StoreErrorKind::BranchDeltaReplacementGap,
                format!(
                    "branch delta replacement proof for layer {} drifted onto branch `{}`",
                    replacement_layer_id.0, entry.branch_id.0
                ),
            ));
        }
        if entry.commit_ids.is_empty() {
            return Err(StoreError::new(
                StoreErrorKind::BranchDeltaReplacementGap,
                format!(
                    "branch delta replacement proof for layer {} published an empty replaced segment",
                    replacement_layer_id.0
                ),
            ));
        }
        if entry.commit_ids.last().copied() != Some(entry.target_frontier_commit_id) {
            return Err(StoreError::new(
                StoreErrorKind::BranchDeltaReplacementGap,
                format!(
                    "branch delta replacement proof for layer {} does not end at its declared replaced target frontier",
                    replacement_layer_id.0
                ),
            ));
        }
        let expected_digest = stable_branch_delta_layer_authority_digest(
            &entry.branch_id,
            entry.base_frontier_commit_id,
            entry.target_frontier_commit_id,
            &entry.commit_ids,
            self.canonicalization_version,
        );
        if entry.authority_basis_digest != expected_digest {
            return Err(StoreError::new(
                StoreErrorKind::BranchDeltaDigestMismatch,
                format!(
                    "branch delta replacement proof for layer {} drifted from authoritative replaced-layer basis",
                    replacement_layer_id.0
                ),
            ));
        }
        Ok(())
    }

    fn verify_branch_shared_base_record(
        &self,
        record: &BranchSharedBaseRecord,
    ) -> Result<(), StoreError> {
        if record.delta_family_version != BRANCH_DELTA_FAMILY_VERSION {
            return Err(StoreError::new(
                StoreErrorKind::BranchDeltaFamilyVersionUnsupported,
                format!(
                    "branch `{}` used unsupported delta family version {}",
                    record.branch_id.0, record.delta_family_version
                ),
            ));
        }
        let branch = self
            .branch_records
            .get(&record.branch_id.0)
            .ok_or_else(|| StoreError::unknown_branch(&record.branch_id))?;
        if branch.created_from_branch.as_ref() != Some(&record.source_branch_id)
            || branch.created_from_commit_id != record.source_frontier_commit_id
        {
            return Err(StoreError::new(
                StoreErrorKind::BranchDeltaPublicationGap,
                format!(
                    "branch `{}` shared-base record drifted from authoritative branch creation basis",
                    record.branch_id.0
                ),
            ));
        }
        if !self.branch_records.contains_key(&record.source_branch_id.0) {
            return Err(StoreError::unknown_branch(&record.source_branch_id));
        }
        if let Some(frontier_commit_id) = record.source_frontier_commit_id {
            let frontier_record = self.commit_record(frontier_commit_id).ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BranchDeltaPublicationGap,
                    format!(
                        "shared-base branch `{}` references missing source frontier commit {}",
                        record.branch_id.0, frontier_commit_id.0
                    ),
                )
            })?;
            if frontier_record.envelope.branch_context != record.source_branch_id {
                return Err(StoreError::new(
                    StoreErrorKind::BranchDeltaPublicationGap,
                    format!(
                        "shared-base branch `{}` source frontier {} drifted off source branch `{}`",
                        record.branch_id.0, frontier_commit_id.0, record.source_branch_id.0
                    ),
                ));
            }
        }
        let expected_digest = stable_shared_base_authority_digest(
            &record.source_branch_id,
            record.source_frontier_commit_id,
            self.canonicalization_version,
        );
        if record.authority_basis_digest != expected_digest {
            return Err(StoreError::new(
                StoreErrorKind::BranchDeltaDigestMismatch,
                format!(
                    "branch `{}` shared-base digest drifted from authoritative basis",
                    record.branch_id.0
                ),
            ));
        }
        Ok(())
    }

    fn verify_branch_delta_layer_record(
        &self,
        record: &BranchDeltaLayerRecord,
    ) -> Result<(), StoreError> {
        if record.delta_family_version != BRANCH_DELTA_FAMILY_VERSION {
            return Err(StoreError::new(
                StoreErrorKind::BranchDeltaFamilyVersionUnsupported,
                format!(
                    "branch delta layer {} used unsupported family version {}",
                    record.branch_delta_layer_id.0, record.delta_family_version
                ),
            ));
        }
        if !self.branch_records.contains_key(&record.branch_id.0) {
            return Err(StoreError::unknown_branch(&record.branch_id));
        }
        if record.commit_ids.is_empty() {
            return Err(StoreError::new(
                StoreErrorKind::BranchDeltaPublicationGap,
                format!(
                    "branch delta layer {} published an empty commit segment",
                    record.branch_delta_layer_id.0
                ),
            ));
        }
        let target_record = self
            .commit_record(record.target_frontier_commit_id)
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BranchDeltaPublicationGap,
                    format!(
                        "branch delta layer {} target commit {} missing",
                        record.branch_delta_layer_id.0, record.target_frontier_commit_id.0
                    ),
                )
            })?;
        if target_record.envelope.branch_context != record.branch_id {
            return Err(StoreError::new(
                StoreErrorKind::BranchDeltaPublicationGap,
                format!(
                    "branch delta layer {} target commit {} drifted onto branch `{}`",
                    record.branch_delta_layer_id.0,
                    record.target_frontier_commit_id.0,
                    target_record.envelope.branch_context.0
                ),
            ));
        }
        if record.commit_ids.last().copied() != Some(record.target_frontier_commit_id) {
            return Err(StoreError::new(
                StoreErrorKind::BranchDeltaPublicationGap,
                format!(
                    "branch delta layer {} does not end at its declared target frontier",
                    record.branch_delta_layer_id.0
                ),
            ));
        }
        for commit_id in &record.commit_ids {
            let commit_record = self.commit_record(*commit_id).ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BranchDeltaPublicationGap,
                    format!(
                        "branch delta layer {} references missing commit {}",
                        record.branch_delta_layer_id.0, commit_id.0
                    ),
                )
            })?;
            if commit_record.envelope.branch_context != record.branch_id {
                return Err(StoreError::new(
                    StoreErrorKind::BranchDeltaPublicationGap,
                    format!(
                        "branch delta layer {} commit {} drifted onto branch `{}`",
                        record.branch_delta_layer_id.0,
                        commit_id.0,
                        commit_record.envelope.branch_context.0
                    ),
                ));
            }
        }
        let mut expected_parent = record.base_frontier_commit_id;
        for commit_id in &record.commit_ids {
            let commit_record = self.commit_record(*commit_id).ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BranchDeltaPublicationGap,
                    format!(
                        "branch delta layer {} references missing commit {}",
                        record.branch_delta_layer_id.0, commit_id.0
                    ),
                )
            })?;
            match commit_record.envelope.commit.parents.as_slice() {
                [] if expected_parent.is_none() => {}
                [parent] if Some(*parent) == expected_parent => {}
                [parent] => {
                    return Err(StoreError::new(
                        StoreErrorKind::BranchDeltaPublicationGap,
                        format!(
                            "branch delta layer {} commit {} expected parent {:?} but found {}",
                            record.branch_delta_layer_id.0,
                            commit_id.0,
                            expected_parent.map(|id| id.0),
                            parent.0
                        ),
                    ));
                }
                _ => {
                    return Err(StoreError::new(
                        StoreErrorKind::BranchDeltaPublicationGap,
                        format!(
                            "branch delta layer {} commit {} requires merge-aware widening, which persisted delta segments do not admit",
                            record.branch_delta_layer_id.0, commit_id.0
                        ),
                    ));
                }
            }
            expected_parent = Some(*commit_id);
        }
        self.verify_branch_delta_layer_artifacts(record)?;
        if record.replacement_of_layer_ids.is_empty() != record.replacement_lineage_proof.is_empty()
        {
            return Err(StoreError::new(
                StoreErrorKind::BranchDeltaReplacementGap,
                format!(
                    "branch delta layer {} replacement ids and lineage proof drifted apart",
                    record.branch_delta_layer_id.0
                ),
            ));
        }
        if record.replacement_of_layer_ids.len() != record.replacement_lineage_proof.len() {
            return Err(StoreError::new(
                StoreErrorKind::BranchDeltaReplacementGap,
                format!(
                    "branch delta layer {} replacement ids and lineage proof length drifted apart",
                    record.branch_delta_layer_id.0
                ),
            ));
        }

        let mut seen_replacements = std::collections::BTreeSet::new();
        for (index, replaced_layer_id) in record.replacement_of_layer_ids.iter().enumerate() {
            if *replaced_layer_id == record.branch_delta_layer_id {
                return Err(StoreError::new(
                    StoreErrorKind::BranchDeltaShadowAuthorityViolation,
                    format!(
                        "branch delta layer {} cannot replace itself",
                        record.branch_delta_layer_id.0
                    ),
                ));
            }
            if replaced_layer_id.0 > record.branch_delta_layer_id.0
                || !seen_replacements.insert(replaced_layer_id.0)
            {
                return Err(StoreError::new(
                    StoreErrorKind::BranchDeltaReplacementGap,
                    format!(
                        "branch delta layer {} published an illegal replacement reference to layer {}",
                        record.branch_delta_layer_id.0, replaced_layer_id.0
                    ),
                ));
            }
            let proof_entry = record.replacement_lineage_proof.get(index).ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::BranchDeltaReplacementGap,
                    format!(
                        "branch delta layer {} replacement lineage proof missing entry {}",
                        record.branch_delta_layer_id.0, index
                    ),
                )
            })?;
            if proof_entry.layer_id != *replaced_layer_id {
                return Err(StoreError::new(
                    StoreErrorKind::BranchDeltaReplacementGap,
                    format!(
                        "branch delta layer {} replacement lineage proof drifted from replacement id order at position {}",
                        record.branch_delta_layer_id.0, index
                    ),
                ));
            }
            self.verify_branch_delta_replacement_proof_entry(
                record.branch_delta_layer_id,
                &record.branch_id,
                proof_entry,
            )?;
            if self
                .branch_delta_layer_records
                .contains_key(&replaced_layer_id.0)
            {
                return Err(StoreError::new(
                    StoreErrorKind::BranchDeltaShadowAuthorityViolation,
                    format!(
                        "branch delta layer {} still shadows live replaced layer {}",
                        record.branch_delta_layer_id.0, replaced_layer_id.0
                    ),
                ));
            }
        }
        if !record.replacement_lineage_proof.is_empty() {
            let first_entry = &record.replacement_lineage_proof[0];
            if first_entry.base_frontier_commit_id != record.base_frontier_commit_id {
                return Err(StoreError::new(
                    StoreErrorKind::BranchDeltaReplacementGap,
                    format!(
                        "branch delta layer {} replacement proof drifted from replacement base frontier",
                        record.branch_delta_layer_id.0
                    ),
                ));
            }
            let last_entry = record
                .replacement_lineage_proof
                .last()
                .expect("non-empty replacement proof has a last entry");
            if last_entry.target_frontier_commit_id != record.target_frontier_commit_id {
                return Err(StoreError::new(
                    StoreErrorKind::BranchDeltaReplacementGap,
                    format!(
                        "branch delta layer {} replacement proof drifted from replacement target frontier",
                        record.branch_delta_layer_id.0
                    ),
                ));
            }

            let mut expected_proof_base = record.base_frontier_commit_id;
            let mut concatenated_commit_ids = Vec::new();
            for entry in &record.replacement_lineage_proof {
                if entry.base_frontier_commit_id != expected_proof_base {
                    return Err(StoreError::new(
                        StoreErrorKind::BranchDeltaReplacementGap,
                        format!(
                            "branch delta layer {} replacement proof lost contiguous basis chaining",
                            record.branch_delta_layer_id.0
                        ),
                    ));
                }
                concatenated_commit_ids.extend(entry.commit_ids.iter().copied());
                expected_proof_base = Some(entry.target_frontier_commit_id);
            }
            if concatenated_commit_ids != record.commit_ids {
                return Err(StoreError::new(
                    StoreErrorKind::BranchDeltaReplacementGap,
                    format!(
                        "branch delta layer {} replacement proof drifted from the replacement commit interval",
                        record.branch_delta_layer_id.0
                    ),
                ));
            }
        }
        let expected_digest = stable_branch_delta_layer_authority_digest(
            &record.branch_id,
            record.base_frontier_commit_id,
            record.target_frontier_commit_id,
            &record.commit_ids,
            self.canonicalization_version,
        );
        if record.authority_basis_digest != expected_digest {
            return Err(StoreError::new(
                StoreErrorKind::BranchDeltaDigestMismatch,
                format!(
                    "branch delta layer {} digest drifted from authoritative basis",
                    record.branch_delta_layer_id.0
                ),
            ));
        }
        Ok(())
    }

    pub fn verify_delta_record_family(&self) -> Result<(), StoreError> {
        for record in self.branch_shared_base_records.values() {
            self.verify_branch_shared_base_record(record)?;
        }
        for record in self.branch_delta_layer_records.values() {
            self.verify_branch_delta_layer_record(record)?;
        }
        Ok(())
    }
}
