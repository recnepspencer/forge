# Application Lifecycle

## What This Feature Is

The application lifecycle prepares, launches, runs, inspects, and shuts down
one Worth UI application while the platform keeps graph, Query, host, mounted
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
- `WorthUiApplicationBuilder::freeze()`
- `WorthUiApp::launch()`
- `WorthUiActiveApplicationSession::execute_mounted_frame(...)`
- `WorthUiActiveApplicationSession::inspect(...)`
- `WorthUiActiveApplicationSession::shutdown()`
- `UiMountedFrameOutcome`
- `WorthUiMountedFrameExecutionStop`

Mounted request, deadline, outcome, and recovery types are re-exported by
`worth_ui::facade::app`. Application code does not import a mounted runtime
module.

## Core Mental Model

`freeze` prepares one inseparable application generation: authored meaning,
capabilities, graph, Query bindings, host plan, and inspection indexes. Launch
consumes that prepared app and returns the only ordinary running owner.

`execute_mounted_frame` collects admitted inputs, advances the runtime,
assembles all required surfaces, presents through the host contract, and
publishes only a complete frame. A receipt reports what happened; it cannot be
used to execute or publish another frame.

## How It Executes

```text
WorthUi::app()
-> register capabilities, source, Query views, and host
-> freeze
-> launch
-> register/mount application-specific surfaces as required
-> execute_mounted_frame
-> Published | Unchanged | Reconciled
   | RejectedBeforeEffects | InFlight | PresentationIndeterminate
   | RetentionDenied | AdmissionDenied | CompletionDenied
-> inspect or continue through the returned typed outcome
-> shutdown
```

A pre-effect denial leaves the predecessor publication current. If native
effects may have started, the runtime retains semantic truth but marks affected
bindings uncertain until the typed recovery path completes.

## Small Example

```rust
use worth_ui::facade::app::{
    UiMountedFrameOutcome, UiMountedFrameRequest, UiPresentationDeadline, WorthUi,
};

let app = WorthUi::app()
    .freeze()
    .expect("application preparation should succeed");
let mut session = app.launch().expect("application should launch");

let outcome = session
    .execute_mounted_frame(
        UiMountedFrameRequest::all_bound_surfaces(),
        UiPresentationDeadline::at_tick(1),
        0,
        |_sources| {},
    )
    .expect("mounted-frame transition should start");

match outcome {
    UiMountedFrameOutcome::Published(receipt)
    | UiMountedFrameOutcome::Unchanged(receipt)
    | UiMountedFrameOutcome::Reconciled(receipt) => {
        observe_publication(receipt);
    }
    other => handle_non_success(other),
}
```

This is the smallest honest visible-frame call. Query-free applications do not
create dummy Query work.

## Executable Fresh-Reader Contract

The following is the exact downstream program used by the compiler-contract
matrix and the application-contract runtime suite. It uses only the ordinary
product facade, branches over every mounted outcome and start-stop family, and
executes the real empty-application path.

<!-- compile-run:ordinary-mounted-frame -->
```rust
use worth_ui::facade::app::{
    UiMountedFrameOutcome, UiMountedFramePublicationReceipt, UiMountedFrameRequest,
    UiMountedFrameRetentionRejection, UiMountedIndeterminateFrame,
    UiMountedPresentationAdmissionRejection, UiMountedPresentationCompletionDenial,
    UiMountedPresentationInFlight, UiMountedRejectedFrame, UiPresentationDeadline, WorthUi,
    WorthUiMountedFrameExecutionStop,
};

fn main() {
    run();
}

pub fn run() {
    let app = WorthUi::app()
        .freeze()
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
    }
}

fn observe_stop(stop: &WorthUiMountedFrameExecutionStop<'_>) {
    match stop {
        WorthUiMountedFrameExecutionStop::PublicationLease(_) => {}
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
- Register installed Query views before `freeze`; submit settled projection
  input inside the mounted-frame source closure.
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
cargo run --manifest-path workspaces/worth-ui/Cargo.toml -p worth-ui-platform-pulse -- --source-root $sourceRoot
```

The process watches
`workspaces/worth-ui/apps/platform-pulse/app/main.wui`. On first launch, the
160-by-96 native window is filled with the admitted blue `#2f81f7` rectangle.
The rectangle is mounted runtime meaning translated through the canonical host
contract; the eframe shell does not draw a second application-owned shape.
Successful first publication prints
`WORTH_UI_PLATFORM_PULSE_PUBLISHED` with the active application generation and
mounted frame identity.

To exercise a valid replacement, change only this line:

```text
token theme.platform_pulse.fill = "theme.platform_pulse.blue";
```

to:

```text
token theme.platform_pulse.fill = "theme.platform_pulse.green";
```

After the operating-system watcher settles the file, the complete successor is
prepared and published atomically. The window becomes admitted green
`#3fb950`, and `WORTH_UI_PLATFORM_PULSE_REPLACED` reports the predecessor,
successor, and published frame identities. This is whole-application
replacement, not Milestone 3.12 semantic rebind.

To see denial preservation, replace the file temporarily with:

```text
component platform.pulse.component.seed {
```

The source compiler reports a typed diagnostic. No successor publishes, and
the last admitted green generation and pixels remain visible. Restore the
checked-in source exactly:

```text
component platform.pulse.component.seed {}
surface platform.pulse.surface.main {}
token theme.platform_pulse.fill = "theme.platform_pulse.blue";
```

The watcher then publishes the recovered blue application. Close the native
window normally to shut down the operating-system watcher, release the
registered host surface, and consume the active application shutdown path.

### Current Certification Posture

The workflows have three deliberately separate claims:

1. The commands above are human product-entry workflows. They run the actual
   `main`, `eframe::run_native`, native window loop, source watcher, and public
   application lifecycle.
2. The consolidated in-process integration lane proves the production
   filesystem watcher, public application shell, mounted publication,
   replacement, denial preservation, shutdown receipts, and egui translation
   inside the certification process:

   ```powershell
   cargo test --manifest-path workspaces/worth-ui/Cargo.toml -p worth-ui-certification --test application_contracts platform_pulse
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
- [Application inspection](./inspection.md)
- [Query-backed UI views](./query-binding.md)
