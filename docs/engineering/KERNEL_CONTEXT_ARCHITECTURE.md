# Kernel Context Architecture

**Status:** Proposal — requires review before implementation.

> Unifies configuration management and decision logging into two clean
> abstractions: a declarative **KernelConfig** (read-only, cascade-resolved)
> and a scope-based **KernelSpan** (write-only, implicit). Together they
> eliminate the `&mut ModelingContext` threading burden (~50 call sites)
> and the 6 scattered config struct problem.

---

## 1. Problem Statement

### 1.1 Scattered Configuration

Six config/policy structs are spread across `forge-kernel`:

| Struct                      | Fields | Controls                                           |
| --------------------------- | ------ | -------------------------------------------------- |
| `TolerancePolicy`           | 2      | spatial + angular coincidence                      |
| `TangencyPolicy`            | 2      | near-tangent detection                             |
| `SliverPolicy`              | 2      | min face area, max slivers                         |
| `GapClosurePolicy`          | 1      | max auto-closure gap                               |
| `PrecisionEscalationPolicy` | 1      | bit-length threshold                               |
| `ToleranceConfig`           | 13     | residual, degeneracy, AABB, ambiguity, etc.        |
| `ValidationConfig`          | 3      | active checkpoints, geometric checks, entity limit |

These are dumped into `ModelingContext` as 7 separate fields with no shared
structure, no per-object override support, and no contract integration.

### 1.2 Parameter Threading Burden

`&mut ModelingContext` appears in 50+ function signatures in `operations/` alone.
But the mutation is almost exclusively **write-only decision logging** (~40/50 sites).
Only ~6 sites actually read config. The `&mut` threading tax is paid by every function
just so a few can log decisions.

### 1.3 No Per-Object Overrides

`ToleranceConfig` is globally flat. There's no mechanism for:

- Imported STEP bodies with 0.1mm tolerance alongside native bodies at 1μm
- Features that need tighter ambiguity bands for aerospace work
- Operations that temporarily relax sliver policy for chamfers

The `PolicyKind` boolean override hierarchy (session → model → feature → operation)
exists but numeric thresholds don't participate.

---

## 2. Architecture: Two Clean Abstractions

```
┌───────────────────────────────────────────────────┐
│  Pipeline Executor                                │
│                                                   │
│  1. Resolve config cascade → ResolvedConfig       │
│  2. Enter KernelSpan                              │
│  3. Execute feature/operation                     │
│  4. Exit span → collect DecisionLog + metrics     │
└───────────────────┬───────────────────────────────┘
                    │
        ┌───────────┴───────────┐
        │                       │
   &ResolvedConfig         KernelSpan
   (explicit param,        (scope-based,
    read-only)              write-only)
```

### 2.1 KernelConfig — The Declarative Root

One struct that absorbs all 7 scattered configs:

```rust
/// Single declarative root for all kernel configuration.
/// Lives in forge-kernel::core::config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelConfig {
    /// Numeric thresholds for geometry computations.
    pub tolerance: ToleranceSection,
    /// Boolean policies for ambiguous situations.
    pub policy: PolicySection,
    /// Parameters for iterative numerical solvers (NURBS/Intersections).
    pub solver: SolverSection,
    /// Validation checkpoint configuration.
    pub validation: ValidationSection,
    /// Precision escalation rules.
    pub precision: PrecisionSection,
}
```

#### ConfigSection Trait

Each section implements a shared trait for self-contained defaults and
validation, keeping `KernelConfig` clean and each section independently testable:

```rust
/// Trait for each section of KernelConfig.
/// Guarantees every section can produce its own defaults
/// and validate its invariants independently.
pub trait ConfigSection: Default + Serialize + DeserializeOwned {
    /// Named defaults (same as Default::default but explicit).
    fn defaults() -> Self;
    /// Validate invariants *within this section* (e.g., spatial_tolerance > 0).
    /// Called by KernelConfig::validate().
    /// Note: Cross-section invariants (like gap closure vs ambiguity band)
    /// are checked later in `KernelConfig::cross_validate()`. Keeping this
    /// isolated ensures each section remains independently testable.
    fn validate(&self) -> Result<(), KernelError>;
}
```

Example for `ToleranceSection`:

```rust
impl ConfigSection for ToleranceSection {
    fn defaults() -> Self { Self::default() }
    fn validate(&self) -> Result<(), KernelError> {
        if self.spatial_tolerance <= 0.0 {
            return Err(KernelError::InvalidConfig {
                field: "spatial_tolerance".into(),
                reason: "must be positive".into(),
            });
        }
        if self.ambiguity_band_factor <= 1.0 {
            return Err(KernelError::InvalidConfig {
                field: "ambiguity_band_factor".into(),
                reason: "must be > 1.0".into(),
            });
        }
        Ok(())
    }
}
```

Each section flattens the existing structs:

````rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToleranceSection {
    /// Unit system for all linear tolerances in this section.
    /// Prevents silent scale errors when mixing meters/mm bodies.
    pub unit_system: UnitSystem,
    // From TolerancePolicy
    pub spatial_tolerance: f64,       // default: 1e-6
    pub angular_tolerance: f64,       // default: 1e-6
    // From TangencyPolicy
    pub min_transversal_angle: f64,   // default: 1e-3
    pub max_tangent_gap: f64,         // default: 1e-4
    // From SliverPolicy
    pub min_face_area: f64,           // default: 1e-10
    pub max_slivers_per_op: usize,    // default: 3
    // From GapClosurePolicy
    pub max_gap_closure: f64,         // default: 1e-4
    // From ToleranceConfig (all 13 fields)
    pub residual: f64,                // default: 1e-8
    pub degeneracy: f64,              // default: 1e-12
    pub ambiguity_band_factor: f64,   // default: 10.0
    // ... etc.
}

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolverSection {
    /// Maximum iterations for Newton-Raphson / iterative solvers (NURBS, Fillets).
    pub max_iterations: usize,        // default: 50
    /// Target numeric residual for solver convergence.
    pub convergence_tolerance: f64,   // default: 1e-10
    /// Bail-out threshold if a solver is diverging.
    pub divergence_threshold: f64,    // default: 1e2
}
````

/// Unit system for linear measurements.
/// Attached to ToleranceSection to prevent silent scale mismatches
/// when importing bodies from different CAD systems. #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnitSystem {
Meters,
Millimeters,
Inches,
}

````

#### Centralized Defaults

All magic numbers (`1e-6`, `1e-10`, etc.) live in a single
`forge-kernel::core::config::defaults` module — the one source of truth.
Section `Default` impls delegate to named constants from this module:

```rust
// forge-kernel::core::config::defaults
pub const SPATIAL_TOLERANCE: f64 = 1e-6;
pub const ANGULAR_TOLERANCE: f64 = 1e-6;
pub const MIN_TRANSVERSAL_ANGLE: f64 = 1e-3;
pub const MAX_TANGENT_GAP: f64 = 1e-4;
pub const MIN_FACE_AREA: f64 = 1e-10;
pub const MAX_SLIVERS_PER_OP: usize = 3;
pub const AMBIGUITY_BAND_FACTOR: f64 = 10.0;
pub const RESIDUAL_TOLERANCE: f64 = 1e-8;
pub const DEGENERACY_THRESHOLD: f64 = 1e-12;
// ... every default, one place
````

> [!IMPORTANT]
> `KernelConfig` is serializable (JSON/TOML), so users can save/load/diff
> configuration presets. This is the future "Settings" panel in the UI.

### 2.2 Cascade Resolution

Override hierarchy matching existing `PolicyKind` pattern:

```
Session Default → Model Override → Feature Override → Operation Override
```

Each level is a **sparse partial** — only the fields being overridden:

```rust
/// Sparse override — only populated fields take effect.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConfigOverride {
    pub spatial_tolerance: Option<f64>,
    pub ambiguity_band_factor: Option<f64>,
    pub max_slivers_per_op: Option<usize>,
    // ... every field is Optional
}

/// Frozen, fully-resolved configuration for the current scope.
/// Passed as `&ResolvedConfig` — no &mut needed.
#[derive(Debug, Clone)]
pub struct ResolvedConfig {
    /// The effective values after cascade resolution.
    config: KernelConfig,
    /// Which scope provided each value (for tracing/audit).
    provenance: ConfigProvenance,
}

/// Where a config value came from, with enough detail to trace
/// through complex CAD history trees.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigSource {
    /// Which level of the cascade provided this value.
    pub scope: ConfigScope,
    /// Optional handle to the specific entity that set the override.
    /// e.g., FeatureHandle(0xAF32) "Imported_Bolt_4" — a life-saver
    /// when a boolean fails due to unexpected tolerance changes.
    pub origin: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigScope {
    SessionDefault,
    ModelOverride,
    FeatureOverride,
    OperationOverride,
}

impl ResolvedConfig {
    /// Query which scope set a given field, including
    /// the specific entity that provided the override.
    ///
    /// Essential for debugging "why did this tolerance change
    /// midway through a feature?" scenarios.
    pub fn source_of(&self, field: &str) -> &ConfigSource { ... }

    /// Multiplier to convert values from the configured
    /// unit system into meters (the kernel's internal standard).
    /// e.g., if UnitSystem is Millimeters, this returns 0.001.
    pub fn scale_factor(&self) -> f64 {
        match self.config.tolerance.unit_system {
            UnitSystem::Meters => 1.0,
            UnitSystem::Millimeters => 0.001,
            UnitSystem::Inches => 0.0254,
        }
    }
}
```

Resolution at the pipeline executor:

```rust
fn resolve_config(
    session: &KernelConfig,          // session default
    model: Option<&ConfigOverride>,  // per-model
    feature: Option<&ConfigOverride>,// from FeatureContract
    operation: Option<&ConfigOverride>,
) -> Result<ResolvedConfig, KernelError> {
    let resolved = // ... apply cascade ...
    resolved.cross_validate()?;  // check inter-section invariants
    Ok(resolved)
}
```

#### Cross-Section Validation

Individual `ConfigSection::validate()` checks single-field invariants (e.g.,
`spatial_tolerance > 0`), but some invariants span sections. For example:

- `spatial_tolerance × ambiguity_band_factor` defines the effective ambiguity
  zone — if a user overrides one but not the other, the zone could be larger
  than their smallest feature
- `max_gap_closure` should not exceed `spatial_tolerance × ambiguity_band_factor`

```rust
impl KernelConfig {
    /// Check inter-section invariants after cascade resolution.
    /// Called automatically by resolve_config().
    pub fn cross_validate(&self) -> Result<(), KernelError> {
        let gray_zone = self.tolerance.spatial_tolerance
            * self.tolerance.ambiguity_band_factor;

        if self.tolerance.max_gap_closure > gray_zone {
            return Err(KernelError::InvalidConfig {
                field: "max_gap_closure vs ambiguity band".into(),
                reason: format!(
                    "gap closure {} exceeds ambiguity zone {}",
                    self.tolerance.max_gap_closure, gray_zone
                ),
            });
        }
        // ... additional cross-section checks
        Ok(())
    }
}
```

> [!CAUTION]
> `cross_validate()` runs after every cascade resolution, not just at
> construction time. A valid session config + a valid feature override
> can produce an invalid resolved config if they interact badly.

### 2.3 FeatureContract Integration

Features declare their config requirements:

```rust
pub trait FeatureContract: Send + Sync {
    fn required_policies(&self) -> &[PolicyKind];
    fn audit_level(&self) -> AuditLevel;

    /// Config overrides this feature requires.
    /// Default: no overrides (use cascade defaults).
    fn config_overrides(&self) -> Option<ConfigOverride> {
        None
    }
}
```

Example: aerospace fillet declares tighter ambiguity:

```rust
impl FeatureContract for AerospaceFillet {
    fn config_overrides(&self) -> Option<ConfigOverride> {
        Some(ConfigOverride {
            ambiguity_band_factor: Some(5.0),
            min_face_area: Some(1e-12),
            ..Default::default()
        })
    }
}
```

### 2.4 KernelSpan — Scope-Based Decision Logging

Replaces `&mut ModelingContext` for write-only logging (the 40+ sites):

```rust
/// Scope-based decision collector.
///
/// Internally uses a shared `Arc<Mutex<SpanCollector>>` rather than
/// bare thread-local storage. The handle is `Clone + Send`, enabling
/// cross-thread propagation for parallel iterators.
pub struct KernelSpan {
    collector: Arc<Mutex<SpanCollector>>,
}

impl KernelSpan {
    /// Enter a new span. Installs the collector in thread-local
    /// storage and returns an RAII guard.
    pub fn enter(name: &str) -> KernelSpanGuard { ... }

    /// Record a decision in the active span.
    ///
    /// In debug builds, panics if no span is active (catches
    /// forgotten `KernelSpan::enter()` calls in tests).
    pub fn record_decision(decision: TracedDecision) {
        debug_assert!(
            Self::is_active(),
            "KernelSpan::record_decision called outside of an active span. \
             Did you forget KernelSpan::enter()?"
        );
        // ... push to thread-local or attached collector
    }

    /// Record a warning in the active span.
    pub fn record_warning(warning: KernelWarning) { ... }

    /// Whether a span is currently active on this thread.
    pub fn is_active() -> bool { ... }

    /// Convenience for the check_tolerance! pattern.
    pub fn check_tolerance(
        margin: f64,
        location: [f64; 3],
        kind: DecisionKind,
    ) { ... }

    /// Create a handle that can be sent to worker threads
    /// (e.g., inside `par_iter()` closures). Workers attach
    /// the handle via `KernelSpan::attach()` before recording.
    pub fn handle(&self) -> KernelSpanHandle { ... }

    /// Attach an existing span handle on a worker thread.
    /// Decisions recorded on this thread go to the parent span.
    pub fn attach(handle: KernelSpanHandle) -> KernelSpanGuard { ... }
}
```

> [!WARNING]
> `record_decision` includes a `debug_assert!` that panics in tests if called
> outside an active span. This catches forgotten `KernelSpan::enter()` calls
> early rather than silently dropping decisions in production.

#### Parallel Safety (The "Zombie Span" Problem)

If an operation spawns `faces.par_iter().map(...)`, bare `thread_local!`
storage would be empty on Rayon worker threads — decisions vanish silently.

The collector uses `Arc<Mutex<SpanCollector>>` so the handle can be cloned
into parallel closures. Pattern:

```rust
fn classify_faces_parallel(faces: &[FaceId], config: &ResolvedConfig) {
    let span_handle = KernelSpan::current_handle();

    faces.par_iter().for_each(|face| {
        let _guard = KernelSpan::attach(span_handle.clone());
        // Now record_decision works on this worker thread
        let result = classify_face(face, config);
        KernelSpan::record_decision(result.decision);
    });
}
```

> [!NOTE]
> This mirrors `tracing`'s subscriber/dispatcher pattern. The `Mutex` is
> per-span (not global), so contention is limited to the same operation.
> For 99% of current code (single-threaded pipeline), the `Mutex` is
> uncontended and optimized away by the compiler.

```rust
/// RAII guard — collects the span's output when dropped.
pub struct KernelSpanGuard { ... }

impl KernelSpanGuard {
    /// Extract the accumulated DecisionLog + metrics + warnings.
    pub fn finish(self) -> SpanOutput { ... }
}

pub struct SpanOutput {
    pub decision_log: DecisionLog,
    pub warnings: Vec<KernelWarning>,
    pub metrics: OperationMetrics,
    pub lineage_delta: LineageDelta,
    /// Snapshot of the resolved config that was active during this span.
    /// Stored alongside the DecisionLog so undo/redo history replay
    /// can reconstruct exactly what config was in effect.
    pub config_snapshot: ResolvedConfig,
}
```

Usage at the executor:

```rust
// Pipeline executor
fn execute(&self, feature: &dyn Feature, state: KernelState) -> Result<...> {
    let config = resolve_config(&self.session_config, ...);
    let guard = KernelSpan::enter(&feature.name());

    // Feature code uses &config (explicit) and KernelSpan::record_decision (implicit)
    let output = feature.execute_typed(&config, state)?;

    let span_output = guard.finish();
    // Wrap into OperationResult with span_output.decision_log, etc.
}
```

Usage in operation code:

```rust
// Before: fn split_face(draft: &mut MutableDraft, ctx: &mut ModelingContext, tol: f64)
// After:
fn split_face(draft: &mut MutableDraft, config: &ResolvedConfig) {
    let tol = config.spatial_tolerance();
    // ...
    KernelSpan::record_decision(TracedDecision { ... });
}
```

### 2.5 Future-Proofing: NURBS, Fillets, and Local Tolerances

Complex operations like surface blending (fillets) and NURBS intersections
stress tolerance systems heavily. The architecture supports them explicitly:

**1. Iterative Solvers**
NURBS and complex surface intersections require iterative root-finding. Parameters
for these (max iterations, convergence target) live in their own `SolverSection`, separate
from the static coincidence bounds in `ToleranceSection`.

**2. Dynamic Config Escalation**
A boolean or fillet operation is allowed to run an intersection, get an `Ambiguous`
result, generate a new tightened `ResolvedConfig`, and retry. The config is
frozen _per span_, not globally locked for the entire feature execution.

**3. Local Entity Tolerances (The "Tolerant Edge")**
While `KernelConfig` provides the global/feature scope bounds, aerospace B-Reps
eventually require per-edge or per-vertex "local" tolerances (where one complex edge
has a 0.01mm tolerance while the rest of the body is 1μm).

- **DON'T:** Put per-entity tolerances in `KernelConfig`.
- **DO:** Store local tolerances as semantic tags in the `forge-topo` `AttributeStore`.
- **DO:** Use `KernelConfig::policy.respect_local_tolerances` to dictate whether solvers
  should query the attribute store before falling back to the global `ToleranceSection`.

---

## 3. Migration Plan

### Phase 1: Unified KernelConfig (non-breaking)

1. Create `KernelConfig` + sections, populated from existing structs
2. Add `ResolvedConfig` with `From<&ModelingContext>` adapter
3. Keep `ModelingContext` — it delegates to `KernelConfig` internally
4. Gradually move callers from `ctx.get_tolerance_config()` to `&ResolvedConfig`

### Phase 2: KernelSpan (parallel rollout)

1. Implement `KernelSpan` with thread-local `RefCell<Vec<TracedDecision>>`
2. Add `KernelSpan::enter()` at the pipeline executor (alongside existing `ModelingContext`)
3. Migrate logging-only call sites to `KernelSpan::record_decision()`
4. Functions that only log drop `&mut ModelingContext` from their signature

### Phase 3: Remove ModelingContext (cleanup)

1. Once all sites migrated: `ModelingContext` becomes a thin shell over `KernelConfig` + `KernelSpan`
2. Eventually `ModelingContext::new()` just creates a `KernelConfig` and enters a root span
3. `&mut ModelingContext` removed from all operation signatures

### Phase 4: Per-Object Config + UI Integration

1. Add `ConfigOverride` to `FeatureContract`
2. Add cascade resolution to the executor
3. Wire to UI settings panel (JSON-serializable `KernelConfig` ↔ settings file)

---

## 4. Architecture Rules

### 4.1 Config Is Read-Only Below the Executor

- **DO:** Pass `&ResolvedConfig` to functions that need tolerance values
- **DON'T:** Pass `&mut KernelConfig` anywhere — config is immutable during execution

### 4.2 Logging Is Implicit

- **DO:** Use `KernelSpan::record_decision()` for write-only logging
- **DO:** Use `KernelSpan::handle()` + `attach()` when spawning parallel work
- **DON'T:** Thread `&mut` just for logging; that's what the span handles

### 4.3 Lower Crates Still Get Individual Values

- **DO:** At the kernel boundary, destructure `ResolvedConfig` into individual `f64` params for `forge-geom`/`forge-topo` (Architecture §3.2 unchanged)
- **DON'T:** Import `ResolvedConfig` in `forge-geom` or `forge-topo`

### 4.4 Serialization Boundary

- **DO:** `KernelConfig` and `ConfigOverride` are `Serialize + Deserialize`
- **DO:** `ResolvedConfig` tracks provenance (which scope set which value)
- **DON'T:** Serialize `ResolvedConfig` — it's ephemeral; serialize the inputs (config + overrides)

### 4.5 Hot-Loop Performance

If `forge-geom` code accesses a tolerance value inside a loop that runs
millions of times (e.g., intersection curve sampling), the overhead of
dereferencing through a wrapper struct hurts.

- **DO:** Copy required `f64` values into stack-local variables before entering
  hot loops. This is already natural since lower crates receive individual `f64`
  params, not `ResolvedConfig`
- **DO:** Mark `ResolvedConfig` accessor methods `#[inline]` so the compiler can
  eliminate indirection even when called from kernel-level code
- **DON'T:** Call `config.tolerance().spatial_tolerance()` inside a tight loop —
  pull the value into a `let tol = ...` before the loop

```rust
// Good: stack-local before hot loop
let tol = config.tolerance().spatial_tolerance;
for edge in edges {
    let len = compute_edge_length(edge);
    if len < tol { /* ... */ }
}

// Bad: accessor chain inside hot loop
for edge in edges {
    if compute_edge_length(edge) < config.tolerance().spatial_tolerance {
        /* ... */
    }
}
```

---

## 5. What This Replaces

| Before                             | After                                                               |
| ---------------------------------- | ------------------------------------------------------------------- |
| `TolerancePolicy`                  | `KernelConfig::tolerance.spatial_tolerance` + `angular_tolerance`   |
| `TangencyPolicy`                   | `KernelConfig::tolerance.min_transversal_angle` + `max_tangent_gap` |
| `SliverPolicy`                     | `KernelConfig::tolerance.min_face_area` + `max_slivers_per_op`      |
| `GapClosurePolicy`                 | `KernelConfig::tolerance.max_gap_closure`                           |
| `PrecisionEscalationPolicy`        | `KernelConfig::precision.bit_length_threshold`                      |
| `ToleranceConfig` (13 fields)      | `KernelConfig::tolerance.*` (absorbed)                              |
| `ValidationConfig`                 | `KernelConfig::validation.*`                                        |
| `ctx.log_decision(...)` (40 sites) | `KernelSpan::record_decision(...)`                                  |
| `&mut ModelingContext` (50 sites)  | `&ResolvedConfig` (~6 sites) + `KernelSpan` (implicit)              |

## 6. Edge Cases

| Edge Case          | Potential Failure                                                                     | Mitigation                                                                                                                             |
| ------------------ | ------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| **Parallelism**    | `KernelSpan` missing on Rayon worker threads — decisions vanish                       | `Arc<Mutex<SpanCollector>>` + `handle()/attach()` (§2.4)                                                                               |
| **Undo/Redo**      | Config changes during history playback produce different results than original        | `SpanOutput` stores `config_snapshot: ResolvedConfig` alongside the `DecisionLog` — replay uses the exact config from the original run |
| **Unit Mismatch**  | Meters vs millimeters — spatial_tolerance of `1e-6` means 1μm in meters but 1nm in mm | `UnitSystem` field on `ToleranceSection` (§2.1). `cross_validate()` checks consistency between unit system and tolerance magnitudes    |
| **Stale Override** | Feature deleted but its `ConfigOverride` still referenced in cascade                  | `resolve_config()` validates all override sources are live before resolution                                                           |
| **Config Drift**   | User edits config JSON mid-session while operations are running                       | `ResolvedConfig` is frozen at span entry — in-flight operations are immune to external changes                                         |

---

## 7. Verification Plan

### Automated Tests

- Unit tests for cascade resolution (session → model → feature → operation)
- Unit tests for KernelSpan enter/record/finish lifecycle
- Unit tests for `cross_validate()` catching inter-section invariant violations
- Unit test for `UnitSystem` mismatch detection
- Integration test: feature with `config_overrides()` gets resolved values
- Integration test: `SpanOutput.config_snapshot` matches the config used during execution
- Regression: all existing pipeline + boolean tests pass unchanged

### Manual Verification

- Serialize `KernelConfig` to JSON, edit, reload — confirm values take effect
- Inspect `ResolvedConfig.provenance` — confirm override source tracking works
- Undo/redo an operation that used a feature-level config override — confirm replay uses stored config
