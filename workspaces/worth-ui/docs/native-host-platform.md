# Native Host Platform

## What This Feature Is

The native host platform is the framework-owned application and host boundary.
It binds one Worth application to one qualified native profile without giving
product code a raw adapter, event-loop client, graphics object, or wake port.

Milestone 3.14.1 Phase 1 is intentionally effect-free. It freezes the public
authority progression, protocol revision, capacities, dependency direction,
and qualification identities. It does not open a window, event loop, surface,
device, queue, watcher, or worker.

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
-> Phase 1 stop with exact terminal resource census
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

Phase 1 returns a typed stop before native effects and reports zero live
resources. A preparation denial reports its preparation identity, cause,
reverse closure count, readiness closure count, zero terminal census, and
`event_loop_client_published: false`.

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

## Qualification

The checked-in Phase 1 identities are:

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

## Current Limits

Phase 1 has no native effects. Window/event-loop/device activation, native
filled-rectangle presentation, retained damage replay, text shaping/raster,
input/IME, capture, and full shutdown fault injection belong to the ordered
later phases of Milestone 3.14.1.

## Anti-Patterns

- Do not call a product-facing host selector or keep a default hidden host.
- Do not import runtime internals from a host or platform crate.
- Do not pass a complete projection to a successor-frame host operation.
- Do not treat a profile digest, baseline identity, or cost report as authority.
- Do not infer success after an uncertain or partially completed native effect.
- Do not activate later-phase effects behind a Phase 1 placeholder API.
