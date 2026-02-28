use forge_core::errors::{
    MergeError, PersistentResolutionIncompatibility, PersistentResolutionRole,
};
use forge_core::provenance::SnapshotHandleRef;
use forge_core::tracing::{
    CandidateValueSummary, DecisionId, ReidentificationCompatibilitySummary,
    ReidentificationFailureCauseSummary, ReidentificationModeSummary,
    ReidentificationOutcome as ReidentificationTraceOutcome, ReidentificationTracePayload,
    TraceAdjunctRecord, TracedDecision,
};
use forge_core::EntityKind as TraceEntityKind;
use forge_core::KernelError;
use forge_topo::attributes::EntityKey;
use forge_topo::handles::FaceId;
use forge_topo::topology::history::lineage_link::{
    PersistentNameRef, ReidentificationCompatibility, ReidentificationMode, ReidentificationQuery,
    ReidentificationQueryResult,
};
use forge_topo::topology::naming::{resolve_name, resolve_selector};

use crate::core::ModelingContext;
use crate::core::{
    build_resolution_decision, ResolutionCandidate, ResolutionCandidates, ResolutionEvidence,
    ResolutionIncompatibility, ResolutionQuery, ResolutionResult, ResolverMatchKind, ResolverRoute,
};

use super::super::schema::{
    MergeRegionSelection, MergeRegionSelectionPersistent, PersistentFaceRef,
};

#[derive(Debug, Clone, Copy)]
pub(crate) enum FaceResolutionFallbackPipeline {
    DirectThenLineageThenHybrid,
}

/// Resolve a persistent selection to a snapshot `MergeRegionSelection`, tracing
/// every missing/ambiguous/incompatible outcome and failing closed.
pub fn resolve_merge_region_selection_persistent(
    state: &crate::core::KernelState,
    selection: &MergeRegionSelectionPersistent,
    ctx: &mut ModelingContext,
) -> Result<MergeRegionSelection, KernelError> {
    let arena = state.topology().arena();
    let mut selected = forge_topo::bitset::EntityBitset::for_faces(arena);
    let mut protected = forge_topo::bitset::EntityBitset::for_faces(arena);

    let surviving_face = resolve_single_face_ref(
        state.topology(),
        selection.get_surviving_face(),
        PersistentResolutionRole::SurvivingFace,
        FaceResolutionFallbackPipeline::DirectThenLineageThenHybrid,
        ctx,
    )?;

    for pref in selection.get_selected_faces() {
        let fid = resolve_single_face_ref(
            state.topology(),
            pref,
            PersistentResolutionRole::SelectedFace,
            FaceResolutionFallbackPipeline::DirectThenLineageThenHybrid,
            ctx,
        )?;
        selected.insert(fid.index())?;
    }
    for pref in selection.get_protected_faces() {
        let fid = resolve_single_face_ref(
            state.topology(),
            pref,
            PersistentResolutionRole::ProtectedFace,
            FaceResolutionFallbackPipeline::DirectThenLineageThenHybrid,
            ctx,
        )?;
        protected.insert(fid.index())?;
    }

    if !selected.contains(surviving_face.index())? {
        return Err(KernelError::InvalidInput {
            message: "Persistent surviving_face did not resolve into selected_faces set".into(),
            context: None,
        });
    }

    Ok(MergeRegionSelection::with_radial_selectors(
        selected,
        protected,
        surviving_face,
        selection.get_radial_selectors().to_vec(),
    ))
}

pub(crate) fn resolve_single_face_ref(
    topo_state: &forge_topo::transactions::TopologyState,
    pref: &PersistentFaceRef,
    role: PersistentResolutionRole,
    fallback: FaceResolutionFallbackPipeline,
    ctx: &mut ModelingContext,
    ) -> Result<FaceId, KernelError> {
    let result = resolve_face_ref_result(topo_state, pref, fallback);
    let role_tag = persistent_role_tag(role);
    let decision_id = DecisionId(hash_resolution_query_id(pref, role_tag));
    let decision = build_resolution_decision(decision_id, &result);
    let payload = result.to_trace_payload(
        decision_id,
        Some("sheet_region_merge".to_string()),
        Some(role_tag.to_string()),
    );

    ctx.get_decision_log_mut().record(decision.clone());
    ctx.push_trace_adjunct(TraceAdjunctRecord::from_resolution_payload(&payload));
    if let Some(reid_payload) =
        build_reidentification_trace_payload(topo_state, pref, fallback, decision_id, role_tag)
    {
        reid_payload
            .validate_against_decision(&decision)
            .map_err(|e| KernelError::InternalError {
                message: format!(
                    "ReidentificationTracePayload consistency violation (role={}): {:?}",
                    role_tag, e
                ),
                context: None,
            })?;
        ctx.push_trace_adjunct(TraceAdjunctRecord::from_reidentification_payload(
            &reid_payload,
        ));
    }

    match result {
        ResolutionResult::Resolved { value, .. } => Ok(FaceId::new(
            value.snapshot_ref.index,
            value.snapshot_ref.generation,
        )),
        ResolutionResult::Ambiguous {
            query, candidates, ..
        } => Err(KernelError::MergeFailure(
            MergeError::PersistentResolutionAmbiguous {
                role,
                candidate_count: candidates.len() as u32,
                query: query.to_trace_summary(),
            },
        )),
        ResolutionResult::Missing { query, .. } => Err(KernelError::MergeFailure(
            MergeError::PersistentResolutionMissing {
                role,
                query: query.to_trace_summary(),
            },
        )),
        ResolutionResult::Incompatible {
            query,
            incompatibility,
        } => Err(KernelError::MergeFailure(
            MergeError::PersistentResolutionIncompatible {
                role,
                incompatibility: map_resolution_incompatibility(&incompatibility),
                query: query.to_trace_summary(),
            },
        )),
    }
}

pub(crate) fn resolve_face_ref_result(
    topo_state: &forge_topo::transactions::TopologyState,
    pref: &PersistentFaceRef,
    fallback: FaceResolutionFallbackPipeline,
) -> ResolutionResult<ResolutionCandidate> {
    let direct = resolve_face_ref_direct(topo_state.arena(), pref);
    match (&direct, fallback) {
        (
            ResolutionResult::Missing { .. },
            FaceResolutionFallbackPipeline::DirectThenLineageThenHybrid,
        ) => resolve_face_ref_lineage_fallback(topo_state, pref, direct),
        _ => direct,
    }
}

pub(crate) fn resolve_face_ref_direct(
    arena: &forge_topo::b_rep::TopologyArena,
    pref: &PersistentFaceRef,
) -> ResolutionResult<ResolutionCandidate> {
    let (query, keys): (ResolutionQuery, Vec<EntityKey>) = match pref {
        PersistentFaceRef::Name(name) => {
            if name.get_kind() != TraceEntityKind::Face {
                return ResolutionResult::Incompatible {
                    query: ResolutionQuery::PersistentName(name.clone()),
                    incompatibility: ResolutionIncompatibility::UnsupportedEntityKind {
                        requested: name.get_kind(),
                    },
                };
            }
            (
                ResolutionQuery::PersistentName(name.clone()),
                resolve_name(arena, name),
            )
        }
        PersistentFaceRef::Selector(sel) => (
            ResolutionQuery::Selector(sel.clone()),
            resolve_selector(arena, sel),
        ),
    };

    let mut candidates = Vec::new();
    for key in keys {
        let EntityKey::Face(fid) = key else {
            return ResolutionResult::Incompatible {
                query,
                incompatibility: ResolutionIncompatibility::UnsupportedEntityKind {
                    requested: entity_key_kind(key),
                },
            };
        };
        candidates.push(ResolutionCandidate {
            entity_kind: TraceEntityKind::Face,
            persistent_ref: persistent_face_ref_label(pref),
            snapshot_ref: SnapshotHandleRef::new(
                TraceEntityKind::Face,
                fid.index(),
                fid.generation(),
            ),
            route: ResolverRoute::DirectPersistentName,
            match_kind: match pref {
                PersistentFaceRef::Name(_) => ResolverMatchKind::ExactPersistentName,
                PersistentFaceRef::Selector(_) => ResolverMatchKind::SelectorMatch,
            },
            provenance_tag: Some("direct_persistent_name".into()),
            provenance_detail: None,
        });
    }

    let evidence = ResolutionEvidence {
        routes_attempted: vec![ResolverRoute::DirectPersistentName],
        initial_candidate_count: candidates.len() as u32,
        surviving_candidate_count: candidates.len() as u32,
        filters_applied: Vec::new(),
        notes: Vec::new(),
    };

    match candidates.len() {
        0 => ResolutionResult::Missing { query, evidence },
        1 => ResolutionResult::Resolved {
            value: candidates.pop().expect("len=1"),
            route: ResolverRoute::DirectPersistentName,
            evidence,
        },
        _ => ResolutionResult::Ambiguous {
            query,
            candidates: ResolutionCandidates::from_vec(candidates),
            evidence,
        },
    }
}

fn resolve_face_ref_lineage_fallback(
    topo_state: &forge_topo::transactions::TopologyState,
    pref: &PersistentFaceRef,
    direct_missing: ResolutionResult<ResolutionCandidate>,
) -> ResolutionResult<ResolutionCandidate> {
    let arena = topo_state.arena();
    match direct_missing {
        ResolutionResult::Missing {
            query,
            mut evidence,
        } => {
            evidence
                .routes_attempted
                .push(ResolverRoute::LineageReidentified);
            let PersistentFaceRef::Name(name) = pref else {
                evidence.routes_attempted.push(ResolverRoute::Hybrid);
                evidence
                    .notes
                    .push("lineage fallback unsupported for selector queries in V1".into());
                return ResolutionResult::Incompatible {
                    query,
                    incompatibility: ResolutionIncompatibility::UnsupportedResolverMode {
                        mode: format!("fallback_pipeline:{}", pref_kind(pref)),
                    },
                };
            };

            if name.get_kind() != TraceEntityKind::Face {
                return ResolutionResult::Incompatible {
                    query,
                    incompatibility: ResolutionIncompatibility::UnsupportedEntityKind {
                        requested: name.get_kind(),
                    },
                };
            }

            let reid_query = ReidentificationQuery {
                target: PersistentNameRef {
                    ancestry_hash: name.get_ancestry_hash(),
                    kind: name.get_kind(),
                    ordinal: name.get_ordinal(),
                },
                mode: ReidentificationMode::Descendants,
            };
            match forge_topo::topology::history::lineage_link::resolve_reidentification_query_v1(
                arena,
                topo_state.lineage_events(),
                topo_state.reidentification_link_index(),
                &reid_query,
            ) {
                ReidentificationQueryResult::Resolved { candidate, .. } => {
                    ResolutionResult::Resolved {
                        value: topology_reid_candidate_to_kernel_face_candidate(pref, candidate),
                        route: ResolverRoute::LineageReidentified,
                        evidence,
                    }
                }
                ReidentificationQueryResult::Ambiguous { candidates, .. } => {
                    let converted = candidates
                        .into_iter()
                        .filter_map(|c| {
                            topology_reid_candidate_to_kernel_face_candidate_checked(pref, c)
                        })
                        .collect::<Vec<_>>();
                    ResolutionResult::Ambiguous {
                        query,
                        candidates: ResolutionCandidates::from_vec(converted),
                        evidence,
                    }
                }
                ReidentificationQueryResult::Missing { evidence: topo_ev } => {
                    if matches!(
                        topo_ev.compatibility,
                        ReidentificationCompatibility::MissingLinkage { .. }
                    ) {
                        // Differentiate pure "no descendants" vs legacy/index-only history by suspected cause.
                        if matches!(topo_ev.suspected_cause, Some(forge_topo::topology::history::lineage_link::ReidentificationFailureCause::SubstrateNotBuilt)) {
                            return ResolutionResult::Incompatible {
                                query,
                                incompatibility: ResolutionIncompatibility::LegacyIndexOnlyLineageHistory,
                            };
                        }
                    }
                    ResolutionResult::Missing { query, evidence }
                }
                ReidentificationQueryResult::Incompatible { evidence: topo_ev } => {
                    let incompatibility = match topo_ev.compatibility {
                        ReidentificationCompatibility::Unavailable => {
                            ResolutionIncompatibility::SubstrateUnavailable
                        }
                        ReidentificationCompatibility::SchemaVersionMismatch {
                            supported,
                            recorded,
                        } => ResolutionIncompatibility::SchemaVersionMismatch {
                            expected: supported,
                            actual: recorded,
                        },
                        ReidentificationCompatibility::MissingLinkage { .. } => {
                            ResolutionIncompatibility::LegacyIndexOnlyLineageHistory
                        }
                        ReidentificationCompatibility::UnsupportedMode { .. } => {
                            ResolutionIncompatibility::UnsupportedResolverMode {
                                mode: "lineage_descendants_v1".into(),
                            }
                        }
                        ReidentificationCompatibility::UnsupportedEntityOrigin { origin } => {
                            ResolutionIncompatibility::UnsupportedEntityOrigin {
                                origin: map_topo_origin_kind_to_persistent(origin.clone()),
                            }
                        }
                        _ => ResolutionIncompatibility::Other {
                            code: "topo_reid_incompatible".into(),
                            detail: format!("{:?}", topo_ev.compatibility),
                        },
                    };
                    ResolutionResult::Incompatible {
                        query,
                        incompatibility,
                    }
                }
            }
        }
        other => other,
    }
}

fn topology_reid_candidate_to_kernel_face_candidate(
    pref: &PersistentFaceRef,
    record: forge_topo::topology::history::lineage_link::ReidentificationCandidate,
) -> ResolutionCandidate {
    ResolutionCandidate {
        entity_kind: TraceEntityKind::Face,
        persistent_ref: persistent_face_ref_label(pref),
        snapshot_ref: SnapshotHandleRef::new(
            TraceEntityKind::Face,
            record.snapshot_ref.index,
            record.snapshot_ref.generation,
        ),
        route: ResolverRoute::LineageReidentified,
        match_kind: ResolverMatchKind::LineageDescendant,
        provenance_tag: Some("reidentification_link_index".into()),
        provenance_detail: Some(format!(
            "parent_linkage_mode={:?};origin={:?};epoch={}",
            record.link_evidence.parent_linkage_mode,
            record.link_evidence.origin_kind,
            record.link_evidence.epoch
        )),
    }
}

fn topology_reid_candidate_to_kernel_face_candidate_checked(
    pref: &PersistentFaceRef,
    record: forge_topo::topology::history::lineage_link::ReidentificationCandidate,
) -> Option<ResolutionCandidate> {
    if record.snapshot_ref.kind != forge_core::EntityKind::Face {
        return None;
    }
    Some(topology_reid_candidate_to_kernel_face_candidate(
        pref, record,
    ))
}

fn hash_resolution_query_id(pref: &PersistentFaceRef, role: &str) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    match pref {
        PersistentFaceRef::Name(name) => {
            for word in [
                name.get_kind() as u64,
                (name.get_ancestry_hash() >> 64) as u64,
                name.get_ancestry_hash() as u64,
                name.get_ordinal() as u64,
            ] {
                h ^= word;
                h = h.wrapping_mul(0x100000001b3);
            }
        }
        PersistentFaceRef::Selector(sel) => {
            for b in format!("selector:{:?}", sel).as_bytes() {
                h ^= *b as u64;
                h = h.wrapping_mul(0x100000001b3);
            }
        }
    }
    for b in role.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

fn persistent_face_ref_label(pref: &PersistentFaceRef) -> String {
    match pref {
        PersistentFaceRef::Name(name) => format!(
            "face:{:032x}:{}",
            name.get_ancestry_hash(),
            name.get_ordinal()
        ),
        PersistentFaceRef::Selector(sel) => format!("selector:{:?}", sel),
    }
}

fn pref_kind(pref: &PersistentFaceRef) -> &'static str {
    match pref {
        PersistentFaceRef::Name(_) => "name",
        PersistentFaceRef::Selector(_) => "selector",
    }
}

fn entity_key_kind(key: EntityKey) -> TraceEntityKind {
    match key {
        EntityKey::Face(_) => TraceEntityKind::Face,
        EntityKey::Edge(_) => TraceEntityKind::Edge,
        EntityKey::Vertex(_) => TraceEntityKind::Vertex,
        EntityKey::Shell(_) => TraceEntityKind::Shell,
    }
}

fn persistent_role_tag(role: PersistentResolutionRole) -> &'static str {
    match role {
        PersistentResolutionRole::SurvivingFace => "surviving_face",
        PersistentResolutionRole::SelectedFace => "selected_face",
        PersistentResolutionRole::ProtectedFace => "protected_face",
    }
}

pub(crate) fn map_resolution_incompatibility(
    inc: &crate::core::ResolutionIncompatibility,
) -> PersistentResolutionIncompatibility {
    match inc {
        crate::core::ResolutionIncompatibility::UnsupportedEntityKind { requested } => {
            PersistentResolutionIncompatibility::UnsupportedEntityKind {
                requested: *requested,
            }
        }
        crate::core::ResolutionIncompatibility::MissingLineageStore => {
            PersistentResolutionIncompatibility::MissingLineageStore
        }
        crate::core::ResolutionIncompatibility::SubstrateUnavailable => {
            PersistentResolutionIncompatibility::SubstrateUnavailable
        }
        crate::core::ResolutionIncompatibility::UnsupportedEntityOrigin { origin } => {
            PersistentResolutionIncompatibility::UnsupportedEntityOrigin { origin: *origin }
        }
        crate::core::ResolutionIncompatibility::SchemaVersionMismatch { expected, actual } => {
            PersistentResolutionIncompatibility::SchemaVersionMismatch {
                expected: *expected,
                actual: *actual,
            }
        }
        crate::core::ResolutionIncompatibility::LineageStoreVersionMismatch { .. } => {
            PersistentResolutionIncompatibility::UnsupportedLineageFallback
        }
        crate::core::ResolutionIncompatibility::LegacyIndexOnlyLineageHistory => {
            PersistentResolutionIncompatibility::UnsupportedLineageFallback
        }
        crate::core::ResolutionIncompatibility::UnsupportedResolverMode { .. } => {
            PersistentResolutionIncompatibility::UnsupportedLineageFallback
        }
        crate::core::ResolutionIncompatibility::Other { code, .. } => {
            PersistentResolutionIncompatibility::Other { code: code.clone() }
        }
    }
}

fn map_topo_origin_kind_to_persistent(
    origin: forge_topo::topology::history::lineage_link::EntityOriginKind,
) -> forge_core::errors::PersistentResolutionOriginKind {
    match origin {
        forge_topo::topology::history::lineage_link::EntityOriginKind::TopoOperator => {
            forge_core::errors::PersistentResolutionOriginKind::TopoOperator
        }
        forge_topo::topology::history::lineage_link::EntityOriginKind::GeometricIntersection => {
            forge_core::errors::PersistentResolutionOriginKind::GeometricIntersection
        }
        forge_topo::topology::history::lineage_link::EntityOriginKind::ConstraintSolver => {
            forge_core::errors::PersistentResolutionOriginKind::ConstraintSolver
        }
        forge_topo::topology::history::lineage_link::EntityOriginKind::Unknown => {
            forge_core::errors::PersistentResolutionOriginKind::Unknown
        }
    }
}

fn build_reidentification_trace_payload(
    topo_state: &forge_topo::transactions::TopologyState,
    pref: &PersistentFaceRef,
    fallback: FaceResolutionFallbackPipeline,
    decision_id: DecisionId,
    role_tag: &str,
) -> Option<ReidentificationTracePayload> {
    if !matches!(
        fallback,
        FaceResolutionFallbackPipeline::DirectThenLineageThenHybrid
    ) {
        return None;
    }
    let PersistentFaceRef::Name(name) = pref else {
        return None;
    };
    if name.get_kind() != TraceEntityKind::Face {
        return None;
    }
    // Only attach a topo re-identification payload when direct lookup missed and the
    // lineage fallback path is relevant. This is deterministic and keeps adjunct volume bounded.
    if !resolve_name(topo_state.arena(), name).is_empty() {
        return None;
    }

    let query = ReidentificationQuery {
        target: PersistentNameRef {
            ancestry_hash: name.get_ancestry_hash(),
            kind: name.get_kind(),
            ordinal: name.get_ordinal(),
        },
        mode: ReidentificationMode::Descendants,
    };
    let topo_result =
        forge_topo::topology::history::lineage_link::resolve_reidentification_query_v1(
            topo_state.arena(),
            topo_state.lineage_events(),
            topo_state.reidentification_link_index(),
            &query,
        );
    let (outcome, evidence) = match topo_result {
        ReidentificationQueryResult::Resolved { evidence, .. } => {
            (ReidentificationTraceOutcome::Resolved, evidence)
        }
        ReidentificationQueryResult::Ambiguous { evidence, .. } => {
            (ReidentificationTraceOutcome::Ambiguous, evidence)
        }
        ReidentificationQueryResult::Missing { evidence } => {
            (ReidentificationTraceOutcome::Missing, evidence)
        }
        ReidentificationQueryResult::Incompatible { evidence } => {
            (ReidentificationTraceOutcome::Incompatible, evidence)
        }
    };
    Some(ReidentificationTracePayload {
        decision_id,
        query_entity_kind: name.get_kind(),
        query_ancestry_hash_hex: format!("{:032x}", name.get_ancestry_hash()),
        query_ordinal: name.get_ordinal(),
        outcome,
        compatibility: map_topo_reid_compatibility(&evidence.compatibility),
        suspected_cause: evidence
            .suspected_cause
            .as_ref()
            .map(map_topo_reid_failure_cause),
        mode_used: map_topo_reid_mode(evidence.mode_used),
        records_scanned: evidence.records_scanned,
        candidates_pre_filter: evidence.candidates_pre_filter,
        candidates_post_filter: evidence.candidates_post_filter,
        index_schema_version: evidence.index_schema_version,
        operation_scope_id: Some("sheet_region_merge".into()),
        source_scope_id: Some(role_tag.to_string()),
    })
}

fn map_topo_reid_mode(
    mode: forge_topo::topology::history::lineage_link::ReidentificationMode,
) -> ReidentificationModeSummary {
    match mode {
        forge_topo::topology::history::lineage_link::ReidentificationMode::Descendants => {
            ReidentificationModeSummary::Descendants
        }
        forge_topo::topology::history::lineage_link::ReidentificationMode::Ancestors => {
            ReidentificationModeSummary::Ancestors
        }
        forge_topo::topology::history::lineage_link::ReidentificationMode::Hybrid => {
            ReidentificationModeSummary::Hybrid
        }
    }
}

fn map_topo_origin_kind(
    origin: forge_topo::topology::history::lineage_link::EntityOriginKind,
) -> forge_core::tracing::ReidentificationOriginKindSummary {
    match origin {
        forge_topo::topology::history::lineage_link::EntityOriginKind::TopoOperator => {
            forge_core::tracing::ReidentificationOriginKindSummary::TopoOperator
        }
        forge_topo::topology::history::lineage_link::EntityOriginKind::GeometricIntersection => {
            forge_core::tracing::ReidentificationOriginKindSummary::GeometricIntersection
        }
        forge_topo::topology::history::lineage_link::EntityOriginKind::ConstraintSolver => {
            forge_core::tracing::ReidentificationOriginKindSummary::ConstraintSolver
        }
        forge_topo::topology::history::lineage_link::EntityOriginKind::Unknown => {
            forge_core::tracing::ReidentificationOriginKindSummary::Unknown
        }
    }
}

fn map_topo_reid_compatibility(
    c: &forge_topo::topology::history::lineage_link::ReidentificationCompatibility,
) -> ReidentificationCompatibilitySummary {
    match c {
        forge_topo::topology::history::lineage_link::ReidentificationCompatibility::Available => ReidentificationCompatibilitySummary::Available,
        forge_topo::topology::history::lineage_link::ReidentificationCompatibility::Unavailable => ReidentificationCompatibilitySummary::Unavailable,
        forge_topo::topology::history::lineage_link::ReidentificationCompatibility::SchemaVersionMismatch { recorded, supported } => {
            ReidentificationCompatibilitySummary::SchemaVersionMismatch { recorded: *recorded, supported: *supported }
        }
        forge_topo::topology::history::lineage_link::ReidentificationCompatibility::MissingLinkage { kind } => {
            ReidentificationCompatibilitySummary::MissingLinkage { kind: *kind }
        }
        forge_topo::topology::history::lineage_link::ReidentificationCompatibility::UnsupportedMode { mode } => {
            ReidentificationCompatibilitySummary::UnsupportedMode { mode: map_topo_reid_mode(*mode) }
        }
        forge_topo::topology::history::lineage_link::ReidentificationCompatibility::UnsupportedEntityOrigin { origin } => {
            ReidentificationCompatibilitySummary::UnsupportedEntityOrigin { origin: map_topo_origin_kind(origin.clone()) }
        }
    }
}

fn map_topo_reid_failure_cause(
    c: &forge_topo::topology::history::lineage_link::ReidentificationFailureCause,
) -> ReidentificationFailureCauseSummary {
    match c {
        forge_topo::topology::history::lineage_link::ReidentificationFailureCause::EntityDeleted => ReidentificationFailureCauseSummary::EntityDeleted,
        forge_topo::topology::history::lineage_link::ReidentificationFailureCause::ToleranceSnapVariant => ReidentificationFailureCauseSummary::ToleranceSnapVariant,
        forge_topo::topology::history::lineage_link::ReidentificationFailureCause::UnsupportedOriginClass { origin } => {
            ReidentificationFailureCauseSummary::UnsupportedOriginClass { origin: map_topo_origin_kind(origin.clone()) }
        }
        forge_topo::topology::history::lineage_link::ReidentificationFailureCause::SubstrateNotBuilt => ReidentificationFailureCauseSummary::SubstrateNotBuilt,
    }
}
