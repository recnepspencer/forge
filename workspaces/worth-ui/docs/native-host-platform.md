# Native Host Platform

## What This Feature Is

The native host platform is the framework-owned application and host boundary.
It binds one Worth application to one qualified native profile, runs its native
window and graphics lifecycle, and closes its resources without giving product
code a raw adapter, event-loop client, graphics object, or wake port.

Platform and application preparation remain effect-free. No window, event
loop, surface, device, queue, or physical-work worker exists until the
application returns a prepared definition. After that gate succeeds,
`run(...)` enters the qualified native host and may perform native effects.

## Stable Entry Points

- `WorthUiNativePlatform::prepare(UiNativePlatformProfile)`
- `UiPreparedNativePlatform::run(UiNativeApplicationDefinition)`
- `UiNativeApplicationPreparation::builder()`
- `UiNativeApplicationPreparation::complete()`
- `UiNativeApplicationPreparation::deny(...)`
- `UiNativeApplicationPreparationOutcome`
- `UiNativePlatformOutcome`

The platform privately issues `UiNativePlatformBindingGrant`. The grant,
preparation scope, prepared application, and denial payload are move-only and
have private fields. There is no parts conversion or host replacement route.

## Preparation Progression

```text
qualified UiNativePlatformProfile
-> WorthUiNativePlatform::prepare
-> UiPreparedNativePlatform with one application slot
-> run(application definition)
-> UiNativeApplicationPreparation with the platform-bound Worth builder
-> application registration through a borrowing builder view
-> Prepared(UiPreparedNativeApplication)
   | Denied(UiNativeApplicationPreparationDenial)
-> qualified native host and event-loop execution
-> Closed(UiNativePlatformCloseReceipt)
   | Stopped(UiNativePlatformStopReport)
```

Calling `builder()` borrows the internal builder. It cannot extract or freeze
it, replace the native host, or retain a second application lane. `complete`
consumes the whole preparation scope and is the only way to produce a prepared
native application.

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
let prepared_platform = WorthUiNativePlatform::prepare(profile)?;
let outcome = prepared_platform.run(Application);
```

An application-preparation denial still occurs before native effects or native
host construction. Once preparation succeeds, `run(...)` returns either a
close receipt or a typed stop report. Both expose the terminal resource census;
a stop with retained external obligations also exposes cleanup authority.

## Presentation Contract

Runtime owns presentation meaning and emits one sealed revision-4 work item:

- `Initial` carries every attributed command, stable total order, initial
  logical damage, auxiliary reconstruction state, and the surface-issued
  transparent baseline.
- `Delta` carries only command changes, affected order edits, owner-issued
  logical damage, and an auxiliary successor when that meaning changed.
- `Unchanged` carries exact predecessor/successor affinity and no command,
  order, damage, or native work.

Host mechanics retain commands by owner-issued identity. They may build
mechanical indexes for execution, but they do not receive the complete
projection on ordinary successor frames and do not rediscover semantic deltas.
Candidate retained state commits only after every required surface succeeds.

## Physical Work Progression

Host-native owns physical resources and effects. The device owner retains the
adapter/device/queue generation, while the presentation-surface owner retains
the surface and current target. Concrete WGPU and Winit handles remain inside
the WGPU backend; the graphics port carries only associated backend mechanics.
The native adapter's observation of an external effect is the source of
physical completion truth.

A private physical Signal runtime inside host-native owns how that work
progresses. It admits bounded work, tracks the current physical attempt, emits
exact readiness wakes, and governs retry, timeout, cancellation, supersession,
recovery scheduling, and shutdown ordering. One runtime is retained for each
native host/device lifecycle; it is not created per presentation.

```text
runtime presentation work
-> host-native reserves physical owners
-> private physical Signal runtime admits bounded work
-> Winit readiness transport wakes the native event thread
-> native adapter submits or polls the external effect
-> typed physical observation returns to Signal
-> Signal settles, retries, supersedes, or schedules recovery
-> host-native commits or releases the physical owners
```

Signal owns progression, not effects. It does not store raw WGPU handles,
allocate atlas storage, submit command buffers, poll the device, or decide that
an external consequence completed. Readiness is eligibility to progress work;
it is not completion evidence or effect authority. The Winit readiness
registry transports wakes only and does not provide a second retry or
currentness scheduler.

An observation must match the exact owner-issued physical runtime, work, and
attempt. Stale, duplicate, foreign, or superseded observations cannot settle
current work. A rejection known to occur before effects releases its owners.
An effects-indeterminate observation enters retained recovery until host-native
can reconcile or close the physical consequence.

Shutdown first stops new admissions, then drains retained completion and
recovery obligations. Signal state and native resource ownership are disposed
only after those obligations reach a terminal posture.

## Presented-Source Readback

The Windows native host records one capture source only after presentation has
settled successfully. That source binds the host session, frame, presentation
attempt, surface, binding generation, presentation epoch, current client
transform, and the retained realized regions that produced the pixels. A draw
list, reconstructed image, or later compositor capture cannot substitute for
that identity.

Capture admission is deliberately two-step:

```text
request -> reserve one bounded logical slot and padded byte capacity (no GPU effect)
poll    -> revalidate the exact source, allocate owners, and submit the copy
poll    -> Pending | canonical tight RGBA8 pixels | completion indeterminate
```

The native owner permits at most four readback slots and 16 MiB of aggregate
padded capacity. WGPU's 256-byte row alignment remains internal; successful
results remove row padding and expose top-left, tightly packed RGBA8 bytes.
Resize, scale, binding, presentation, and derived-state changes invalidate
unsubmitted source reservations. Already submitted copies retain their exact
predecessor source until terminal observation. Cancelling the caller's request
does not cancel or forget physical work that may already have begun.

Cancellation before submission proves that readback did not begin.
Cancellation after submission can prove only that readback may have begun. A
device timeout remains pending. An unknown physical completion enters bounded
capture recovery and keeps its slot, padded bytes, readback buffer, and pending
submission owner charged. A map failure or malformed artifact observed after
GPU settlement is completion-indeterminate but physically releasable. Ordinary
admission and governed close retries progress retained recovery; host graphics
remain live until it settles. Runtime snapshot retention and disposal remain
runtime policy, not native policy.

## Relationship To Query Invalidation

The physical Signal runtime is separate from the Query-semantic Signal graph.
The physical runtime schedules native work; Query tracks which
application-visible presentation meaning is pending, current, stale, failed,
cancelled, superseded, or unresolved. Typed physical completion evidence may
later be admitted to the installed Query correspondence, but Query does not
submit, poll, retry, wake, recover, or release WGPU work.

The two graphs do not share runtime identities, aspect slots, request handles,
completion envelopes, capacities, or shutdown receipts. Runtime imports
neither Signal nor Query, and no separate manual physical scheduler runs beside
host-native's private Signal owner.

## Qualification

The checked-in qualified identities are:

- text profile: `worth-ui-body-default-v1`;
- native profile: `worth-ui-windows-dx12-v1`;
- protocol: revision 4;
- observation schema: revision 6;
- native dependency versions: `winit 0.30.13`, `wgpu 29.0.4`,
  `rustybuzz 0.20.1`, and `swash 0.2.10`.

The manifest digests are fixed by the milestone specification and verified by
the native host qualification tests. The profile is not selected from ambient
system fonts, an environment variable, or an adapter default.

## Cost And Failure Posture

Presentation reports structural and physical amplification separately:
delta rows, draw-list and order mutations, damage regions, index probes,
intersections, replay, cleared/rendered/presented pixels, GPU writes, passes,
copies, acquisitions, submissions, and presents. Unchanged reports exact zero
for all of them.

A pre-effect denial preserves the current publication. If an effect may have
started, the affected binding becomes indeterminate and must be reconciled or
closed. A successful surface in a partially failed multi-surface attempt does
not promote the candidate frame.

## Recovery And Hostile Close

Timeout and occlusion do not spend reconstruction authority in a local retry
loop. The runtime retains the exact mounted frame and reconstruction authority,
then admits another attempt only after a later qualified redraw or visibility
readiness generation. Validation rejection remains pre-effect. Surface
outdated, surface loss, device loss, and indeterminate completion enter the
typed recovery registry.

Physical graphics recovery is global to the native host generation, not local
to one semantic binding. One pending recovery epoch prepares and commits one
generation-bearing device/surface fact. Every affected binding must consume
that same fact through its own semantic reconstruction before the epoch can be
released. A stronger device-loss requirement supersedes a previously prepared
surface-only epoch instead of opening a parallel recovery lane.

Zero-sized Windows surfaces suspend presentation. Restoring a nonzero client
area publishes a later readiness generation and a monotonic successor-barrier
ordinal; retained reconstruction resumes from current mounted authority and
presents a distinct product frame. A later minimize/restore cycle cannot reuse
the earlier title/barrier observation. The Windows lifecycle courtroom verifies
each successor presentation before it accepts compositor pixels.

Close uses one ordered shutdown progression: stop admission, settle external
presentation and readback obligations, release derived/recovery state, release
native resources, then publish `Closed` only when the complete census is zero.
Close during preparation, any presentation stage, readback, queued readiness,
or a held application attempt uses this same progression. Certification
observes the production readiness registry and runtime-issued mounted attempt;
it does not mint census-only stand-ins.

## Current Limits

The physical Signal runtime is private host-native machinery, not an
application-facing scheduling API. Application-visible async presentation
posture and the native-completion-to-Query correspondence are still being
stabilized and are not public control surfaces. Product code cannot select a
Signal runtime, supply raw physical handles, or replace native recovery policy.

## Anti-Patterns

- Do not call a product-facing host selector or keep a default hidden host.
- Do not import runtime internals from a host or platform crate.
- Do not pass a complete projection to a successor-frame host operation.
- Do not treat a profile digest, baseline identity, or cost report as authority.
- Do not infer success after an uncertain or partially completed native effect.
- Do not run a manual retry, timeout, currentness, or wake scheduler beside the
  private physical Signal runtime.
- Do not route physical WGPU progression through Query or treat semantic
  invalidation as permission to perform native effects.
- Do not give Signal raw WGPU handles, atlas storage ownership, or effect
  authority.
