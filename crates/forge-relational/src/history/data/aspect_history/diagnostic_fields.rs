use crate::diagnostics::data::{RelationalDiagnosticFields, RelationalDiagnosticValue};
use crate::publication::patch::data::CanonicalAspectSet;

use super::{
    AspectFilter, AspectFilterMode, AspectHistoryCommitSpan, AspectHistoryLineageEventSpan,
    AspectHistoryResolutionTrace, HistoryAspectQueryTarget,
};

pub(super) fn aspect_history_resolution_trace_fields(
    trace: &AspectHistoryResolutionTrace,
) -> RelationalDiagnosticFields {
    RelationalDiagnosticValue::object([
        (
            "requested_target",
            history_query_target_value(&trace.requested_target),
        ),
        (
            "branch_id",
            RelationalDiagnosticValue::string(trace.branch_id.0.clone()),
        ),
        ("filter", filter_value(trace.filter.as_ref())),
        (
            "resolved_aspects",
            canonical_aspect_set_value(&trace.resolved_aspects),
        ),
        (
            "searched_commit_span",
            RelationalDiagnosticValue::optional(trace.searched_commit_span.map(commit_span_value)),
        ),
        (
            "searched_lineage_event_span",
            RelationalDiagnosticValue::optional(
                trace
                    .searched_lineage_event_span
                    .map(lineage_event_span_value),
            ),
        ),
        (
            "returned_entries",
            RelationalDiagnosticValue::Unsigned(trace.returned_entries),
        ),
        (
            "traversed_commits",
            RelationalDiagnosticValue::Unsigned(trace.traversed_commits),
        ),
        (
            "traversed_lineage_events",
            RelationalDiagnosticValue::Unsigned(trace.traversed_lineage_events),
        ),
    ])
    .into()
}

fn history_query_target_value(target: &HistoryAspectQueryTarget) -> RelationalDiagnosticValue {
    match target {
        HistoryAspectQueryTarget::Entity(entity_id) => RelationalDiagnosticValue::object([
            ("target_kind", RelationalDiagnosticValue::string("entity")),
            ("entity", RelationalDiagnosticValue::EntityId(*entity_id)),
        ]),
        HistoryAspectQueryTarget::Relation(relation_id) => RelationalDiagnosticValue::object([
            ("target_kind", RelationalDiagnosticValue::string("relation")),
            (
                "relation",
                RelationalDiagnosticValue::RelationId(*relation_id),
            ),
        ]),
        HistoryAspectQueryTarget::Lineage(lineage_id) => RelationalDiagnosticValue::object([
            ("target_kind", RelationalDiagnosticValue::string("lineage")),
            (
                "lineage_id",
                RelationalDiagnosticValue::LineageId(*lineage_id),
            ),
        ]),
    }
}

fn filter_value(filter: Option<&AspectFilter>) -> RelationalDiagnosticValue {
    RelationalDiagnosticValue::optional(filter.map(|filter| {
        RelationalDiagnosticValue::object([
            ("filter_mode", filter_mode_value(filter.mode)),
            (
                "requested_aspects",
                RelationalDiagnosticValue::array(
                    filter
                        .aspects
                        .iter()
                        .cloned()
                        .map(RelationalDiagnosticValue::AspectKey),
                ),
            ),
        ])
    }))
}

fn filter_mode_value(mode: AspectFilterMode) -> RelationalDiagnosticValue {
    match mode {
        AspectFilterMode::Any => RelationalDiagnosticValue::string("any"),
        AspectFilterMode::All => RelationalDiagnosticValue::string("all"),
    }
}

fn canonical_aspect_set_value(aspects: &CanonicalAspectSet) -> RelationalDiagnosticValue {
    RelationalDiagnosticValue::array(
        aspects
            .iter()
            .cloned()
            .map(RelationalDiagnosticValue::AspectKey),
    )
}

fn commit_span_value(span: AspectHistoryCommitSpan) -> RelationalDiagnosticValue {
    RelationalDiagnosticValue::object([
        (
            "first_commit_id",
            RelationalDiagnosticValue::Unsigned(span.first_commit_id.0),
        ),
        (
            "last_commit_id",
            RelationalDiagnosticValue::Unsigned(span.last_commit_id.0),
        ),
    ])
}

fn lineage_event_span_value(span: AspectHistoryLineageEventSpan) -> RelationalDiagnosticValue {
    RelationalDiagnosticValue::object([
        (
            "first_event_id",
            RelationalDiagnosticValue::Unsigned(span.first_event_id),
        ),
        (
            "last_event_id",
            RelationalDiagnosticValue::Unsigned(span.last_event_id),
        ),
    ])
}
