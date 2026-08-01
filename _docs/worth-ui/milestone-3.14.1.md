# Milestone 3.14.1: Aspect-Native Host Platform and egui Retirement

## Status and Placement

Status: planned specification for the nine-phase slice immediately after
Milestone 3.14 and before Milestone 3.15.

Milestones 3.10-3.14 established one real application lifecycle, exact mounted
identity, observation-driven rebind, projected product data, and
presentation-bound interaction and intent. This milestone replaces the
interim `egui`/`eframe` mechanics beneath those contracts before portal,
focus, motion, appearance, and later native services would otherwise be built
against a host scheduled for deletion.

The milestone is deliberately broad in native mechanics and narrow in product
meaning. It does not reopen authored meaning, Query authority, targeting,
intent admission, consequence publication, or appearance semantics.

Treat this as the largest single 3.x platform-migration slice. Its nine phases
are proof and authority gates, not equal units of effort; implementation
planning must budget roughly one quarter to one third of the milestone for
Phase 1. Earlier line-count estimates are not scope limits or acceptance
criteria, and they do not license compressed trust handoffs, deferred
qualification, or reduced proof.

## Goal and Central Claim

The same mounted frame and observation contracts drive a Worth-owned native
platform. A native mechanics crate whose only WUI dependency is the host
contract owns the event loop, window, device, surface, retained draw list,
text, input translation, capture, and shutdown; a higher application-platform
crate composes those mechanics with the public Worth UI lifecycle. Runtime
issues exact presentation work and total paint order, and the host lowers it
into attributable pixels without rediscovering UI meaning. After parity is
proved, every `egui` and `eframe` dependency and ordinary path is deleted.

The closure claim is:

```text
admitted runtime change
-> owner-issued Initial | Delta | Unchanged presentation work
-> receipt-keyed retained native commands
-> bounded shaping, atlas, and GPU effects
-> presented-source pixels plus compositor-visible client pixels,
   or typed non-success at their owning boundary
-> existing publication, observation, capture, and recovery contracts
```

Every native draw command remains causally bound to mounted node receipt,
frame, surface, and binding generation. Derived draw-list, atlas, backing
texture, and GPU state can be destroyed and reconstructed from the current
mounted projection without changing application truth.

### Explicit non-goals

- new authored layout, appearance, component, portal, focus, motion, command,
  selection, or accessibility semantics;
- a Worth-owned text shaper, font manager, IME composition engine or service,
  compositor, or GPU API; host translation of OS IME observations into the
  inherited 3.14 contract remains required;
- system-font discovery or renderer-selected fallback;
- complex-script certification beyond the explicitly qualified
  `BodyDefault` support set;
- general vector tessellation, arbitrary paths, images, icons, canvas, or
  realtime rendering;
- platforms beyond the currently executable-certified Windows lane;
- a second application composition root, binary, executable-world harness,
  or permanent host-selection mode.

## Current Boundary and Required Closure

| Current fact | Defect exposed by native ownership | Required closure |
| --- | --- | --- |
| `UiMountedFrameConsumptionView` carries a complete projection. | A host can rescan or diff the whole projection and become a second damage planner. | Runtime/mounting issues typed initial, exact delta, or unchanged presentation work. |
| Paint mechanics carry `layer_semantic_order`. | Equal-layer commands have no host-independent total order, while dense replacement ordinals would amplify local insertion. | Mounting issues stable order identities plus an initial total sequence or exact relative order edits. |
| `BodyDefault` records size, weight, clipping, and baseline posture. | egui still selects the concrete font and metrics. | One repository-pinned font profile, asset digest, support posture, and shared measurement/shaping configuration are frozen before native text ships. |
| Platform Pulse implements `eframe::App` and requests continuous repaint. | Product code owns platform scheduling and unchanged turns still reach native presentation. | The Worth host owns the event loop and wakes only for typed OS, application-readiness, deadline, capture, or recovery causes. |
| egui dependencies exist outside `worth-ui-host-egui`. | Deleting one adapter leaves a false retirement claim. | Certification and Pulse migrate; the isolated egui-era theme/component crates are removed rather than prematurely ported. |

The required host-contract additions refine transport and proof carriage; they
do not change mounted semantic meaning. If implementation discovers that
native input cannot preserve a 3.14 interaction distinction, mounted rows do
not determine deterministic pixels, or measurement cannot preserve the
declared text profile, that is a reopened predecessor finding. A lossy shim or
adapter-local policy is not an allowed migration answer.

## Decisive Product Courtroom: `HP-01`

### Real world

Run `PulseNativeParityWorld` through the existing `worth-ui-platform-pulse`
binary, checked-in `main.wui`, public application composition root, OS
filesystem and Query watchers, native window, public intent provider, visual
snapshot contract, lifecycle stream, and existing executable-world target on
executable-certified Windows.

During coexistence, a migration-only launch grant selects either the egui host
or Worth native host inside the same binary. It is not a product setting and
is deleted at cutover. The external runner applies the same causal actions and
adjudicates both processes through OS-delivered input, process lifecycle,
actual client-area pixels, and the versioned Pulse observation stream.

### Hostile sequence

1. Launch with no Query value and observe the pending frame, attributed blue
   background, yellow identity target, semantic text posture, and first-frame
   publication.
2. Publish the first current value, replace it with a longer second value, and
   prove the text pixels and mounted correlation follow the existing Query
   path.
3. Exercise the 3.14 native activation, denied operability, confirmation,
   cancellation, completed product effect, and visible consequence actions by
   OS input against the exact presented frame.
4. Run the identity snapshot, point trace, overlay publication, surface
   readback, comparison, overlay clear, and explicit snapshot retirement.
5. Apply the green source edit, malformed source, exact predecessor
   preservation, restored blue source, incompatible Query schema, schema stop,
   and compatible recovery without replacing the window or application
   composition root.
6. Leave the application quiescent across an observed idle interval and issue
   an explicit unchanged mounted-frame attempt in the in-process companion
   world. Neither path may mutate the draw list, shape or rasterize glyphs,
   acquire a surface, encode commands, submit GPU work, or present.
7. Close the real native window while no product mutation is pending. Join all
   workers and require zero live window, surface, device, queue, backing
   texture, draw-list, GPU-buffer, atlas, readback, capture, watcher, Query,
   intent, and application resources.

### Required verdict

- Both hosts produce identical semantic, mounted receipt, frame, binding,
  interaction, intent, consequence, and lifecycle evidence for the same
  versioned causal-action manifest. The recorded migration grant is the only
  differing world input. Host-mechanical identities and timestamps are
  excluded only by a named parity contract, never by a generic normalization
  pass.
- Native filled-rectangle control points retain the established pixel values.
  Glyph-region expectations change exactly once. The rebaseline record names
  predecessor and successor pixels, font asset digest, text profile,
  dependency versions, DPI, surface/color posture, and external adjudication.
- Every native pixel contribution resolves through a retained command to its
  exact mounted receipt and generation set.
- A denied or indeterminate native effect preserves the prior semantic
  publication and returns the existing typed recovery posture; no renderer
  state is promoted into application truth.
- Unchanged work reports exact zero through every named native counter.
- After native parity closes, rerunning the courtroom is possible only through
  the native host because the migration selector, egui shell, adapter, and
  dependency edges no longer exist.

### Defects this courtroom must convict

The courtroom turns red if an implementation repaints continuously, diffs the
full projection in the host, loses command receipts during preparation,
retargets input to a newer frame, invents geometry or order, captures an
internal data structure instead of rendered pixels, silently changes font
fallback, publishes after an uncertain effect, leaks native resources, or
keeps egui as a fallback.

## Supporting Proof Portfolio

### `HP-02`: Delta and amplification courtroom

Use `MountedPresentationWorld::maximum_overlap` to build one causally valid
mounted world at the current maximum admitted sizes: 2,048 filled rectangles,
2,048 semantic-text rows, and the 1 MiB text-byte ceiling. Replace one text
row, remove one rectangle, and insert one rectangle while all other rows remain
identical. Then submit an unchanged successor.

Command mutation and order planning must consume work proportional to the
three changed commands plus explicitly affected order/damage entries, not all
retained commands. Physical damage replay additionally consumes exactly the
retained commands whose visible bounds intersect admitted damage, in total
paint order. Removing or moving a command clears its vacated region and
replays every intersecting predecessor command required to restore the
correct pixels. A damage index narrows that set without a retained-list scan;
lawful complete overlap may still require complete replay and must report it.

Named counters distinguish rows carried in the delta, draw-list and order
mutations, damage-index probes, intersecting and replayed commands, cleared,
rendered, and presented pixels, GPU writes, render passes, surface copies,
acquisitions, submissions, and presents. If the native surface requires a
full-area copy or present after local rendering, that physical amplification
is reported separately and is never relabeled local.

An independent ordered-pixel model adjudicates command retention, removal,
clipping, overlap, and equal-layer order. Mutation controls remove delta
carriage, force a full scan, widen damage, or change the total-order tie break;
each must fail for its named reason.

### `HP-03`: Text and atlas courtroom

Use `MountedPresentationWorld::qualified_text` with the pinned `BodyDefault`
asset, repeated glyphs, new glyphs, clipped runs, maximum lawful run bytes,
unsupported code points, atlas capacity pressure, live-command pinning,
eviction, DPI replacement, and destroyed derived text state.

Measurement and rendering must use the same selected profile and shaping
configuration. Unsupported text denies before presentation effects with a
typed support outcome; it never invokes a system fallback. Eviction cannot
remove glyphs referenced by the retained draw list. Destroying the atlas and
glyph buffers followed by reconstruction from mounted authority produces the
same qualified pixels and does not alter semantic publication.

### `HP-04`: Native lifecycle courtroom

Drive the exhaustive schedule through `NativeLifecycleProtocolWorld`, then run
the environment-real subset through `WindowsNativeBoundaryWorld`: input before
the first completed presentation and while a successor is in flight; zero-
sized/minimized surfaces; resize and DPI changes around input; surface lost/
outdated/timeout posture; device loss; capture readback in flight; close with
application readiness queued; close during prepared upload; close after
submission but before completion; and recovery followed by ordinary
presentation. Cross close with one held 3.14 application attempt so platform
shutdown proves the inherited cancellation or settlement posture rather than
only native-resource disposal.

Against one exact runtime-owned local draft recipient, deliver Windows IME
preedit, composition commit, cancel, and native-range conversion, then repeat
with no recipient, stale presentation/input affinity, unsupported text, and an
unprovable range. The oracle distinguishes visible preedit, committed draft
text, semantic `edit-commit`, payload formation, and intent effects so no host
event can collapse those phases or manufacture focus authority.

Input coordinates use the DPI and presentation basis observed at event time.
Every before-effect stop, in-flight state, indeterminate effect, recovery
handle, reconstruction, and shutdown disposition is typed. Fault injection
proves protocol logic but does not claim a real GPU failure; required native
integration separately crosses the actual window, surface, device, and
readback boundary.

### `HP-05`: Retirement and topology courtroom

`HostRetirementTopologyWorld` uses workspace metadata, dependency inversion,
source inventory, public examples, boundary checks, and positive/negative
compile twins to prove:

- zero `egui`, `eframe`, and `egui_extras` workspace dependency edges;
- no `WorthUiHostKind::Egui`, `WorthUiHostContract::egui`, egui adapter,
  eframe shell, migration selector, or compatibility facade;
- no vendor event, window, surface, font, or GPU types in the `worth-ui`
  product facade or host contract;
- host adapters cannot import runtime internals or mint mounted presentation
  work; and
- external callers cannot construct receiptless draw commands or promote
  capture, diagnostics, lifecycle, or host identities into authority.

## World Architecture and Fixture Provenance

A courtroom world is a typed, causally complete baseline plus one named
scenario delta. It is not a bag of constructors. World construction,
action, observation, and teardown return distinct typed outcomes, and every
valid identity, receipt, generation, resource, and authority handle comes from
the production owner or a narrow world compiler proven to establish the same
postconditions. Raw literals are reserved for invalid-input, protocol, pixel-
contract, and corruption claims where the literal is itself disputed.

The milestone uses four stateful worlds and one static proof world inside the
existing test targets. They do not create another product composition root,
binary, executable-world harness, Cargo target, or mutable universal fixture.

| Courtroom | Governing world | Honest proof boundary |
| --- | --- | --- |
| `HP-01` | `PulseNativeParityWorld` | real Cargo-built product process, public composition root, OS window/input/capture, product evidence, and explicit teardown |
| `HP-02` | `MountedPresentationWorld::maximum_overlap` | public Worth application through production lowering, graph, mounting, presentation-work issuance, native-host execution, and independent ordered-pixel model |
| `HP-03` | `MountedPresentationWorld::qualified_text` | the same mounted authority with the committed text profile, independent glyph/atlas models, external pixel contract, and derived-state destruction |
| `HP-04` | `NativeLifecycleProtocolWorld` plus `WindowsNativeBoundaryWorld` | deterministic production lifecycle protocol under injected external outcomes, separately joined to the real Windows event-loop/window/surface/device/readback boundary |
| `HP-05` | `HostRetirementTopologyWorld` | workspace metadata, supported build configurations, source/dependency inventories, compile twins, examples, and post-deletion executable reachability |

### `PulseNativeParityWorld`

This world extends the existing executable Platform Pulse machinery. It
installs byte-identical checked-in `.wui`, Query, and intent inputs into fresh
isolated roots for each host run and records one versioned causal-action
manifest containing every external edit, input action, observation checkpoint,
ordering edge, and deadline. The egui/native migration grant is
the only differing input, and the world records both grant identity and action-
manifest digest. Runs are sequential under the existing exclusive native-
desktop lease; they share no mutable installation or process state.

The external pixel oracle consumes a versioned, test-owned control-point
manifest containing literal logical coordinates, expected RGBA values and
tolerances, DPI/profile identity, and the qualified font digest. It may share
stable wire and identity types, but it may not import production geometry,
order, color-selection, shaping, rasterization, damage, or expected-image
functions. Phase 8 changes glyph-region expectations exactly once through the
ledgered rebaseline; native source-target readback and OS client capture remain
separate observations. Failure retains the action manifest, profile, source,
bounded lifecycle trace, both pixel observations where available, and complete
teardown disposition.

### `MountedPresentationWorld`

The mounted world compiler enters through the public Worth application facade,
production file- or Rust-authored lowering, graph construction, surface
registration, allocation, mounting, and runtime-owned presentation-work issuer.
It returns semantic handles such as `world.surface`, `world.initial`,
`world.text_row`, and `world.current_presentation`; it cannot expose raw
identity constructors, host command constructors, atlas entries, GPU offsets,
or a method that asks the host to invent a delta.

`maximum_overlap` compiles one immutable maximum-size baseline once and applies
isolated owner-issued deltas for sparse, partial-overlap, and complete-overlap
regimes. `qualified_text` uses the same authority path with repeated/new/
unsupported glyph classes, exact capacity boundaries, DPI/profile replacement,
and reconstruction. The ordered-pixel oracle implements clipping, clearing,
overlap, and total order without calling production ordering, damage, draw-list,
or rendering code. The atlas oracle independently models capacity, pinning,
candidate eviction, and retained bytes without calling the production eviction
classifier. Reuse recompiles from immutable semantic baselines or reconstructs
isolated derived state; no proof-bearing authority or resource handle is cloned,
and no scenario shares mutable draw-list, atlas, device, or counter fate.

### `NativeLifecycleProtocolWorld`

This world runs the production native lifecycle orchestrator against
contractual window, surface/graphics, and readback ports whose real
implementations own vendor calls. Scripted implementations inject only named
external observations from acquisition, encoding/submission, present-handoff,
and readback calls. Pre-effect preparation denials arise from public malformed,
stale, unsupported, or over-capacity inputs, never an injected lifecycle
verdict. Scripted ports cannot return the expected platform verdict, mint
presentation epochs, change application truth, or replace the orchestrator.
They exist because the external effects are real boundaries; there is no
`cfg(test)` lifecycle, alternate composition root, or validation weakening.

An independent exhaustive transition model consumes the same fault schedule
and predicts effect posture, predecessor retention, recovery authority,
resource census, and valid next actions without calling the production state
machine. This world proves protocol behavior only. It cannot claim a real GPU
failure, OS event, compositor result, or vendor cleanup.

### `WindowsNativeBoundaryWorld`

This world enters through `WorthUiNativePlatform` and the existing Platform
Pulse application in the one serialized native courtroom. It crosses the real
Windows event loop, process-bound window, observed startup DPI, resize/
minimize/restore, surface/device acquisition, OS input, presentation-source
readback, compositor-visible capture, normal close, worker join, and residue
checks. It also closes once with queued readiness and a held 3.14 application
attempt.

Only environment-qualified facts receive real-boundary credit. A single-
monitor runner proves its observed DPI basis but not a cross-monitor DPI
transition; injected DPI/device-loss schedules remain protocol evidence.
Conversely, a real successful surface/device/readback path cannot substitute
for the deterministic partial-effect matrix. The ledger names which world
supports each `HP-04` claim.

### `HostRetirementTopologyWorld`

The retirement world reads real workspace metadata and source on the exact
post-cutover tree. It audits default and all-feature workspace graphs, every
supported Windows target/profile, binaries, examples, test fixtures, lockfile,
boundary rules, and source/doc inventories. Positive twins prove the native
and headless homes remain lawful; negative twins prove forbidden dependency
and authority directions. It cannot use a curated crate list, default-feature-
only metadata, ignored source roots, or absence from one binary as proof of
retirement.

Each courtroom carries at least one cheap mutation control in the ordinary
gate, and every listed mutant must fail for its own causal reason:

| Courtroom | Required mutants |
| --- | --- |
| `HP-01` | omit or reorder one causal action; substitute production-expected pixels for OS capture; reuse mutable installation state; retain the egui launch path after cutover |
| `HP-02` | remove owner delta carriage; force retained-list discovery; widen damage; break equal-layer order; omit vacated-pixel replay |
| `HP-03` | permit system fallback; diverge measurement/render profiles; evict a pinned glyph; retain stale DPI glyphs; skip derived-state reconstruction |
| `HP-04` | admit an illegal transition; drop or duplicate a wake; promote indeterminate effect to success; collapse IME preedit or composition commit into semantic `edit-commit`; skip resource disposal; replace a claimed real boundary with a scripted result |
| `HP-05` | hide a retired edge behind a feature, example, fixture, lockfile entry, ignored source root, or curated metadata scope |

Every courtroom ledger row records world identity and version, baseline digest,
scenario delta, generated seed where applicable, authority provenance,
production entry, independent oracle, fault boundary, retained failure
artifact, teardown result, and total construction/execution cost. A broken
world stops as fixture failure and cannot satisfy a product denial assertion.

## Product Decision Lock

### Presentation work is owner-issued and compiler-total

The host consumption contract exposes exactly one move-only presentation-work
variant:

```rust
pub enum UiMountedPresentationWork<'frame> {
    Initial(UiMountedPresentationSeed<'frame>),
    Delta(UiMountedPresentationDelta<'frame>),
    Unchanged(UiMountedPresentationUnchanged),
}
```

Runtime/mounting is the only producer. `Initial` carries a complete admitted
surface projection and establishes the retained generation. `Delta` carries
exact predecessor/successor frame and binding affinity, command insertions,
replacements, removals, order changes, and logical damage issued from the
admitted mounting/rebind scope. `Unchanged` proves equivalence to the retained
generation and carries no projection rows.

Removal and replacement work retains enough predecessor affinity to validate
the retained command. Logical damage covers the union of predecessor and
successor visible bounds after clipping, so vacated pixels cannot survive a
removal or move. The host may coalesce those regions mechanically but cannot
omit them or derive a broader semantic scope from the full projection.

This contract publishes Worth host protocol revision 4: mounted-frame,
mounted-presentation, and measurement schemas advance to revision 4 while the
3.14 observation schema remains revision 6. Both hosts implement that exact
contract during coexistence. Older or mixed revisions reject before effects;
there is no downgrade or reinterpretation path, and the superseded protocol
window is retired with egui at cutover.

The host may reject a malformed or stale delta before effects. It may not
recover by scanning the successor projection, comparing semantic digests,
inventing removals, widening logical scope without accounting, or rebuilding
an unchanged generation. Full reconstruction is a separately named cold or
recovery operation consuming current mounted authority.

Every drawable mounted row carries a stable `UiMountedPaintOrderIdentity`
issued by mounting. `Initial` carries the complete total sequence; `Delta`
carries exact insert-before/after, move-before/after, and removal edits over
those identities plus a successor-order integrity value. A local insertion
does not renumber unrelated commands. A genuinely global semantic reorder may
carry global work and must report it. Layer meaning remains inspectable, but
the host never supplies a family-, receipt-, hash-, or arrival-based tie break.
A new drawable family cannot compile into presentation until it carries the
same order and receipt contract.

### Native commands are derived, attributed, and non-authoritative

The native host lowers mounted rows into private command variants. A command
contains a generational command key, mounted node receipt, mounted instance,
frame, semantic surface, binding generation, paint-order identity, geometry,
clip, and the mechanic-specific payload. There is no receiptless constructor
and no generic raw-paint escape hatch.

Draw-list keys, GPU offsets, glyph references, atlas coordinates, backing
textures, and surface epochs are mechanical identities. They can correlate
evidence but cannot target an interaction, construct presentation work,
publish a frame, or reconstruct mounted authority.

### The platform owns scheduling and native resources

`worth-ui-host-native` owns the native event loop on its required thread, all
windows and surfaces registered beneath that loop, graphics adapter/device/
queue acquisition, input translation, redraw admission, capture completion,
device recovery, and normal close. It depends on `worth-ui-host-contract`, not
runtime, the product facade, Query binding, DSL, inspection, or application
code.

`worth-ui-native-platform` is the higher application composition owner. It
depends on the public `worth-ui` facade and `worth-ui-host-native`, binds one
application driver to one native event loop, and coordinates only public
lifecycle operations. The Worth runtime session remains on the event-loop
thread unless an existing owner explicitly grants another posture.

The mechanics boundary has one typed event-loop client contract, not arbitrary
closures. Its methods consume host-issued wake grants and return only an
exhaustive `Continue | WaitUntil | Close` directive. It exposes no raw paint,
window mutation, targeting, publication, or device operation. Implementing
that inert client contract grants no permission to register or run it. Event-
loop admission additionally consumes a move-only platform-binding grant issued
through the public Worth application preparation path; a client
implementation, serialized value, host identity, or mechanics dependency
cannot mint or replace that grant. The canonical client and binding are
private to `worth-ui-native-platform`; product code implements the higher Worth
application contract and never the mechanics client directly.

Worker threads may carry only typed readiness observations through bounded
channels and a native event-loop wake proxy. They do not receive the runtime
session, host adapter, window, device, mounted projection, or publication
authority.

A wake is a level-triggered readiness hint, never the semantic payload.
Each owner commits work to its bounded owner queue before signalling its
generation-bearing readiness slot. Coalescing preserves the newest ready
generation and cannot erase pending work. One event-loop turn drains a
profile-bounded slice in a frozen deterministic priority order; if an owner
remains ready, the client yields and re-arms exactly one wake. Queue
exhaustion, stale wake generation, coalescing, per-owner work consumed, turns
yielded, and starvation bounds are typed and counted. Shutdown closes wake
registration before worker join, so a late proxy signal cannot resurrect the
platform or strand owner work.

The application-side driver is owned above the adapter by
`worth-ui-native-platform`. It may submit owner-issued source, Query, intent,
deadline, and shutdown readiness into existing public lifecycle operations.
It cannot receive vendor graphics types, draw callbacks, raw paint access,
direct target selection, or a second publication method. The mechanics crate
sees application readiness only as an opaque bounded wake slot; it learns no
source, Query, or intent meaning. Every wake is classified, coalesced under an
explicit policy, bounded, and attributed in the lifecycle cost report.

Ordinary scheduling is event driven. A redraw is admitted only by new
presentation work, an exact capture request, required native recovery, or a
future framework-issued deadline. Cursor movement, idle polling, diagnostic
inspection, and an unchanged semantic turn do not inherently authorize GPU
work.

### Native input preserves the inherited IME contract

Windows remains the composition owner. The native host translates OS IME
`preedit`, `commit`, and `cancel` observations plus native text ranges into the
existing host-neutral observation contract. Translation binds the exact local
draft recipient, input revision, and event-time presentation/DPI basis and
carries the existing proof that canonical UTF-8 ranges fall on Unicode scalar
boundaries. The host cannot infer a recipient from OS focus, tree position,
current coordinates, or a newer presentation.

Preedit may change only visible runtime-owned draft posture and never enters a
payload. IME commit updates committed draft text but does not itself mint the
semantic `edit-commit`; only the declared commit gesture may do so. Cancel
discards the active composition posture. Missing recipient, stale affinity,
unsupported text, or unprovable native-range conversion returns its distinct
typed stop before payload or intent effects and preserves the prior draft as
required by the inherited 3.14 lifecycle. General focus routing remains 3.15
work; this milestone adds no Worth-owned IME service or composition policy.

### Text mechanics have a frozen trust boundary

Before native API implementation advances, Phase 1 commits one exact,
redistributable repository-pinned font asset and normative profile manifest
for `BodyDefault`. The manifest records the asset digest and license, exact
supported code-point set, size, weight, baseline, hinting/subpixel policy,
normalization, DPI rules, shaping/raster dependency versions, and qualification
evidence. The set includes at least printable Basic Latin `U+0020..U+007E`, but
there is no implementation-selected or environment-dependent additional
coverage. Changing the asset, set, or mechanics profile is a protocol/profile
change requiring explicit requalification. System font lookup, tofu
substitution, and fallback are forbidden.

Runtime completes semantic value, profile, width/clip policy, baseline posture,
and layout constraints before the mechanics boundary. The shaping dependency
may map that input to glyph identities, positions, advances, and metrics; it
cannot choose another value, profile, fallback, wrapping policy, clipping, or
mounted identity. The same qualified text mechanics profile serves host
measurement and glyph formation.

The atlas is bounded by pages, dimensions, entries, and bytes. Its key includes
font asset/profile generation, glyph identity, size/DPI, and every raster
parameter capable of changing pixels. Candidate allocation and eviction finish
before uploads. Live retained commands pin their glyph entries. Saturation
denies before effects or enters a named reconstructive posture; it cannot grow
without bound or silently discard live glyphs.

### Damage and presentation are physically honest

Each native surface owns a retained presentation target suitable for both the
qualified render path and exact readback. It is derived state. Logical damage
comes from runtime-issued command changes; the host may coalesce or lower that
damage into physical scissors only under a named deterministic policy.
The host maintains a derived spatial index over retained command bounds and
uses it to issue one total-order replay plan for commands intersecting those
scissors. Clearing damage without replaying underlying commands, replaying
only changed commands, or scanning all retained commands for an ordinary local
delta is incorrect. Destroying the index and rebuilding it from the retained
draw list changes no pixels or authority.

Swapchain acquisition, full-target copy, compositor damage, and presentation
are distinct costs. If the selected platform/API cannot preserve prior
swapchain contents, the host retains its own presentation target and reports
any whole-surface transfer required to present it. Local CPU or render damage
does not license a false local end-to-end claim.

Mounted RGBA values, the native surface format, linear/sRGB conversion,
premultiplication, DPI rounding, clip half-open semantics, and readback
canonicalization are frozen in the host profile and covered by external pixel
control points. Phase 1 commits the exact Windows backend, adapter-selection
posture, surface/target formats, present/alpha modes, blend equations,
anti-aliasing posture, shader inputs, coordinate rounding, and dependency
versions as one versioned native profile manifest. Vendor defaults and runtime
environment discovery are observations admitted by that profile, never policy.

### Capture observes the exact presented-source target

Visual capture copies from the same retained target used to produce the
presentation identified by the existing snapshot contract. It proves the
exact source pixels handed to native presentation, not post-compositor scanout.
The executable-world client-area observation separately proves the compositor-
visible window. The qualified Windows profile freezes the transform and color
relationship between those two observation levels, and `HP-01` requires them
to agree at named control points without treating one as a substitute for the
other. Capture admission binds frame, surface, binding generation,
presentation epoch, coordinate transform, byte budget, and deadline before the
copy begins. Completion waits for the relevant GPU fence and canonicalizes
pixels into the existing artifact without changing their identity basis.

A draw list, atlas, command buffer, expected-color table, or reconstructed
image is not a snapshot. Capture cancellation before effects and uncertain
completion after copy/submit remain distinct. Readback buffers and mapped
bytes are framework-managed, bounded, disposable, and included in shutdown
census.

### Failure and recovery preserve authority

Presentation prepares and validates protocol, generation affinity, command
changes, total order, text support, capacities, atlas allocation, and GPU
resource requirements before effects. Candidate retained and atlas state stays
separate until the host reaches the declared effect boundary.

Prepared, surface-acquired, encoded, submitted, present-handoff, completed,
cancelled-before-effects, and indeterminate states are distinct consuming
types. A presentation epoch is issued only after the qualified native present
handoff succeeds; it means host completion at that declared boundary, not
proof of physical scanout. Input binds only the last such completed epoch,
never a prepared or in-flight successor. Input before the first completed
epoch, or whose event-time DPI/surface basis cannot resolve to that epoch,
produces a typed no-presentation-basis stop rather than current-coordinate
retargeting.

After effects may have begun, cancellation is not rollback. The host reports
in-flight or indeterminate posture through the existing mounted presentation
contract. Runtime keeps the predecessor semantic publication and exact
recovery authority. Device or surface reconstruction consumes current mounted
authority through a named reconstructive operation; it never uses a cache,
pixel snapshot, lifecycle event, or diagnostic record as source truth.

Shutdown owns and accounts for windows, surfaces, adapter/device/queue,
presentation targets, draw lists, GPU buffers, atlas pages/entries, pending
submissions, readback buffers/maps, event proxies, and application wake
registrations. Normal close is not complete until all census values are zero.

### Coexistence is bounded and deletion is final

Dual-host coexistence exists only inside this milestone and only for parity.
It shares the same host contract, Pulse composition root, binary, external
runner, causal-action manifest, and evidence schema; only the recorded host
grant differs. It may not introduce a common lowest-denominator renderer,
host-neutral pixel normalization, a permanent feature flag, second product
composition root, or parallel parity harness.

Revision-4 work in the egui host is deliberate, deletion-bound migration
scaffolding. Phase 9 removes that implementation, but `HP-01` is honest only
when both hosts consume the exact same protocol and mounted work. Its cost
belongs to the migration lane and cannot be optimized away through parity over
different protocol revisions, evidence-level normalization, or an egui-only
reinterpretation of revision 4.

At cutover:

- `worth-ui-host-egui` and the eframe Pulse shell are removed;
- `WorthUiHostKind::Native` becomes the only native-display kind;
  `WorthUiHostKind::Egui` and its constructor are removed and exhaustive
  matches are repaired;
- egui-specific certification is replaced by host-contract or native-platform
  evidence;
- the isolated `worth-ui-theme` and `worth-ui-components` egui-era Rust crates
  are removed rather than ported into pre-3.16 architecture;
- assets from those crates survive only if an existing non-egui semantic owner
  and current consumer are proven; they are not moved into a speculative asset
  bucket; and
- workspace enforcement permanently rejects all egui-family dependency edges.

## Public Developer Experience

The public type topology and launch calls below are normative:

```rust
use worth_ui_native_platform::{
    UiNativePlatformOutcome, UiNativePlatformProfile, UiNativeWindowSpec,
    WorthUiNativePlatform,
};

let profile = UiNativePlatformProfile::single_window(
    UiNativeWindowSpec::new("WORTH UI Platform Pulse", [160, 96]),
);

let outcome = WorthUiNativePlatform::prepare(profile)?
    .run(PlatformPulseApplication::new(launch, publisher));

match outcome {
    UiNativePlatformOutcome::Closed(receipt) => observe_close(receipt),
    UiNativePlatformOutcome::PreparationDenied(denial) => report(denial),
    UiNativePlatformOutcome::Stopped(stop) => recover_or_report(stop),
}
```

`WorthUiNativePlatform` is exported by `worth-ui-native-platform`. Application
authors do not import `worth-ui-host-native`, `winit`, `wgpu`, shaping,
rasterization, or native-window handle types. The application object supplies
product composition and typed readiness; the platform supplies the only native
host grant and consumes the application at run. Close, preparation denial,
in-flight native effects, device recovery, and terminal resource posture are
typed outcomes rather than logged side effects.

`prepare(profile)` performs effect-free static profile validation and returns
a move-only prepared platform or a typed profile denial; it creates no event
loop, window, surface, or device. `run(application)` consumes that prepared
value, owns all native effects, and returns `UiNativePlatformOutcome` directly.
`PreparationDenied` means application/host admission stopped before native
resource effects. Programmer defects do not masquerade as that outcome.

The application preparation contract receives a public Worth UI builder
already bound to the platform-issued host grant. Product code may register
source, Query, intent, and inspection capabilities but cannot replace, clone,
extract, or downcast the native host. The completed `WorthUiApp` returns to the
platform for launch on the same event-loop thread.

## Compile-Time and Mechanical Enforcement

- `UiMountedPresentationWork`, its initial/delta/unchanged variants, paint
  order identities/plans, and command changes have private fields. They are
  opened only by the existing mounted presentation lease and consumed by the
  host.
- Initial, delta, unchanged, reconstruction, and completion are distinct
  proof-bearing types. A delta cannot be applied to the wrong retained frame,
  surface, binding, capability, device, or profile generation.
- Native draw-command constructors are private to the projection translator
  and require a complete mounted row; a receiptless or generationless command
  is unrepresentable.
- The mounted world compiler exposes semantic handles and typed actions with
  private fields. It cannot construct presentation work, native commands,
  atlas entries, platform authority, or expected production outcomes directly.
- Damage-index entries derive only from retained attributed commands. Delta
  execution consumes owner-issued damage plus a mechanical intersection/replay
  plan; neither the index nor replay plan can construct semantic damage or
  presentation work.
- Text profile, shaped run, rasterized glyph, atlas reservation, uploaded
  glyph, and retained glyph reference are distinct phases. External
  dependencies cannot mint mounted or publication authority.
- Native IME observation phase, canonical range-conversion receipt, runtime
  draft transition, and semantic `edit-commit` are distinct types. The native
  translator can construct only host-neutral observations and conversion
  evidence; it cannot construct a draft transition, semantic interaction,
  payload, intent, or focus authority.
- Exact font and native platform manifests carry protocol/profile identity;
  mixed or environment-selected profiles reject before effects.
- Window, surface, device, presentation target, atlas, readback, and event-loop
  resources are registered under framework-owned generational handles and
  require exhaustive shutdown propagation.
- Event-loop client implementation, wake proxy, wake observation, and
  platform-binding grant are distinct types. Only the move-only binding grant
  can admit a client; none of the other types can construct, clone, serialize,
  or recover it.
- Prepared, acquired, encoded, submitted, present-handoff, capture-readback,
  recovery, and terminal resource states are consuming types with only their
  lawful next transitions visible.
- Scripted window, surface/graphics, and readback ports can inject only typed
  external observations. They cannot mint lifecycle or platform outcomes,
  bypass the production orchestrator, or exist only under `cfg(test)`.
- Independent pixel, ordering, damage, atlas, and lifecycle oracles live
  outside the production mechanics they adjudicate. Boundary/source checks
  reject imports from disputed production algorithms, and the executable pixel
  oracle consumes a versioned test-owned control-point manifest.
- Vendor types remain private to `worth-ui-host-native`; boundary and
  dependency checks reject imports from `worth-ui`, runtime, DSL, Query
  binding, and inspection.
- The host contract and native mechanics cannot import runtime or product
  internals. Runtime cannot import the native host implementation.
- `worth-ui-native-platform` may depend only on the public Worth UI facade and
  the native mechanics facade; it cannot import runtime internals or vendor
  modules.
- Public outcome matches are exhaustive and `#[must_use]`; wildcard
  translations over lifecycle, presentation, capture, or recovery topology
  are forbidden.
- The workspace gate rejects `egui`, `eframe`, `egui_extras`, the retired
  adapter, and the migration selector. Positive/negative twins prove the
  intended native and headless homes remain lawful.
- Compiler evidence stays in the existing consolidated compiler sessions.
  Production compilation and local model tests enforce private topology that
  has no public misuse value.

## Cost Contract

Ordinary host work is:

```text
delta validation       O(k + o)
draw-list mutation     O(k log n) worst case, with bounded indexed lookup
damage selection       O(d log n + r) for indexed intersection and order
damage replay          O(r + p_render)
text shaping           O(t + g_new) only for changed runs
atlas lookup/update    O(g_changed) with bounded candidate allocation
GPU upload             O(bytes changed or reconstructed)
presentation           explicit damaged work + physical surface amplification
```

`n` is retained command count, `k` changed commands, `o` owner-issued order
changes, `d` damage regions, `r` retained commands intersecting admitted
damage, `p_render` pixels actually rerendered, `t` changed text bytes, and `g`
glyphs. Complete overlap may lawfully make `r = n`; the intersection index
must prove that breadth rather than paying `O(n)` to discover a local result.
The final implementation may prove stronger bounds but may not weaken an
ordinary local delta to a retained-list scan.

Counters separately expose:

- initial/delta/unchanged work and rows consumed;
- commands inserted, replaced, removed, reordered, reused, and retained;
- logical damage regions/pixels, damage-index probes, commands selected and
  replayed, cleared/rerendered pixels, and physical copy/present pixels;
- shaped runs, text bytes, glyphs, atlas hits/misses/rasterizations/evictions,
  upload bytes, and retained atlas bytes;
- buffer writes, encoder/pass counts, surface acquisitions, submissions,
  presents, fences, capture copies, and readback bytes;
- window/surface/device reconstruction and every retained resource class; and
- wake causes, coalescing, queue saturation, and dropped diagnostic richness.

Elapsed time supplements these counters and never substitutes for slope proof.
Cold start, ordinary delta, unchanged, capture, resize/DPI reconstruction,
device recovery, migration parity, and diagnostic materialization remain
separate cost lanes.

Courtroom construction has the same cost discipline. Maximum-size worlds
compile immutable semantic baselines once per existing test process, then
derive isolated scenario state and owner-issued deltas. No scenario shares
mutable draw-list, atlas, device, counter, installation, or process fate. Real
process/window/device startup is paid only by the serialized worlds whose claim
requires that boundary; exhaustive schedules remain in independent models.

## Architectural Destination

### Ownership

| Owner | Owns | Excludes |
| --- | --- | --- |
| runtime mounting | mounted projection, total paint order, exact presentation work, logical damage, publication affinity | native scheduling, GPU resources, font fallback |
| `worth-ui-host-contract` | inert presentation-work protocol, mounted mechanics, host observations, capture and typed outcomes | runtime truth, vendor types, native implementation |
| `worth-ui-host-native` | native scheduling, windows, surfaces, graphics device, derived draw list, text mechanics, input translation, atlas, readback, resource recovery | runtime, authored meaning, targeting, intents, Query, publication |
| `worth-ui-native-platform` | composition of public application lifecycle with opaque native wakes and the native host grant | runtime internals, vendor mechanics, drawing, target or intent authority |
| application composition root | product source/Query/intent wiring and typed readiness | event-loop ownership, raw input translation, drawing |
| inspection | bounded immutable host and presentation evidence | operational construction or recovery |
| certification | independent models, governed worlds, parity and deletion proof | production authority |

### Destination tree

`create` means populated in this milestone. Committed successors show where
known work enters; no empty placeholders are created merely to realize the
diagram.

```text
_docs/worth-ui/
  milestone-3.14.1.md                            [create: governing design]
  milestone-3.14.1-proof-ledger.csv              [create: closure evidence]

workspaces/worth-ui/
  crates/
    worth-ui-host-contract/src/
      mounted_frame/
        presentation.rs                         [modify]
        presentation_work/                      [create: owner-issued work]
          {mod,initial,delta,unchanged,command_change,
           damage,paint_order}.rs
        presentation_cost.rs                    [modify]
      mounted_projection/
        {static_paint,semantic_text}.rs          [modify: total order/profile]
        text_profile.rs                         [create]
      runtime/
        runtime_host_contract.rs                [modify; delete egui kind]
        native_platform_binding.rs              [create: affine run admission]
      visual_snapshot/                          [modify: native readback posture]

    worth-ui-runtime/src/mounting/projection/
      lowering/delta.rs                         [modify]
      presentation_work/                        [create: mounting authority]
        {mod,initial,delta,logical_damage,order}.rs

    worth-ui-host-native/                       [create: host-contract-only WUI dependency]
      Cargo.toml
      assets/fonts/body-default/
        {font-asset,LICENSE,profile.toml}        [create: exact qualified asset]
      src/
        lib.rs                                  [facade only]
        native/
          event_loop/{mod,client,client_binding,run,wake_grant,wake_proxy}.rs
          window/{mod,port,registry,lifecycle,dpi,close}.rs
          platform/windows.rs                   [current certified platform]
        presentation/
          {mod,session,transaction,cost,reconstruction}.rs
          command/{mod,filled_rect,glyph_run}.rs
          draw_list/{mod,state,delta,order}.rs
          damage/{mod,logical_lowering,intersection_index,
                  replay_plan,physical_report}.rs
          surface/{mod,port,binding,target,present}.rs
        graphics/
          {mod,device,buffer,submission,failure}.rs
          backend/{mod,port,wgpu}.rs             [external contract/private vendor]
        text/
          {mod,profile,profile_manifest,measurement,shaping,rasterization}.rs
          atlas/{mod,state,reservation,eviction,cost}.rs
        input/
          {mod,observation,pointer,keyboard,text_ime}.rs [OS translation only]
        capture/
          {mod,port,admission,readback,completion,cancellation}.rs
        lifecycle/
          {mod,resource_registry,census,recovery,shutdown}.rs

    worth-ui-native-platform/                   [create: product composition]
      Cargo.toml
      src/
        lib.rs                                  [facade only]
        platform/
          {mod,profile,prepare,run,outcome}.rs
        application/
          {mod,driver,readiness,wake_slot,shutdown}.rs

    worth-ui-host-egui/                         [remove after parity]
    worth-ui-theme/                             [remove; isolated egui era]
    worth-ui-components/                        [remove; isolated egui era]

    worth-ui-certification/
      src/topology/host_platform/               [create inside existing suite]
        {mod,dependency_boundary,destination,retirement,
         retirement_world}.rs
      tests/application_contracts/host_platform/ [create inside existing target]
        {mod,presentation_delta,text_atlas,input,capture,recovery,
         lifecycle,cost}.rs
        world/{mod,mounted_presentation,native_lifecycle_protocol}.rs
        oracle/{mod,ordered_pixels,atlas_lifecycle,native_lifecycle}.rs

  apps/platform-pulse/
    src/
      main.rs                                   [modify: Worth native launch]
      application.rs                            [modify: no egui context]
      native_frame.rs                           [replace]
      native_application/                       [create: product driver]
        {mod,composition,readiness,lifecycle}.rs
    tests/executable_world/                     [modify; same target/courtroom]
      adjudication/native_profile_control_points.toml [create]
      courtroom/host_parity.rs                  [create or modify]
      source_delta/causal_action_manifest.rs    [create]
      product_process/host_migration_grant.rs   [create in Phase 8; remove in 9]
```

The stable structural axes are mounted authority, inert host protocol, native
mechanics, application-platform composition, retained presentation, graphics,
text, input observation, capture, and derived evidence. Do not flatten them
into `renderer.rs`, `host.rs`, `state.rs`, `resources.rs`, `helpers`, `common`,
a product callback bag, or the application session.

Committed successors enter additively:

- 3.15 adds framework deadlines and service-ready wake causes without moving
  event-loop ownership; portal layers become new mounted command/order input,
  never host-local popup state;
- 3.16 adds appearance-derived mounted values upstream; the host continues to
  render completed mechanics without theme authority;
- Milestone 4 adds window registrations beneath the existing native window
  registry rather than replacing a single-window loop;
- later vector, icon, canvas, and realtime mechanics add command-family
  siblings beneath presentation with the same receipt/order contract; and
- accessibility adds a distinct mounted effect/observation boundary rather
  than being inferred from draw commands.

## Ordered Phases

### Phase 1: Protocol, qualification, and topology closure

Implement the owner-issued initial/delta/unchanged protocol, exact
predecessor/successor affinity, stable total paint order, logical damage,
expanded cost vocabulary, and the runtime/mounting producer. Update the egui,
headless, and certification consumers through the same revision-4 contract;
none may retain the complete-projection ordinary lane. Commit the exact
`BodyDefault` asset/profile manifest, native Windows profile, pinned dependency
versions and trust record, public DX, capacities, destination tree, and egui
retirement inventory. Add compile/topology/dependency enforcement plus
the governed mounted world compiler, independent delta/order/damage oracles,
and the test-owned control-point manifest with filled-rectangle expectations;
glyph expectations remain pending. No later phase begins while a profile
choice, hidden default, unbounded capacity, mixed protocol, host-side
rediscovery path, or oracle import from disputed production mechanics remains
open. Phase closure amends this specification and the proof ledger with the
exact committed manifest identities; values chosen only in implementation code
do not count as qualification.

Phase 1 remains one atomic trust gate, but its implementation plan closes five
ordered internal batches: protocol and runtime producer; egui, headless, and
certification consumer migration; font/native qualification; governed world
compiler plus independent oracles; and topology/compiler enforcement plus
ledger closure. These batches are not independently trusted phases. Phase 2
waits for their combined exit gate rather than advancing on partial revision-4
adoption or provisional manifests.

Phase 2 may trust complete, ordered, attributable presentation work, frozen
native/text profiles, and mechanically enforced dependency direction.

### Phase 2: First native vertical presentation

Create the native mechanics crate with only the host contract as its WUI
dependency, plus the higher application-platform crate, Worth-owned event loop,
affine platform binding, application driver, level-triggered readiness
scheduling, Windows window/surface/DPI lifecycle, graphics device/queue,
retained target, and resource registry. Present one attributable initial filled
rectangle through `WindowsNativeBoundaryWorld` in a real native window, observe
its actual client pixels, remain quiescent without readiness, and close with
exact zero framework resources. Establish the production external-effect ports
and their real implementations at the boundaries this vertical slice crosses.
This phase stays vertical: it does not end at a window skeleton, clear color,
fake surface, or in-memory renderer.

Phase 3 may trust one owned, visible native lifecycle and one attributable
initial presentation path without continuous repaint or vendor leakage.

### Phase 3: Retained delta, total order, and damage replay

Build the receipt-keyed draw list, exact command mutation and order indexes,
derived damage intersection index, total-order replay plan, staged presentation
transaction, unchanged zero path, reconstruction from mounted authority, and
complete structural/amplification counters. Close `HP-02` and mutation controls
in `MountedPresentationWorld::maximum_overlap` against retained-list scans,
changed-command-only redraw, widened damage, stale delta reuse, vacated pixels,
and equal-layer nondeterminism.

Phase 4 may trust scalable filled-rectangle initial, delta, unchanged,
overlap-correct replay, and reconstruction behavior.

### Phase 4: Pinned text, measurement, and atlas lifecycle

Integrate only the Phase 1-qualified shaping/raster dependencies, shared
measurement profile, glyph-run commands, bounded atlas, live-entry pinning,
candidate eviction, exact unsupported-text denial, DPI/profile generation
replacement, and text reconstruction. Close `HP-03` and produce the externally
adjudicated glyph rebaseline candidate through
`MountedPresentationWorld::qualified_text` without yet changing the cumulative
Pulse expectation.

Phase 5 may trust deterministic qualified `BodyDefault` measurement and pixels
without system fallback, environment-selected coverage, or unbounded retained
state.

### Phase 5: Native input and presentation affinity

Replace egui input translation with host-neutral native observations for every
shipped 3.14 family. Prove exact last-completed-presentation affinity, input
before first presentation, input during successor presentation, event-time
DPI/resize basis, lossless ordering, bounded observation delivery, and the
inherited gesture/IME distinctions. Close the input portion of `HP-04` and the
3.14 interaction mutation controls through `WindowsNativeBoundaryWorld` for
real OS delivery and `NativeLifecycleProtocolWorld` for unavailable or
adversarial schedules. Exercise preedit, composition commit, cancel, canonical
range conversion, and their no-recipient, stale-affinity, unsupported-text, and
unprovable-conversion stops. Prove preedit never enters payload, IME commit does
not mint semantic `edit-commit`, and a native event, current coordinate, or in-
flight frame cannot retarget itself.

Phase 6 may trust the complete native input-to-presented-target path without
targeting, intent, or publication authority in the host.

### Phase 6: Presented-source capture and snapshot integration

Implement capture admission, retained-target copy, fence-bound completion,
canonical readback, bounded buffers/maps, cancellation before effects, and
indeterminate completion after effects may begin. Join presented-source pixels
to the existing snapshot identity and independently compare them with external
client-area control points. Close the capture portion of `HP-04` through both
native worlds and the test-owned control-point manifest, keeping real readback
and injected completion claims distinct. Draw lists, expected-color tables,
reconstructed images, and compositor screenshots cannot impersonate one
another.

Phase 7 may trust exact bounded capture of the native presentation source with
typed affinity, cancellation, disposal, and external pixel correlation.

### Phase 7: Failure, recovery, and hostile shutdown

Complete typed surface/device loss, timeout/outdated posture, resize/minimize
reconstruction, prepared/acquired/encoded/submitted/present-handoff effect
states, close during upload/presentation/readback, and reconstruction only from
current mounted authority. Cross shutdown with queued readiness and a held 3.14
application attempt, then prove exact zero for every platform, native, worker,
application, Query, intent, capture, draw-list, atlas, and GPU resource class.
Close all remaining `HP-04` rows with fault-injected protocol evidence and real
native integration kept explicitly distinct. The protocol world runs the
independent transition model; the Windows world supplies only the real-boundary
facts its environment actually exercised.

Phase 8 may trust the complete native product path under ordinary, denied,
in-flight, indeterminate, recovery, and shutdown postures.

### Phase 8: Dual-host cumulative parity and native candidate freeze

Introduce the migration-only launch grant and run `HP-01` against egui and
Worth native hosts through the same Pulse binary, composition root, runner,
versioned causal-action manifest, and evidence schema in
`PulseNativeParityWorld`. Record the migration-grant identity as the sole
differing world input. Close cumulative semantic, receipt, interaction, intent,
consequence, capture, lifecycle, and control-pixel parity; adjudicate and record
exactly one glyph-region rebaseline. Freeze the proven native candidate and
cutover inventory. This phase does not delete or silently bypass the
predecessor until parity evidence is final-source green.

Phase 9 may trust a migration-complete native candidate whose only remaining
work is authority cutover and predecessor removal.

### Phase 9: Native cutover, egui deletion, and final closure

Make the Worth native host the sole native-display path, then delete the egui
host, eframe shell, migration selector, egui-era theme/component crates, old
glyph expectations, and every dependency edge. Close `HP-05`, continuing
documentation, exact cost budgets, proof ledger, exhaustive host-kind repairs,
and constitutional/recurrence gates through `HostRetirementTopologyWorld` on
the exact post-deletion source. Rerun `PulseNativeParityWorld` natively so
deletion cannot manufacture a green result by removing parity coverage.

No phase creates a new integration-test target, executable-world target,
binary, nested Cargo invocation, compiler session, product composition root, or
universal mutable fixture. Hostile lifecycle and environment qualification are
consolidated into the deliberately serialized `WindowsNativeBoundaryWorld`;
`PulseNativeParityWorld` pays only for its cumulative product journey and
cannot reuse hidden process state. Combinatorial state, maximum-table, overlap,
and long fault schedules use cheap independent models inside existing targets.
On the recorded executable-certified Windows reference environment, warm
ordinary execution
remains at or below 60 seconds, the real native lifecycle courtroom at or
below 30 seconds, each `HP-01` host journey at or below 45 seconds, and the
temporary sequential dual-host run at or below 90 seconds. There are no blind
retries; the ordinary gate retains the cheapest mutation-sensitive example of
every guarantee.

## Documentation Deliverables

| Document | Continuing audience and required truth | Verification |
| --- | --- | --- |
| `workspaces/worth-ui/README.md` and `AI_README.md` | Developers discover the Worth native launch path and understand that mounted/runtime meaning remains above native mechanics. | Public launch example compiles and dependency audit stays green. |
| `workspaces/worth-ui/docs/native-host-platform.md` | Application authors and maintainers learn lifecycle, wake scheduling, supported platform/text posture, OS IME translation versus runtime draft authority, typed failures, recovery, capture, cost, and anti-patterns. | Examples compile; `HP-02`-`HP-04` prove described behavior. |
| `workspaces/worth-ui/docs/application-lifecycle.md` | The cumulative Pulse command and visible journey use the Worth host, including native actions and exact-zero close. | `HP-01`. |
| `workspaces/worth-ui/docs/architecture.md` and `runtime-subsystems.md` | Ownership of presentation work, native platform mechanics, derived retained state, and 3.15 insertion is accurate. | Topology and dependency enforcement. |
| `workspaces/worth-ui/docs/visual-inspection.md` | Snapshot capture is exact presented-source-target readback with typed affinity, cancellation, budget, disposal, and a distinct compositor-visible observation posture. | Native capture and anti-substitution evidence. |
| `workspaces/worth-ui/docs/interaction-and-intents.md` | Native observations replace egui translation without changing targeting, IME/draft phase distinctions, or intent authority. | 3.14 cumulative input evidence. |
| `_docs/worth-ui/worth_ui_roadmap.md` | 3.14.1 contract closure and 3.15 dependency remain current. | Documentary consistency audit. |

Delete or rewrite every continuing example that names eframe, egui context,
adapter repaint, or the retired host. Do not create a milestone closeout guide
or migration guide with no post-cutover audience; the ledger records the
temporary coexistence and glyph rebaseline.

## Must Ship and Preserve

Ship the owner-issued presentation-work protocol, total paint order, Worth
native mechanics and application-platform crates, event-driven lifecycle,
retained attributable draw list, indexed overlap-correct damage replay, filled
rectangles, pinned `BodyDefault` text and atlas, shared measurement mechanics,
level-triggered readiness, native 3.14 input observations, exact presented-
source-target capture, failure/recovery lifecycle, exact resource census,
the governed five-world proof portfolio, independent oracles, versioned causal-
action and control-point manifests, dual-host parity, one glyph rebaseline,
final egui deletion, and recurrence gates described above.

Preserve all closed 3.10-3.14 guarantees, especially:

- the public application facade and one cumulative Platform Pulse composition
  root;
- exact mounted node, frame, surface, binding, presentation, snapshot, and
  interaction identity;
- the single 3.12 observation/rebind/publication path;
- Query-owned projection and mutation authority;
- presentation-bound targeting, typed intent admission, provider effects, and
  consequence handoff;
- predecessor semantic truth under denial or uncertain native effects;
- Query-free and unchanged semantic zero-cost posture;
- bounded inspection and non-authoritative diagnostics; and
- the consolidated integration, executable-world, and compiler topology.

## Acceptance and Successor Handoff

Milestone 3.14.1 closes only when `HP-01`-`HP-05` have independent,
mutation-sensitive evidence; all governed native resources reach exact zero;
the one glyph rebaseline is recorded; public examples compile; continuing docs
agree; no egui-family dependency or path remains; and formatting, strict lint,
line-cap, test-topology, boundary, agent-context, ordinary certification,
compile-contract, native integration, and executable-world gates are green on
the exact final source.

`milestone-3.14.1-proof-ledger.csv` is the single closure ledger. It records
phase, requirement, owner, production boundary, world identity/version,
baseline digest, scenario delta, generated seed where applicable, authority
provenance, production entry, independent oracle, mutation control, fault-
injection boundary, retained failure artifact, teardown result, construction/
execution cost, exact command, source identity, font/native profile identity,
platform/dependency versions, structural counters, result, reopen lineage, and
final-source status for `HP-01`-`HP-05`. Font/native qualification, the
migration candidate freeze, glyph rebaseline, and post-deletion native rerun
are named ledger events. Presented-source readback and compositor-visible
client observation remain distinct evidence columns. Fixture failure remains
distinct from product denial or non-success. The ledger explains and certifies;
it cannot configure the host or substitute for production profile types.

The ordinary warm gate must satisfy the numerical phase budgets above.
Maximum-table model proofs reuse immutable worlds inside existing targets,
real GPU/window startup is paid only by the serialized platform/executable
lanes that require it, and no claim is supported solely by a rarely run soak
lane.

Milestone 3.15 may trust a Worth-owned event loop and native presentation
platform where exact mounted deltas become attributable pixels, 3.14 input
families arrive through host-neutral observations, framework deadlines can
wake the loop, exact presented-source pixels can be captured and correlated
with compositor-visible client pixels, and all native resources have typed
failure, recovery, and disposal. It adds portal, focus, motion, command,
scroll, and selection meaning above these mechanics; it may not reintroduce
adapter-local state, generic callbacks, continuous repaint, renderer-selected
appearance, or egui compatibility.
