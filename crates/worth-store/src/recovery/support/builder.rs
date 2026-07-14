use crate::{backend::records::StoreState, failure::StoreErrorKind};

use super::model::{
    SupportArtifactFamily, SupportArtifactRecoveryDisposition, SupportArtifactRecoveryEntry,
    SupportArtifactRecoveryReport,
};

pub(crate) fn build_support_artifact_recovery_report(
    state: &StoreState,
) -> SupportArtifactRecoveryReport {
    let mut entries = Vec::new();

    for summary in state.commit_support_summaries.values() {
        entries.extend(build_commit_support_summary_recovery_entries(
            state, summary,
        ));
    }
    for record in state.schema_support_records.values() {
        if let Err(error) = state.verify_schema_support_record(record) {
            entries.push(SupportArtifactRecoveryEntry {
                family: SupportArtifactFamily::SchemaSupport,
                scope_identity: format!("schema-support:{}", record.artifact_id),
                related_commit_id: Some(record.commit_id.0),
                disposition: disposition_for_support_error(error.kind()),
                kind: error.kind().clone(),
                reason: error.message().to_string(),
            });
        }
    }
    for record in state.lineage_support_records.values() {
        if let Err(error) = state.verify_lineage_support_record(record) {
            entries.push(SupportArtifactRecoveryEntry {
                family: SupportArtifactFamily::LineageSupport,
                scope_identity: format!("lineage-support:{}", record.artifact_id),
                related_commit_id: Some(record.commit_id.0),
                disposition: disposition_for_support_error(error.kind()),
                kind: error.kind().clone(),
                reason: error.message().to_string(),
            });
        }
    }
    for record in state.durable_cursor_identity_records.values() {
        if let Err(error) = state.verify_durable_cursor_identity_record(record) {
            entries.push(SupportArtifactRecoveryEntry {
                family: SupportArtifactFamily::CursorSupport,
                scope_identity: format!("cursor-identity:{}", record.cursor_id),
                related_commit_id: Some(record.latest_basis_commit_id.0),
                disposition: disposition_for_support_error(error.kind()),
                kind: error.kind().clone(),
                reason: error.message().to_string(),
            });
        }
    }
    for record in state.subscriber_checkpoint_records.values() {
        if let Err(error) = state.verify_subscriber_checkpoint_record(record) {
            entries.push(SupportArtifactRecoveryEntry {
                family: SupportArtifactFamily::CursorSupport,
                scope_identity: format!("subscriber-checkpoint:{}", record.artifact_id),
                related_commit_id: Some(record.basis_commit_id.0),
                disposition: disposition_for_support_error(error.kind()),
                kind: error.kind().clone(),
                reason: error.message().to_string(),
            });
        }
    }
    for record in state.embedded_checkpoint_records.values() {
        if let Err(error) = state.verify_embedded_checkpoint_record(record) {
            entries.push(SupportArtifactRecoveryEntry {
                family: SupportArtifactFamily::EmbeddedCheckpoint,
                scope_identity: format!("embedded-checkpoint:{}", record.checkpoint_id),
                related_commit_id: record.basis_commit_id.map(|commit_id| commit_id.0),
                disposition: disposition_for_support_error(error.kind()),
                kind: error.kind().clone(),
                reason: error.message().to_string(),
            });
        }
    }

    SupportArtifactRecoveryReport { entries }
}

fn build_commit_support_summary_recovery_entries(
    state: &StoreState,
    summary: &crate::backend::records::CommitSupportSummaryRecord,
) -> Vec<SupportArtifactRecoveryEntry> {
    let mut entries = Vec::new();
    let scope_families = |schema: bool, lineage: bool| {
        let mut families = Vec::new();
        if schema {
            families.push(SupportArtifactFamily::SchemaSupport);
        }
        if lineage {
            families.push(SupportArtifactFamily::LineageSupport);
        }
        if families.is_empty() {
            families.push(SupportArtifactFamily::SchemaSupport);
        }
        families
    };

    let Some(commit_record) = state.commit_envelopes.get(&summary.commit_id.0) else {
        let error = StoreErrorKind::CommitSupportPublicationGap;
        let families = scope_families(
            summary.emitted_schema_artifact,
            summary.emitted_lineage_artifact,
        );
        for family in families {
            entries.push(SupportArtifactRecoveryEntry {
                family,
                scope_identity: format!(
                    "commit-support-summary:{family_scope}:commit:{}",
                    summary.commit_id.0,
                    family_scope = family_scope_name(family)
                ),
                related_commit_id: Some(summary.commit_id.0),
                disposition: disposition_for_support_error(&error),
                kind: error.clone(),
                reason: format!(
                    "support summary references missing commit {}",
                    summary.commit_id.0
                ),
            });
        }
        return entries;
    };

    let expected_schema = commit_record.envelope.schema_transition.is_some()
        || commit_record
            .envelope
            .schema_continuation_descriptor
            .is_some()
        || commit_record
            .envelope
            .schema_reconciliation_descriptor
            .is_some();
    let expected_lineage = !commit_record.envelope.lineage_event_ids().is_empty()
        || !commit_record.envelope.lineage_events().is_empty();

    if summary.branch_id != commit_record.envelope.branch_context {
        for family in scope_families(expected_schema, expected_lineage) {
            entries.push(SupportArtifactRecoveryEntry {
                family,
                scope_identity: format!(
                    "commit-support-summary:{family_scope}:commit:{}",
                    summary.commit_id.0,
                    family_scope = family_scope_name(family)
                ),
                related_commit_id: Some(summary.commit_id.0),
                disposition: disposition_for_support_error(
                    &StoreErrorKind::CommitSupportPublicationGap,
                ),
                kind: StoreErrorKind::CommitSupportPublicationGap,
                reason: format!(
                    "support summary for commit {} drifted from commit branch context",
                    summary.commit_id.0
                ),
            });
        }
    }

    if summary.emitted_schema_artifact != expected_schema {
        entries.push(SupportArtifactRecoveryEntry {
            family: SupportArtifactFamily::SchemaSupport,
            scope_identity: format!(
                "commit-support-summary:schema:commit:{}",
                summary.commit_id.0
            ),
            related_commit_id: Some(summary.commit_id.0),
            disposition: disposition_for_support_error(
                &StoreErrorKind::CommitSupportPublicationGap,
            ),
            kind: StoreErrorKind::CommitSupportPublicationGap,
            reason: format!(
                "schema support summary for commit {} did not match canonical envelope content",
                summary.commit_id.0
            ),
        });
    }
    if summary.emitted_lineage_artifact != expected_lineage {
        entries.push(SupportArtifactRecoveryEntry {
            family: SupportArtifactFamily::LineageSupport,
            scope_identity: format!(
                "commit-support-summary:lineage:commit:{}",
                summary.commit_id.0
            ),
            related_commit_id: Some(summary.commit_id.0),
            disposition: disposition_for_support_error(
                &StoreErrorKind::CommitSupportPublicationGap,
            ),
            kind: StoreErrorKind::CommitSupportPublicationGap,
            reason: format!(
                "lineage support summary for commit {} did not match canonical envelope content",
                summary.commit_id.0
            ),
        });
    }

    if expected_schema {
        let expected_id = format!("schema-support:{}", summary.commit_id.0);
        if summary.schema_support_artifact_id.as_deref() != Some(expected_id.as_str()) {
            entries.push(SupportArtifactRecoveryEntry {
                family: SupportArtifactFamily::SchemaSupport,
                scope_identity: format!("commit-support-summary:schema:commit:{}", summary.commit_id.0),
                related_commit_id: Some(summary.commit_id.0),
                disposition: disposition_for_support_error(&StoreErrorKind::CommitSupportPublicationGap),
                kind: StoreErrorKind::CommitSupportPublicationGap,
                reason: format!(
                    "schema support summary for commit {} did not point at the required schema support artifact",
                    summary.commit_id.0
                ),
            });
        } else if !state.schema_support_records.contains_key(&expected_id) {
            entries.push(SupportArtifactRecoveryEntry {
                family: SupportArtifactFamily::SchemaSupport,
                scope_identity: format!(
                    "commit-support-summary:schema:commit:{}",
                    summary.commit_id.0
                ),
                related_commit_id: Some(summary.commit_id.0),
                disposition: disposition_for_support_error(
                    &StoreErrorKind::CommitSupportPublicationGap,
                ),
                kind: StoreErrorKind::CommitSupportPublicationGap,
                reason: format!(
                    "schema support artifact for commit {} missing while summary claimed it exists",
                    summary.commit_id.0
                ),
            });
        }
    } else if summary.schema_support_artifact_id.is_some() {
        entries.push(SupportArtifactRecoveryEntry {
            family: SupportArtifactFamily::SchemaSupport,
            scope_identity: format!("commit-support-summary:schema:commit:{}", summary.commit_id.0),
            related_commit_id: Some(summary.commit_id.0),
            disposition: disposition_for_support_error(&StoreErrorKind::CommitSupportPublicationGap),
            kind: StoreErrorKind::CommitSupportPublicationGap,
            reason: format!(
                "commit {} recorded schema support artifact identity without schema support content",
                summary.commit_id.0
            ),
        });
    }

    if expected_lineage {
        let expected_id = format!("lineage-support:{}", summary.commit_id.0);
        if summary.lineage_support_artifact_id.as_deref() != Some(expected_id.as_str()) {
            entries.push(SupportArtifactRecoveryEntry {
                family: SupportArtifactFamily::LineageSupport,
                scope_identity: format!("commit-support-summary:lineage:commit:{}", summary.commit_id.0),
                related_commit_id: Some(summary.commit_id.0),
                disposition: disposition_for_support_error(&StoreErrorKind::CommitSupportPublicationGap),
                kind: StoreErrorKind::CommitSupportPublicationGap,
                reason: format!(
                    "lineage support summary for commit {} did not point at the required lineage support artifact",
                    summary.commit_id.0
                ),
            });
        } else if !state.lineage_support_records.contains_key(&expected_id) {
            entries.push(SupportArtifactRecoveryEntry {
                family: SupportArtifactFamily::LineageSupport,
                scope_identity: format!("commit-support-summary:lineage:commit:{}", summary.commit_id.0),
                related_commit_id: Some(summary.commit_id.0),
                disposition: disposition_for_support_error(&StoreErrorKind::CommitSupportPublicationGap),
                kind: StoreErrorKind::CommitSupportPublicationGap,
                reason: format!(
                    "lineage support artifact for commit {} missing while summary claimed it exists",
                    summary.commit_id.0
                ),
            });
        }
    } else if summary.lineage_support_artifact_id.is_some() {
        entries.push(SupportArtifactRecoveryEntry {
            family: SupportArtifactFamily::LineageSupport,
            scope_identity: format!("commit-support-summary:lineage:commit:{}", summary.commit_id.0),
            related_commit_id: Some(summary.commit_id.0),
            disposition: disposition_for_support_error(&StoreErrorKind::CommitSupportPublicationGap),
            kind: StoreErrorKind::CommitSupportPublicationGap,
            reason: format!(
                "commit {} recorded lineage support artifact identity without lineage support content",
                summary.commit_id.0
            ),
        });
    }

    entries
}

fn disposition_for_support_error(kind: &StoreErrorKind) -> SupportArtifactRecoveryDisposition {
    match kind {
        StoreErrorKind::CursorEquivalenceViolation | StoreErrorKind::CheckpointShapeViolation => {
            SupportArtifactRecoveryDisposition::RequireQuarantine
        }
        _ => SupportArtifactRecoveryDisposition::RequireRebuild,
    }
}

fn family_scope_name(family: SupportArtifactFamily) -> &'static str {
    match family {
        SupportArtifactFamily::SchemaSupport => "schema",
        SupportArtifactFamily::LineageSupport => "lineage",
        SupportArtifactFamily::CursorSupport => "cursor",
        SupportArtifactFamily::EmbeddedCheckpoint => "embedded-checkpoint",
    }
}
