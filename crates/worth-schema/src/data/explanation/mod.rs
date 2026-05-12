use forge_relational::facade::runtime::RelationalRuntime;
use forge_runtime_bridge::facade::BridgeDiagnosticsFacade;
use forge_signal::facade::{diagnostics_for_graph, SignalGraph};
use serde::{Deserialize, Serialize};

use crate::data::tracing::{
    AuthorityTraceAnchor, AuthorityTraceEvidence, BoundaryEnvelope, BoundaryFailure,
    BridgeTraceAnchor, BridgeTraceEvidence, DecisionTrace, DerivedTraceAnchor,
    DerivedTraceEvidence, IntegrityMarkers, SignalTraceAnchor, SignalTraceEvidence,
    TraceAvailability,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NarrativeLine {
    pub heading: String,
    pub body: String,
}

impl NarrativeLine {
    pub fn new(heading: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            heading: heading.into(),
            body: body.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityNarrative {
    pub availability: TraceAvailability,
    pub headline: String,
    pub branch_id: String,
    pub latest_commit_id: Option<u64>,
    pub latest_snapshot_id: Option<u64>,
    pub branch_head_commit_id: Option<u64>,
    pub branch_head_matches_latest_commit: bool,
    pub changed_record_count: usize,
    pub changed_records: Vec<String>,
    pub changed_aspects: Vec<String>,
    pub lineage_event_count: usize,
    pub story_lines: Vec<NarrativeLine>,
    pub query_hints: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BridgeRouteNarrative {
    pub route_identity: String,
    pub invalidation_identity: String,
    pub snapshot_identity: String,
    pub subscription_slice_identity: String,
    pub route_entry_count: usize,
    pub invalidation_target_count: usize,
    pub invalidation_targets: Vec<String>,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BridgeHistoricalNarrative {
    pub record_identity: String,
    pub declaration_identity: String,
    pub branch_identity: String,
    pub commit_identity: String,
    pub snapshot_identity: String,
    pub materialization_path: String,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BridgeNarrative {
    pub availability: TraceAvailability,
    pub headline: String,
    pub route_count: usize,
    pub historical_record_count: usize,
    pub routes: Vec<BridgeRouteNarrative>,
    pub historical_records: Vec<BridgeHistoricalNarrative>,
    pub story_lines: Vec<NarrativeLine>,
    pub query_hints: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedNarrative {
    pub availability: TraceAvailability,
    pub headline: String,
    pub branch_id: String,
    pub snapshot_id: u64,
    pub version_id: u64,
    pub truth_basis_digest: String,
    pub entity_count: usize,
    pub relation_count: usize,
    pub touched_aspects: Vec<String>,
    pub invalidation_target_count: usize,
    pub fallback_classes: Vec<String>,
    pub equivalence_digest: Option<String>,
    pub story_lines: Vec<NarrativeLine>,
    pub query_hints: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignalNarrative {
    pub availability: TraceAvailability,
    pub headline: String,
    pub node_id: String,
    pub replay_cursor: Option<u64>,
    pub execution_record_id: Option<u64>,
    pub semantic_segment_id: Option<u64>,
    pub lineage_artifact_id: Option<u64>,
    pub replay_event_count: usize,
    pub explanation_availability: Option<String>,
    pub provenance_availability: Option<String>,
    pub story_lines: Vec<NarrativeLine>,
    pub query_hints: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NarratedTrace {
    pub headline: String,
    pub causal_story: Vec<String>,
    pub query_hints: Vec<String>,
    pub authority: Option<AuthorityNarrative>,
    pub bridge: Option<BridgeNarrative>,
    pub derived: Option<DerivedNarrative>,
    pub signal: Option<SignalNarrative>,
}

fn availability_or_present(availability: Option<TraceAvailability>) -> TraceAvailability {
    availability.unwrap_or(TraceAvailability::Present)
}

pub fn explain_authority_trace(
    runtime: &RelationalRuntime,
    anchor: &AuthorityTraceAnchor,
    evidence: Option<&AuthorityTraceEvidence>,
) -> AuthorityNarrative {
    let latest_commit_id = anchor.latest_commit_id();
    let latest_snapshot_id = anchor.latest_snapshot_id();
    let branch_head_commit_id = runtime
        .history()
        .branch_head(&anchor.branch_id)
        .map(|head| head.commit_id);
    let branch_head_matches_latest_commit =
        latest_commit_id.is_some() && latest_commit_id == branch_head_commit_id;
    let inspection = latest_commit_id
        .and_then(|commit_id| runtime.inspect_what_happened().inspect_commit(commit_id));
    let changed_record_count = inspection
        .as_ref()
        .map(|inspection| inspection.changed_records.len())
        .unwrap_or(0);
    let changed_records = inspection
        .as_ref()
        .map(|inspection| {
            inspection
                .changed_records
                .iter()
                .map(|record| format!("{record:?}"))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let changed_aspects = inspection
        .as_ref()
        .map(|inspection| {
            inspection
                .changed_aspects
                .iter()
                .map(|aspect| format!("{aspect:?}"))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let lineage_event_count = inspection
        .as_ref()
        .map(|inspection| inspection.lineage_event_ids.len())
        .unwrap_or(0);
    let commit_count = evidence
        .map(|evidence| evidence.commit_count)
        .unwrap_or(anchor.commit_ids.len());
    let headline = format!(
        "Authority committed {} mutation batch(es) on branch `{}`. The latest commit {} the live branch head.",
        commit_count,
        anchor.branch_id.0,
        if branch_head_matches_latest_commit {
            "matches"
        } else {
            "does not match"
        }
    );
    let mut story_lines = vec![
        NarrativeLine::new(
            "Commit",
            format!(
                "Latest authoritative commit is `{}` and latest snapshot is `{}`.",
                latest_commit_id
                    .map(|id| id.0.to_string())
                    .unwrap_or_else(|| "unavailable".to_string()),
                latest_snapshot_id
                    .map(|id| id.0.to_string())
                    .unwrap_or_else(|| "unavailable".to_string())
            ),
        ),
        NarrativeLine::new(
            "Change Footprint",
            format!(
                "The latest inspected commit changed {} record(s), touched {} aspect tag(s), and emitted {} lineage event(s).",
                changed_record_count,
                changed_aspects.len(),
                lineage_event_count
            ),
        ),
    ];
    if let Some(evidence) = evidence {
        story_lines.push(NarrativeLine::new(
            "Commit Pipeline",
            format!(
                "Authority retained {} published commit log(s) spanning {} pipeline phase(s) and {} invariant result(s).",
                evidence.published_commit_count,
                evidence.total_phase_count,
                evidence.invariant_result_count
            ),
        ));
    }
    if !changed_aspects.is_empty() {
        story_lines.push(NarrativeLine::new(
            "Changed Aspects",
            format!(
                "Observed relational aspect tags: {}.",
                changed_aspects.join(", ")
            ),
        ));
    }
    let query_hints = vec![
        format!(
            "Use RelationalRuntime::inspect_what_happened().inspect_commit(CommitId({})) for the full authoritative diff.",
            latest_commit_id.map(|id| id.0).unwrap_or_default()
        ),
        format!(
            "Use AuthorityTraceAnchor::open_latest_snapshot(...) to reopen snapshot `{}` directly.",
            latest_snapshot_id.map(|id| id.0).unwrap_or_default()
        ),
    ];

    AuthorityNarrative {
        availability: TraceAvailability::Present,
        headline,
        branch_id: anchor.branch_id.0.clone(),
        latest_commit_id: latest_commit_id.map(|id| id.0),
        latest_snapshot_id: latest_snapshot_id.map(|id| id.0),
        branch_head_commit_id: branch_head_commit_id.map(|id| id.0),
        branch_head_matches_latest_commit,
        changed_record_count,
        changed_records,
        changed_aspects,
        lineage_event_count,
        story_lines,
        query_hints,
    }
}

pub fn explain_bridge_trace(
    diagnostics: &BridgeDiagnosticsFacade,
    anchor: &BridgeTraceAnchor,
    evidence: Option<&BridgeTraceEvidence>,
) -> BridgeNarrative {
    let routes = anchor
        .route_identities
        .iter()
        .filter_map(|route_identity| {
            diagnostics
                .route_record_for_route_identity(route_identity)
                .map(|record| diagnostics.explain_route_record(&record))
        })
        .map(|explanation| {
            let invalidation_targets = explanation
                .invalidation_targets()
                .iter()
                .map(|target| format!("{target:?}"))
                .collect::<Vec<_>>();
            BridgeRouteNarrative {
                route_identity: explanation.route_identity().as_str().to_string(),
                invalidation_identity: explanation.invalidation_identity().as_str().to_string(),
                snapshot_identity: explanation.snapshot_identity().as_str().to_string(),
                subscription_slice_identity: explanation
                    .subscription_slice_identity()
                    .as_str()
                    .to_string(),
                route_entry_count: explanation.route_entries().len(),
                invalidation_target_count: explanation.invalidation_targets().len(),
                summary: format!(
                    "Route `{}` lowered one truth event into {} invalidation target(s) through subscription slice `{}`.",
                    explanation.route_identity().as_str(),
                    explanation.invalidation_targets().len(),
                    explanation.subscription_slice_identity().as_str()
                ),
                invalidation_targets,
            }
        })
        .collect::<Vec<_>>();
    let historical_records = anchor
        .historical_record_identities
        .iter()
        .filter_map(|record_identity| {
            diagnostics
                .historical_record_for_record_identity(record_identity)
                .map(|record| diagnostics.explain_historical_evaluation_record(&record))
        })
        .map(|explanation| BridgeHistoricalNarrative {
            record_identity: explanation.record_identity().as_str().to_string(),
            declaration_identity: explanation.declaration_identity().as_str().to_string(),
            branch_identity: explanation.branch_identity().as_str().to_string(),
            commit_identity: explanation.commit_identity().as_str().to_string(),
            snapshot_identity: explanation.snapshot_identity().as_str().to_string(),
            materialization_path: format!("{:?}", explanation.materialization_path()),
            summary: format!(
                "Historical evaluation `{}` replayed branch `{}` commit `{}` through `{}`.",
                explanation.record_identity().as_str(),
                explanation.branch_identity().as_str(),
                explanation.commit_identity().as_str(),
                format!("{:?}", explanation.materialization_path())
            ),
        })
        .collect::<Vec<_>>();
    let availability = availability_or_present(evidence.map(|evidence| evidence.availability));
    let headline = format!(
        "Bridge retained {} route explanation(s) and {} historical evaluation explanation(s).",
        routes.len(),
        historical_records.len()
    );
    let mut story_lines = Vec::new();
    if let Some(first) = routes.first() {
        story_lines.push(NarrativeLine::new("Routing", first.summary.clone()));
    }
    if let Some(first) = historical_records.first() {
        story_lines.push(NarrativeLine::new(
            "Historical Evaluation",
            first.summary.clone(),
        ));
    }
    if let Some(evidence) = evidence {
        story_lines.push(NarrativeLine::new(
            "Anchor Coverage",
            format!(
                "Bridge preserved {} route identity anchor(s), {} invalidation identity anchor(s), and {} snapshot identity anchor(s).",
                evidence.route_identities.len(),
                evidence.invalidation_identities.len(),
                evidence.snapshot_identities.len()
            ),
        ));
    }
    let query_hints = vec![
        "Use BridgeDiagnosticsFacade::route_record_for_route_identity(...) to recover the canonical route record.".to_string(),
        "Use BridgeDiagnosticsFacade::historical_record_for_record_identity(...) to recover the canonical historical evaluation record.".to_string(),
    ];

    BridgeNarrative {
        availability,
        headline,
        route_count: routes.len(),
        historical_record_count: historical_records.len(),
        routes,
        historical_records,
        story_lines,
        query_hints,
    }
}

pub fn explain_derived_trace(
    runtime: &RelationalRuntime,
    anchor: &DerivedTraceAnchor,
    evidence: Option<&DerivedTraceEvidence>,
    integrity_markers: Option<&IntegrityMarkers>,
) -> DerivedNarrative {
    let reopened = anchor.open_snapshot(runtime);
    let entity_count = reopened
        .as_ref()
        .map(|view| view.entities().len())
        .unwrap_or(0);
    let relation_count = reopened
        .as_ref()
        .map(|view| view.relations().len())
        .unwrap_or(0);
    let touched_aspects = integrity_markers
        .map(|markers| {
            markers
                .touched_aspects
                .iter()
                .map(|aspect| format!("{aspect:?}"))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let invalidation_target_count = evidence
        .map(|evidence| evidence.invalidation_target_count)
        .unwrap_or(0);
    let fallback_classes = evidence
        .map(|evidence| evidence.fallback_classes.clone())
        .unwrap_or_default();
    let equivalence_digest = evidence.and_then(|evidence| evidence.equivalence_digest.clone());
    let availability = availability_or_present(evidence.map(|evidence| evidence.availability));
    let headline = format!(
        "Derived trace reopened authoritative snapshot `{}` on branch `{}` with {} entit(ies) and {} relation(s).",
        anchor.snapshot_id.0, anchor.branch_id.0, entity_count, relation_count
    );
    let mut story_lines = vec![
        NarrativeLine::new(
            "Truth Basis",
            format!(
                "Derived work is anchored to truth digest `{}` at version `{}`.",
                anchor.truth_basis_identity.mutation_batch_digest_hex, anchor.version_id.0
            ),
        ),
        NarrativeLine::new(
            "Reopened Snapshot",
            format!(
                "The runtime could reopen the authoritative snapshot and inspect {} entity record(s) plus {} relation record(s).",
                entity_count, relation_count
            ),
        ),
        NarrativeLine::new(
            "Invalidation Breadth",
            format!(
                "Derived fallout currently advertises {} invalidation target(s).",
                invalidation_target_count
            ),
        ),
    ];
    if !touched_aspects.is_empty() {
        story_lines.push(NarrativeLine::new(
            "Touched  Aspects",
            format!(
                " marked these aspects as touched: {}.",
                touched_aspects.join(", ")
            ),
        ));
    }
    if !fallback_classes.is_empty() {
        story_lines.push(NarrativeLine::new(
            "Fallbacks",
            format!(
                "Derived execution reported fallback classes: {}.",
                fallback_classes.join(", ")
            ),
        ));
    }
    if let Some(digest) = &equivalence_digest {
        story_lines.push(NarrativeLine::new(
            "Parity Anchor",
            format!("Derived equivalence currently resolves to digest `{digest}`."),
        ));
    }
    let query_hints = vec![
        format!(
            "Use DerivedTraceAnchor::open_snapshot(...) to inspect truth snapshot `{}` directly.",
            anchor.snapshot_id.0
        ),
        "Use the derived trace anchor together with  read/certification surfaces to compare current fallout against parity and diagnostics artifacts.".to_string(),
    ];

    DerivedNarrative {
        availability,
        headline,
        branch_id: anchor.branch_id.0.clone(),
        snapshot_id: anchor.snapshot_id.0,
        version_id: anchor.version_id.0,
        truth_basis_digest: anchor
            .truth_basis_identity
            .mutation_batch_digest_hex
            .clone(),
        entity_count,
        relation_count,
        touched_aspects,
        invalidation_target_count,
        fallback_classes,
        equivalence_digest,
        story_lines,
        query_hints,
    }
}

pub fn explain_signal_trace(
    graph: &SignalGraph,
    anchor: &SignalTraceAnchor,
    evidence: Option<&SignalTraceEvidence>,
) -> Result<SignalNarrative, forge_signal::facade::SignalError> {
    let observer = graph.observe();
    let explanation = observer.explain(anchor.node)?;
    let replay = if let Some(lineage_artifact_id) = anchor.lineage_artifact_id {
        observer.replay_for_artifact(lineage_artifact_id)
    } else {
        observer.replay_for_node(anchor.node)
    };
    let forensic = diagnostics_for_graph(graph).forensic();
    let (_, explanation_availability) = forensic.materialize_explanation_artifact(anchor.node)?;
    let (_, provenance_availability) = forensic.materialize_provenance_artifact(anchor.node)?;
    let replay_event_count = replay.len();
    let changed_region_count = explanation.changed_regions.len();
    let upstream_count = explanation.upstream.len();
    let lineage_artifact_id = anchor.lineage_artifact_id.map(|id| id.0);
    let headline = format!(
        "Signal tracked node `{:?}` through {} replay event(s); execution record {:?} and semantic segment {:?} explain why it moved.",
        anchor.node,
        replay_event_count,
        anchor.execution_record_id,
        anchor.semantic_segment_id
    );
    let mut story_lines = vec![
        NarrativeLine::new(
            "Node Explanation",
            format!(
                "Node `{:?}` is in state `{:?}` with {} upstream cause(s), {} changed region(s), and propagation_suppressed={}.",
                anchor.node,
                explanation.state,
                upstream_count,
                changed_region_count,
                explanation.propagation_suppressed
            ),
        ),
        NarrativeLine::new(
            "Execution Lineage",
            format!(
                "Signal attached execution record {:?}, semantic segment {:?}, and lineage artifact {:?} to this node.",
                anchor.execution_record_id,
                anchor.semantic_segment_id,
                lineage_artifact_id
            ),
        ),
    ];
    if let Some(first) = replay.first() {
        story_lines.push(NarrativeLine::new(
            "Replay",
            format!(
                "Replay starts at cursor {} on branch `{:?}` with event kind `{:?}`.",
                first.cursor.0, first.branch_id, first.kind
            ),
        ));
    }
    if !explanation.causal_links.is_empty() {
        let first = &explanation.causal_links[0];
        story_lines.push(NarrativeLine::new(
            "Causal Link",
            format!(
                "The first causal link is `{:?}` with disposition `{:?}` and scope provenance `{:?}`.",
                first.kind, first.disposition, first.scope.kind
            ),
        ));
    }
    let query_hints = vec![
        format!(
            "Use diagnostics.explain({:?}) to inspect the full node explanation for this signal node.",
            anchor.node
        ),
        if let Some(lineage_artifact_id) = lineage_artifact_id {
            format!(
                "Use diagnostics.replay_for_artifact(LineageArtifactId({})) to follow this node's retained lineage artifact.",
                lineage_artifact_id
            )
        } else {
            format!(
                "Use diagnostics.replay_for_node({:?}) to inspect replay history for this signal node.",
                anchor.node
            )
        },
    ];
    Ok(SignalNarrative {
        availability: availability_or_present(evidence.map(|evidence| evidence.availability)),
        headline,
        node_id: format!("{:?}", anchor.node),
        replay_cursor: anchor.replay_cursor.map(|cursor| cursor.0),
        execution_record_id: anchor.execution_record_id,
        semantic_segment_id: anchor.semantic_segment_id,
        lineage_artifact_id,
        replay_event_count,
        explanation_availability: Some(format!("{explanation_availability:?}")),
        provenance_availability: Some(format!("{provenance_availability:?}")),
        story_lines,
        query_hints,
    })
}

pub fn narrate_decision_trace(
    runtime: &RelationalRuntime,
    bridge_diagnostics: Option<&BridgeDiagnosticsFacade>,
    signal_graph: Option<&SignalGraph>,
    decision_trace: &DecisionTrace,
    integrity_markers: &IntegrityMarkers,
) -> NarratedTrace {
    let authority = decision_trace
        .authority_anchor()
        .map(|anchor| explain_authority_trace(runtime, anchor, decision_trace.authority.as_ref()));
    let bridge = decision_trace.bridge_anchor().and_then(|anchor| {
        bridge_diagnostics.map(|diagnostics| {
            explain_bridge_trace(diagnostics, anchor, decision_trace.bridge.as_ref())
        })
    });
    let derived = decision_trace.derived_anchor().map(|anchor| {
        explain_derived_trace(
            runtime,
            anchor,
            decision_trace.derived.as_ref(),
            Some(integrity_markers),
        )
    });
    let signal = decision_trace.signal_anchor().and_then(|anchor| {
        signal_graph.and_then(|graph| {
            explain_signal_trace(graph, anchor, decision_trace.signal.as_ref()).ok()
        })
    });

    let mut causal_story = Vec::new();
    let mut query_hints = Vec::new();
    if let Some(authority) = &authority {
        causal_story.push(authority.headline.clone());
        causal_story.extend(
            authority
                .story_lines
                .iter()
                .map(|line| format!("{}: {}", line.heading, line.body)),
        );
        query_hints.extend(authority.query_hints.iter().cloned());
    }
    if let Some(bridge) = &bridge {
        causal_story.push(bridge.headline.clone());
        causal_story.extend(
            bridge
                .story_lines
                .iter()
                .map(|line| format!("{}: {}", line.heading, line.body)),
        );
        query_hints.extend(bridge.query_hints.iter().cloned());
    }
    if let Some(derived) = &derived {
        causal_story.push(derived.headline.clone());
        causal_story.extend(
            derived
                .story_lines
                .iter()
                .map(|line| format!("{}: {}", line.heading, line.body)),
        );
        query_hints.extend(derived.query_hints.iter().cloned());
    }
    if let Some(signal) = &signal {
        causal_story.push(signal.headline.clone());
        causal_story.extend(
            signal
                .story_lines
                .iter()
                .map(|line| format!("{}: {}", line.heading, line.body)),
        );
        query_hints.extend(signal.query_hints.iter().cloned());
    }
    let headline = if let Some(authority) = &authority {
        if let Some(derived) = &derived {
            format!("{} {}", authority.headline, derived.headline)
        } else if let Some(signal) = &signal {
            format!("{} {}", authority.headline, signal.headline)
        } else {
            authority.headline.clone()
        }
    } else if let Some(bridge) = &bridge {
        bridge.headline.clone()
    } else if let Some(derived) = &derived {
        derived.headline.clone()
    } else if let Some(signal) = &signal {
        signal.headline.clone()
    } else {
        "No narrated runtime trace is currently available.".to_string()
    };

    NarratedTrace {
        headline,
        causal_story,
        query_hints,
        authority,
        bridge,
        derived,
        signal,
    }
}

pub fn narrate_boundary_envelope<T>(
    runtime: &RelationalRuntime,
    bridge_diagnostics: Option<&BridgeDiagnosticsFacade>,
    signal_graph: Option<&SignalGraph>,
    envelope: &BoundaryEnvelope<T>,
) -> NarratedTrace {
    narrate_decision_trace(
        runtime,
        bridge_diagnostics,
        signal_graph,
        envelope.decision_trace(),
        envelope.integrity_markers(),
    )
}

pub fn narrate_boundary_failure<E>(
    runtime: &RelationalRuntime,
    bridge_diagnostics: Option<&BridgeDiagnosticsFacade>,
    signal_graph: Option<&SignalGraph>,
    failure: &BoundaryFailure<E>,
) -> NarratedTrace {
    narrate_decision_trace(
        runtime,
        bridge_diagnostics,
        signal_graph,
        failure.decision_trace(),
        failure.integrity_markers(),
    )
}

#[cfg(test)]
mod tests;
