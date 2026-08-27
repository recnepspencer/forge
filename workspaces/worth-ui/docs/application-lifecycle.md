# Application Lifecycle

## What This Feature Is

The application lifecycle prepares, launches, runs, inspects, and shuts down
one Worth UI application while the framework keeps graph, Query, host, mounted
publication, and generation state coherent. Application code holds one
`WorthUiActiveApplicationSession`.

## Why You Use It

- Launch Query-free or Query-backed UI through one path.
- Present headless and native-host frames with the same mounted contract.
- Preserve the last published frame when preparation or presentation denies.
- Handle in-flight or uncertain native effects through typed outcomes.
- Inspect the active generation without receiving execution authority.

## Stable Entry Points

- `worth_ui::facade::app::WorthUi::app()`
- `worth_ui_native_platform::WorthUiNativePlatform::prepare(...)`
- `UiNativeApplicationDefinition::prepare(...)`
- `UiNativeApplicationPreparation::builder()` and `complete()`
- `WorthUiApp::launch()`
- `WorthUiActiveApplicationSession::execute_mounted_frame(...)`
- `WorthUiNativeApplicationShell::begin_source_rebind(...)`
- `WorthUiActiveApplicationSession::inspect(...)`
- `WorthUiActiveApplicationSession::shutdown()`
- `UiMountedFrameOutcome`
- `WorthUiMountedFrameExecutionStop`
- `UiSourceRebindRequest`
- `UiRebindOutcome`
- `WorthUiNativeSourceRebindDenial`

Mounted request, deadline, outcome, and recovery types are re-exported by
`worth_ui::facade::app`. Application code does not import a mounted runtime
module.

## Core Mental Model

The native preparation scope carries the only host-bound builder. An unbound
builder cannot freeze, and a bound builder cannot replace its host. `complete`
freezes one inseparable application generation: authored meaning,
capabilities, graph, Query bindings, host plan, and inspection indexes. Launch
consumes that prepared app and returns the only ordinary running owner.

`execute_mounted_frame` collects admitted inputs, advances the runtime,
assembles all required surfaces, presents through the host contract, and
publishes only a complete frame. A receipt reports what happened; it cannot be
used to execute or publish another frame.

After launch, `begin_source_rebind` is the ordinary bridge from one settled
filesystem snapshot to semantic classification, bounded consequence planning,
canonical host presentation, and atomic successor publication. It borrows the
same running shell; it does not launch or swap in a second application.

## How It Executes

```text
WorthUi::app()
-> framework-owned one-shot host binding
-> register capabilities, source, Query views, intents, and inspection policy
-> complete native preparation
-> launch
-> register/mount application-specific surfaces as required
-> execute_mounted_frame
-> Published | Unchanged | Reconciled
   | RejectedBeforeEffects | InFlight | PresentationIndeterminate
   | RetentionDenied | AdmissionDenied | CompletionDenied
-> inspect or continue through the returned typed outcome
-> for a settled edit, begin_source_rebind
-> Published | ObservedNoChange | Duplicate | SupersededBeforeEffects
   | TimedOutBeforeEffects | CancelledBeforeEffects
   | RejectedBeforeEffects | InFlight | Indeterminate
-> shutdown
```

A pre-effect denial leaves the predecessor publication current. If native
effects may have started, the runtime retains semantic truth but marks affected
bindings uncertain until the typed recovery path completes.

## Small Example

```rust
use worth_ui_native_platform::{
    UiNativeApplicationDefinition, UiNativeApplicationPreparation,
    UiNativeApplicationPreparationOutcome, UiNativePlatformProfile,
    UiNativeWindowSpec, WorthUiNativePlatform,
};

struct Application;

impl UiNativeApplicationDefinition for Application {
    fn prepare(
        self,
        preparation: UiNativeApplicationPreparation,
    ) -> UiNativeApplicationPreparationOutcome {
        preparation.complete()
    }
}

let profile = UiNativePlatformProfile::single_window(UiNativeWindowSpec::new(
    "WORTH UI",
    [160, 96],
));
let platform = WorthUiNativePlatform::prepare(profile)?;
let outcome = platform.run(Application);
```

Preparation is effect-free. After it succeeds, `run` enters the qualified
native lifecycle and returns an exhaustive closed, stopped, or
application-preparation-denied outcome. Query-free applications do not create
dummy Query work.

## Executable Fresh-Reader Contract

The following is the exact downstream program used by the compiler-contract
matrix and the application-contract runtime suite. It uses the ordinary
product facade plus the sealed certification headless transition, branches over every
mounted outcome and start-stop family, and executes the real empty-application
path.

<!-- compile-run:ordinary-mounted-frame -->
```rust
use worth_ui::facade::app::{
    UiMountedFrameOutcome, UiMountedFramePublicationReceipt, UiMountedFrameRequest,
    UiMountedFrameRetentionRejection, UiMountedIndeterminateFrame,
    UiMountedPresentationAdmissionRejection, UiMountedPresentationCompletionDenial,
    UiMountedPresentationInFlight, UiMountedRejectedFrame, UiMountedSupersededFrame,
    UiPresentationDeadline, WorthUi, WorthUiMountedFrameExecutionStop,
};

fn main() {
    run();
}

pub fn run() {
    let app = WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .freeze()
        .map(
            worth_ui_runtime::facade::entry::WorthUiCertificationApplicationTransition::activate_headless,
        )
        .expect("empty application preparation should succeed");
    let mut session = app.launch().expect("empty application should launch");
    let outcome = match session.execute_mounted_frame(
        UiMountedFrameRequest::all_bound_surfaces(),
        UiPresentationDeadline::at_tick(1),
        0,
        |_| {},
    ) {
        Ok(outcome) => outcome,
        Err(stop) => {
            observe_stop(&stop);
            return;
        }
    };

    match outcome {
        UiMountedFrameOutcome::Published(receipt)
        | UiMountedFrameOutcome::Unchanged(receipt)
        | UiMountedFrameOutcome::Reconciled(receipt) => {
            observe_publication(&receipt);
        }
        UiMountedFrameOutcome::RejectedBeforeEffects(rejection) => {
            observe_rejection(&rejection);
        }
        UiMountedFrameOutcome::InFlight(in_flight) => {
            observe_in_flight(&in_flight);
        }
        UiMountedFrameOutcome::PresentationIndeterminate(indeterminate) => {
            observe_indeterminate(&indeterminate);
        }
        UiMountedFrameOutcome::RetentionDenied(rejection) => {
            observe_retention_denial(&rejection);
        }
        UiMountedFrameOutcome::AdmissionDenied(rejection) => {
            observe_admission_denial(&rejection);
        }
        UiMountedFrameOutcome::CompletionDenied(denial) => observe_completion_denial(&denial),
        UiMountedFrameOutcome::Superseded(superseded) => observe_superseded(&superseded),
    }
}

fn observe_stop(stop: &WorthUiMountedFrameExecutionStop<'_>) {
    match stop {
        WorthUiMountedFrameExecutionStop::PublicationLease(_) => {}
        WorthUiMountedFrameExecutionStop::HostMeasurement(_) => {}
        WorthUiMountedFrameExecutionStop::FrameworkTransition(transition) => {
            let _ = transition.generation_identity();
        }
        WorthUiMountedFrameExecutionStop::Preparation(_) => {}
    }
}

fn observe_publication(receipt: &UiMountedFramePublicationReceipt) {
    let _ = receipt.cost_report();
}

fn observe_rejection(rejection: &UiMountedRejectedFrame) {
    let _ = rejection.cost_report();
}

fn observe_in_flight(in_flight: &UiMountedPresentationInFlight) {
    let _ = in_flight.cost_report();
}

fn observe_indeterminate(indeterminate: &UiMountedIndeterminateFrame) {
    let _ = indeterminate.cost_report();
}

fn observe_retention_denial(rejection: &UiMountedFrameRetentionRejection) {
    let _ = rejection.frame().cost_report();
}

fn observe_admission_denial(rejection: &UiMountedPresentationAdmissionRejection) {
    let _ = rejection.frame().cost_report();
}

fn observe_completion_denial(_denial: &UiMountedPresentationCompletionDenial) {}

fn observe_superseded(superseded: &UiMountedSupersededFrame) {
    let _ = superseded.cost_report();
}
```

## Real Example

```rust
use worth_ui::facade::app::{
    UiMountedFrameOutcome, UiMountedFrameRequest, UiPresentationDeadline,
    WorthUiMountedFrameExecutionStop,
};

let outcome = match session.execute_mounted_frame(
    UiMountedFrameRequest::all_bound_surfaces(),
    UiPresentationDeadline::at_tick(clock.deadline()),
    clock.now(),
    |sources| collect_application_inputs(sources),
) {
    Ok(outcome) => outcome,
    Err(WorthUiMountedFrameExecutionStop::PublicationLease(denial)) => {
        return retry_after_publication_lease(denial);
    }
    Err(WorthUiMountedFrameExecutionStop::FrameworkTransition(stop)) => {
        return report_generation_stop(stop.generation_identity());
    }
    Err(WorthUiMountedFrameExecutionStop::Preparation(denial)) => {
        return preserve_predecessor(denial);
    }
};

match outcome {
    UiMountedFrameOutcome::Published(receipt)
    | UiMountedFrameOutcome::Unchanged(receipt)
    | UiMountedFrameOutcome::Reconciled(receipt) => publish_observation(receipt),
    UiMountedFrameOutcome::RejectedBeforeEffects(rejection) => {
        preserve_predecessor(rejection)
    }
    UiMountedFrameOutcome::InFlight(handle) => retain_in_flight(handle),
    UiMountedFrameOutcome::PresentationIndeterminate(handle) => {
        require_typed_reconciliation(handle)
    }
    UiMountedFrameOutcome::RetentionDenied(denial) => preserve_predecessor(denial),
    UiMountedFrameOutcome::AdmissionDenied(denial) => preserve_predecessor(denial),
    UiMountedFrameOutcome::CompletionDenied(denial) => preserve_predecessor(denial),
}
```

The advanced branches begin with typed values returned by the ordinary call.
The app may retain or route those values, but it cannot fabricate them from an
identity, host result, or inspection receipt.

## How It Relates To Other Features

- Add file or Rust composition as described in
  [Authored composition](./authored-composition.md).
- Register installed scalar or collection projections before `freeze`; submit
  Query-issued observations through `begin_projection_rebind(...)`.
- Use [Application inspection](./inspection.md) on the prepared app or active
  session.
- Host adapters consume only the sealed mounted mechanics prepared by runtime.

## Inspection And Debugging

Use `generation_identity()` to correlate lifecycle outcomes. Use `inspect(...)`
for generation-bound read-only evidence. Publication and denial cost reports
describe work already performed; materialize rich reports only when needed.

## Anti-Patterns

- Calling a raw frame transition or lane executor.
- Importing runtime, mounting, or publication internals.
- Treating an identity, digest, or receipt as executable authority.
- Assuming every non-success is safe to retry the same way.
- Rebuilding Query or host state inside application callbacks.

## Current Limits

Surface registration, mounting, and allocation setup depend on the
application’s declared capabilities. The empty example is useful for lifecycle
shape; it is not a substitute for a real application’s graph and host setup.

## Platform Pulse

Platform Pulse is the permanent, human-visible application used to prove the
real lifecycle as Worth UI grows. From the repository root, run:

```powershell
cargo run --manifest-path workspaces/worth-ui/Cargo.toml -p worth-ui-platform-pulse
```

That no-argument workflow uses the checked-in source root. To run the same
product composition root against another installation, pass an existing
absolute directory containing `main.wui`:

```powershell
$sourceRoot = (Resolve-Path workspaces/worth-ui/apps/platform-pulse/app).Path
$queryRoot = Join-Path $env:TEMP "worth-ui-platform-pulse-query"
$intentRoot = Join-Path $env:TEMP "worth-ui-platform-pulse-intent"
New-Item -ItemType Directory -Force -Path $queryRoot | Out-Null
New-Item -ItemType Directory -Force -Path $intentRoot | Out-Null
Remove-Item -LiteralPath (Join-Path $queryRoot "platform-pulse-value.json") -ErrorAction SilentlyContinue
Copy-Item workspaces/worth-ui/apps/platform-pulse/intent_samples/ready.json `
  (Join-Path $intentRoot "platform-pulse-intent.json") -Force
cargo run --manifest-path workspaces/worth-ui/Cargo.toml -p worth-ui-platform-pulse -- `
  --source-root $sourceRoot --query-source-root $queryRoot --intent-source-root $intentRoot
```

The process watches
`workspaces/worth-ui/apps/platform-pulse/app/main.wui`. On first launch, the
160-by-96 native client area contains an admitted blue `#2f81f7` background
and a yellow `#f2cc60` inset target. The target occupies the half-open logical
region `[48, 24]` through `[112, 72]`; `[80, 48]` is its inspection point and
`[16, 16]` is the background control point. Both shapes are mounted runtime
meaning translated through the canonical host contract; native mechanics do
not invent a second application-owned shape. Successful first publication prints
`FirstFramePublished` in the `WORTH_UI_PLATFORM_PULSE_EVENT ` stream with the
active application generation and mounted frame identity.

After first publication, the application captures that exact mounted frame,
resolves both points, and temporarily publishes a magenta identity border
around the inset target. The border remains visible for two seconds and then
clears through another successor mounted frame. The console emits this initial
visual sequence inside the current version-9 lifecycle protocol:

```text
VisualSnapshotCaptured
VisualPointTrace
VisualOverlayPublished
VisualOverlayCleared
```

Each is a `WORTH_UI_PLATFORM_PULSE_EVENT ` prefixed JSON envelope. The snapshot
event binds the captured pixels to snapshot, presentation-attempt, frame,
surface, binding-generation, and presentation-epoch identity. In
`VisualPointTrace`, inspect:

```text
outcome.VisualPointTrace.snapshot
outcome.VisualPointTrace.target.hit.mounted.node_receipt
outcome.VisualPointTrace.target.hit.authored_semantic_name
outcome.VisualPointTrace.target.hit.source_artifact_path
outcome.VisualPointTrace.background.hit.mounted.node_receipt
```

The target authored name is
`component:platform.pulse.component.identity_target`. Its mounted receipt must
differ from the background receipt. `VisualOverlayPublished.base_frame` is the
captured frame; `published_frame` is its overlay successor.

For a pretty-printed human view of the same observation stream, run:

```powershell
cargo run --manifest-path workspaces/worth-ui/Cargo.toml -p worth-ui-platform-pulse |
  ForEach-Object {
    if ($_ -like 'WORTH_UI_PLATFORM_PULSE_EVENT *') {
      ($_ -replace '^WORTH_UI_PLATFORM_PULSE_EVENT ', '') |
        ConvertFrom-Json |
        ConvertTo-Json -Depth 20
    } else {
      $_
    }
  }
```

This projection is for reading the receipt stream. It does not create visual
truth or grant authority back to the console.

### Native Actions And Intent Postures

The same page contains a yellow action control and, while confirmation is
pending, a distinct purple confirmation control. Their authored route is a
typed `activate` declaration; the native shell reports pointer observations
and does not own a callback or product effect.

The third launch root watches `platform-pulse-intent.json`. It is a deliberately
small external product boundary that controls operability and the provider
gate. To drive it from another PowerShell window, retain the `$intentRoot` from
the launch command and use:

```powershell
function Set-PulseIntent([int]$Revision, [string]$Operability, [string]$Gate) {
  @{
    protocol = "worth-ui.platform-pulse.intent-source"
    schema_version = 1
    revision = $Revision
    operability = $Operability
    executor_gate = $Gate
  } | ConvertTo-Json | Set-Content `
    -LiteralPath (Join-Path $intentRoot "platform-pulse-intent.json") `
    -Encoding utf8
}
```

With revision 1 `ready` and `held`, click the yellow action. The real native
input crosses presented-frame targeting and produces `IntentPosturePublished`
with `Admitted`, then `IntentExecutorStarted`; the attempt remains held. Release
the external gate without clicking again:

```powershell
Set-PulseIntent 2 ready released
```

The existing attempt completes once. `QueryAction` reports `Executed`, the
separate Query owner issues the fact, `IntentPosturePublished` reports
`Completed`, and mounted text becomes `ACTION 1`. `IntentCausalTrace` ties the
input revision, host sequence, mounted target, route, payload, operability,
admission, attempt, Query fact, successor frame, and independently observed
pixels together without becoming authority.

The confirmation journey is intentionally freshness-sensitive:

```powershell
Set-PulseIntent 3 confirmation_required held
```

Click yellow to issue a `ConfirmationRequired` posture and reveal the purple
control. Then change the product input before accepting it:

```powershell
Set-PulseIntent 4 confirmation_required released
```

Clicking the old purple control reports `StaleConfirmation`; it cannot execute.
Click yellow again to obtain a fresh challenge, then click the newly published
purple control. The admitted attempt completes and mounted text becomes
`ACTION 4`.

Denial is equally visible and effect-free. `Set-PulseIntent 5 disabled released`
or `Set-PulseIntent 6 denied released`, followed by a yellow click, publishes
`Denied` without starting a provider or changing Query. To prove replacement
cancellation, set revision 7 to `ready` and `held`, click yellow, then remove
the `interaction activate routes platform.pulse.action.route;` line from the
identity-target component. The ordinary source rebind publishes a successor
without that route and the predecessor attempt publishes `Cancelled`; restoring
the checked-in source does not resurrect it.

Every admitted product-input revision emits `IntentInputAdmitted`. Malformed,
non-monotonic, missing, relative, or non-directory intent roots stop at their
typed source or launch boundary. They do not silently retain a permissive
boolean or reread the file on each frame.

### Projected Product Data

The checked-in application declares one live scalar projection:
`platform.pulse.status`. At launch, Worth UI prepares the public Query host
installation, the Query host installs it, and the returned scalar registration
joins `WorthUi::app()` before freeze. With no
`platform-pulse-value.json` in the query source root, the mounted status is
pending and the event stream reports `QueryProjectionIssued` followed by
`QueryProjectionPublished` with pending posture.

While the process remains open, create the first external value from another
PowerShell window:

```powershell
$queryRoot = Join-Path $env:TEMP "worth-ui-platform-pulse-query"
'{"status":"ONLINE","revision":1}' |
  Set-Content -LiteralPath (Join-Path $queryRoot "platform-pulse-value.json") -Encoding utf8
```

The operating-system watcher settles that file, Query issues a current native
text observation, and the ordinary projection rebind publishes `ONLINE` as
mounted semantic text in the same window. The event pair carries the exact
projection/fact, application generation, mounted frame, node, presentation,
and pixel correlation.

Then replace only the external value:

```powershell
'{"status":"UPDATED-LONG","revision":2}' |
  Set-Content -LiteralPath (Join-Path $queryRoot "platform-pulse-value.json") -Encoding utf8
```

The visible text changes through the same installation, observation, rebind,
mount, and host path; background and authored identity controls remain fixed.
The executable-world certification also introduces an incompatible declared
schema, proves a typed schema stop preserves the exact predecessor value and
pixels, restores compatibility, and observes recovery. The denial does not
manufacture a fallback value or replace Query authority with UI state.

To exercise a valid replacement, change only this line:

```text
token theme.platform_pulse.fill = "theme.platform_pulse.blue";
```

to:

```text
token theme.platform_pulse.fill = "theme.platform_pulse.green";
```

After the operating-system watcher settles the file, the held snapshot enters
`begin_source_rebind`. Semantic classification selects the affected consumers,
the immutable plan preserves stable authored identity for this color-only
change, and the canonical host publishes one successor. The background becomes
admitted green `#3fb950` while the yellow inset target remains distinct.
`RebindPublished` reports predecessor generation, successor generation, source
revision, planned and realized work, and mounted frame identities.

The Pulse then captures the exact successor and emits `VisualComparison`.
That comparison borrows the predecessor snapshot, successor snapshot, and
published rebind receipt. It reports preserved identity and differing retained
pixels without recapturing either frame. `VisualSnapshotRetired` proves that
the predecessor snapshot was explicitly superseded and its registered resource
was released.

To see denial preservation, replace the file temporarily with:

```text
component platform.pulse.component.seed {
```

The source compiler reports a typed diagnostic through
`RebindDeniedPreserving`. No successor publishes, and the exact green
generation, mounted frame, window, and pixels remain current. Restore the
checked-in source exactly:

```text
component platform.pulse.component.seed {}
component platform.pulse.component.identity_target {
  interaction activate routes platform.pulse.action.route;
}
component platform.pulse.component.projected_status {
  content projection platform.pulse.status
  interaction activate confirms platform.pulse.action.route;
}
surface platform.pulse.surface.main {}
query_scalar platform.pulse.status {
  view platform.pulse.status
  field status
  require text
  lifecycle live
}
intent platform.pulse.action.route {
  definition platform.pulse.action;
  interaction activate;
  payload action_input_revision application-unsigned64 platform.pulse.action.input-revision;
  operability platform.pulse.action.operability
    mutability-application-boolean platform.pulse.action.mutable
    readiness-application-boolean platform.pulse.action.ready
    policy-application-boolean platform.pulse.action.policy-allowed;
  confirmation platform.pulse.action.confirmation
    application-boolean platform.pulse.action.confirmation-required;
  concurrency target-route-single-flight;
  consequences mounted-posture-and-query platform.pulse.status;
}
token theme.platform_pulse.fill = "theme.platform_pulse.blue";
token theme.platform_pulse.identity_target_fill = "theme.platform_pulse.yellow";
token theme.platform_pulse.confirmation_fill = "theme.platform_pulse.purple";
token theme.platform_pulse.projected_status.text = "theme.platform_pulse.white";
```

The same process and window then publish the recovered blue successor through
the same rebind path. Close the native window normally to shut down both
operating-system watchers, settle pending intent input, close the Query live
owner and consumer lease, release the registered host surface, and consume the
active application shutdown path. The terminal `ShutdownCompleted` observation
must report joined Query and intent watchers, zero pending input, and zero live
Query sources, attempts, resources, consumer leases, retained projections, and
projection receipts, alongside zero live captures, snapshots, comparison
projections, rebind handles, pixel bytes, structural bytes, pending overlays,
published overlays, and clearing overlays. The ordinary lifecycle certification
separately proves zero live gesture, draft, admission, confirmation, execution,
recovery, and consequence resources from framework-owned census receipts. The
visual fields include
`cancelled_visual_capture_count`, `disposed_visual_snapshot_count`,
`disposed_visual_pixel_bytes`, `disposed_visual_structural_bytes`,
`cancelled_pending_overlay_count`, `disposed_published_overlay_count`, and
`disposed_clearing_overlay_count`.

### Current Certification Posture

The workflows have three deliberately separate claims:

1. The commands above are human product-entry workflows. They run the actual
   `main`, Worth native window loop, source watcher, and public
   application lifecycle.
2. The consolidated in-process integration lane proves the production
   filesystem watcher, public application shell, mounted publication,
   replacement, denial preservation, shutdown receipts, and host-contract presentation
   inside the certification process:

   ```powershell
   cargo test --manifest-path workspaces/worth-ui/Cargo.toml -p worth-ui-certification --test application_contracts
   ```

   It does not claim executable product entry or an operating-system window.
3. The permanent executable-world lane launches Cargo's exact pulse binary
   against an isolated copy of the canonical source, applies edits outside the
   child, and joins process-bound native pixels to typed product lifecycle
   observations:

   ```powershell
   cargo test --manifest-path workspaces/worth-ui/Cargo.toml -p worth-ui-platform-pulse --features executable-world --test executable_world -- --nocapture
   ```

   On Windows this lane is executable-certified. Other platforms retain an
   explicit compile-only posture until their native adapters run in a required
   real lane; a successful compile is not native certification.

Product stdout prefixes each versioned JSON lifecycle envelope with
`WORTH_UI_PLATFORM_PULSE_EVENT `. The observation-only pulse library decodes
the stream, rejects unsupported versions, foreign runs, sequence gaps, events
after termination, and byte/event budget overruns, and never grants product
authority back to the runner.

An uncaught executable-world failure retains a bounded diagnostic directory
named `worth-ui-platform-pulse-failure-<pid>-<ordinal>` under the operating
system temp directory. Its `manifest.json` records the primary failure,
independent teardown disposition, platform posture, and accepted lifecycle
trace; `source.wui` records the final isolated source when available. Each
bundle is capped at 64 MiB and is retained by default for diagnosis. Expected
hostile tests explicitly discard their bundles after asserting the contents.
A passing run removes its sandbox, closes its native window, joins its
lifecycle reader, leaves no child process, and creates no retained bundle.

[Milestone 3.10.3](../../../_docs/worth-ui/milestone-3.10.3.md) closed this
corrective executable-world foundation. Later Platform Pulse milestones extend
these same human, integration, and executable lanes rather than creating a new
composition root or universal fixture.

## Related Docs

- [Worth UI architecture](./architecture.md)
- [Authored composition](./authored-composition.md)
- [Interaction and intents](./interaction-and-intents.md)
- [Hot rebind](./hot-rebind.md)
- [Application inspection](./inspection.md)
- [Query-backed UI views](./query-binding.md)
