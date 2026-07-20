# Application Lifecycle

## What This Feature Is

The application lifecycle is the public Worth UI path for preparing, launching,
running, inspecting, and replacing one UI application without handing executable
plan authority to application code. Application developers hold a
`WorthUiActiveApplicationSession`; it keeps the application, Query, host,
inspection, and frame generations coherent.

## Why You Use It

- Launch a Query-free or Query-backed UI as one prepared application.
- Run headless or native-host frames through the same runtime-owned lifecycle.
- Inspect a replacement before cutover without making it executable.
- Replace a running application atomically while preserving the last valid
  generation on denial.
- Observe reload and steady-frame cost at the production boundary that performed
  the work.

## Stable Entry Points

- `worth_ui::facade::app::WorthUi::app()`
- `WorthUiBuilder::freeze()`
- `WorthUiApp::launch()`
- `WorthUiActiveApplicationSession::execute_framework_turn(...)`
- `WorthUiActiveApplicationSession::prepare_replacement(...)`
- `WorthUiActiveApplicationSession::lower_prepared_replacement(...)`
- `WorthUiLoweredApplicationReplacement::summary()`
- `WorthUiLoweredApplicationReplacement::cost_envelope()`
- `WorthUiActiveApplicationSession::stage_prepared_replacement(...)`
- `WorthUiActiveApplicationSession::activate_prepared_replacement(...)`
- activated and semantic-no-op `reload_cost()` accessors
- active lane completion `cost_receipt()` methods

Declarations, installed Query views, host choice, and advanced lane targets enter
through their named `worth_ui::facade` modules. Raw plan builders, plan digests,
lowerers, and executors are not application entry points.

## Core Mental Model

Prepared application authority is one inseparable generation of declared UI
meaning, graph authority, capability support, Query binding, host-session plan,
and inspection indexes. Launch consumes it and produces the active session—the
only ordinary owner allowed to execute it.

A replacement candidate is not a future active plan. Preparation and lowering
derive candidate artifacts and compact observations. Staging retains the exact
candidate authority for a cutover attempt. Only activation at a session-bound
frame boundary may publish a new executable generation or prove an exact
semantic no-op.

Receipts are observations, not capabilities. A generation identity or digest can
explain work, but cannot activate a candidate, execute a plan, or recreate Query
authority.

## How It Executes

```text
application declarations and optional installed Query views
-> freeze (fallible preparation)
-> launch (one active application session)
-> framework turns and session-owned lane execution

replacement submission
-> prepare candidate
-> admit candidate graph/allocation facts
-> lower candidate (expensive reconstruction)
-> inspect compact candidate summary and completed lowering cost
-> stage exact candidate authority
-> obtain a session-bound activation boundary from a framework turn
-> activate atomically
-> Activated(receipt) | SemanticNoOp(receipt) | typed denial
```

The phases remain separate because they have different authority and denial
boundaries. A transient frame-boundary denial returns retry authority for the
same staged candidate instead of forcing a rebuild.

## Small Example

```rust
use worth_ui::facade::{app::WorthUi, host::WorthUiHeadlessHost};

let app = WorthUi::app()
    .with_host(WorthUiHeadlessHost)
    .freeze()?;

let mut session = app.launch()?;
let completion = session.execute_framework_turn(|_turn| {});
assert_eq!(completion.generation_identity(), session.generation_identity());
# Ok::<(), Box<dyn std::error::Error>>(())
```

This is the smallest honest example: even a Query-free headless application is
prepared and launched as one authority. Optional systems add registrations; they
do not change the lifecycle.

## Real Example

```rust
let mut prepared = session.prepare_replacement(submission)?;

// Application graph/measurement code admits the candidate-owned catalog delta
// through `prepared`'s candidate admission surfaces.
let admitted_delta = admit_application_catalog_delta(&mut prepared)?;

let lowered = session.lower_prepared_replacement(*prepared)?;
observe_candidate(lowered.summary(), lowered.cost_envelope());
let pending = session.stage_prepared_replacement(lowered)?;

let boundary = session
    .execute_framework_turn(|_turn| {})
    .into_completion()
    .into_execution()
    .map_err(|_| "framework turn did not yield execution authority")?
    .into_activation_boundary();

match session.activate_prepared_replacement(pending, admitted_delta, boundary, None)? {
    WorthUiApplicationReplacementOutcome::Activated(receipt) => {
        observe_reload_cost(receipt.reload_cost()?);
    }
    WorthUiApplicationReplacementOutcome::SemanticNoOp(receipt) => {
        observe_reload_cost(receipt.reload_cost()?);
    }
}
```

The omitted catalog helper is application-specific: its entries come from the
candidate graph and measurement declarations, so a generic fabricated catalog
would be dishonest. `summary()` and `cost_envelope()` are derived observations;
they cannot stage, activate, or execute anything. The pending cutover retains
candidate authority, the framework turn lends the exact boundary, and the
outcome reports either the new active generation or exact executable
equivalence. `reload_cost()` derives from the production work that actually ran.

For a frame, execute through `WorthUiActiveFrameworkTurnExecution`, then call the
returned lane completion's `cost_receipt()` only when cost evidence is needed.
That explicit materialization keeps reporting allocations outside the measured
executor interval. Each lane receipt carries its exact host-output generation
and proves executed breadth did not exceed requested breadth.

## How It Relates To Other Features

- Register an installed view from [Query-backed UI views](./query-binding.md)
  before `freeze`; Query-free applications require no Query ceremony.
- Headless and egui hosts consume the same sealed host-output envelope and never
  receive candidate or active plan data.
- Candidate inspection describes candidate-only facts before cutover. Active
  inspection observes only the current published generation.
- Durable-state hooks are an advanced lowering control. Use
  `lower_prepared_replacement_with_state_hooks(...)` only for a declared state
  family.

## Inspection And Debugging

- Borrow `session.generation_identity()` for the current active generation.
- Use `prepared.inspect_candidate(...)` for candidate graph/evidence inspection.
- Use `lowered.summary()` for affected scope and classifications;
  `lowered.cost_envelope()` reports reconstruction work already performed.
- Inspect `WorthUiApplicationReplacementOutcome` exhaustively. A semantic no-op
  is a successful equivalence decision, not a swallowed update.
- Use `reload_cost()` for reconstructive work and lane `cost_receipt()` for
  ordinary frame work. Their schemas are intentionally distinct.
- Rich evidence expansion is explicit and does not mutate equivalence or frame
  counters.

## Anti-Patterns

- Do not compare artifact or plan digests to decide no-op.
- Do not construct or pass a plan to a frame executor.
- Do not activate from a candidate summary, inspection receipt, or cost receipt.
- Do not collapse phase denials into a boolean or log string.
- Do not rebuild a candidate after a transient boundary denial; use its retry
  authority.
- Do not count host-adapter or renderer allocation as executor work.
- Do not copy Query explanations into UI diagnostics; retain Query-owned evidence
  references.

## Current Limits

- Candidate catalog admission remains explicit because allocation truth is
  application-specific.
- There is no one-call replacement helper; the explicit phases are the stable
  authority model.
- Cost receipts certify runtime boundary work, not wall-clock or native renderer
  cost.
- Advanced lane target construction stays separate from the common turn path.

## Related Docs

- [Worth UI runtime orientation](./worth-ui-readme.md)
- [Query-backed UI views](./query-binding.md)
- [Compact AI discovery guide](../AI_README.md)
