//! In-memory trace index for hierarchical drill-down.
//!
//! Reads `DecisionLog` JSON files from a directory, indexes them by ID,
//! and provides level-by-level query methods for the REST API.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use forge_core::tracing::TraceAdjunctRecord;
use forge_core::{DecisionLog, DecisionTier, SpanSummaryEntry, TraceEvent, TracedDecision};
use serde::{Deserialize, Serialize};

/// Metadata stored alongside each trace file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceMeta {
    /// Unique identifier (derived from filename).
    pub id: String,
    /// Human-readable test/operation name.
    pub name: String,
    /// When the trace was recorded (ISO 8601).
    pub timestamp: String,
    /// Total number of decisions.
    pub total_decisions: usize,
    /// Number of Tier 2+ interesting decisions.
    pub interesting_count: usize,
    /// Number of spans.
    pub span_count: usize,
    /// State hash from the operation result.
    pub state_hash: u128,
    /// Operation status: "ok" or "error".
    pub status: String,
}

/// A stored trace: metadata + the full DecisionLog.
#[derive(Debug)]
struct StoredTrace {
    meta: TraceMeta,
    log: DecisionLog,
    adjuncts: Vec<TraceAdjunctRecord>,
}

/// On-disk trace file format (what tests write).
#[derive(Debug, Serialize, Deserialize)]
pub struct TraceFile {
    /// Human-readable name (test name or operation description).
    pub name: String,
    /// ISO 8601 timestamp.
    pub timestamp: String,
    /// State hash from the OperationResult.
    pub state_hash: u128,
    /// Operation status: "ok" or "error".
    #[serde(default = "default_status")]
    pub status: String,
    /// The full DecisionLog.
    pub log: DecisionLog,
    /// Typed adjunct payloads attached to decisions (optional, versioned).
    #[serde(default)]
    pub adjuncts: Vec<TraceAdjunctRecord>,
}

/// Default status for backward compatibility with old trace files.
fn default_status() -> String {
    "ok".to_string()
}

/// Thread-safe trace store.
pub struct TraceStore {
    traces: BTreeMap<String, StoredTrace>,
    trace_dir: PathBuf,
}

/// Span-level view returned by the API (no decisions, just stats).
#[derive(Debug, Serialize)]
pub struct SpanView {
    /// Span ID.
    pub span_id: u64,
    /// Human-readable name.
    pub name: String,
    /// Number of decisions in this span.
    pub total_decisions: usize,
    /// Highest tier decision in this span.
    pub max_tier: String,
    /// Wall-clock duration in microseconds.
    pub duration_micros: u64,
}

/// Trace overview returned by `/api/traces/:id` (spans but no decisions).
#[derive(Debug, Serialize)]
pub struct TraceOverview {
    pub meta: TraceMeta,
    pub spans: Vec<SpanView>,
    pub display_interesting: String,
}

/// Single decision detail.
#[derive(Debug, Serialize)]
pub struct DecisionView {
    pub id: u64,
    pub kind: String,
    pub tier: String,
    pub margin: f64,
    pub span_id: Option<u64>,
    pub entity: String,
    pub context: String,
    pub display: String,
}

impl TraceStore {
    /// Create a new store watching the given directory.
    pub fn new(trace_dir: PathBuf) -> Self {
        Self {
            traces: BTreeMap::new(),
            trace_dir,
        }
    }

    /// Load all `.json` trace files from the trace directory.
    ///
    /// Supports two formats:
    /// - **Array format** (current): `[{trace1}, {trace2}, ...]` — one file with multiple traces
    /// - **Single format** (legacy): `{trace}` — one trace per file
    pub fn reload(&mut self) -> usize {
        self.traces.clear();
        let dir = &self.trace_dir;
        if !dir.exists() {
            return 0;
        }

        let mut entries: Vec<_> = std::fs::read_dir(dir)
            .into_iter()
            .flatten()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map(|x| x == "json").unwrap_or(false))
            .collect();

        entries.sort_by_key(|e| e.file_name());

        for entry in &entries {
            let Ok(content) = std::fs::read_to_string(entry.path()) else {
                continue;
            };

            if let Ok(trace_files) = serde_json::from_str::<Vec<TraceFile>>(&content) {
                for trace_file in trace_files {
                    self.insert_trace(trace_file);
                }
            } else if let Ok(trace_file) = serde_json::from_str::<TraceFile>(&content) {
                self.insert_trace(trace_file);
            }
        }

        self.traces.len()
    }

    /// Insert a single trace into the store, deriving its ID from the name.
    fn insert_trace(&mut self, trace_file: TraceFile) {
        let id = trace_file.name.replace("::", "_");

        let interesting_count = trace_file.log.interesting_only().len();
        let span_count = trace_file
            .log
            .get_events()
            .iter()
            .filter(|e| matches!(e, TraceEvent::StartSpan { .. }))
            .count();
        let total_decisions = trace_file.log.decisions().count();

        let meta = TraceMeta {
            id: id.clone(),
            name: trace_file.name,
            timestamp: trace_file.timestamp,
            total_decisions,
            interesting_count,
            span_count,
            state_hash: trace_file.state_hash,
            status: trace_file.status,
        };

        self.traces.insert(
            id,
            StoredTrace {
                meta,
                log: trace_file.log,
                adjuncts: trace_file.adjuncts,
            },
        );
    }

    /// List all traces (summary only, no decisions).
    pub fn list_traces(&self) -> Vec<&TraceMeta> {
        self.traces.values().map(|t| &t.meta).collect()
    }

    /// Get trace overview: metadata + span stats (no decisions).
    pub fn get_trace_overview(&self, id: &str) -> Option<TraceOverview> {
        let stored = self.traces.get(id)?;
        let span_summaries = compute_span_summaries_from_log(&stored.log);

        let spans: Vec<SpanView> = span_summaries
            .iter()
            .map(|ss| SpanView {
                span_id: ss.span_id.0,
                name: ss.name.clone(),
                total_decisions: ss.total_decisions,
                max_tier: format!("{}", ss.max_tier),
                duration_micros: ss.duration_micros,
            })
            .collect();

        Some(TraceOverview {
            meta: stored.meta.clone(),
            spans,
            display_interesting: stored.log.display_interesting(),
        })
    }

    /// Get decisions within a specific span.
    pub fn get_span_decisions(&self, trace_id: &str, span_id: u64) -> Option<Vec<DecisionView>> {
        let stored = self.traces.get(trace_id)?;
        let decisions: Vec<DecisionView> = stored
            .log
            .decisions()
            .filter(|d| d.get_span_id().map(|s| s.0 == span_id).unwrap_or(false))
            .map(decision_to_view)
            .collect();
        Some(decisions)
    }

    /// Get a single decision by its index in the full event stream.
    pub fn get_decision(&self, trace_id: &str, decision_idx: usize) -> Option<DecisionView> {
        let stored = self.traces.get(trace_id)?;
        stored
            .log
            .decisions()
            .nth(decision_idx)
            .map(decision_to_view)
    }

    /// Get the raw DecisionLog JSON for a trace.
    pub fn get_raw_log(&self, trace_id: &str) -> Option<&DecisionLog> {
        self.traces.get(trace_id).map(|t| &t.log)
    }

    /// Get raw typed adjunct payloads stored alongside the trace (may be empty).
    pub fn get_raw_adjuncts(&self, trace_id: &str) -> Option<&[TraceAdjunctRecord]> {
        self.traces.get(trace_id).map(|t| t.adjuncts.as_slice())
    }

    /// Get the trace directory path.
    pub fn trace_dir(&self) -> &Path {
        &self.trace_dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use forge_core::tracing::TraceAdjunctRecord;
    use forge_core::{
        CandidateValueSummary, DecisionContext, DecisionId, DecisionKind, DecisionLog,
        DecisionTier, PolicyDecisionTracePayload, PolicyKind, PolicyResolutionOutcome,
        PolicyResolutionSource, TracedDecision,
    };

    #[test]
    fn trace_store_preserves_unknown_and_known_adjuncts_on_reload() {
        let dir = tempfile::tempdir().expect("tempdir");
        let trace_path = dir.path().join("trace.json");

        let mut log = DecisionLog::new();
        log.record(TracedDecision::new(
            DecisionId(7),
            DecisionKind::Exact,
            DecisionTier::Deterministic,
            0.0,
            DecisionContext::Degeneracy {
                description: "fixture".into(),
            },
        ));

        let known = TraceAdjunctRecord::from_policy_payload(&PolicyDecisionTracePayload {
            decision_id: DecisionId(7),
            policy_kind: PolicyKind::CoincidentGeometry,
            operation_scope_id: Some("sheet_region_merge".into()),
            query_location: [0.0, 0.0, 0.0],
            measured_margin: 1.0e-7,
            threshold: Some(1.0e-6),
            overridable: true,
            candidate_summary: CandidateValueSummary::EnumTag {
                type_name: "Foo".into(),
                variant: "Bar".into(),
            },
            outcome: PolicyResolutionOutcome::AcceptedPotentialValue,
            source: PolicyResolutionSource::DefaultPolicy,
            source_scope: None,
            default_used: true,
        });
        let unknown = TraceAdjunctRecord::new(
            DecisionId(7),
            "future_payload",
            999,
            serde_json::json!({"x": 1, "nested": ["a", 2]}),
        );

        let file = vec![TraceFile {
            name: "fixture::trace".into(),
            timestamp: "0".into(),
            state_hash: 123,
            status: "ok".into(),
            log,
            adjuncts: vec![unknown.clone(), known.clone()],
        }];
        std::fs::write(
            &trace_path,
            serde_json::to_string_pretty(&file).expect("serialize trace file"),
        )
        .expect("write trace file");

        let mut store = TraceStore::new(dir.path().to_path_buf());
        assert_eq!(store.reload(), 1);
        let adjuncts = store
            .get_raw_adjuncts("fixture_trace")
            .expect("adjuncts for stored trace");
        assert_eq!(adjuncts.len(), 2);
        assert!(adjuncts.contains(&known));
        assert!(adjuncts.contains(&unknown));
    }
}

fn decision_to_view(d: &TracedDecision) -> DecisionView {
    DecisionView {
        id: d.get_id().0,
        kind: format!("{:?}", d.get_kind()),
        tier: format!("{}", d.get_tier()),
        margin: d.get_margin(),
        span_id: d.get_span_id().map(|s| s.0),
        entity: format!("{:?}", d.get_entity_scope()),
        context: format!("{:?}", d.get_context()),
        display: format!("{}", d),
    }
}

/// Recompute span summaries from a DecisionLog.
fn compute_span_summaries_from_log(log: &DecisionLog) -> Vec<SpanSummaryEntry> {
    let mut spans: Vec<SpanSummaryEntry> = Vec::new();

    for event in log.get_events() {
        if let TraceEvent::StartSpan { id, name, .. } = event {
            spans.push(SpanSummaryEntry {
                span_id: *id,
                name: name.clone(),
                total_decisions: 0,
                max_tier: DecisionTier::Deterministic,
                duration_micros: 0,
            });
        }
    }

    for d in log.decisions() {
        if let Some(sid) = d.get_span_id() {
            if let Some(entry) = spans.iter_mut().find(|s| s.span_id == sid) {
                entry.total_decisions += 1;
                if d.get_tier() > entry.max_tier {
                    entry.max_tier = d.get_tier();
                }
            }
        }
    }

    for event in log.get_events() {
        if let TraceEvent::EndSpan {
            id,
            duration_micros,
        } = event
        {
            if let Some(entry) = spans.iter_mut().find(|s| s.span_id == *id) {
                entry.duration_micros = *duration_micros;
            }
        }
    }

    spans
}
