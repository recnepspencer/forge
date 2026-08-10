use super::{
    ReplayArtifactProofInput, ReplayArtifactProofReport, ReplayMismatchClass,
    ReplayParityProofReport, MERGE_PROOF_SCHEMA_VERSION,
};

fn replay_mismatch_classes(
    expected: &ReplayArtifactProofInput,
    replayed: &ReplayArtifactProofInput,
) -> Vec<ReplayMismatchClass> {
    let mut mismatch_classes = Vec::new();
    let compare_optional =
        |left: &Option<String>,
         right: &Option<String>,
         missing_class: ReplayMismatchClass,
         mismatch_class: ReplayMismatchClass,
         output: &mut Vec<ReplayMismatchClass>| match (left, right) {
            (Some(left), Some(right)) => {
                if left != right {
                    output.push(mismatch_class);
                }
            }
            (None, Some(_)) | (Some(_), None) => output.push(missing_class),
            (None, None) => {}
        };
    if !expected
        .proof_schema_version
        .starts_with(MERGE_PROOF_SCHEMA_VERSION)
        || !replayed
            .proof_schema_version
            .starts_with(MERGE_PROOF_SCHEMA_VERSION)
    {
        mismatch_classes.push(ReplayMismatchClass::LegacyMergeArtifactUnsupported);
    }
    if expected.proof_schema_version != replayed.proof_schema_version {
        mismatch_classes.push(ReplayMismatchClass::ProofSchemaVersionMismatch);
    }
    compare_optional(
        &expected.registry_bundle_digest,
        &replayed.registry_bundle_digest,
        ReplayMismatchClass::MissingRegistryBundleDigest,
        ReplayMismatchClass::RegistryBundleDigestMismatch,
        &mut mismatch_classes,
    );
    compare_optional(
        &expected.lowered_strategy_bundle_digest,
        &replayed.lowered_strategy_bundle_digest,
        ReplayMismatchClass::MissingLoweredStrategyBundleDigest,
        ReplayMismatchClass::LoweredStrategyBundleDigestMismatch,
        &mut mismatch_classes,
    );
    compare_optional(
        &expected.merge_plan_digest,
        &replayed.merge_plan_digest,
        ReplayMismatchClass::MissingMergePlanDigest,
        ReplayMismatchClass::MergePlanDigestMismatch,
        &mut mismatch_classes,
    );
    compare_optional(
        &expected.merge_result_digest,
        &replayed.merge_result_digest,
        ReplayMismatchClass::MissingMergeResultDigest,
        ReplayMismatchClass::MergeResultDigestMismatch,
        &mut mismatch_classes,
    );
    compare_optional(
        &expected.lineage_digest,
        &replayed.lineage_digest,
        ReplayMismatchClass::MissingLineageDigest,
        ReplayMismatchClass::LineageDigestMismatch,
        &mut mismatch_classes,
    );
    match (&expected.strategy_witness, &replayed.strategy_witness) {
        (Some(left), Some(right)) => {
            if left != right {
                mismatch_classes.push(ReplayMismatchClass::StrategyWitnessMismatch);
            }
        }
        (None, Some(_)) | (Some(_), None) => {
            mismatch_classes.push(ReplayMismatchClass::MissingStrategyWitness);
        }
        (None, None) => {}
    }
    match (&expected.scoped_merge_proof, &replayed.scoped_merge_proof) {
        (Some(left), Some(right)) => {
            if left != right {
                mismatch_classes.push(ReplayMismatchClass::ScopedMergeProofMismatch);
            }
        }
        (None, Some(_)) | (Some(_), None) => {
            mismatch_classes.push(ReplayMismatchClass::MissingScopedMergeProof);
        }
        (None, None) => {}
    }
    match (
        &expected.compatibility_witness,
        &replayed.compatibility_witness,
    ) {
        (Some(left), Some(right)) => {
            if left != right {
                mismatch_classes.push(ReplayMismatchClass::CompatibilityWitnessMismatch);
            }
        }
        (None, Some(_)) | (Some(_), None) => {
            mismatch_classes.push(ReplayMismatchClass::MissingCompatibilityWitness);
        }
        (None, None) => {}
    }
    if expected.branch_state_digest != replayed.branch_state_digest {
        mismatch_classes.push(ReplayMismatchClass::BranchStateDigestMismatch);
    }
    mismatch_classes
}

pub fn replay_parity_proof_report(
    expected_branch_id: u64,
    expected_branch_name: impl Into<String>,
    expected_snapshot_id: Option<u64>,
    expected_state_digest: impl Into<String>,
    replayed_branch_id: u64,
    replayed_branch_name: impl Into<String>,
    replayed_snapshot_id: Option<u64>,
    replayed_state_digest: impl Into<String>,
) -> ReplayParityProofReport {
    let expected_state_digest = expected_state_digest.into();
    let replayed_state_digest = replayed_state_digest.into();
    let mismatch_classes = if expected_state_digest == replayed_state_digest {
        Vec::new()
    } else {
        vec![ReplayMismatchClass::BranchStateDigestMismatch]
    };
    ReplayParityProofReport {
        proof_schema_version: MERGE_PROOF_SCHEMA_VERSION.to_owned(),
        expected_branch_id,
        expected_branch_name: expected_branch_name.into(),
        expected_snapshot_id,
        expected_state_digest: expected_state_digest.clone(),
        replayed_branch_id,
        replayed_branch_name: replayed_branch_name.into(),
        replayed_snapshot_id,
        replayed_state_digest: replayed_state_digest.clone(),
        parity: mismatch_classes.is_empty(),
        mismatch_classes,
    }
}

pub fn replay_artifact_proof_report(
    expected: ReplayArtifactProofInput,
    replayed: ReplayArtifactProofInput,
) -> ReplayArtifactProofReport {
    let mismatch_classes = replay_mismatch_classes(&expected, &replayed);
    ReplayArtifactProofReport {
        proof_schema_version: MERGE_PROOF_SCHEMA_VERSION.to_owned(),
        expected,
        replayed,
        parity: mismatch_classes.is_empty(),
        mismatch_classes,
    }
}
