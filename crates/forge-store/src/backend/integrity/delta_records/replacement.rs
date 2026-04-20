use crate::{
    backend::records::{BranchDeltaLayerRecord, BranchDeltaReplacementProofEntry, StoreState},
    delta::{BRANCH_DELTA_FAMILY_VERSION, BranchDeltaLayerId, stable_branch_delta_layer_authority_digest},
    failure::{StoreError, StoreErrorKind},
};
use forge_relational::facade::history::BranchId;

impl StoreState {
    pub(super) fn verify_branch_delta_replacement_proof_entry(
        &self,
        replacement_layer_id: BranchDeltaLayerId,
        replacement_branch_id: &BranchId,
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

    pub(super) fn verify_replacement_lineage(
        &self,
        record: &BranchDeltaLayerRecord,
    ) -> Result<(), StoreError> {
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
            if self.branch_delta_layer_records.contains_key(&replaced_layer_id.0) {
                return Err(StoreError::new(
                    StoreErrorKind::BranchDeltaShadowAuthorityViolation,
                    format!(
                        "branch delta layer {} still shadows live replaced layer {}",
                        record.branch_delta_layer_id.0, replaced_layer_id.0
                    ),
                ));
            }
        }
        if record.replacement_lineage_proof.is_empty() {
            return Ok(());
        }
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
        Ok(())
    }
}
