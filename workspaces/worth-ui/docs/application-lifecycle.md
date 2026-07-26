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

## Related Docs

- [Worth UI architecture](./architecture.md)
- [Authored composition](./authored-composition.md)
- [Application inspection](./inspection.md)
- [Query-backed UI views](./query-binding.md)
