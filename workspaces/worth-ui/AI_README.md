# Worth UI Discovery

Worth UI is the product-facing UI authority. Start in `worth-ui` and use its
named `facade` modules. Reach into a lower crate only when maintaining the
owner implementation of that boundary.

For the larger architectural map, read `docs/worth-ui-readme.md`. For the exact
application and replacement workflow, read `docs/application-lifecycle.md`.
For installed Query views and projection consumption, read
`docs/query-binding.md`.

## Ordinary Application Lifecycle

The ordinary lifecycle is a typed progression:

```text
WorthUiBuilder
-> Result<WorthUiApp, WorthUiApplicationPreparationDenial>
-> Result<WorthUiActiveApplicationSession, WorthUiRuntimeLaunchDenial>
-> framework turns / inspection / replacement
```

`WorthUiApp` is prepared application authority. It inseparably binds the
canonical runtime artifact, declaration artifacts, graph snapshot, capability
snapshot, Query binding plan, host-session plan, derived inspection indexes,
and one application-generation identity.

`WorthUiApp::launch` consumes the prepared application and returns
`WorthUiActiveApplicationSession`. The active session is the ordinary owner of
runtime execution, active inspection, Query projection submission, host
observation capability, retained allocation evidence, and replacement
cutover. Do not split these concerns into separately launched objects.

```rust
use worth_ui::facade::app::WorthUi;
use worth_ui::facade::host::WorthUiHeadlessHost;
use worth_ui::facade::inspection::{
    UiInspectionQuery, UiInspectionScope, UiInspectionTarget,
};

let app = WorthUi::app()
    .with_host(WorthUiHeadlessHost)
    // register declarations and capabilities here
    .freeze()
    .expect("application preparation should admit");

let mut session = app.launch().expect("prepared application should launch");
let completion = session.execute_framework_turn(|turn| {
    // collect admitted Query, host, interaction, or resize input here
});
let generation = completion.generation_identity().clone();
drop(completion.into_completion());

let inspection = session.inspect(UiInspectionQuery::new(
    UiInspectionTarget::product_root(),
    UiInspectionScope::graph(),
));
assert_eq!(&generation, inspection.generation_identity());
```

The snippet shows lifecycle shape; use the named registration and inspection
types for the application being built.

## Authored Composition

File-authored and Rust-authored inputs enter through
`worth_ui::facade::source`. Both lanes lower to one sealed
`WorthUiWatchedCandidateSubmission`, which carries the candidate artifact,
declaration source, source revision, ordering receipt, provenance, and ingress
counters together.

The builder consumes that submission with `with_candidate_submission` before
`freeze`. Preparation is fallible and publishes no partial application
authority on denial.

Rust-authored modules use `WorthUiRustAuthoredArtifactInputModule` and
`WorthUiArtifactInputBodyAtom`; callers do not serialize native composition to
JSON or reconstruct a finished artifact from digests.

## Replacement

Replacement stays inside the active session:

```text
candidate submission
-> prepare_replacement
-> Prepared candidate authority
-> candidate-owned graph/allocation admission
-> lower_prepared_replacement
-> summary() / cost_envelope() observation
-> stage_prepared_replacement
-> framework-turn activation boundary
-> activate_prepared_replacement
```

Preparation does not decide semantic no-op. The current surface carries a
successfully prepared candidate through allocation admission, complete plan
lowering, exact executable comparison, and the activation decision. Only that
final decision may return `WorthUiApplicationReplacementOutcome::SemanticNoOp`;
an artifact digest is never sufficient authority to skip lowering.

Successful cutover changes the application generation atomically. Invalid,
foreign, stale, or incomplete candidates preserve the last complete active
generation. The host session remains bound to the active application session.

Lowering is reconstructive work and is intentionally visible in the method
name. `summary()` and `cost_envelope()` are compact observations of work already
performed; neither can stage, activate, or execute the candidate. Activated and
semantic-no-op outcomes both expose `reload_cost()` derived from the real
production work.

## Framework Turns

`WorthUiActiveApplicationSession::execute_framework_turn` is the one ordinary
frame owner. Its closure collects admitted source inputs; after collection the
owner closes and pumps once, creates a proof-bearing transition plan, executes
the selected policy family, and publishes a typed completion.

Use source capabilities supplied by the turn:

- `query_projection` for an installed Query projection outcome
- `host_measurement` for observations collected by the active host capability
- `interaction` for graph-node interaction state
- resize sources for preview or durable resize input

A raw host adapter or raw runtime object cannot submit work independently.

Frame-executable plan lowering is runtime-owner implementation work. Ordinary
consumers do not construct plan inputs, allocate runtime handles, choose lane
strategy, compare plan digests, or pass a plan to an executor. The active
application session remains the authority that owns the executable generation
and lends only scoped frame access. Plan and frame-cost inspection is
observation, never an activation or execution path.

Each active lane completion exposes `cost_receipt()`. Call it only when evidence
is needed: report materialization stays outside the measured executor interval.
The receipt is bound to the exact host-output generation and records requested
and executed breadth; certification denies `executed > requested`. Host-adapter
and renderer work is outside that executor boundary.

## Authority Checks

When following a value across the system, preserve these bindings:

- application generation: prepared app, active session, framework completion,
  replacement receipt, and active inspection
- graph authority: graph lookup, mounted transitions, allocation catalog,
  replan selection, and allocation receipts
- Query authority: registered installed view and projection outcome
- host authority: active application session, host-session identity, and host
  measurement capability
- source authority: candidate composition, declaration source, source revision,
  and ordering receipt

Raw IDs and digests are reportable evidence. They do not promote authority.
Query projection also crosses as one sealed outcome; do not split it into local
basis/status/fact/digest truth or convert Foundational native aspect values
through JSON or text for operational UI use.

## Crate Ownership

- `worth-ui`: public named facades and product entry
- `worth-ui-runtime`: preparation, declarations, graph, allocation, active
  lifecycle, framework turns, inspection bridge, and replacement
- `worth-ui-query-binding`: the only Query-to-Worth-UI translation boundary
- `worth-ui-host-contract`: host capability and observation contracts
- `worth-ui-host-egui`: egui mechanics behind the host contract
- `worth-ui-theme`: semantic tokens projected into host visuals
- `worth-ui-components`: presentational components without runtime semantics
- `worth-ui-certification`: hostile production-path and structural proof

## Before Claiming Completion

Run the relevant workspace tests and strict Clippy, then the Worth UI test
topology gate, Rust line-cap gate, repository boundary check, and agent-context
check. A green behavioral test does not override a red authority or source
reachability gate.
