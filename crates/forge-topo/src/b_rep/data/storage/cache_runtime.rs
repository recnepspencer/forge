//! Topology cache-domain runtime and effect mapping.

use std::collections::{BTreeMap, BTreeSet};

use forge_core::KernelError;
use forge_signal::facade::{
    BatchedDirtySet, CheckpointBarrier, CheckpointEvaluator, CheckpointPolicy, CheckpointRuntime,
    DomainImpact, EffectMapping, SignalError,
};
use smallvec::{smallvec, SmallVec};

use crate::handles::{FaceId, HalfEdgeId, ShellId, VertexId};

use super::arena::TopologyArena;

/// Topology cache domains coordinated by runtime policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TopoCacheDomain {
    /// Cached radial ring valence per halfedge slot.
    RadialValence,
    /// Derived index: face -> halfedges.
    FaceHalfedges,
    /// Derived index: vertex -> outgoing halfedges.
    VertexHalfedges,
    /// Derived index: shell -> faces.
    ShellFaces,
}

/// Canonical target keys for scoped cache invalidation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TopoCacheTarget {
    HalfEdge(HalfEdgeId),
    Face(FaceId),
    Vertex(VertexId),
    Shell(ShellId),
}

/// One deterministic refresh record emitted during a cache checkpoint flush.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheRefreshTraceEntry {
    pub barrier: CheckpointBarrier,
    pub domain: TopoCacheDomain,
    pub impact: DomainImpactKind,
    pub targets: Vec<TopoCacheTarget>,
}

/// Compact impact discriminant for refresh trace rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DomainImpactKind {
    Global,
    Scoped,
}

impl CacheRefreshTraceEntry {
    /// Stable line encoding for replay-level determinism assertions.
    pub fn encode(&self) -> String {
        let mut out = format!("{:?}:{:?}:{:?}", self.barrier, self.domain, self.impact);
        for target in &self.targets {
            out.push(':');
            out.push_str(&encode_target(*target));
        }
        out
    }
}

/// Policy controls for Tier-0 cache scheduling.
#[derive(Debug, Clone)]
pub struct TopoCachePolicy {
    pub allow_global_fallback: bool,
    pub strict_fresh_at_commit: bool,
    pub global_invalidation_budget: BTreeMap<TopoCacheDomain, u64>,
}

impl Default for TopoCachePolicy {
    fn default() -> Self {
        let mut budget = BTreeMap::new();
        budget.insert(TopoCacheDomain::RadialValence, 0);
        budget.insert(TopoCacheDomain::FaceHalfedges, 0);
        budget.insert(TopoCacheDomain::VertexHalfedges, 0);
        budget.insert(TopoCacheDomain::ShellFaces, 0);
        Self {
            allow_global_fallback: false,
            strict_fresh_at_commit: true,
            global_invalidation_budget: budget,
        }
    }
}

/// Runtime counters for cache invalidation and refresh behavior.
#[derive(Debug, Clone, Default)]
pub struct TopoCacheTelemetry {
    pub global_invalidations_by_domain: BTreeMap<TopoCacheDomain, u64>,
    pub scoped_invalidations_by_domain: BTreeMap<TopoCacheDomain, u64>,
    pub flushes_by_domain: BTreeMap<TopoCacheDomain, u64>,
}

impl TopoCacheTelemetry {
    fn inc_scoped(&mut self, domain: TopoCacheDomain, count: u64) {
        *self
            .scoped_invalidations_by_domain
            .entry(domain)
            .or_insert(0) += count;
    }

    fn inc_global(&mut self, domain: TopoCacheDomain) {
        *self
            .global_invalidations_by_domain
            .entry(domain)
            .or_insert(0) += 1;
    }

    fn inc_flush(&mut self, domain: TopoCacheDomain) {
        *self.flushes_by_domain.entry(domain).or_insert(0) += 1;
    }
}

/// Mutation effects emitted by topology mutation paths.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopoCacheEffect {
    /// One or more radial links changed.
    RadialLinksChanged {
        half_edges: SmallVec<[HalfEdgeId; 4]>,
    },
    /// One or more face-index memberships changed.
    FaceHalfedgesChanged { faces: SmallVec<[FaceId; 2]> },
    /// One or more vertex-index memberships changed.
    VertexHalfedgesChanged { vertices: SmallVec<[VertexId; 2]> },
    /// One or more shell-index memberships changed.
    ShellFacesChanged { shells: SmallVec<[ShellId; 2]> },
    /// Explicit fallback for broad mutations where exact scope is unknown.
    GlobalInvalidate {
        domain: TopoCacheDomain,
        reason_code: &'static str,
        site: &'static str,
    },
}

#[derive(Debug, Clone, Copy)]
struct TopoEffectMapping;

impl EffectMapping for TopoEffectMapping {
    type Domain = TopoCacheDomain;
    type Effect = TopoCacheEffect;
    type Impact = TopoCacheTarget;

    fn route(effect: &Self::Effect, sink: &mut BatchedDirtySet<Self::Domain, Self::Impact>) {
        match effect {
            TopoCacheEffect::RadialLinksChanged { half_edges } => {
                sink.mark_domain_scoped_many(
                    TopoCacheDomain::RadialValence,
                    half_edges.iter().copied().map(TopoCacheTarget::HalfEdge),
                );
            }
            TopoCacheEffect::FaceHalfedgesChanged { faces } => {
                sink.mark_domain_scoped_many(
                    TopoCacheDomain::FaceHalfedges,
                    faces.iter().copied().map(TopoCacheTarget::Face),
                );
            }
            TopoCacheEffect::VertexHalfedgesChanged { vertices } => {
                sink.mark_domain_scoped_many(
                    TopoCacheDomain::VertexHalfedges,
                    vertices.iter().copied().map(TopoCacheTarget::Vertex),
                );
            }
            TopoCacheEffect::ShellFacesChanged { shells } => {
                sink.mark_domain_scoped_many(
                    TopoCacheDomain::ShellFaces,
                    shells.iter().copied().map(TopoCacheTarget::Shell),
                );
            }
            TopoCacheEffect::GlobalInvalidate { domain, .. } => sink.mark_domain_global(*domain),
        }
    }
}

#[derive(Debug)]
struct TopoCacheEvaluator {
    barrier: CheckpointBarrier,
    trace: Vec<CacheRefreshTraceEntry>,
}

impl TopoCacheEvaluator {
    fn new(barrier: CheckpointBarrier) -> Self {
        Self {
            barrier,
            trace: Vec::new(),
        }
    }
}

impl CheckpointEvaluator for TopoCacheEvaluator {
    type Domain = TopoCacheDomain;
    type Impact = TopoCacheTarget;
    type Context = TopologyArena;

    fn refresh(
        &mut self,
        domain: Self::Domain,
        impact: DomainImpact<Self::Impact>,
        arena: &mut Self::Context,
    ) -> Result<(), SignalError> {
        let impact_kind = if impact.is_global() {
            DomainImpactKind::Global
        } else {
            DomainImpactKind::Scoped
        };
        let scoped_targets: Vec<TopoCacheTarget> = impact.scoped().collect();

        match domain {
            TopoCacheDomain::RadialValence => {
                if impact.is_global() {
                    arena.rebuild_cached_radial_valence().map_err(kernel_to_signal)?;
                } else {
                    let mut seeds = BTreeSet::new();
                    for target in &scoped_targets {
                        let TopoCacheTarget::HalfEdge(he) = target else {
                            continue;
                        };
                        if *he != HalfEdgeId::DANGLING {
                            seeds.insert(*he);
                        }
                    }
                    for he in seeds {
                        if arena.get_half_edge(he).is_ok() {
                            arena
                                .refresh_cached_radial_valence_for_ring(he)
                                .map_err(kernel_to_signal)?;
                        }
                    }
                }
            }
            TopoCacheDomain::FaceHalfedges => {
                if impact.is_global() {
                    arena.rebuild_face_halfedge_index().map_err(kernel_to_signal)?;
                } else {
                    let mut faces = BTreeSet::new();
                    for target in &scoped_targets {
                        let TopoCacheTarget::Face(face) = target else {
                            continue;
                        };
                        if *face != FaceId::DANGLING {
                            faces.insert(*face);
                        }
                    }
                    for face in faces {
                        if arena.get_face(face).is_ok() {
                            arena
                                .rebuild_face_halfedges_for_face(face)
                                .map_err(kernel_to_signal)?;
                        } else {
                            arena.remove_face_halfedge_index_entry(face);
                        }
                    }
                }
            }
            TopoCacheDomain::VertexHalfedges => {
                if impact.is_global() {
                    arena
                        .rebuild_vertex_halfedge_index()
                        .map_err(kernel_to_signal)?;
                } else {
                    let mut vertices = BTreeSet::new();
                    for target in &scoped_targets {
                        let TopoCacheTarget::Vertex(vertex) = target else {
                            continue;
                        };
                        if *vertex != VertexId::DANGLING {
                            vertices.insert(*vertex);
                        }
                    }
                    for vertex in vertices {
                        if arena.get_vertex(vertex).is_ok() {
                            arena
                                .rebuild_vertex_halfedges_for_vertex(vertex)
                                .map_err(kernel_to_signal)?;
                        } else {
                            arena.remove_vertex_halfedge_index_entry(vertex);
                        }
                    }
                }
            }
            TopoCacheDomain::ShellFaces => {
                if impact.is_global() {
                    arena.rebuild_shell_face_index().map_err(kernel_to_signal)?;
                } else {
                    let mut shells = BTreeSet::new();
                    for target in &scoped_targets {
                        let TopoCacheTarget::Shell(shell) = target else {
                            continue;
                        };
                        if *shell != ShellId::DANGLING {
                            shells.insert(*shell);
                        }
                    }
                    for shell in shells {
                        if arena.get_shell(shell).is_ok() {
                            arena
                                .rebuild_shell_faces_for_shell(shell)
                                .map_err(kernel_to_signal)?;
                        } else {
                            arena.remove_shell_face_index_entry(shell);
                        }
                    }
                }
            }
        }

        self.trace.push(CacheRefreshTraceEntry {
            barrier: self.barrier,
            domain,
            impact: impact_kind,
            targets: scoped_targets,
        });
        Ok(())
    }
}

fn kernel_to_signal(err: KernelError) -> SignalError {
    SignalError::internal(err.to_string())
}

fn signal_to_kernel(err: SignalError) -> KernelError {
    KernelError::InternalError {
        message: format!("signal runtime error: {err}"),
        context: None,
    }
}

/// Runtime cache coordinator state for `TopologyArena`.
#[derive(Debug, Clone)]
pub struct TopoCacheRuntime {
    runtime: CheckpointRuntime<TopoCacheDomain, TopoCacheTarget>,
    policy: TopoCachePolicy,
    telemetry: TopoCacheTelemetry,
    policy_violations: Vec<String>,
}

impl Default for TopoCacheRuntime {
    fn default() -> Self {
        let mut checkpoint_policy = CheckpointPolicy::new(CheckpointBarrier::PerOperation);
        checkpoint_policy.set_barrier(
            TopoCacheDomain::RadialValence,
            CheckpointBarrier::PerOperation,
        );
        checkpoint_policy.set_barrier(
            TopoCacheDomain::FaceHalfedges,
            CheckpointBarrier::PerOperation,
        );
        checkpoint_policy.set_barrier(
            TopoCacheDomain::VertexHalfedges,
            CheckpointBarrier::PerOperation,
        );
        checkpoint_policy.set_barrier(TopoCacheDomain::ShellFaces, CheckpointBarrier::PerOperation);

        Self {
            runtime: CheckpointRuntime::new(checkpoint_policy),
            policy: TopoCachePolicy::default(),
            telemetry: TopoCacheTelemetry::default(),
            policy_violations: Vec::new(),
        }
    }
}

impl TopoCacheRuntime {
    /// Snapshot of telemetry counters.
    pub fn telemetry(&self) -> &TopoCacheTelemetry {
        &self.telemetry
    }

    /// Configure strict cache policy.
    pub fn set_policy(&mut self, policy: TopoCachePolicy) {
        self.policy = policy;
    }

    /// Record a mutation effect into the deterministic dirty-state map.
    pub fn mark_effect(&mut self, effect: TopoCacheEffect) {
        match &effect {
            TopoCacheEffect::RadialLinksChanged { half_edges } => {
                self.telemetry
                    .inc_scoped(TopoCacheDomain::RadialValence, half_edges.len() as u64);
            }
            TopoCacheEffect::FaceHalfedgesChanged { faces } => {
                self.telemetry
                    .inc_scoped(TopoCacheDomain::FaceHalfedges, faces.len() as u64);
            }
            TopoCacheEffect::VertexHalfedgesChanged { vertices } => {
                self.telemetry
                    .inc_scoped(TopoCacheDomain::VertexHalfedges, vertices.len() as u64);
            }
            TopoCacheEffect::ShellFacesChanged { shells } => {
                self.telemetry
                    .inc_scoped(TopoCacheDomain::ShellFaces, shells.len() as u64);
            }
            TopoCacheEffect::GlobalInvalidate {
                domain,
                reason_code,
                site,
            } => {
                self.telemetry.inc_global(*domain);
                if !self.policy.allow_global_fallback {
                    self.policy_violations.push(format!(
                        "Global cache invalidation is disabled for {:?} (reason={}, site={})",
                        domain, reason_code, site
                    ));
                }
            }
        }

        self.runtime.record_effect::<TopoEffectMapping>(&effect);
    }

    fn enforce_policy_pre_flush(&self) -> Result<(), KernelError> {
        if let Some(first) = self.policy_violations.first() {
            return Err(KernelError::InternalError {
                message: first.clone(),
                context: None,
            });
        }
        Ok(())
    }

    fn enforce_policy_post_flush(&self, checkpoint: CheckpointBarrier) -> Result<(), KernelError> {
        if checkpoint == CheckpointBarrier::PerCommit {
            for (domain, observed) in &self.telemetry.global_invalidations_by_domain {
                let budget = self
                    .policy
                    .global_invalidation_budget
                    .get(domain)
                    .copied()
                    .unwrap_or(0);
                if *observed > budget {
                    return Err(KernelError::InternalError {
                        message: format!(
                            "Global invalidation budget exceeded for {:?}: observed {}, budget {}",
                            domain, observed, budget
                        ),
                        context: None,
                    });
                }
            }

            if self.policy.strict_fresh_at_commit
                && self.runtime.dirty().dirty_domains().next().is_some()
            {
                return Err(KernelError::InternalError {
                    message: "Cache policy violation: dirty domains remain at PerCommit checkpoint"
                        .to_string(),
                    context: None,
                });
            }
        }

        Ok(())
    }

    /// Apply refreshes scheduled for this checkpoint.
    pub fn apply_checkpoint(
        &mut self,
        arena: &mut TopologyArena,
        checkpoint: CheckpointBarrier,
    ) -> Result<Vec<CacheRefreshTraceEntry>, KernelError> {
        self.enforce_policy_pre_flush()?;

        let mut evaluator = TopoCacheEvaluator::new(checkpoint);
        self.runtime
            .flush(checkpoint, &mut evaluator, arena)
            .map_err(signal_to_kernel)
            .map(|_| ())?;

        for item in &evaluator.trace {
            self.telemetry.inc_flush(item.domain);
        }

        self.enforce_policy_post_flush(checkpoint)?;
        Ok(evaluator.trace)
    }

    /// Ensure one domain is fresh immediately (used for lazy read paths).
    pub fn ensure_fresh(
        &mut self,
        arena: &mut TopologyArena,
        domain: TopoCacheDomain,
    ) -> Result<(), KernelError> {
        let mut evaluator = TopoCacheEvaluator::new(CheckpointBarrier::OnDemandRead);
        self.runtime
            .ensure_fresh(domain, &mut evaluator, arena)
            .map_err(signal_to_kernel)
            .map(|_| ())?;

        for item in &evaluator.trace {
            self.telemetry.inc_flush(item.domain);
        }
        Ok(())
    }
}

impl TopologyArena {
    /// Mark a cache effect against this arena's runtime coordinator.
    pub(crate) fn mark_cache_effect(&mut self, effect: TopoCacheEffect) {
        self.cache_runtime.mark_effect(effect);
    }

    /// Snapshot cache telemetry.
    pub(crate) fn cache_telemetry(&self) -> &TopoCacheTelemetry {
        self.cache_runtime.telemetry()
    }

    /// Set strict policy controls for cache invalidation/refresh.
    pub(crate) fn set_cache_policy(&mut self, policy: TopoCachePolicy) {
        self.cache_runtime.set_policy(policy);
    }

    /// Apply cache refreshes due for the given checkpoint.
    pub(crate) fn apply_cache_checkpoint(
        &mut self,
        checkpoint: CheckpointBarrier,
    ) -> Result<Vec<CacheRefreshTraceEntry>, KernelError> {
        let mut runtime = std::mem::take(&mut self.cache_runtime);
        let result = runtime.apply_checkpoint(self, checkpoint);
        self.cache_runtime = runtime;
        result
    }

    /// Ensure one cache domain is fresh immediately.
    pub(crate) fn ensure_cache_domain_fresh(
        &mut self,
        domain: TopoCacheDomain,
    ) -> Result<(), KernelError> {
        let mut runtime = std::mem::take(&mut self.cache_runtime);
        let result = runtime.ensure_fresh(self, domain);
        self.cache_runtime = runtime;
        result
    }

    /// Choke-point setter for halfedge radial links.
    ///
    /// Emits scoped radial cache effects and invalidates local cached entries.
    pub fn set_half_edge_radial_next(
        &mut self,
        he: HalfEdgeId,
        next: HalfEdgeId,
    ) -> Result<(), KernelError> {
        let old = self.get_half_edge(he)?.radial_next();
        self.get_half_edge_mut(he)?.set_radial_next(next);

        self.invalidate_radial_cache_hint(he);
        self.invalidate_radial_cache_hint(old);
        self.invalidate_radial_cache_hint(next);

        let mut half_edges = smallvec![he];
        if old != HalfEdgeId::DANGLING {
            half_edges.push(old);
        }
        if next != HalfEdgeId::DANGLING {
            half_edges.push(next);
        }
        self.mark_cache_effect(TopoCacheEffect::RadialLinksChanged { half_edges });
        Ok(())
    }

    fn invalidate_radial_cache_hint(&mut self, he: HalfEdgeId) {
        if he == HalfEdgeId::DANGLING {
            return;
        }
        let idx = he.index() as usize;
        if idx < self.metadata.radial_valence.len() {
            self.metadata.radial_valence[idx] = 0;
        }
    }
}

fn encode_target(target: TopoCacheTarget) -> String {
    match target {
        TopoCacheTarget::HalfEdge(id) => format!("HE{}:{}", id.index(), id.generation()),
        TopoCacheTarget::Face(id) => format!("F{}:{}", id.index(), id.generation()),
        TopoCacheTarget::Vertex(id) => format!("V{}:{}", id.index(), id.generation()),
        TopoCacheTarget::Shell(id) => format!("S{}:{}", id.index(), id.generation()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity_lifecycle::make_vertex_face::MakeVertexFace;
    use crate::entity_lifecycle::split_edge::SplitEdge;
    use crate::transactions::TopologyState;

    #[test]
    fn global_invalidation_is_rejected_by_default_policy() {
        let mut arena = TopologyArena::new();
        arena.mark_cache_effect(TopoCacheEffect::GlobalInvalidate {
            domain: TopoCacheDomain::RadialValence,
            reason_code: "test_global",
            site: "cache_runtime::tests",
        });

        let err = arena
            .apply_cache_checkpoint(CheckpointBarrier::PerCommit)
            .unwrap_err();
        let msg = format!("{err}");
        assert!(
            msg.contains("Global cache invalidation is disabled"),
            "unexpected error: {msg}"
        );
    }

    #[test]
    fn scoped_impact_targets_are_flushed_in_canonical_order() {
        let mut arena = TopologyArena::new();
        let v2 = VertexId::new(2, 0);
        let v1 = VertexId::new(1, 0);
        arena.mark_cache_effect(TopoCacheEffect::VertexHalfedgesChanged {
            vertices: smallvec![v2, v1],
        });

        let trace = arena
            .apply_cache_checkpoint(CheckpointBarrier::PerOperation)
            .unwrap();
        assert_eq!(trace.len(), 1);
        assert_eq!(trace[0].domain, TopoCacheDomain::VertexHalfedges);
        assert_eq!(
            trace[0].targets,
            vec![TopoCacheTarget::Vertex(v1), TopoCacheTarget::Vertex(v2)]
        );
    }

    #[test]
    fn global_invalidation_budget_is_enforced_at_commit() {
        let mut arena = TopologyArena::new();
        let mut policy = TopoCachePolicy::default();
        policy.allow_global_fallback = true;
        policy
            .global_invalidation_budget
            .insert(TopoCacheDomain::FaceHalfedges, 0);
        arena.set_cache_policy(policy);

        arena.mark_cache_effect(TopoCacheEffect::GlobalInvalidate {
            domain: TopoCacheDomain::FaceHalfedges,
            reason_code: "integration_test",
            site: "cache_runtime::tests",
        });
        let err = arena
            .apply_cache_checkpoint(CheckpointBarrier::PerCommit)
            .unwrap_err();
        let msg = format!("{err}");
        assert!(
            msg.contains("Global invalidation budget exceeded"),
            "unexpected error: {msg}"
        );
    }

    #[test]
    fn strict_fresh_at_commit_rejects_remaining_dirty_domains() {
        let mut arena = TopologyArena::new();
        // Default policy is strict at commit.
        arena.mark_cache_effect(TopoCacheEffect::VertexHalfedgesChanged {
            vertices: smallvec![VertexId::new(7, 0)],
        });
        // VertexHalfedges refresh policy defaults to PerOperation, so PerCommit
        // does not flush it and strict freshness must reject the state.
        let err = arena
            .apply_cache_checkpoint(CheckpointBarrier::PerCommit)
            .unwrap_err();
        let msg = format!("{err}");
        assert!(
            msg.contains("dirty domains remain at PerCommit checkpoint"),
            "unexpected error: {msg}"
        );
    }

    fn permissive_policy() -> TopoCachePolicy {
        let mut policy = TopoCachePolicy::default();
        policy.allow_global_fallback = true;
        policy.strict_fresh_at_commit = false;
        policy
            .global_invalidation_budget
            .insert(TopoCacheDomain::RadialValence, u64::MAX);
        policy
            .global_invalidation_budget
            .insert(TopoCacheDomain::FaceHalfedges, u64::MAX);
        policy
            .global_invalidation_budget
            .insert(TopoCacheDomain::VertexHalfedges, u64::MAX);
        policy
            .global_invalidation_budget
            .insert(TopoCacheDomain::ShellFaces, u64::MAX);
        policy
    }

    fn sample_arena() -> TopologyArena {
        let mut draft = TopologyState::empty().into_mutation();
        let mvf = draft
            .execute(MakeVertexFace {
                shell_kind: crate::b_rep::ShellKind::Sheet,
            })
            .unwrap()
            .into_value();
        let _se = draft
            .execute(SplitEdge {
                edge: mvf.half_edge,
            })
            .unwrap()
            .into_value();
        draft.commit().unwrap().arena().clone()
    }

    #[test]
    fn radial_valence_targeted_matches_global_refresh() {
        let base = sample_arena();
        let start = base.iter_half_edges().next().unwrap().0;

        let mut targeted = base.clone();
        targeted.set_cache_policy(permissive_policy());
        targeted.mark_cache_effect(TopoCacheEffect::RadialLinksChanged {
            half_edges: smallvec![start],
        });
        targeted
            .apply_cache_checkpoint(CheckpointBarrier::PerOperation)
            .unwrap();

        let mut global = base.clone();
        global.set_cache_policy(permissive_policy());
        global.mark_cache_effect(TopoCacheEffect::GlobalInvalidate {
            domain: TopoCacheDomain::RadialValence,
            reason_code: "parity_test",
            site: "cache_runtime::tests",
        });
        global
            .apply_cache_checkpoint(CheckpointBarrier::PerOperation)
            .unwrap();

        assert_eq!(
            targeted.metadata.radial_valence,
            global.metadata.radial_valence
        );
    }

    #[test]
    fn face_halfedges_targeted_matches_global_refresh() {
        let base = sample_arena();
        let face = base.iter_faces().next().unwrap().0;

        let mut targeted = base.clone();
        targeted.set_cache_policy(permissive_policy());
        targeted.mark_cache_effect(TopoCacheEffect::FaceHalfedgesChanged {
            faces: smallvec![face],
        });
        targeted
            .apply_cache_checkpoint(CheckpointBarrier::PerOperation)
            .unwrap();

        let mut global = base.clone();
        global.set_cache_policy(permissive_policy());
        global.mark_cache_effect(TopoCacheEffect::GlobalInvalidate {
            domain: TopoCacheDomain::FaceHalfedges,
            reason_code: "parity_test",
            site: "cache_runtime::tests",
        });
        global
            .apply_cache_checkpoint(CheckpointBarrier::PerOperation)
            .unwrap();

        assert_eq!(
            targeted.indexes.face_halfedges,
            global.indexes.face_halfedges
        );
    }

    #[test]
    fn vertex_halfedges_targeted_matches_global_refresh() {
        let base = sample_arena();
        let vertex = base.iter_vertices().next().unwrap().0;

        let mut targeted = base.clone();
        targeted.set_cache_policy(permissive_policy());
        targeted.mark_cache_effect(TopoCacheEffect::VertexHalfedgesChanged {
            vertices: smallvec![vertex],
        });
        targeted
            .apply_cache_checkpoint(CheckpointBarrier::PerOperation)
            .unwrap();

        let mut global = base.clone();
        global.set_cache_policy(permissive_policy());
        global.mark_cache_effect(TopoCacheEffect::GlobalInvalidate {
            domain: TopoCacheDomain::VertexHalfedges,
            reason_code: "parity_test",
            site: "cache_runtime::tests",
        });
        global
            .apply_cache_checkpoint(CheckpointBarrier::PerOperation)
            .unwrap();

        assert_eq!(
            targeted.indexes.vertex_halfedges,
            global.indexes.vertex_halfedges
        );
    }

    #[test]
    fn shell_faces_targeted_matches_global_refresh() {
        let base = sample_arena();
        let shell = base.iter_shells().next().unwrap().0;

        let mut targeted = base.clone();
        targeted.set_cache_policy(permissive_policy());
        targeted.mark_cache_effect(TopoCacheEffect::ShellFacesChanged {
            shells: smallvec![shell],
        });
        targeted
            .apply_cache_checkpoint(CheckpointBarrier::PerOperation)
            .unwrap();

        let mut global = base.clone();
        global.set_cache_policy(permissive_policy());
        global.mark_cache_effect(TopoCacheEffect::GlobalInvalidate {
            domain: TopoCacheDomain::ShellFaces,
            reason_code: "parity_test",
            site: "cache_runtime::tests",
        });
        global
            .apply_cache_checkpoint(CheckpointBarrier::PerOperation)
            .unwrap();

        assert_eq!(targeted.indexes.shell_faces, global.indexes.shell_faces);
    }

    #[test]
    fn on_demand_fresh_accessors_flush_pending_domains() {
        let mut arena = sample_arena();
        let face = arena.iter_faces().next().unwrap().0;
        let vertex = arena.iter_vertices().next().unwrap().0;
        let shell = arena.iter_shells().next().unwrap().0;

        arena.mark_cache_effect(TopoCacheEffect::FaceHalfedgesChanged {
            faces: smallvec![face],
        });
        arena.mark_cache_effect(TopoCacheEffect::VertexHalfedgesChanged {
            vertices: smallvec![vertex],
        });
        arena.mark_cache_effect(TopoCacheEffect::ShellFacesChanged {
            shells: smallvec![shell],
        });

        let _ = arena.halfedges_of_face_fresh(face).unwrap();
        let _ = arena.halfedges_from_vertex_fresh(vertex).unwrap();
        let _ = arena.faces_of_shell_fresh(shell).unwrap();

        assert!(
            arena
                .cache_runtime
                .runtime
                .dirty()
                .dirty_domains()
                .next()
                .is_none(),
            "on-demand fresh reads should drain pending dirty domains"
        );
    }
}
