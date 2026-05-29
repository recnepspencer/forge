use crate::diagnostics::data::RelationalDiagnosticValue;
use crate::durability::authority::diagnostics::recovery::durable_identity_fields::contract_id_array;
use crate::durability::data::RecoveryCompatibilityMismatch;
use crate::history::data::CommitId;
use crate::schema::data::ContractId;

pub(super) fn compatibility_mismatch_fields(
    mismatch: &RecoveryCompatibilityMismatch,
) -> RelationalDiagnosticValue {
    match mismatch {
        RecoveryCompatibilityMismatch::SchemaRegistryShape {
            expected_primary_schema_version,
            found_primary_schema_version,
            expected_entity_kind_count,
            found_entity_kind_count,
            expected_relation_kind_count,
            found_relation_kind_count,
        } => RelationalDiagnosticValue::object([
            (
                "mismatch",
                RelationalDiagnosticValue::string("SchemaRegistryShape"),
            ),
            (
                "expected_primary_schema_version",
                RelationalDiagnosticValue::SchemaVersionId(*expected_primary_schema_version),
            ),
            (
                "found_primary_schema_version",
                RelationalDiagnosticValue::SchemaVersionId(*found_primary_schema_version),
            ),
            (
                "expected_entity_kind_count",
                RelationalDiagnosticValue::unsigned(*expected_entity_kind_count),
            ),
            (
                "found_entity_kind_count",
                RelationalDiagnosticValue::unsigned(*found_entity_kind_count),
            ),
            (
                "expected_relation_kind_count",
                RelationalDiagnosticValue::unsigned(*expected_relation_kind_count),
            ),
            (
                "found_relation_kind_count",
                RelationalDiagnosticValue::unsigned(*found_relation_kind_count),
            ),
        ]),
        RecoveryCompatibilityMismatch::EntityAspectPlanRevision {
            kind_id,
            kind_name,
            expected_revision,
            found_revision,
        } => plan_revision_mismatch_fields(
            "EntityAspectPlanRevision",
            *kind_id,
            kind_name,
            None,
            *expected_revision,
            *found_revision,
            &[],
            &[],
        ),
        RecoveryCompatibilityMismatch::RelationAspectPlanRevision {
            kind_id,
            kind_name,
            expected_revision,
            found_revision,
        } => plan_revision_mismatch_fields(
            "RelationAspectPlanRevision",
            *kind_id,
            kind_name,
            None,
            *expected_revision,
            *found_revision,
            &[],
            &[],
        ),
        RecoveryCompatibilityMismatch::RelationIntegrityPlanRevision {
            kind_id,
            kind_name,
            contract_family,
            expected_revision,
            found_revision,
            expected_contract_ids,
            found_contract_ids,
        } => plan_revision_mismatch_fields(
            "RelationIntegrityPlanRevision",
            *kind_id,
            kind_name,
            Some(format!("{contract_family:?}")),
            *expected_revision,
            *found_revision,
            expected_contract_ids,
            found_contract_ids,
        ),
        RecoveryCompatibilityMismatch::RuntimeProfile { expected, found } => {
            expected_found_text_mismatch("RuntimeProfile", expected, found)
        }
        RecoveryCompatibilityMismatch::RuntimeName { expected, found } => {
            expected_found_text_mismatch("RuntimeName", expected, found)
        }
        RecoveryCompatibilityMismatch::DescriptorSemanticsVersion { expected, found } => {
            RelationalDiagnosticValue::object([
                (
                    "mismatch",
                    RelationalDiagnosticValue::string("DescriptorSemanticsVersion"),
                ),
                (
                    "expected",
                    RelationalDiagnosticValue::DescriptorSemanticsVersion(*expected),
                ),
                (
                    "found",
                    RelationalDiagnosticValue::DescriptorSemanticsVersion(*found),
                ),
            ])
        }
        RecoveryCompatibilityMismatch::DescriptorCanonicalizationVersion { expected, found } => {
            RelationalDiagnosticValue::object([
                (
                    "mismatch",
                    RelationalDiagnosticValue::string("DescriptorCanonicalizationVersion"),
                ),
                (
                    "expected",
                    RelationalDiagnosticValue::DescriptorCanonicalizationVersion(*expected),
                ),
                (
                    "found",
                    RelationalDiagnosticValue::DescriptorCanonicalizationVersion(*found),
                ),
            ])
        }
        RecoveryCompatibilityMismatch::SchemaTransitionArtifact { commit_id, detail } => {
            commit_artifact_mismatch("SchemaTransitionArtifact", *commit_id, detail)
        }
        RecoveryCompatibilityMismatch::ContinuationDescriptor {
            commit_id,
            boundary_fingerprint,
            detail,
        } => RelationalDiagnosticValue::object([
            (
                "mismatch",
                RelationalDiagnosticValue::string("ContinuationDescriptor"),
            ),
            (
                "commit_id",
                RelationalDiagnosticValue::CommitId(CommitId(*commit_id)),
            ),
            (
                "boundary_fingerprint",
                RelationalDiagnosticValue::optional(
                    boundary_fingerprint.map(RelationalDiagnosticValue::SchemaBoundaryFingerprint),
                ),
            ),
            ("detail", RelationalDiagnosticValue::string(detail)),
        ]),
        RecoveryCompatibilityMismatch::ReconciliationDescriptor { commit_id, detail } => {
            commit_artifact_mismatch("ReconciliationDescriptor", *commit_id, detail)
        }
        RecoveryCompatibilityMismatch::SchemaLineage { commit_id, detail } => {
            commit_artifact_mismatch("SchemaLineage", *commit_id, detail)
        }
    }
}

fn plan_revision_mismatch_fields(
    mismatch_name: &'static str,
    kind_id: crate::identity::data::KindId,
    kind_name: &str,
    contract_family: Option<String>,
    expected_revision: u128,
    found_revision: u128,
    expected_contract_ids: &[ContractId],
    found_contract_ids: &[ContractId],
) -> RelationalDiagnosticValue {
    RelationalDiagnosticValue::object([
        ("mismatch", RelationalDiagnosticValue::string(mismatch_name)),
        ("kind_id", RelationalDiagnosticValue::KindId(kind_id)),
        ("kind_name", RelationalDiagnosticValue::string(kind_name)),
        (
            "contract_family",
            RelationalDiagnosticValue::optional(
                contract_family.map(RelationalDiagnosticValue::string),
            ),
        ),
        (
            "expected_revision",
            RelationalDiagnosticValue::string(expected_revision.to_string()),
        ),
        (
            "found_revision",
            RelationalDiagnosticValue::string(found_revision.to_string()),
        ),
        (
            "expected_contract_ids",
            contract_id_array(expected_contract_ids),
        ),
        ("found_contract_ids", contract_id_array(found_contract_ids)),
    ])
}

fn expected_found_text_mismatch(
    mismatch_name: &'static str,
    expected: &str,
    found: &str,
) -> RelationalDiagnosticValue {
    RelationalDiagnosticValue::object([
        ("mismatch", RelationalDiagnosticValue::string(mismatch_name)),
        ("expected", RelationalDiagnosticValue::string(expected)),
        ("found", RelationalDiagnosticValue::string(found)),
    ])
}

fn commit_artifact_mismatch(
    mismatch_name: &'static str,
    commit_id: u64,
    detail: &str,
) -> RelationalDiagnosticValue {
    RelationalDiagnosticValue::object([
        ("mismatch", RelationalDiagnosticValue::string(mismatch_name)),
        (
            "commit_id",
            RelationalDiagnosticValue::CommitId(CommitId(commit_id)),
        ),
        ("detail", RelationalDiagnosticValue::string(detail)),
    ])
}
