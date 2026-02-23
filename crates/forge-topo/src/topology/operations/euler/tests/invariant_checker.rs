//! Automated topology invariant checker.
//!
//! Runs structural invariants after every operator and reports the FIRST
//! operator that breaks topology. This is diagnostic tooling, not production code.
//!
//! ## Usage
//!
//! ```ignore
//! let report = diagnose_op_chain(|runner| {
//!     let mvf = runner.run("MVF", |d| apply_op(d, MakeVertexFace).map(|r| r.into_value()))?;
//!     runner.run("SE1", |d| apply_op(d, SplitEdge { edge: mvf.half_edge, parameter: 0.5 }).map(|r| r.into_value()))?;
//!     Ok(())
//! });
//! println!("{}", report.summary());
//! ```

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use crate::arena::TopologyArena;
use crate::handles::{HalfEdgeId, FaceId};
use crate::state::{TopologyState, MutableDraft};
use forge_core::KernelError;

/// A violation found by the invariant checker.
#[derive(Debug)]
pub struct Violation {
    pub invariant: &'static str,
    pub detail: String,
}

/// Result of checking all invariants on an arena.
#[derive(Debug)]
pub struct CheckResult {
    pub violations: Vec<Violation>,
}

impl CheckResult {
    pub fn is_ok(&self) -> bool { self.violations.is_empty() }

    pub fn summary(&self) -> String {
        if self.violations.is_empty() {
            return "All invariants hold.".to_string();
        }
        let mut s = format!("{} violations:\n", self.violations.len());
        for (i, v) in self.violations.iter().enumerate() {
            s.push_str(&format!("  [{}] {}: {}\n", i + 1, v.invariant, v.detail));
        }
        s
    }
}

/// Snapshot of arena counts at a given step.
#[derive(Debug, Clone)]
pub struct ArenaSnapshot {
    pub vertices: usize,
    pub half_edges: usize,
    pub faces: usize,
    pub edges: usize,
    pub loops: usize,
}

impl ArenaSnapshot {
    pub fn capture(arena: &TopologyArena) -> Self {
        Self {
            vertices: arena.vertex_count(),
            half_edges: arena.half_edge_count(),
            faces: arena.face_count(),
            edges: arena.edge_count(),
            loops: arena.loop_count(),
        }
    }
}

impl std::fmt::Display for ArenaSnapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "V={} HE={} F={} E={} L={}", self.vertices, self.half_edges, self.faces, self.edges, self.loops)
    }
}

/// Per-step diagnosis record.
#[derive(Debug)]
pub struct StepRecord {
    pub name: String,
    pub before: ArenaSnapshot,
    pub after: ArenaSnapshot,
    pub check: CheckResult,
}

/// Full diagnosis report from a chain of operations.
pub struct DiagnosisReport {
    pub steps: Vec<StepRecord>,
    pub wiring_dump_at_failure: Option<String>,
}

impl DiagnosisReport {
    pub fn is_clean(&self) -> bool {
        self.steps.iter().all(|s| s.check.is_ok())
    }

    pub fn first_failure(&self) -> Option<&StepRecord> {
        self.steps.iter().find(|s| !s.check.is_ok())
    }

    pub fn summary(&self) -> String {
        let mut s = String::new();
        for (i, step) in self.steps.iter().enumerate() {
            let status = if step.check.is_ok() { "✓" } else { "✗" };
            s.push_str(&format!(
                "  [{}] {} {}: {} → {}\n",
                status, i, step.name, step.before, step.after
            ));
            if !step.check.is_ok() {
                s.push_str(&step.check.summary());
            }
        }
        if let Some(dump) = &self.wiring_dump_at_failure {
            s.push_str("\n--- Wiring dump at first failure ---\n");
            s.push_str(dump);
        }
        s
    }
}

/// Runner passed to the closure in `diagnose_op_chain`.
///
/// Tracks before/after snapshots and checks invariants after each op.
pub struct DiagnosisRunner<'a> {
    draft: &'a mut MutableDraft,
    steps: Vec<StepRecord>,
    failed: bool,
}

impl<'a> DiagnosisRunner<'a> {
    /// Execute a named operation and check invariants afterward.
    ///
    /// Returns the operation's output, or the first error encountered.
    /// After a failure, subsequent `run` calls are skipped.
    pub fn run<F, T>(&mut self, name: &str, op_fn: F) -> Result<T, KernelError>
    where
        F: FnOnce(&mut MutableDraft) -> Result<T, KernelError>,
    {
        if self.failed {
            return Err(KernelError::InternalError {
                message: format!("Skipped '{}': previous step already failed", name),
                context: None,
            });
        }

        let before = ArenaSnapshot::capture(self.draft.arena());
        let result = op_fn(self.draft);

        let after = ArenaSnapshot::capture(self.draft.arena());
        let check = check_all_invariants(self.draft.arena());

        if !check.is_ok() {
            self.failed = true;
        }

        self.steps.push(StepRecord {
            name: name.to_string(),
            before,
            after,
            check,
        });

        result
    }

    /// Get a read-only reference to the draft's arena.
    pub fn arena(&self) -> &TopologyArena {
        self.draft.arena()
    }

    /// Get a mutable reference to the draft (for manual wiring in tests).
    pub fn draft_mut(&mut self) -> &mut MutableDraft {
        self.draft
    }
}

/// Run a sequence of operations with full invariant checking after each step.
///
/// The closure receives a `DiagnosisRunner` which wraps each operation
/// with before/after snapshots and structural invariant checks.
/// Returns a `DiagnosisReport` with a step-by-step summary and
/// wiring dump at the first failure point.
pub fn diagnose_op_chain<F>(builder: F) -> DiagnosisReport
where
    F: FnOnce(&mut DiagnosisRunner<'_>) -> Result<(), KernelError>,
{
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let mut runner = DiagnosisRunner {
        draft: &mut draft,
        steps: Vec::new(),
        failed: false,
    };

    let _ = builder(&mut runner);

    let wiring_dump = if runner.steps.iter().any(|s| !s.check.is_ok()) {
        Some(dump_all_wiring(runner.draft.arena()))
    } else {
        None
    };

    DiagnosisReport {
        steps: runner.steps,
        wiring_dump_at_failure: wiring_dump,
    }
}

/// Dump complete halfedge wiring state for diagnosis.
pub fn dump_all_wiring(arena: &TopologyArena) -> String {
    let mut s = String::new();
    s.push_str(&format!(
        "Arena: V={} HE={} F={} E={} L={}\n",
        arena.vertex_count(), arena.half_edge_count(),
        arena.face_count(), arena.edge_count(), arena.loop_count()
    ));
    for (id, data) in arena.iter_half_edges() {
        let next_origin = arena.get_half_edge(data.next())
            .map(|n| format!("V{}", n.origin().index()))
            .unwrap_or_else(|_| "INVALID".to_string());
        s.push_str(&format!(
            "  HE[{}]: V{}→{} next=HE{} prev=HE{} radial=HE{} face=F{} edge=E{}\n",
            id.index(), data.origin().index(), next_origin,
            data.next().index(), data.prev().index(),
            data.radial_next().index(), data.face().index(), data.edge().index(),
        ));
    }
    for (id, data) in arena.iter_vertices() {
        s.push_str(&format!("  V[{}]: outgoing=HE{}\n", id.index(), data.outgoing().index()));
    }
    for (id, data) in arena.iter_edges() {
        s.push_str(&format!("  E[{}]: half_edge=HE{}\n", id.index(), data.half_edge().index()));
    }
    for (face_id, _) in arena.iter_faces() {
        let edge_count = crate::topology::queries::traverse::FaceEdgeIterator::new(arena, face_id)
            .map(|iter| iter.count())
            .unwrap_or(0);
        s.push_str(&format!("  F[{}]: {} edges in loop\n", face_id.index(), edge_count));
    }
    s
}

/// Check all structural invariants on an arena. Returns all violations found.
pub fn check_all_invariants(arena: &TopologyArena) -> CheckResult {
    let mut violations = Vec::new();
    check_edge_vertex_consistency(arena, &mut violations);
    check_loop_closure(arena, &mut violations);
    check_prev_consistency(arena, &mut violations);
    check_radial_closure(arena, &mut violations);
    check_origin_edge_agreement(arena, &mut violations);
    CheckResult { violations }
}

/// INV-1: Every EdgeId must have exactly 1 or 2 distinct endpoint vertices.
fn check_edge_vertex_consistency(arena: &TopologyArena, violations: &mut Vec<Violation>) {
    let mut edge_verts: BTreeMap<u32, BTreeSet<u32>> = BTreeMap::new();
    let mut edge_halfedges: BTreeMap<u32, Vec<u32>> = BTreeMap::new();

    for (he_id, he_data) in arena.iter_half_edges() {
        let edge_idx = he_data.edge().index();
        let origin = he_data.origin().index();
        let target = match arena.get_half_edge(he_data.next()) {
            Ok(next) => next.origin().index(),
            Err(_) => {
                violations.push(Violation {
                    invariant: "EdgeVertexConsistency",
                    detail: format!("he[{}].next is invalid", he_id.index()),
                });
                continue;
            }
        };
        edge_verts.entry(edge_idx).or_default().insert(origin);
        edge_verts.entry(edge_idx).or_default().insert(target);
        edge_halfedges.entry(edge_idx).or_default().push(he_id.index());
    }

    for (edge_idx, verts) in &edge_verts {
        if verts.len() > 2 {
            let hes = edge_halfedges.get(edge_idx).unwrap();
            let mut he_details = Vec::new();
            for &he_idx in hes {
                for (he_id, he_data) in arena.iter_half_edges() {
                    if he_id.index() == he_idx {
                        let next_origin = arena.get_half_edge(he_data.next())
                            .map(|n| n.origin().index())
                            .unwrap_or(u32::MAX);
                        he_details.push(format!(
                            "he[{}](V{}→V{}, E{}, F{}, radial=he[{}])",
                            he_idx, he_data.origin().index(), next_origin,
                            he_data.edge().index(), he_data.face().index(),
                            he_data.radial_next().index()
                        ));
                        break;
                    }
                }
            }
            violations.push(Violation {
                invariant: "EdgeVertexConsistency",
                detail: format!(
                    "E{} has {} vertices {:?}\n      halfedges: {}",
                    edge_idx, verts.len(), verts, he_details.join(", ")
                ),
            });
        }
    }
}

/// INV-2: Every face loop must close (walking next returns to start).
fn check_loop_closure(arena: &TopologyArena, violations: &mut Vec<Violation>) {
    for (face_id, face_data) in arena.iter_faces() {
        let loop_data = match arena.get_loop(face_data.outer_loop()) {
            Ok(l) => l,
            Err(_) => {
                violations.push(Violation {
                    invariant: "LoopClosure",
                    detail: format!("F{} outer loop is invalid", face_id.index()),
                });
                continue;
            }
        };
        let start = loop_data.half_edge();
        let mut cur = start;
        let bound = arena.half_edge_count() + 1;
        for step in 0..bound {
            let he = match arena.get_half_edge(cur) {
                Ok(h) => h,
                Err(_) => {
                    violations.push(Violation {
                        invariant: "LoopClosure",
                        detail: format!("F{}: he[{}] is invalid at step {}", face_id.index(), cur.index(), step),
                    });
                    break;
                }
            };
            if he.face() != face_id {
                violations.push(Violation {
                    invariant: "LoopClosure",
                    detail: format!("F{}: he[{}] belongs to F{}", face_id.index(), cur.index(), he.face().index()),
                });
                break;
            }
            cur = he.next();
            if cur == start { break; }
            if step == bound - 1 {
                violations.push(Violation {
                    invariant: "LoopClosure",
                    detail: format!("F{}: loop doesn't close after {} steps", face_id.index(), bound),
                });
            }
        }
    }
}

/// INV-3: For every halfedge, he.prev.next == he AND he.next.prev == he.
fn check_prev_consistency(arena: &TopologyArena, violations: &mut Vec<Violation>) {
    for (he_id, he_data) in arena.iter_half_edges() {
        if let Ok(prev) = arena.get_half_edge(he_data.prev()) {
            if prev.next() != he_id {
                violations.push(Violation {
                    invariant: "PrevConsistency",
                    detail: format!("he[{}].prev.next = he[{}] (expected he[{}])", 
                        he_id.index(), prev.next().index(), he_id.index()),
                });
            }
        }
        if let Ok(next) = arena.get_half_edge(he_data.next()) {
            if next.prev() != he_id {
                violations.push(Violation {
                    invariant: "PrevConsistency",
                    detail: format!("he[{}].next.prev = he[{}] (expected he[{}])", 
                        he_id.index(), next.prev().index(), he_id.index()),
                });
            }
        }
    }
}

/// INV-4: Every radial ring must close (walking radial_next returns to start).
fn check_radial_closure(arena: &TopologyArena, violations: &mut Vec<Violation>) {
    let mut visited: BTreeSet<u32> = BTreeSet::new();
    for (he_id, _) in arena.iter_half_edges() {
        if visited.contains(&he_id.index()) { continue; }
        let mut cur = he_id;
        let bound = arena.half_edge_count() + 1;
        for step in 0..bound {
            visited.insert(cur.index());
            let he = match arena.get_half_edge(cur) {
                Ok(h) => h,
                Err(_) => {
                    violations.push(Violation {
                        invariant: "RadialClosure",
                        detail: format!("he[{}] radial ring: he[{}] invalid at step {}", he_id.index(), cur.index(), step),
                    });
                    break;
                }
            };
            cur = he.radial_next();
            if cur == he_id { break; }
            if step == bound - 1 {
                violations.push(Violation {
                    invariant: "RadialClosure",
                    detail: format!("he[{}] radial ring doesn't close after {} steps", he_id.index(), bound),
                });
            }
        }
    }
}

/// INV-5: All halfedges sharing an EdgeId must have origins drawn from ≤2 vertices.
fn check_origin_edge_agreement(arena: &TopologyArena, violations: &mut Vec<Violation>) {
    let mut edge_origins: BTreeMap<u32, BTreeSet<u32>> = BTreeMap::new();
    for (_he_id, he_data) in arena.iter_half_edges() {
        let edge_idx = he_data.edge().index();
        edge_origins.entry(edge_idx).or_default().insert(he_data.origin().index());
    }
    for (edge_idx, origins) in &edge_origins {
        if origins.len() > 2 {
            violations.push(Violation {
                invariant: "OriginEdgeAgreement",
                detail: format!("E{} has halfedges originating from {} vertices: {:?}", edge_idx, origins.len(), origins),
            });
        }
    }
}

/// Run a sequence of named operations, checking invariants after each one.
/// Returns the name of the first operation that breaks invariants, along with the violations.
pub fn find_first_breaking_op<F>(ops: &[(&str, F)]) -> Option<(String, CheckResult)>
where F: Fn() -> CheckResult {
    for (name, check_fn) in ops {
        let result = check_fn();
        if !result.is_ok() {
            return Some((name.to_string(), result));
        }
    }
    None
}
