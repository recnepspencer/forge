True Delta Debugger — Engineering Specification
Problem Statement
When a 150-step Menger chain fails with MissingTwin at step 130, the developer currently has no automated way to:

Isolate in time: Find the minimal subset of operations that reproduces the fault (not just binary search for a prefix).
Isolate in space: Extract the minimal submesh around the defect site.
Identify the root cause: Surface the marginal decisions that could flip.
Propose a fix: Sweep tolerance parameters to suggest policy tuning.
The existing 
delta_debug
 in forge-core is a prefix binary search (finds the earliest failing step). The true Zeller ddmin finds the 1-minimal subset — a fundamentally harder and more powerful question.

Existing Foundation (What We Have)
Primitive	Location	Status
Prefix binary search	
delta_debug.rs
✅ Working, PV-36 passes
N-ring BFS extractor	
eval.rs
✅ Working, PV-35/35b pass
ExtractedRegion
 with JSON + to_arena()	
schema.rs
✅ Full serialize/reconstruct
CounterfactualResult
 + 
DecisionOverride
schema.rs
✅ Schema complete
execute_boolean_with_overrides
eval.rs
✅ Wired to pipeline
CausalChain
 / 
CausalStep
 / 
ChainSummary
schema.rs
✅ Schema complete
BooleanResult
 w/ 
ReplayLog
 + LineageEvent	
schema.rs
✅ Full provenance
TopologyState
 (immutable, Arc<TopologyArena>)	
state.rs
✅ O(1) clone via Arc
Structural hashing	
hashing.rs
✅ Deterministic
Architecture: 5 Layers, No New Crates
The debugger is a thin query/orchestration layer over existing primitives, placed in forge-kernel/src/analysis/diagnostic/.

Layer 1: Snapshot-Memoized Chain Runner
Layer 2: True ddmin Engine
Layer 3: Marginal Decision Indexer
Layer 4: Spatial Isolation (N-ring)
Layer 5: Counterfactual Sweep + Fix Suggestion
Output: Auto-generated #[test]
Proposed Changes
Layer 1 — Snapshot-Memoized Chain Runner
[NEW] 
chain_runner.rs
Executes a sequence of 
BooleanInput
 against a base solid, maintaining Arc<TopologyState> snapshots at every step for O(1) fork-and-replay.

rust
pub struct ChainSnapshot {
    step: usize,
    topology: TopologyState,   // Arc-backed, O(1) clone
    geometry: GeometryStore,
}
pub struct ChainRunner {
    base_solid: (TopologyState, GeometryStore),
    steps: Vec<BooleanStep>,
    snapshots: Vec<ChainSnapshot>,  // populated during first run
}
pub struct BooleanStep {
    tool_topology: TopologyState,
    tool_geometry: GeometryStore,
    operation: BooleanOp,
}
impl ChainRunner {
    /// Run the full chain, capturing O(1) snapshots at every step.
    pub fn run_full(&mut self) -> Result<ChainRunResult, KernelError>;
    /// Run a SUBSET of steps (by index) from the base solid.
    /// Uses memoized snapshot of the highest valid prefix.
    pub fn run_subset(&self, indices: &[usize]) -> OracleVerdict;
    /// Fork from step N and run a single step forward.
    pub fn fork_at(&self, step: usize) -> Result<(TopologyState, GeometryStore), KernelError>;
}
IMPORTANT

Key insight: 
TopologyState
 already uses Arc<TopologyArena> internally. Storing 500 snapshots costs only 500 Arc clones (~500 pointer increments), NOT 500 full arena copies. This makes the entire memoization strategy nearly free.

Layer 2 — True ddmin Subset Minimizer
[NEW] 
ddmin.rs
Implements Zeller's ddmin algorithm, parameterized over an oracle function.

rust
pub enum OracleVerdict {
    Reproduced,   // Oracle hit the EXACT target error signature
    FailedOther,  // Oracle hit a different error
    Passed,       // No error
}
pub struct ErrorSignature {
    error_kind: String,        // e.g. "MissingTwin", "TopologyViolation"
    error_message_prefix: String,
}
pub struct DdminResult {
    minimal_subset: Vec<usize>,  // 1-minimal operation indices
    probes_used: usize,
    original_count: usize,
    reduction_ratio: f64,
}
/// Zeller's ddmin: find a 1-minimal subset of `indices` that
/// reproduces `target_error` according to `oracle`.
pub fn ddmin<F>(
    indices: &[usize],
    oracle: F,
    target_error: &ErrorSignature,
) -> Result<DdminResult, KernelError>
where
    F: FnMut(&[usize]) -> OracleVerdict;
Algorithm (Zeller, 1999):

Start with full set C, granularity n = 2.
Partition C into n chunks Δ₁..Δₙ.
For each chunk Δᵢ: if oracle(Δᵢ) == Reproduced, recurse on Δᵢ with n=2.
For each complement ∇ᵢ = C \ Δᵢ: if oracle(∇ᵢ) == Reproduced, recurse on ∇ᵢ with n = max(n-1, 2).
If neither works, double granularity: n = min(2n, |C|).
Terminate when n > |C| (1-minimal).
Layer 3 — Marginal Decision Indexer
[NEW] 
marginal.rs
Scans a DecisionLog and extracts the decisions whose margin falls within a configurable threshold of zero. These are the ONLY decisions that could possibly have gone wrong.

rust
pub struct MarginalDecision {
    decision: TracedDecision,
    margin: f64,
    margin_ratio: f64,  // margin / tolerance_threshold
    step_index: usize,
}
pub struct MarginalIndex {
    decisions: Vec<MarginalDecision>,
    total_decisions_scanned: usize,
}
/// Extract all decisions from `log` whose margin is within
/// `threshold_multiplier × tolerance` of the boundary.
pub fn index_marginal_decisions(
    log: &DecisionLog,
    threshold_multiplier: f64,  // e.g. 10.0 = within 10× of tolerance
) -> MarginalIndex;
Layer 4 — Spatial Isolation (Existing)
The existing 
extract_n_ring
 is already complete. No changes needed.

Layer 5 — Counterfactual Sweep + Fix Suggestion
[NEW] 
tolerance_sweep.rs
Iterates over marginal decisions from Layer 3, tests counterfactual outcomes, and proposes specific tolerance tuning.

rust
pub struct FixSuggestion {
    parameter_name: String,         // e.g. "coincidence_tolerance"
    current_value: f64,
    suggested_value: f64,
    decisions_resolved: usize,
    counterfactual_valid: bool,
    side_effects: String,           // e.g. "0 other test cases affected"
}
pub struct SweepResult {
    marginal_count: usize,
    suggestions: Vec<FixSuggestion>,
    summary: String,
}
/// Sweep all marginal decisions: for each, run a counterfactual replay
/// with the decision flipped. If the counterfactual passes validation,
/// produce a FixSuggestion.
pub fn sweep_marginal_decisions(
    runner: &ChainRunner,
    failing_step: usize,
    marginals: &MarginalIndex,
) -> Result<SweepResult, KernelError>;
Orchestrator — The diagnose Entry Point
[NEW] 
mod.rs
rust
pub struct DiagnosticReport {
    /// Which step first fails (prefix bisect, O(log n))
    pub failing_step: DeltaDebugResult,
    /// The 1-minimal subset of ops (ddmin, O(n log n) worst case)
    pub minimal_subset: Option<DdminResult>,
    /// The spatial region around the defect
    pub region: Option<ExtractedRegion>,
    /// Marginal decisions at the failing step
    pub marginals: MarginalIndex,
    /// Fix suggestions from counterfactual sweep
    pub suggestions: SweepResult,
    /// Auto-generated test code (Rust source)
    pub generated_test: String,
}
/// One-call diagnostic: runs the full pipeline.
pub fn diagnose_chain(
    base_solid: (TopologyState, GeometryStore),
    steps: Vec<BooleanStep>,
    target_error: ErrorSignature,
) -> Result<DiagnosticReport, KernelError>;
Acceptance Criteria
AC-1: ddmin Isolation Correctness
Given a 150-step chain where only steps {23, 47, 130} together cause MissingTwin, ddmin must return exactly those 3 indices (or a subset thereof that still reproduces).

AC-2: ddmin Probe Bound
For a chain of length N, ddmin must terminate in ≤ N × log₂(N) oracle calls. For N=150, that's ≤ 1,088 probes.

AC-3: Snapshot Memoization
Running fork_at(step) must NOT re-execute steps 0..step. It must clone the Arc in O(1) and only execute the single requested step.

AC-4: Marginal Decision Precision
Given a DecisionLog with 50,000 decisions where exactly 7 have margin < 10× tolerance, index_marginal_decisions must return exactly those 7 decisions, in margin-ascending order.

AC-5: Counterfactual Validity
When a marginal decision is flipped and the counterfactual passes topological validation, the FixSuggestion.counterfactual_valid must be true. When the counterfactual breaks topology, it must be false.

AC-6: Region Extraction (Already Passing)
PV-35, PV-35b remain green. No regressions.

AC-7: Generated Test Compilability
The generated_test string in DiagnosticReport must be valid Rust source that compiles as a standalone #[test] function when placed in a file with standard forge imports.

Test Plan
Unit Tests (in forge-kernel/src/analysis/diagnostic/tests.rs)
All tests can be run via:

bash
cargo test -p forge-kernel diagnostic --lib
Test	Description
ddmin_finds_single_critical_op	20-step chain, only step 7 is critical → returns [7]
ddmin_finds_multi_critical_ops	50-step chain, steps {5, 20, 45} together cause fault → returns 3-element subset
ddmin_respects_probe_bound	200-step chain → asserts probes_used ≤ 200 * 8
ddmin_handles_first_step_failure	Steps {0} alone causes fault → returns [0]
ddmin_handles_all_steps_required	All steps required → returns full set, terminates
snapshot_fork_is_o1	Run 100-step chain, fork_at(50) completes in < 1ms
marginal_index_filters_correctly	Inject 7 near-boundary decisions among 1000 → exactly 7 returned
marginal_index_sorts_by_margin	Returned decisions are in ascending margin order
sweep_produces_valid_suggestion	Chain with 1 marginal decision, flip heals it → suggestion.counterfactual_valid == true
sweep_rejects_bad_flip	Chain with 1 marginal decision, flip breaks it → suggestion.counterfactual_valid == false
generated_test_compiles	Generate test string, write to temp file, cargo check it
Integration Tests (in forge-kernel tests)
bash
cargo test -p forge-kernel -- diagnostic_integration
Test	Description
diagnose_menger_chain	10-cube Menger chain union with injected corruption at step 7 → full DiagnosticReport produced, failing_step == 7
diagnose_coplanar_boundary	5-step chain with coplanar face at step 3 → marginal decisions surfaced
Existing Tests (Must Remain Green)
bash
cargo test -p forge-kernel pv_p3_2
cargo test -p forge-core delta_debug
These ensure no regressions to: PV-35, PV-35b, PV-36, and the core 
delta_debug
 bisection.

Unknowns & Research Required
1. Oracle Semantics for Subset Execution (CRITICAL)
WARNING

The hardest open question. Zeller's ddmin tests subsets of operations. But Boolean operations are not generally commutative — the result of running steps {5, 20, 45} depends on the ORDER and the intermediate state between them.

Current hypothesis: Run the subset in original order against the base solid. Step 5 runs first against base, producing state S5. Step 20 runs against S5, producing S20. Step 45 runs against S20. This preserves operation ordering within the subset.

Risk: Some failures may be "setup-dependent" — step 45 only fails because step 30 (not in the subset) created a face configuration that step 45 then breaks. If step 30 is removed, step 45 may pass trivially without encountering the problematic geometry at all, making ddmin return a false positive (subset that "passes" when it should fail).

Mitigation: Run the ddmin with TWO oracle modes:

Strict: Subset must produce the exact same ErrorSignature (same error variant + same entity index).
Relaxed: Subset must produce any error of the same variant.
Research needed: Empirical testing on real MB chain failures to determine which mode yields more useful results.

2. Arc Clone Cost at Scale
TopologyState
 cloning is O(1) via Arc, but at 500+ steps, we are holding 500 live Arc references to arenas that may each contain 50,000+ entities. This could cause significant memory pressure.

Hypothesis: For a 6-face cube that grows to ~600 faces over 100 unions, each arena is ~50 KB. 500 snapshots = ~25 MB. Acceptable.

Research needed: Benchmark actual memory usage on realistic chains. If memory exceeds 500 MB for 500-step chains, implement LRU eviction with re-computation on miss.

3. Chain Runner Geometry Store Copying
GeometryStore is NOT Arc-backed — it's a plain struct with BTreeMap internals. Cloning it at every step is O(N) where N = number of faces. For large chains this could dominate runtime.

Mitigation options (in priority order):

Make GeometryStore use Arc<BTreeMap<...>> internally (structural sharing) — cleanest
Only snapshot 
TopologyState
 and re-derive geometry on demand from the step's 
BooleanResult
Accept O(N) clone cost and benchmark to ensure it's within budget
Research needed: Profile GeometryStore::clone() cost on a 500-face solid. If < 1ms, option 3 is fine.

4. ddmin Termination on Non-Monotonic Faults
Zeller's ddmin assumes a somewhat monotonic failure property: if subset S fails, then any superset of S also fails. Boolean chains may violate this — adding an operation might HEAL a failure by creating geometry that compensates for earlier corruption.

Mitigation: If ddmin detects non-monotonicity (a superset passes where a subset failed), log a NonMonotonicWarning and fall back to the simpler prefix binary search result from the existing 
delta_debug
.

Research needed: Determine how frequently real kernel failures are non-monotonic.

5. Generated Test Code Quality
Auto-generating valid Rust #[test] functions requires:

Correct imports for all used types
Serializing GeometryStore positions and planes as literal arrays
Correct make_cube calls or inline solid construction
Risk: Generated code may not cover all edge cases (e.g., solids with non-standard construction).

Mitigation: Start with a template-based approach using make_cube (the most common test primitive) and expand as needed. Include a fallback that serializes the minimal state to JSON and loads it via 
from_json
 rather than inlining geometry.

File Manifest
Action	File	LOC (est.)
[NEW]	forge-kernel/src/analysis/diagnostic/mod.rs	~50
[NEW]	forge-kernel/src/analysis/diagnostic/chain_runner.rs	~150
[NEW]	forge-kernel/src/analysis/diagnostic/ddmin.rs	~120
[NEW]	forge-kernel/src/analysis/diagnostic/marginal.rs	~80
[NEW]	forge-kernel/src/analysis/diagnostic/tolerance_sweep.rs	~100
[NEW]	forge-kernel/src/analysis/diagnostic/test_gen.rs	~60
[NEW]	forge-kernel/src/analysis/diagnostic/tests.rs	~200
[MODIFY]	
forge-kernel/src/analysis/mod.rs
+2 lines
Total		~762