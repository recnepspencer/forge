# Milestone 3.14.1: Aspect-Native Host Platform and egui Retirement

## Status and Placement

Status: implementation specification for the ten-phase slice immediately
after Milestone 3.14 and before Milestone 3.15. Phase 1-2 qualification and
public-authority records are closed and proved. The future Phase 4 text gate is
deliberately reopened by the framework-grade Unicode and emoji requirement
below: `worth-ui-body-default-v1` remains immutable predecessor evidence, while
`worth-ui-global-text-v2` must close its exact assets, dependencies, generated
Unicode data, capacities, and conformance corpus before Phase 4 implementation
planning. A different asset, dependency, capacity, signature, or platform
policy is a specification change rather than implementation discretion.

Milestones 3.10-3.14 established one real application lifecycle, exact mounted
identity, observation-driven rebind, projected product data, and
presentation-bound interaction and intent. This milestone replaces the
interim `egui`/`eframe` mechanics beneath those contracts before portal,
focus, motion, appearance, and later native services would otherwise be built
against a host scheduled for deletion.

The milestone is deliberately broad in native mechanics and narrow in product
meaning. It does not reopen authored meaning, Query authority, targeting,
intent admission, consequence publication, or appearance semantics.

Treat this as the largest single 3.x platform-migration slice. Its ten phases
are proof and authority gates, not equal units of effort; implementation
planning must budget roughly one quarter to one third of the milestone for
Phase 1. Earlier line-count estimates are not scope limits or acceptance
criteria, and they do not license compressed trust handoffs, deferred
qualification, or reduced proof.

## Goal and Central Claim

The same mounted frame and observation contracts drive a Worth-owned native
platform. A native mechanics crate whose only WUI dependency is the host
contract owns the event loop, window, device, surface, retained draw list,
input translation, capture, and shutdown. A host-neutral Worth text-mechanics
crate owns qualified fonts, Unicode analysis, shaping, line layout,
measurement, cluster geometry, and glyph rasterization for both headless and
native consumers; the native host alone owns GPU atlases and presentation.
A higher application-platform crate composes those mechanics with the public
Worth UI lifecycle. Runtime issues exact presentation work and total paint
order, and the host lowers it into attributable pixels without rediscovering
UI meaning. After parity is proved, every `egui` and `eframe` dependency and
ordinary path is deleted.

The closure claim is:

```text
admitted runtime change
-> owner-issued Initial | Delta | Unchanged presentation work
-> receipt-keyed retained native commands
-> canonical Unicode analysis, bidi/line layout, complex shaping,
   deterministic font fallback, and original-range cluster geometry
-> bounded alpha/color glyph raster, emoji, atlas, and GPU effects
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
- a Worth-authored shaping algorithm, IME composition engine or service,
  compositor, or GPU API; WORTH owns text orchestration, qualification, font
  collection and layout authority while consuming pinned Unicode, shaping,
  and raster dependencies, and host translation of OS IME observations into
  the inherited 3.14 contract remains required;
- ambient system-font discovery or renderer-selected fallback in the ordinary
  lane; the framework instead admits repository-pinned and explicitly
  application-bundled, content-addressed font collections;
- rich-text authoring, text-editor selection/focus semantics, spell checking,
  hyphenation dictionaries, or vertical writing; Phase 4 must nevertheless
  produce the grapheme, cluster, visual-order, caret-stop, selection-geometry,
  and writing-mode-ready artifacts those successor semantics consume;
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
| `BodyDefault` records size, weight, clipping, and baseline posture. | The Phase 1 profile proves only printable Basic Latin, one left-to-right run, no wrapping, no fallback, and grayscale glyphs; that is a migration seed, not framework-grade text. | A host-neutral, versioned Unicode text platform freezes deterministic font collection and fallback, complex shaping, bidirectional and line layout, grapheme/cluster mapping, color emoji, shared measurement/rendering artifacts, bounded caches/atlases, and typed coverage disposition before native text ships. |
| Platform Pulse implements `eframe::App` and requests continuous repaint. | Product code owns platform scheduling and unchanged turns still reach native presentation. | The Worth host owns the event loop and wakes only for typed OS, application-readiness, deadline, capture, or recovery causes. |
| egui dependencies exist outside `worth-ui-host-egui`. | Deleting one adapter leaves a false retirement claim. | Certification and Pulse migrate; the isolated egui-era theme/component crates are removed rather than prematurely ported. |

The required host-contract additions refine transport and proof carriage; they
do not change mounted semantic meaning. If implementation discovers that
native input cannot preserve a 3.14 interaction distinction, mounted rows do
not determine deterministic pixels, or measurement cannot preserve the
declared text profile, that is a reopened predecessor finding. A lossy shim or
adapter-local policy is not an allowed migration answer.

## Pre-Implementation Specification-Closure Gate

This gate is specification work, not Phase 1 implementation. No production
edit, dependency addition, font ingestion, protocol revision, implementation
plan, or phase ledger may begin until both records below are committed into
this specification and its continuing authoritative documents. A planning
document may investigate candidates, but it cannot turn an unresolved choice
into implementation discretion.

### Qualified native and text profile record

The governing design must name one exact redistributable `BodyDefault` font
asset and license, asset digest, normative supported code-point set, shaping
and rasterization dependencies and versions, normalization, hinting/subpixel
posture, size/weight/baseline metrics, DPI rules, atlas capacities, and the
unsupported-text compatibility outcome. The support set is a product contract,
not whatever glyphs a dependency or operating system happens to find. It must
state explicitly whether every currently admitted Platform Pulse value is
supported and how arbitrary otherwise-valid 3.13 Unicode projection text
stops before presentation effects while preserving predecessor publication.

The same record must freeze the exact Windows event/window backend, graphics
backend, adapter-selection rule, surface and retained-target formats,
present/alpha modes, blend equations, antialiasing posture, shader inputs,
coordinate and DPI rounding, clear/baseline behavior, dependency versions,
bounded queue/resource capacities, and executable qualification environment.
Repository manifests, assets, profile types, documentation, pixel manifests,
and proof-ledger identities must all derive from this record. Phase 1 consumes
it and proves it; Phase 1 does not choose or amend it.

The closed text record is `worth-ui-body-default-v1`:

| Field | Exact qualified value |
| --- | --- |
| Asset | Noto Sans v2.015, `NotoSans/hinted/ttf/NotoSans-Regular.ttf`, 621,572 bytes |
| Asset SHA-256 | `478c558ea716033cd60c03438f628dfa75694dcf6b5f6d505a2f05fd2b4f3823` |
| Upstream archive | official `NotoSans-v2.015.zip`, 117,491,253 bytes, SHA-256 `0c34df072a3fa7efbb7cbf34950e1f971a4447cffe365d3a359e2d4089b958f5`, release commit `c4a321e123e4d4ff315f57f4e0adf294fe3a95be` |
| License | SIL Open Font License 1.1, repository copy byte-identical to upstream `OFL.txt`, 4,396 bytes, SHA-256 `cee9892f9f0cc8fe882c9e9537ee6a89621d86ee7ceaf70b02e2b2b1c25c061a` |
| Normative support | exactly printable Basic Latin `U+0020..U+007E`; glyphs present in the asset outside this set are not admitted |
| Pulse coverage | every currently checked-in Platform Pulse projected text value is inside the normative set |
| Shaping | `rustybuzz = "=0.20.1"`; horizontal left-to-right, one run, no language or script fallback, no normalization |
| Rasterization | `swash = "=0.2.10"`; hinted outlines, grayscale coverage, no LCD/subpixel or color glyph rendering |
| Face/profile | regular face, weight 400, `14_000` millipoints, alphabetic baseline, no wrapping, clip overflow |
| DPI | physical size is logical size multiplied by event-time scale factor; the scale basis is part of profile generation |
| Rounding | glyph origins use round-to-nearest ties-to-even; rectangle and clip minima floor, maxima ceil; clips are half-open |
| Atlas | four `1024 x 1024` `R8Unorm` pages, at most 4,096 entries and 4 MiB texels; one glyph bitmap is at most `256 x 256`; one staged upload is at most 1 MiB |
| Saturation | live retained glyphs are pinned; candidate admission denies before upload if the exact bounds cannot be met; no fallback, growth, or live eviction |
| Canonical qualification observation | asset, license, manifest, dependency, support-set, and capacity digests under `asset-license-profile-dependency-digest-v1` |

Otherwise-valid Unicode outside `U+0020..U+007E` yields the typed
`UiMountedTextPresentationDenial::UnsupportedCodePoint` before shaping,
rasterization, atlas mutation, upload, surface acquisition, or presentation.
The semantic projection and predecessor publication remain current. The
denial records the first unsupported scalar, its UTF-8 byte range, the mounted
receipt and generation set, and `worth-ui-body-default-v1`; it never replaces
the value with tofu, omission, transliteration, or a system font.

#### Framework-grade text correction and Phase 4 qualification gate

`worth-ui-body-default-v1` remains the exact Phase 1-2 migration record and
its closed ledger evidence is not rewritten. It is not the Phase 4 text
destination. The product requirement for serious framework-grade text
reopens the future text-profile gate and supersedes the earlier Basic-Latin-
only Phase 4 plan with `worth-ui-global-text-v2`.

No Phase 4 production edit or implementation plan may begin until one
canonical `worth-ui-global-text-v2` manifest commits the exact asset list,
face indexes, licenses, byte lengths and SHA-256 digests; exact Unicode data,
shaping, segmentation, line-breaking, and raster dependency versions and
features; generated coverage and fallback indexes; conformance-data digests;
and every capacity below. The choice of a smaller incidental font set, an OS
fallback, or whatever behavior a dependency happens to expose is not
implementation discretion.

The normative contract for that manifest is:

| Field | Required `worth-ui-global-text-v2` contract |
| --- | --- |
| Unicode version | exactly Unicode 17.0 data and conformance tests; upgrading Unicode is a profile/protocol migration rather than an invisible dependency update |
| Text preservation | source UTF-8 bytes and scalar order remain authoritative; analysis or shaping may derive normalized views only when required by the pinned engine, and every grapheme, shaping cluster, glyph, caret stop, and diagnostic maps back to exact original UTF-8 ranges |
| Segmentation | extended grapheme clusters and default word boundaries conform to Unicode Standard Annex #29; line-break opportunities conform to UAX #14, with pinned locale-aware dictionary segmentation for scripts whose ordinary wrapping cannot be honest from space boundaries alone |
| Directionality | full horizontal Unicode Bidirectional Algorithm conformance from UAX #9, including isolates, paired brackets, numbers, neutral resolution, explicit controls, and line-by-line visual reordering; the source remains logical order |
| Run formation | paragraphs split into exhaustive runs by bidi level, script, explicit BCP-47 language or `und`, style, selected face, variation axes, and feature set before shaping; no code-point, font-family, or arrival-order tie break may replace that sequence |
| Shaping | HarfBuzz-compatible complex shaping through one pinned dependency and configuration for Arabic, Hebrew, Indic, Southeast Asian, Tibetan, Hangul, combining-mark, ligature, and joining behavior; cluster level preserves monotone grapheme mapping and never permits a line, caret, selection, fallback, or ellipsis boundary inside an indivisible shaping cluster |
| Default font collection | a repository-pinned Noto release catalog, including the required script faces, CJK coverage, symbols, math, and a Unicode-17-compatible Noto Color Emoji face; the final manifest enumerates every consumed face rather than naming an archive or family as a wildcard |
| Application fonts | an application may register an immutable content-addressed font pack before layout effects; admission validates bytes, face metadata, axes, features, coverage, license record, limits, and fallback position and returns a font-collection generation; ambient installation state never participates |
| Fallback | deterministic cluster-level fallback over the exact admitted collection and generated coverage index; a face must support the complete grapheme/shaping cluster before selection, and fallback cannot split combining sequences, Indic syllables, or emoji sequences |
| Missing coverage | valid UTF-8 never aborts an otherwise admissible presentation merely because a face lacks a glyph; a pinned Last Resort face renders one attributable missing-cluster glyph and records `UiTextCoverageDisposition::MissingCluster` with the original range and attempted collection generation |
| Emoji | all Unicode 17.0 RGI emoji sequences are qualified, including text/emoji variation selectors, keycaps, flags, tag sequences, skin-tone modifiers, gendered and family ZWJ sequences; fallback and line layout treat each qualified sequence as one grapheme/cluster, and rasterization preserves every required color layer or color bitmap |
| Layout | hard line breaks, preserved whitespace, tabs with explicit tab stops, no-wrap, Unicode word-wrap and grapheme-wrap, start/center/end alignment, explicit line height, letter/word spacing, maximum lines, clip, and cluster-safe ellipsis are typed constraints decided before mechanics execution |
| Layout result | one immutable reconstructible `UiQualifiedTextLayout` carries original-range mapping, paragraph and line records, logical and visual runs, selected faces, glyph positions/advances, ink and logical bounds, baselines, break and overflow decisions, coverage dispositions, grapheme-safe caret stops, selection rectangles, profile/font/locale/direction/width/text-scale generations, and exact work counters |
| Measurement/rendering identity | intrinsic measurement, baseline metrics, hit testing, selection geometry, glyph rasterization, and native rendering consume the same qualified layout artifact or its exact identity; no adapter may reshape, refallback, rebreak, or recompute metrics independently |
| Rasterization | grayscale outline glyphs and color outline/bitmap glyphs are distinct typed raster outputs; source order, palette, hinting, antialiasing, fractional-origin quantization, and premultiplication are manifest fields; emoji color is not replaced by the surrounding text color |
| DPI and text scaling | logical layout is keyed by logical constraints and a distinct framework text-scale generation; a pure DPI change replaces raster/atlas generations without changing logical breaks, while a text-scale or width change creates a new layout generation before presentation |
| Bounds | at most 4,096 retained paragraphs, 8 MiB retained UTF-8, 65,536 UTF-8 bytes per paragraph, 262,144 glyphs, 262,144 grapheme/cluster records, 65,536 line records, and 32 style/fallback runs per paragraph; admission denies before shaping when exact declared bounds cannot be satisfied |
| Raster/atlas bounds | four `1024 x 1024` `R8Unorm` alpha pages plus two `2048 x 2048` `Rgba8UnormSrgb` color pages, at most 8,192 total entries, a `512 x 512` maximum glyph image, 36 MiB total atlas texels, and 8 MiB maximum staged upload bytes |
| Saturation | retained layouts pin exact alpha/color glyph entries; complete candidate admission and deterministic unpinned eviction precede raster/upload effects; no live eviction, unbounded growth, silent quality reduction, or color-to-monochrome fallback is allowed |

The default profile must render ordinary multilingual application content and
emoji without caller configuration. `BodyDefault` becomes a semantic style
selection inside the v2 font collection, not the name of one physical face.
The qualified font resolver may select different pinned faces across runs
while preserving one semantic style, layout identity, and mounted receipt.
Custom rich-text authoring remains later product meaning, but the mechanics
input and layout artifact are span-capable from their first v2 revision so
adding rich text does not require replacing the shaping, fallback, line,
measurement, hit-testing, or atlas architecture.

The v1 `UnsupportedCodePoint` denial remains valid only for the already-closed
Phase 1-2 seed profile. Under v2, unsupported formatting, malformed admitted
font bytes, impossible original-range mapping, stale layout/profile affinity,
and capacity exhaustion retain distinct typed denials before effects; ordinary
missing font coverage is an attributable Last Resort disposition, not a
discarded value or a whole-frame denial.

The closed native record is `worth-ui-windows-dx12-v1`:

| Field | Exact qualified value |
| --- | --- |
| Executable lane | Windows 11 x86-64 with a D3D12-capable display adapter and composition-enabled desktop; other platforms return typed profile denial before effects |
| Window/event dependency | `winit = "=0.30.13"` |
| Graphics dependency | `wgpu = "=29.0.4"` with only the DirectX 12 backend admitted; `pollster = "=0.4.0"` for bounded preparation joins |
| Compiled dependency features | `winit`: defaults off, exactly `rwh_06`; `wgpu`: defaults off, exactly `std`, `parking_lot`, `dx12`, and `wgsl` |
| Device admission | runtime backend mask exactly `Backends::DX12`, empty optional device features, and `wgpu 29.0.4 Limits::downlevel_defaults().using_resolution(adapter.limits())`; the downlevel baseline is fixed while only the selected qualified adapter's physical texture-resolution ceilings are retained so the 16,384 window bound is realizable |
| Adapter selection | enumerate surface-compatible DX12 adapters satisfying the exact required limits; reject CPU/Other; sort Discrete, Integrated, Virtual, then vendor ID, device ID, name, and driver-info bytes; consume the first |
| Surface/target | wgpu DX12 `DxgiFromVisual` DirectComposition presentation, `Bgra8UnormSrgb` surface, and `Rgba8UnormSrgb` retained presentation target |
| Presentation | FIFO present mode and premultiplied composite alpha; unsupported exact modes deny rather than fall back |
| Shader/blend | input is logical straight RGBA; shader premultiplies RGB by alpha; color and alpha use source `One`, destination `OneMinusSrcAlpha`, operation `Add` |
| Antialiasing | sample count one; filled rectangles have no renderer-selected edge antialiasing; text uses the qualified grayscale glyph coverage |
| Coordinates | event-time logical coordinates and scale generation; physical minima floor and maxima ceil; half-open scissor and clip bounds |
| Baseline | canonical transparent `[0, 0, 0, 0]`, available only through a same-surface/binding/profile `UiMountedSurfaceBaselineReceipt` |
| Window/surface | one window and one surface in 3.14.1; transparent client background; initial logical size is the application profile size |
| Work bounds | 4,096 retained commands, 2,048 rectangle commands, 2,048 text commands, 4,096 damage regions, 4,096 order edits, and 1 MiB admitted text bytes |
| Scheduling bounds | 32 registered resource owners, eight readiness owners, 64 coalesced causes per owner, eight total ready owner slots, two presentation slots, four capture/readback slots |
| Readback bound | 16 MiB total framework-owned mapped or pending readback bytes; a native GPU submission/readback wait is bounded to 5,000 ms |
| Qualification observations | OS build; adapter name/vendor/device/driver; observed scale factors; required formats, modes, alpha, limits, and DX12 surface compatibility |
| Independent Windows observation | retained-target readback remains distinct from the existing Pulse executable-world `xcap = 0.9.7` WGC client capture, `winsafe = 0.0.28` window binding, and `uiautomation = 0.25.0` input/close observation |

Profile preparation records the operating-system build, selected adapter
identity, vendor/device IDs, driver information, observed scale factors, and
support for every exact required mode as qualification observations. Those
observations cannot change selection policy or silently widen the profile.
The profile identity is the SHA-256 of the canonical UTF-8 manifest containing
the table values above; Phase 1 checks in that manifest and records its digest
in every affected ledger row.

The committed canonical manifest digests are text
`6f140249866e6815e9284fe1c8c959a8bb1b8cab252cfbe8c7c397f9a7eb9b01`
and native
`1c937a22f42660267480a055e48256b25decf0c4cd5d4d7b493e5df034c6c65b`.
These successor digests deliberately reopen and replace the earlier incomplete
manifest identities: every normative field above, including compiled feature
posture, device-limits basis, saturation, and observation boundary, is now
inside the canonical bytes rather than inferred from hard-coded production or
test constants.

That first text digest identifies `worth-ui-body-default-v1` only. The
canonical `worth-ui-global-text-v2` candidate now has manifest digest
`3f059e7b017da04fb958695567bf668e577504cd16463492f1c404a0c0c3e4de`.
Its exact 30-face catalog, Unicode 17 source and conformance data, dependency
pins, generated coverage/fallback indexes, capacities, licenses, and artifact
inventory are repository-owned and mutation-validated. The qualification
ledger row remains `OPEN` until Phase 3 predecessor closure admits the Phase 4
gate; Phase 4 production may consume the candidate only after that row becomes
final-source `PROVED`. Phase 4 may never reuse the v1 digest or mint a
provisional replacement identity.

### Public native-application authority record

The governing design must freeze these named responsibilities before
implementation planning:

- `UiNativePlatformBindingGrant`: a platform-issued, move-only grant whose
  private authority admits exactly one application to exactly one prepared
  native platform; it is neither a host adapter nor a serializable identity;
- `UiNativeApplicationPreparation`: an effect-free, move-only preparation
  scope carrying only a host-neutral public `WorthUiApplicationBuilder` and
  the private platform-binding grant; subsystem workers do not exist yet;
- `UiNativeApplicationDefinition`: the only public product contract accepted
  by `UiPreparedNativePlatform::run`; it consumes the preparation scope and
  can return only a complete prepared application or a typed preparation
  denial;
- `UiPreparedNativeApplication`: the sealed result containing one frozen
  `WorthUiHostNeutralApp` and its private affine platform binding; Phase 2
  consumes this result to create the real application driver and its bounded
  subsystem owners; and
- `UiNativeApplicationPreparationDenial`: the before-native-effects outcome
  produced only by consuming the sealed effect-free preparation scope. Phase 1
  exposes no resource, worker, readiness, cleanup-census, or event-loop-client
  type that this scope could have opened or published.

The final public signatures make construction, consumption, and denial
compiler-visible. Product preparation cannot register or open source, Query,
intent, inspection, or readiness workers: those owners require the frozen
application/runtime authority that does not exist until preparation succeeds.
Phase 2 creates them inside the private application-driver progression. Product
code cannot implement the mechanics event-loop client, extract or replace the
host grant, submit raw wakes, or retain a second application/publication lane.

The closed public progression is:

```rust
pub trait UiNativeApplicationDefinition: Sized {
    fn prepare(
        self,
        preparation: UiNativeApplicationPreparation,
    ) -> UiNativeApplicationPreparationOutcome;
}

#[must_use]
pub enum UiNativeApplicationPreparationOutcome {
    Prepared(UiPreparedNativeApplication),
    Denied(UiNativeApplicationPreparationDenial),
}

impl UiPreparedNativePlatform {
    pub fn run<A: UiNativeApplicationDefinition>(
        self,
        application: A,
    ) -> UiNativePlatformOutcome;
}
```

`UiNativePlatformBindingGrant`, `UiNativeApplicationPreparation`,
`UiPreparedNativeApplication`, and both enum payloads have private fields and
no `Clone`, `Copy`, serialization, public constructor, or parts conversion.
The prepared platform privately issues one grant bound to its native-profile
identity, preparation identity, and single application slot. Any unsuccessful
preparation consumes that slot; a retry requires a newly prepared platform.

### Enforceability correction for the Phase 1-2 implementation

Rust crate privacy cannot distinguish one blessed sibling crate from another
downstream caller. This milestone therefore does not treat sibling-crate
identity, `#[doc(hidden)]`, a Cargo feature, or an unexported constructor in a
different crate as an authority boundary. The enforceable native activation
boundary is one fixed concrete transition: it consumes the move-only
host-neutral application, the private application-slot grant, and the exact
qualified native mechanics bundle. The transition and grant issuer live in
the same `worth-ui-runtime` privacy boundary as host-neutral application
construction; `worth-ui-native-platform` is the public facade over that gate.
It exposes no generic adapter parameter and cannot bind a caller-defined host.
Supported application crates reach it only through `WorthUiNativePlatform`;
repository topology rejects direct product dependencies on runtime and native
mechanics. The temporary egui and certification lanes use equivalently fixed
transitions and no feature-gated issuer.

The application-slot grant remains private to runtime-owned native-platform composition and
prevents reuse or replacement inside the public progression. It is not passed
across crates as a supposed friend token. A low-level integration function
that must be Rust-public for host mechanics composition is not itself the
application authority: it cannot receive or bind a host-neutral Worth
application. Product dependency topology prevents importing that mechanics
surface, while the actual application-binding transition remains crate-private.

Likewise, `worth-ui-host-contract` may expose inert mechanics inputs and
borrowed views needed by host consumers and their focused tests. Those values
are not authoritative presentation work. Only runtime's private lease can seal
them into the owning work envelope, admit them to an active host session, or
mint a completion token. Compiler evidence attacks the real sealing and
admission operations; it does not claim that inert data constructors are
private merely because they cannot open an operational door.

Ledger result artifacts are retained reproducibility and failure-diagnostic
records, not unforgeable authorities. A repository file can always be
hand-authored by someone able to edit the repository. Phase closure therefore
comes only from the governed external closure runner listing and executing the
exact requirement-bound tests on the current source state and then validating
their machine-readable observations. The ledger binds canonical immutable
claim fields while the source-state digest excludes the ledger bytes that the
runner must update. A stored artifact alone, even with internally consistent
digests, cannot change a row to `PROVED`.

`UiNativeApplicationPreparation` owns the host-neutral
`WorthUiApplicationBuilder` and no live subsystem resource. Its public
`builder(&mut self)` returns only a
borrowing `UiNativeApplicationBuilder<'_>`. That borrow mirrors the existing
concrete Worth application registration operations while replacing the owned
builder internally; it cannot freeze, extract, replace, or bind a host. The
scope exposes no worker, readiness, callback, event-loop client, wake proxy,
or raw host-adapter registration. `complete(self)` freezes the internal builder
into a host-neutral application, consumes the binding grant, and is the only
constructor of `UiPreparedNativeApplication`. `deny(self, cause)` proves that
no subsystem or event-loop client could exist and is the only constructor of
`UiNativeApplicationPreparationDenial`. A constant zero cleanup report is not
evidence and is absent; Phase 2 introduces a real census only alongside real
resource owners and consuming stop/join transitions.

Phase 2 consumes `UiPreparedNativeApplication` into a private
`UiNativeApplicationDriverPreparation`. Only that phase or a later native
product-integration phase may ask the real filesystem, Query, intent, and
inspection subsystem boundaries for sealed stopped owners. The first vertical
presentation registers only the application-driver, event-wake, window,
graphics, presentation, and readback owners that it actually opens; it does
not invent placeholder filesystem, Query, intent, or inspection workers.
Whenever a later phase opens one of those real subsystem owners, it must first
register it in the same 32-entry resource registry before activating its
eight-slot readiness capability. The driver retains reverse stop/join and
incomplete-cleanup authority for every owner it has actually opened. A generic
counter thread, owner-kind enum, or test-only refusal posture cannot stand in
for a real subsystem owner.

Phase 1 creates `worth-ui-host-native` as the owner of these checked-in
profiles, capacities, sealed inert mechanics contracts, and trust manifests,
and creates `worth-ui-native-platform` as the effect-free public preparation
and binding owner. Phase 2 activates event-loop, window, surface, and device
effects in those already-existing owners; it does not create placeholder
crates or choose their authority topology. During coexistence the application
platform privately binds either the revision-4 egui mechanics or future native
mechanics through an affine `UiHostMigrationGrant`. The grant is not public
product configuration; Phase 9 admits it only to the existing Pulse runner,
and Phase 10 deletes it with the egui path. Public
`WorthUiApplicationBuilder::with_host(...)` is removed in Phase 1; headless
certification and migration bindings use sealed framework-owned admission.

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
2. Publish the first current value, replace it with a longer mixed-direction
   multilingual value and then one value containing a qualified color-emoji
   ZWJ sequence. Prove layout identity, text/emoji pixels, original source
   ranges, and mounted correlation follow the existing Query path without
   changing Query truth or using host-selected fallback.
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
- Every nontransparent presented-source pixel contribution resolves through a
  retained command to its exact mounted receipt and generation set. Cleared
  uncovered source pixels resolve instead to the exact surface-issued
  transparent baseline receipt; no host-selected clear color or appearance
  value exists.
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

### `HP-02`: Delta and amplification courtroom (Phase 3 closure)

Phase 1 seeds `MountedPresentationWorld::maximum_overlap` through the public
application, production lowering/graph/mounting, runtime work issuer, and real
headless consumer with the 2,048-rectangle ceiling. Its independent manifest
and oracle prove the complete initial order, sparse/partial/full removal,
exact insertion restoration, logical damage, and an unchanged-zero turn.
Phase 3 uses two causally distinct extensions rather than pretending Phase 4
layout or Phase 5 native glyph rendering already exists. The native ordered-
pixel/damage/replay courtroom extends the governed world to the complete
2,048-filled-rectangle ceiling. A separate 4,096-command mixed-carrier world
crosses public lowering, mounting, runtime issuance, and the real headless
consumer with 2,048 filled rectangles and 2,048 inert semantic-text commands at
the revision-4 1 MiB text-byte ceiling. It replaces one text carrier, removes
one rectangle, inserts one rectangle while every other row remains identical,
and then submits an unchanged successor. The mixed world proves delta carriage,
retained indexing, total order, and slope only; it cannot claim Unicode layout,
glyph mechanics, native text pixels, or Phase 4's expanded text capacities.

Command mutation and order planning must consume work proportional to the
three changed commands plus explicitly affected order/damage entries, not all
retained commands. Physical damage replay additionally consumes exactly the
retained commands whose visible bounds intersect admitted damage, in total
paint order. Removing or moving a command clears its vacated region and
replays every intersecting predecessor command required to restore the
correct pixels; any uncovered remainder returns to the surface-issued
transparent baseline rather than an adapter default. A damage index narrows
that set without a retained-list scan;
lawful complete overlap may still require complete replay and must report it.

Named counters distinguish rows carried in the delta, draw-list and order
mutations, damage-index probes, intersecting and replayed commands, cleared,
rendered, and presented pixels, GPU writes, render passes, surface copies,
acquisitions, submissions, and presents. If the native surface requires a
full-area copy or present after local rendering, that physical amplification
is reported separately and is never relabeled local.

The Phase 1 seed's independent model adjudicates command retention, removal,
insertion restoration, logical damage, and total order without reconstructing
expected rows from the production transcript. Phase 3 adds native ordered-
pixel adjudication for the rectangle world and an independent retained-command/
order model for the mixed carrier world. Together they cover clipping,
clearing, overlap, equal-layer order, exact carrier mutation, and retained-state
slope without assigning text pixels to Phase 3. Their mutation controls remove
delta carriage, force a full scan, widen damage, omit vacated replay,
substitute the transparent baseline, change the total-order tie break, or
rescan an unchanged text carrier; each must fail for its named reason.

### `HP-03`: Text and atlas courtroom

Use `MountedPresentationWorld::qualified_text` with the exact
`worth-ui-global-text-v2` font collection and a checked-in, independently
authored corpus containing Latin with combining marks and canonical-
equivalence twins; Greek and Cyrillic; Arabic joining, marks, digits, paired
brackets, and embedded English; Hebrew with numbers; Devanagari and Bengali
conjuncts; Thai and Khmer dictionary-sensitive wrapping; Hangul; CJK
punctuation and wrapping; mixed-script fallback; and Unicode 17.0 emoji
presentation-selector, keycap, flag, tag, modifier, and multi-person ZWJ
sequences. Include hard breaks, tabs, whitespace, narrow and wide wrapping,
grapheme wrapping, maximum lines plus ellipsis, clipping, multiple style and
fallback runs, maximum lawful paragraph bytes, and one valid unassigned scalar
that must use the pinned Last Resort disposition.

The courtroom establishes four independent observations over the same
causally issued mounted rows:

1. Unicode conformance data and a test-owned paragraph oracle adjudicate
   grapheme, word, line-break, bidi-level, logical/visual-order, and original-
   byte-range results without importing production analysis code.
2. Shaping fixtures generated once from the pinned reference HarfBuzz tool and
   exact font assets adjudicate face selection, glyph sequence, advances,
   offsets, cluster ranges, and feature posture; checked-in expected records
   include their tool, data, font, and profile digests.
3. A layout oracle independently fits the shaped records into lines and checks
   baselines, ink/logical bounds, overflow, caret stops, hit-test results, and
   selection rectangles. Measurement requests and native presentation must
   name the same `UiQualifiedTextLayout` identity.
4. The existing external pixel boundary adjudicates monochrome glyph coverage,
   color emoji layers, clipping, ordering, and transparent pixels outside
   glyph ink. The atlas oracle independently models alpha/color capacity,
   pinning, candidate eviction, retained bytes, and reconstruction.

The hostile sequence changes one paragraph and width while leaving all other
text identical, introduces repeated and new alpha/color glyphs, reaches both
atlas capacity boundaries, removes unpinned candidates, preserves live pins,
changes pure DPI, changes framework text scale, destroys every derived layout,
glyph buffer, and atlas page, and reconstructs from mounted authority. Pure
DPI changes preserve logical line and caret geometry while replacing raster
generation; width or text-scale changes replace layout generation. An
unchanged turn performs zero Unicode analysis, fallback probes, shaping, line
layout, rasterization, atlas mutation, upload, surface acquisition, and
presentation work.

The courtroom must turn red if implementation uses an ASCII-only path, shapes
before bidi/run segmentation, reorders source bytes, breaks inside a grapheme
or shaping cluster, chooses fallback per scalar, splits a ZWJ/flag/modifier
sequence, ignores FE0E/FE0F, drops a color layer, uses a system font, allows
measurement and rendering to reshape independently, elides Last Resort
attribution, evicts a pinned glyph, retains stale width/text-scale/DPI state,
clears to an opaque adapter color, scans or reshapes every retained paragraph
for one changed row, or reconstructs pixels from an atlas/cache instead of
mounted authority.

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
with no recipient, stale presentation/input/text-profile affinity, over-
capacity text, and an unprovable range. The oracle distinguishes visible preedit, committed draft
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

- zero `egui`, `eframe`, `egui-wgpu`, and `egui_extras` dependency
  declarations or resolved edges across the repository root workspace, the
  Worth UI workspace, supported member manifests, and final lockfiles;
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
functions. Phase 9 changes glyph-region expectations exactly once through the
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
regimes. `qualified_text` uses the same authority path with the multilingual
and emoji corpus, repeated/new/missing clusters, mixed-direction paragraphs,
exact line/paragraph/alpha-atlas/color-atlas capacity boundaries, pure-DPI,
width, text-scale and font-collection generation replacement, and complete
derived-state reconstruction. The ordered-pixel oracle implements clipping,
clearing, overlap, and total order without calling production ordering, damage,
draw-list, or rendering code. Separate Unicode, shaping-fixture, layout, and
atlas oracles model their exact responsibilities without calling production
analysis, fallback, line fitting, hit testing, or eviction classifiers. Reuse
recompiles from immutable semantic baselines or reconstructs isolated derived
state; no proof-bearing authority or resource handle is cloned, and no scenario
shares mutable layout, draw-list, atlas, device, or counter fate.

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
Pulse application in the one serialized native courtroom. Its Phase 2 seed
crosses the real Windows event loop, process-bound window, observed startup
DPI, surface/device acquisition, presentation-source readback,
compositor-visible capture, normal close, and residue checks. Resize,
minimize/restore, cross-monitor DPI, input, device loss, and held-attempt
schedules remain deterministic protocol evidence until their later real-world
phases; the Phase 2 ledger must not claim those paths from the startup seed.

Only environment-qualified facts receive real-boundary credit. A single-
monitor runner proves its observed DPI basis but not a cross-monitor DPI
transition; injected DPI/device-loss schedules remain protocol evidence.
Conversely, a real successful surface/device/readback path cannot substitute
for the deterministic partial-effect matrix. The ledger names which world
supports each `HP-04` claim.

### `HostRetirementTopologyWorld`

The retirement world reads real workspace metadata and source on the exact
post-cutover tree. It audits the repository-root and Worth UI workspace
manifests, default and all-feature workspace graphs, every supported Windows
target/profile, member and fixture manifests, binaries, examples, final
lockfiles, boundary rules, and source/doc inventories. It parses dependency
declarations as well as resolved metadata so an unused root-workspace entry
cannot survive merely because no current member selects it. Negative fixtures
whose purpose is to prove the prohibition are classified by exact fixture
identity and cannot enter a supported graph. Positive twins prove the native
and headless homes remain lawful; negative twins prove forbidden dependency
and authority directions. It cannot use a curated crate list,
default-feature-only metadata, ignored source roots, or absence from one binary
as proof of retirement.

Each courtroom carries at least one cheap mutation control in the ordinary
gate, and every listed mutant must fail for its own causal reason:

| Courtroom | Required mutants |
| --- | --- |
| `HP-01` | omit or reorder one causal action; substitute production-expected pixels for OS capture; reuse mutable installation state; retain the egui launch path after cutover |
| `HP-02` | remove owner delta carriage; force retained-list discovery; widen damage; break equal-layer order; omit vacated-pixel replay; substitute a host clear color for the surface baseline |
| `HP-03` | retain the Basic-Latin one-run path; shape before bidi/run segmentation; split grapheme, Indic, or emoji ZWJ clusters during fallback/wrapping/ellipsis; ignore emoji variation selectors or color layers; permit system fallback; diverge measurement/render layout identity; evict a pinned alpha/color glyph; retain stale width/text-scale/DPI state; rescan every retained paragraph; skip derived-state reconstruction |
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

Revision 4 remains the complete Phase 1-3 carrier contract and retains its
2,048 semantic-text-row, 4,096-byte-per-row, and 1 MiB aggregate text bounds.
After `worth-ui-global-text-v2` qualification closes, Phase 4 advances the
mounted-frame, presentation, and measurement schemas together to revision 5.
Revision 5 replaces the v1 semantic-text mechanic with span-capable admitted
paragraph constraints and raises only the qualified-text capacities to the v2
limits above: 4,096 paragraphs, 65,536 UTF-8 bytes per paragraph, and 8 MiB
aggregate retained UTF-8. Revision 4 and revision 5 text inputs cannot coexist
inside one agreement, frame, retained generation, or reconstruction. The
revision-5 transition is a cold protocol replacement whose admission completes
before any v2 layout effects; it is not an adapter-side reinterpretation of a
v1 row or a reason to rewrite the proved v1 profile.

The consumer input is the proof-carrying `UiHostProtocolAgreement`, not a raw
protocol contract. Negotiation owns old/mixed-revision denial; headless and
egui prove consumption of the validated revision-4 agreement, while a raw
revision-3/mixed contract cannot be substituted into either consumer. Tests
must not open a second constructor merely to counterfeit an invalid agreement.

The host may reject a malformed or stale delta before effects. It may not
recover by scanning the successor projection, comparing semantic digests,
inventing removals, widening logical scope without accounting, or rebuilding
an unchanged generation. Full reconstruction is a separately named cold or
recovery operation consuming current mounted authority.

Mounting derives one inert `UiMountedPaintOrderIdentity` for every drawable
command identity. Neither identity is orderable; the authoritative meaning is
the runtime-issued sequence that contains it. `Initial` carries the complete
total sequence. `Delta` carries exact `Remove` and `PlaceAfter` edits plus a
successor-order integrity value; `PlaceAfter(identity, None)` establishes the
front and the same operation represents insertion or movement. A local edit
does not renumber unrelated commands. A genuinely global semantic reorder may
carry global work and must report it.

Layer meaning remains inspectable. Equal-layer order across nodes comes from
the mounted semantic-node sequence. Within one node, drawable references are
ordered by their distinct explicit mechanic layers; two drawable mechanics on
the same node and layer deny projection before host effects rather than
manufacturing a family tie break. Command identities are never consulted as a
tie break. Presentation production requires every drawable to be named by
exactly one exhaustive typed drawable-reference source; a missing or duplicate
source is an internal mounting invariant violation before host effects. The
host never supplies a family-, receipt-, hash-, or arrival-based tie break. A
new drawable family cannot compile into presentation until the exhaustive
reference match carries its order and receipt contract.

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

The runtime-side native presentation attribution also carries opaque digests
for the authored-source provenance and authored semantic identity joined from
the exact mounted node receipt through the retained prepared-generation
graph/declaration indexes. The native host does not derive or interpret either
digest. `WindowsNativeBoundaryWorld` compares them with the independently
compiled seed declaration, so changing the authored declaration identity or
consistently reporting the same wrong mounted row cannot satisfy the courtroom
merely through correlated mechanical identities.

### The surface baseline is explicit and carries no appearance authority

Every registered presentation surface carries one move-only
`UiMountedSurfaceBaselineReceipt` issued by the mounting/surface-binding owner.
Milestone 3.14.1 admits exactly one baseline mechanic: canonical transparent
RGBA. The receipt binds semantic surface, binding generation, profile
generation, color/alpha contract, and clear semantics. It is not a mounted node
receipt, does not participate in hit testing or interaction, and cannot mint a
paint command or select an opaque color.

Initial preparation establishes that baseline before replaying the complete
runtime-issued order. Delta damage first restores affected pixels to the same
baseline and then replays every intersecting retained command in total order.
An opaque surface background, window color, theme value, or fallback appearance
must therefore arrive as an attributed runtime-issued filled rectangle; the
native host may not invent it. Presented-source capture preserves transparent
baseline pixels. Compositor-visible client pixels remain a separate OS-level
observation whose relationship to the transparent source is frozen by the
qualified native profile.

The baseline receipt authorizes only deterministic clearing for its exact
surface generation. It cannot publish semantic truth, satisfy a drawable-row
receipt requirement, cross a surface replacement, or be recovered from a
clear color, capture, native surface, or diagnostic record.

The host reports only the inert observation that a registration completed in
the canonical known-empty posture. Runtime/mounting consumes that one current
observation and privately mints the move-only mounted baseline receipt. A
copyable registration request, diagnostic baseline identity, repeated host
confirmation, or reconstructed surface/binding tuple cannot mint a second
receipt. The authoritative surface requirement and baseline lease live with
runtime mounting; the host contract carries only facts a mechanics consumer
must inspect.

### The platform owns scheduling and native resources

`worth-ui-host-native` owns the native event loop on its required thread, all
windows and surfaces registered beneath that loop, graphics adapter/device/
queue acquisition, input translation, redraw admission, capture completion,
device recovery, and normal close. It depends on `worth-ui-host-contract`, not
runtime, the product facade, Query binding, DSL, inspection, or application
code.

`worth-ui-runtime` is the higher application composition owner. The thin
`worth-ui-native-platform` crate re-exports that fixed progression and does not
own a parallel binding transition. Runtime privately binds one host-neutral
application to one qualified mechanics bundle and keeps the Worth session on
the event-loop thread.

The low-level mechanics boundary has one typed event-loop client contract, not
arbitrary closures. Its methods consume host-issued wake grants and return only
an exhaustive `Continue | WaitUntil | Close` directive. Because Rust has no
cross-crate friend visibility, this mechanical integration surface is callable
by a crate that deliberately depends on `worth-ui-host-native`; it is not
described as a platform grant boundary. It cannot receive, construct, or bind a
host-neutral Worth application. The enforceable product authority is the
runtime-private transition that alone owns `UiPreparedNativeApplication` and
its affine binding grant. Supported product topology forbids direct mechanics
dependencies, while focused host mechanics tests may exercise the low-level
contract without claiming application authority.

Worker threads may carry only typed readiness observations through bounded
channels and a native event-loop wake proxy. They do not receive the runtime
session, host adapter, window, device, mounted projection, or publication
authority.

A wake is a level-triggered readiness hint, never the semantic payload.
Each registered owner commits work to its bounded owner slot before signalling
its generation-bearing readiness state. Coalescing preserves the newest ready
generation and cannot erase pending work. The Phase 2 rectangle seed registers
one application presentation owner; later phases add the remaining concrete
filesystem, Query, intent, inspection, input, and capture owners without
replacing this protocol. One event-loop turn drains the committed work; work
committed during or after a drain re-arms exactly one wake. Queue
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
stale text-profile generation, over-capacity text, or unprovable native-range
conversion returns its distinct
typed stop before payload or intent effects and preserves the prior draft as
required by the inherited 3.14 lifecycle. General focus routing remains 3.15
work; this milestone adds no Worth-owned IME service or composition policy.

### Text mechanics have a frozen trust boundary

The closed v1 profile establishes migration provenance only. Before Phase 4,
the separate `worth-ui-global-text-v2` qualification gate commits the exact
default font collection, optional application-font-pack admission contract,
Unicode data and conformance suites, analysis/shaping/raster dependencies,
fallback order, feature/variation posture, layout constraints, raster formats,
and bounds. Changing any of those facts advances the text profile generation
and requires explicit requalification. Ambient OS fonts, locale, renderer
defaults, and dependency upgrades cannot silently change measurement or
pixels.

The host-neutral `worth-ui-text` mechanics crate owns three directionally
ordered derived phases:

```text
UiAdmittedTextParagraph
-> UiAnalyzedTextParagraph
-> UiQualifiedTextLayout
-> UiRasterizedGlyphBatch
```

Runtime owns semantic text, style-profile selection, explicit language/base-
direction, width, wrapping, alignment, line-height, spacing, overflow, clip,
and mounted identity. `worth-ui-text` receives only those completed constraints
plus one admitted font-collection generation. It may analyze, choose a face by
the frozen fallback contract, shape, fit lines, derive cluster geometry, and
rasterize qualified glyphs. It cannot change text, author style, choose a
system font, infer product locale, target interaction, publish a frame, or mint
mounted authority.

`UiAnalyzedTextParagraph` preserves logical source order and carries original-
range grapheme/word/line opportunities, bidi levels, scripts, languages, and
exhaustive run boundaries. `UiQualifiedTextLayout` is the canonical derived
artifact for measurement and rendering. It carries line and visual-run order,
selected face/feature/variation identities, glyph clusters and positions,
logical/ink metrics, baselines, break/overflow decisions, original-range caret
stops and selection rectangles, coverage disposition, complete affinity, and
cost. A measurement result derived from any other shaping/layout pass is
invalid. The artifact is non-authoritative and can be destroyed and rebuilt
from mounted text plus the exact profile/font-collection generations.

The dependency boundary carries that artifact without introducing a crate
cycle or a second owner. `worth-ui-host-contract` defines only inert borrowed
`UiQualifiedTextLayoutView<'layout>` and record-view types sufficient for a
consumer to inspect the qualified result. `worth-ui-text` depends on the host
contract, owns and constructs the concrete immutable layout, and exposes its
borrowed view. Runtime privately retains `Arc<UiQualifiedTextLayout>` beside
the exact mounted text affinity and seals a borrowed view into presentation
work for the active host session. Headless and native hosts may inspect that
view; they cannot construct, clone as authority, reshape, refallback, rebreak,
or extend its lifetime beyond the owning runtime work. The host contract never
imports `worth-ui-text`, and the layout identity alone cannot stand in for the
borrowed records required by measurement or rendering.

Fallback is a font-collection decision over complete clusters, not a loop over
characters. A selected face must shape the complete cluster without `.notdef`
before it is admitted. Emoji presentation selectors, modifiers, regional-
indicator pairs, tag sequences, and ZWJ sequences remain indivisible. If no
qualified face supports the cluster, the pinned Last Resort face produces one
missing-cluster glyph and an explicit coverage disposition while preserving
the exact source range and semantic value. This is distinct from malformed
font, stale generation, impossible mapping, or budget denial.

Raster output distinguishes alpha masks from premultiplied color glyph images.
The native host owns separate bounded alpha and color atlases keyed by font-
collection/profile generation, face, glyph, variation coordinates, palette,
size, raster source, DPI scale, and fractional-origin quantization. Candidate
allocation and deterministic eviction finish before uploads. Retained layouts
pin their exact glyph entries. Saturation denies before effects or enters a
named reconstructive posture; it cannot grow without bound, evict a live glyph,
drop emoji color, or substitute lower-quality pixels.

Text editing, focus, and selection authority remain above mechanics. However,
their later implementations consume v2 original-range mapping, grapheme-safe
caret stops, visual-order hit testing, and selection rectangles directly; they
must not reverse-engineer those facts from glyph positions or introduce a
second text layout engine.

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
control points. The pre-implementation specification-closure gate commits the
exact Windows backend, adapter-selection posture, surface/target formats,
present/alpha modes, blend equations, anti-aliasing posture, shader inputs,
coordinate rounding, and dependency versions as one versioned native profile
manifest. Phase 1 implements and proves that record without changing it.
Vendor defaults and runtime environment discovery are observations admitted by
that profile, never policy.

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
scaffolding. Phase 10 removes that implementation, but `HP-01` is honest only
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

let prepared_platform = WorthUiNativePlatform::prepare(profile)?;
let outcome = prepared_platform.run(
    PlatformPulseApplication::new(launch, publisher),
);

match outcome {
    UiNativePlatformOutcome::Closed(receipt) => observe_close(receipt),
    UiNativePlatformOutcome::ApplicationPreparationDenied(denial) => report(denial),
    UiNativePlatformOutcome::Stopped(stop) => recover_or_report(stop),
}
```

`WorthUiNativePlatform` is exported by `worth-ui-native-platform`. Application
authors do not import `worth-ui-host-native`, `winit`, `wgpu`, shaping,
rasterization, or native-window handle types. The application object supplies
product composition; the platform supplies the only native host grant and
consumes the application at run. Phase 1 exposes only preparation denial and
the explicit `NativeEffectsNotActivatedInPhaseOne` stop. Phase 2 adds typed
close, cleanup, in-flight native-effect, device-recovery, and terminal-resource
outcomes only alongside their real owners and observations.

`prepare(profile)` performs effect-free static profile validation and returns
a move-only `UiPreparedNativePlatform` or a typed profile denial; it creates no
event loop, window, surface, or device. `run(application)` accepts only an
`UiNativeApplicationDefinition`, consumes both it and the prepared platform,
owns all native effects, and returns `UiNativePlatformOutcome` directly.
`ApplicationPreparationDenied` means the application preparation scope closed
before native resource effects and never published an event-loop client. Its
compiler-visible absence of resource-opening operations is the proof; Phase 1
does not manufacture or expose a zero subsystem census. Programmer defects do
not masquerade as that outcome.

The `UiNativeApplicationDefinition` preparation contract consumes one
`UiNativeApplicationPreparation` carrying only a host-neutral public Worth UI
builder and the private affine platform binding. Product code may configure
the host-neutral application but cannot create subsystem or readiness owners,
attach, replace, clone, extract, or downcast a host. Completing the scope seals
the host-neutral application and platform binding into one
`UiPreparedNativeApplication`; product code cannot construct that result from
parts. Phase 2 consumes it, privately creates the real subsystem owners and
readiness progression, composes the fixed native host session, and launches on
the same event-loop thread.

Generic application freeze must therefore succeed without a
`WorthUiHostSessionPlan`. Host binding is removed from generic builder posture
and from frozen application authority. Certification/headless and native
platform composition each own a separate higher transition from the same
host-neutral application into an active session; neither transition exposes a
generic adapter parameter or host-replacement capability.

The exact Rust method signatures and denial payload fields are filled by the
pre-implementation public native-application authority record. They must
preserve the named types and proof relationships above; aliases, generic
callback registration, arbitrary mechanics-client implementations, and a
second host-bearing `WorthUiApplicationBuilder::with_host(...)` path are not
compatible implementations.

## Compile-Time and Mechanical Enforcement

- Runtime's owning `UiMountedPresentationWork` envelope, its
  initial/delta/unchanged choice, lease seal, and completion authority have
  private fields. The host contract may expose inert command, order, damage,
  and input mechanics, but they enter an active host session only through the
  runtime-owned envelope and lease.
- The mounted presentation lease, authoritative work variants, consumption
  view, and completion-token minting live under runtime's private host
  presentation authority. `worth-ui-host-contract` exposes only inert command,
  order, damage, outcome, and cost mechanics. No Cargo feature exposes runtime
  issuance to another crate; Rust features are dependency unification, not
  friend visibility.
- Initial, delta, unchanged, reconstruction, and completion are distinct
  proof-bearing types. A delta cannot be applied to the wrong retained frame,
  surface, binding, capability, device, or profile generation.
- Native draw-command constructors are private to the projection translator
  and require a complete mounted row; a receiptless or generationless command
  is unrepresentable.
- Surface clearing consumes an exact `UiMountedSurfaceBaselineReceipt` for the
  same surface/binding/profile generation. The only 3.14.1 baseline payload is
  canonical transparent RGBA; neither the native nor headless host can supply
  an opaque clear value or use the baseline as a drawable-row receipt.
- The mounted world compiler exposes semantic handles and typed actions with
  private fields. It cannot construct presentation work, native commands,
  atlas entries, platform authority, or expected production outcomes directly.
- Damage-index entries derive only from retained attributed commands. Delta
  execution consumes owner-issued damage plus a mechanical intersection/replay
  plan; neither the index nor replay plan can construct semantic damage or
  presentation work.
- Admitted paragraph, analyzed paragraph, qualified layout, rasterized alpha
  glyph, rasterized color glyph, atlas reservation, uploaded glyph, and
  retained glyph reference are distinct phases. Measurement and rendering
  require the same qualified layout identity. External dependencies cannot
  mint font-collection admission, mounted, interaction, or publication
  authority.
- The v2 font-collection manifest, Unicode conformance-data inventory, default
  fallback order, RGI emoji corpus, and app-font-pack limits are exhaustive
  generated/parsed contracts. Removing a face, script pack, color source,
  conformance file, or fallback entry fails qualification rather than shrinking
  coverage silently.
- Grapheme, shaping-cluster, line-break, bidi, logical/visual-order, caret, and
  original-byte-range identities remain distinct types. Raw byte, scalar,
  glyph, or atlas indexes cannot substitute for one another at public or host
  boundaries.
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
- Event-loop client implementation, wake observation, and platform-binding
  grant are distinct types. The low-level mechanics client is not application
  authority and cannot receive a Worth application; only runtime's move-only
  binding grant can bind the host-neutral application into the fixed native
  product progression.
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
- `worth-ui-host-headless` and `worth-ui-host-native` may depend on
  `worth-ui-host-contract` but not `worth-ui-runtime`, `worth-ui`, DSL, Query,
  inspection, certification, or one another. Runtime-facing session authority
  wraps their inert mechanics contracts from above.
- The host contract and native mechanics cannot import runtime or product
  internals. Runtime's ordinary semantic and mounting modules cannot import
  native mechanics. One isolated host-activation composition module may depend
  on the native mechanics facade solely to name the fixed qualified bundle;
  vendor types and native implementation modules remain unreachable there.
- Runtime's native-platform module privately owns the application-slot grant
  and combines a host-neutral Worth application definition with the fixed
  native mechanics implementation. `worth-ui-native-platform` re-exports only
  that public progression. The generic Worth builder exposes no host-bearing
  bind or replacement transition. No Cargo feature, generic adapter binder,
  caller-defined marker, or host-contract surrogate grants native application
  activation.
- Public outcome matches are exhaustive and `#[must_use]`; wildcard
  translations over lifecycle, presentation, capture, or recovery topology
  are forbidden.
- The workspace gate rejects `egui`, `eframe`, `egui-wgpu`, `egui_extras`, the retired
  adapter, and the migration selector. It parses every repository workspace
  dependency declaration and final lockfile in addition to resolved metadata,
  including the root `Cargo.toml`; `egui-wgpu` is prohibited with the rest of
  the family. Positive/negative twins prove the intended native and headless
  homes remain lawful.
- Compiler evidence stays in the existing consolidated compiler sessions.
  Production compilation and local model tests enforce private topology that
  has no public misuse value.

## Cost Contract

This is the destination contract closed by Phase 3 for retained delta work and
by later text phases for shaping/atlas work. Phase 1 proves only that the
revision-4 carrier is sparse, attributable, and reports exact carried
command/order/damage lengths, plus an unchanged carrier with no rows. It does
not claim that the current producer or migrated headless compatibility
consumer computes that carrier in `O(k)`: their retained-size scans and clones
remain named Phase 3 work. Phase 2 presents one Initial and does not claim a
retained-delta slope.

Ordinary host work is:

```text
delta validation       O(k + o)
draw-list mutation     O(k log n) worst case, with bounded indexed lookup
damage selection       O(d log n + r) for indexed intersection and order
damage replay          O(r + p_render)
Unicode analysis       O(t_changed + c_context) only for changed paragraphs
fallback/run shaping   O(c_affected + f_probes + g_affected)
line layout            O(b_affected + g_affected + l_affected)
atlas lookup/update    O(g_changed) with bounded candidate allocation
GPU upload             O(bytes changed or reconstructed)
presentation           explicit damaged work + physical surface amplification
```

`n` is retained command count, `k` changed commands, `o` owner-issued order
changes, `d` damage regions, `r` retained commands intersecting admitted
damage, `p_render` pixels actually rerendered, `t_changed` changed text bytes,
`c_context` the exact Unicode/shaping context invalidated around those bytes,
`c_affected` affected grapheme/shaping clusters, `f_probes` deterministic font
coverage probes, `g_affected` affected glyphs, `b_affected` affected break
opportunities, and `l_affected` affected lines. Complete overlap may lawfully
make `r = n`, and a context-sensitive script or width change may lawfully
invalidate a complete paragraph; each index and counter must prove that breadth
rather than scanning every retained command or paragraph to discover it. The
final implementation may prove stronger bounds but may not weaken an ordinary
local delta to a retained-list scan or an unchanged paragraph to reanalysis.

Counters separately expose:

- initial/delta/unchanged work and rows consumed;
- commands inserted, replaced, removed, reordered, reused, and retained;
- logical damage regions/pixels, damage-index probes, commands selected and
  replayed, cleared/rerendered pixels, and physical copy/present pixels;
- analyzed paragraphs/bytes/scalars/graphemes, bidi runs, script/language/style
  runs, line-break opportunities, fallback probes, shaped runs/clusters/glyphs,
  lines, caret stops, selection rectangles, alpha/color rasterizations,
  alpha/color atlas hits/misses/evictions, upload bytes, and retained layout/
  atlas bytes;
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
| `worth-ui-host-headless` | production headless mechanics, bounded transcripts, measurement evidence, and record-only presentation over the inert host contract | runtime internals, native resources, application truth, certification-only authority |
| `worth-ui-text` | qualified font collections, Unicode analysis, deterministic fallback, complex shaping, line layout, canonical measurement/cluster geometry, and alpha/color glyph rasterization | authored meaning, mounted authority, GPU resources, system fonts, input/focus/editing authority |
| `worth-ui-host-native` | native scheduling and readiness, windows, surfaces, graphics device, derived draw list, input translation, alpha/color atlas, readback, resource recovery | runtime, authored meaning, text-layout policy, targeting, intents, Query, publication |
| runtime native platform | effect-free application preparation, private native binding, prepared-application driver handoff, and terminal outcome projection | vendor mechanics, drawing, target or intent authority |
| `worth-ui-native-platform` | thin public facade over the runtime-owned native application lifecycle | binding issuance, runtime internals, vendor mechanics, drawing |
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
        surface_baseline.rs                     [create: transparent baseline authority]
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

    worth-ui-runtime/src/host/adapter/           [modify]
      headless_{host,measurement,recorder,
                transcript,translation}.rs       [move to host-headless]

    worth-ui-host-headless/                     [create by moving production headless mechanics]
      Cargo.toml                                [host-contract-only WUI dependency]
      src/
        lib.rs                                  [facade only]
        adapter/{mod,host,measurement,recorder,presentation}.rs
        transcript/{mod,frame,static_paint,semantic_text}.rs

    worth-ui-text/                              [create in Phase 4: shared text mechanics]
      Cargo.toml                                [host-contract-only WUI dependency]
      assets/fonts/                             [exact v2 catalog and licenses]
      profiles/worth-ui-global-text-v2.toml     [canonical qualification manifest]
      src/
        lib.rs                                  [facade only]
        profile/{mod,manifest,identity,capacities,qualification}.rs
        font_collection/
          {mod,admission,face,coverage_index,fallback,application_pack}.rs
        analysis/
          {mod,paragraph,grapheme,word,line_break,bidi,script_language,
           original_range}.rs
        layout/
          {mod,request,run_segmentation,shaping,line_fitting,overflow,
           artifact,caret,selection,cost}.rs
        raster/
          {mod,request,alpha,color,source_order,cost}.rs

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
          {mod,layout_admission,glyph_batch}.rs  [consume worth-ui-text]
          atlas/
            {mod,state,reservation,eviction,pinning,cost}.rs
            page/{mod,alpha,color}.rs
        input/
          {mod,observation,pointer,keyboard,text_ime}.rs [OS translation only]
        capture/
          {mod,port,admission,readback,completion,cancellation}.rs
        lifecycle/
          {mod,resource_registry,census,recovery,shutdown}.rs

    worth-ui-runtime/src/native_platform/       [create: private binding gate]
      {mod,application,application_driver,
       native_platform_binding,outcome,platform,profile}.rs

    worth-ui-native-platform/                   [create: public facade]
      Cargo.toml
      src/
        lib.rs                                  [facade only]

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
      product_process/host_migration_grant.rs   [create in Phase 9; remove in 10]
```

The stable structural axes are mounted authority, inert host protocol,
headless mechanics, shared text mechanics, native mechanics, application-
platform composition, retained presentation, graphics, input observation,
capture, and derived evidence. Headless is a real contract consumer and
production proof surface, not runtime-owned semantics or certification
support; it consumes `worth-ui-text` for the same qualified layout and depends
on no runtime internal. Native consumes that layout/raster facade and alone
owns GPU atlases. Do not flatten these axes
into `renderer.rs`, `text.rs`, `font_manager.rs`, `host.rs`, `state.rs`,
`resources.rs`, `helpers`, `common`, a product callback bag, or the
application session.

Committed successors enter additively:

- 3.15 adds framework deadlines and service-ready wake causes without moving
  event-loop ownership; portal layers become new mounted command/order input,
  never host-local popup state;
- 3.16 adds appearance-derived mounted values upstream; the host continues to
  render completed mechanics without theme authority;
- later focus/editing/accessibility work consumes the existing text layout's
  original-range, grapheme, caret, visual-order, and selection geometry and
  adds semantic owners above it; rich text adds authored span meaning to the
  existing span-capable request rather than a second shaper or renderer;
- Milestone 4 adds window registrations beneath the existing native window
  registry rather than replacing a single-window loop;
- later vector, icon, canvas, and realtime mechanics add command-family
  siblings beneath presentation with the same receipt/order contract; and
- accessibility adds a distinct mounted effect/observation boundary rather
  than being inferred from draw commands.

## Ordered Phases

### Current implementation status

Phase 1-2 production requirements were closed at revision `234c3aaf4` by
thirty `PROVED`, `final_source=true` ledger rows. This specification/roadmap
revision changes governed source bytes, so the operational verifier must
reexecute the immutable prefix and `P3-PREDECESSOR-01` must retain that fresh
current-source handoff before any other Phase 3 proof. The predecessor rows,
nonces, and artifacts are not refreshed or rewritten. This does not reopen
their production behavior or v1 profile evidence. Later phases remain ordered
and open. The Phase 2 rectangle seed does not imply retained-delta, text,
input, resize, capture, recovery, or parity evidence. The framework-grade text
decision separately reopens the future Phase 4 qualification gate.

### Phase 1: Protocol, qualification, and topology closure

Consume the closed qualified native/text profile and public native-application
authority records without altering them. Implement the owner-issued
initial/delta/unchanged protocol, exact
predecessor/successor affinity, stable total paint order, logical damage,
expanded cost vocabulary, explicit transparent surface baseline, and the
runtime/mounting producer. Move presentation issuance, the authoritative
surface requirement, and baseline receipt into runtime-owned private modules;
remove feature-gated cross-crate issuers. Keep native binding authority private
to native composition and make generic application preparation host-neutral.
Move production headless mechanics into
`worth-ui-host-headless`, then update egui, headless, and certification
consumers through the same revision-4 contract; none may retain the
complete-projection ordinary lane or import runtime internals. Materialize the
already-governed `BodyDefault` asset/profile manifest, native Windows profile,
pinned dependency versions and trust record, public native-application types,
capacities, destination tree, and egui retirement inventory exactly as the
pre-implementation records require. Add compile/topology/dependency enforcement
plus the governed mounted world compiler, independent delta/order/damage
oracles, and the test-owned control-point manifest with filled-rectangle
expectations; glyph expectations remain pending. No later phase begins while a
hidden default, unbounded capacity, mixed protocol, host-side rediscovery path,
headless runtime coupling, or oracle import from disputed production mechanics
remains open. Phase closure records the committed manifest identities in the
proof ledger and proves they match the specification; implementation code may
not select or amend them.

Phase 1 remains one atomic trust gate, but its implementation plan closes five
ordered internal batches: protocol and runtime producer; egui plus the moved
headless and certification consumer migration; materialization and proof of
the prequalified font/native/application records; governed world compiler plus
independent oracles; and topology/compiler enforcement plus ledger closure.
These batches are not independently trusted phases. Phase 2 waits for their
combined exit gate rather than advancing on partial revision-4 adoption or
provisional manifests.

Phase 2 may trust complete, ordered, attributable presentation work, the frozen
native profile, the v1 migration text profile, and mechanically enforced
dependency direction. It may not treat v1 as Phase 4 text qualification.

Phase 1 closes only when the ledger proves all twenty exact contracts:
`P1-AFFINITY-01`, `P1-AUTHORITY-01`, `P1-BACKEND-FEATURES-01`,
`P1-BASELINE-01`, `P1-CLOSE-01`, `P1-CONSUMERS-01`, `P1-DAMAGE-01`,
`P1-HEADLESS-01`, `P1-HEADLESS-COST-01`, `P1-ORDER-01`,
`P1-ORDER-SOURCE-01`, `P1-PLATFORM-AUTHORITY-01`,
`P1-PREPARATION-LIFECYCLE-01`, `P1-PRESENTATION-AUTHORITY-01`,
`P1-PRODUCER-01`, `P1-PRODUCER-COST-01`, `P1-PROFILE-01`,
`P1-PROTOCOL-01`, `P1-TOPOLOGY-01`, and `P1-WORLDS-01`. Each row has a
schema-owned exact owner, boundary, world, proof kind, authority source,
mutation family, and counter family. A generic nonblank evidence record cannot
stand in for one of these contracts.

`P1-PRODUCER-COST-01` and `P1-HEADLESS-COST-01` are carrier-shape contracts:
they prove exact sparse payload lengths and unchanged-zero carriage only. They
do not satisfy or rename the Phase 3 computational-slope, retained-index, or
native replay contracts.

### Phase 2: First native vertical presentation

Activate native effects in the Phase 1-created mechanics and runtime-owned
application-platform gate: Worth-owned event loop, private affine platform
binding, application driver, host-native level-triggered readiness
scheduling, Windows window/surface/DPI lifecycle, graphics device/queue,
retained target, and resource registry. Present one attributable initial filled
rectangle through `WindowsNativeBoundaryWorld` in a real native window, observe
its actual client pixels, remain quiescent without readiness, and close with
exact zero framework resources. Establish the production external-effect ports
and their real implementations at the boundaries this vertical slice crosses.
External quiescence compares only the courtroom-owned opaque rectangle control
points. Transparent retained baseline is proved at presented-source readback;
the OS compositor backdrop behind transparent client pixels is observed but is
not assigned to WORTH or frozen to an expected color.
This phase stays vertical: it does not end at a window skeleton, clear color,
fake surface, or in-memory renderer.

Phase 3 may trust one owned, visible native lifecycle and one attributable
initial presentation path without continuous repaint or vendor leakage.

Phase 2 closes only when the single proof ledger contains and proves these
rows: `P2-APPLICATION-01` for the prepared-application driver handoff,
`P2-EVENT-LOOP-01` for event-loop thread ownership, `P2-READINESS-01` for
level-triggered scheduling and quiescence, `P2-WINDOW-01` for the real Windows
window/surface/DPI lifecycle, `P2-GRAPHICS-01` for selected device, queue, and
retained target ownership, `P2-PRESENT-01` for one runtime-attributed initial
filled rectangle, `P2-PIXELS-01` for independent client-area pixels,
`P2-PORTS-01` for the crossed production external-effect ports,
`P2-CLOSE-01` for terminal zero-resource cleanup, and `P2-WORLD-01` for the
environment-qualified `WindowsNativeBoundaryWorld`. Those ten rows are closed
on the governed source revision with exact commands, sources, observations,
teardown, cost, and mutation evidence. Their rectangle-only evidence cannot be
reused to claim Phase 3 retained-delta or Phase 4-5 text behavior; an in-memory
renderer or window-only smoke test cannot prove either successor.

### Phase 3: Retained delta, total order, and damage replay

Build the receipt-keyed draw list, exact command mutation and order indexes,
derived damage intersection index, total-order replay plan, staged presentation
transaction, unchanged zero path, reconstruction from mounted authority, and
complete structural/amplification counters. Close `HP-02` and mutation controls
in `MountedPresentationWorld::maximum_overlap` against retained-list scans,
changed-command-only redraw, widened damage, stale delta reuse, vacated pixels,
and equal-layer nondeterminism.

Phase 3 closes only when the append-only ledger proves
`P3-PREDECESSOR-01`, `P3-STALE-DELTA-01`, `P3-DELTA-SOURCE-01`,
`P3-PRODUCER-SLOPE-01`,
`P3-DRAW-LIST-01`, `P3-TOTAL-ORDER-01`, `P3-DAMAGE-INDEX-01`,
`P3-DAMAGE-REPLAY-01`, `P3-BASELINE-REPLAY-01`, `P3-TRANSACTION-01`,
`P3-UNCHANGED-01`,
`P3-RECONSTRUCTION-01`, `P3-HEADLESS-COST-01`,
`P3-PHYSICAL-AMPLIFICATION-01`, `P3-HP02-WORLD-01`, and `P3-CLOSE-01`.
`P3-PREDECESSOR-01` records the current-source operational revalidation of the
immutable Phase 1-2 prefix. The slope rows compare retained sizes 1, 32, 2,048,
and 4,096 with constant changed/damage/replay widths and require exact retained-
row scan and clone counters rather than elapsed-time thresholds. The native
world row owns the 2,048-rectangle pixel courtroom; the mixed 4,096-command
headless world supports carrier/index/slope rows and cannot supply native text
pixel evidence.

Phase 4 may trust scalable filled-rectangle initial, delta, unchanged,
overlap-correct replay, and reconstruction behavior.

### Phase 4: Qualified Unicode text platform and canonical layout

First close the blocking `worth-ui-global-text-v2` qualification record:
exact font faces/licenses/digests, Unicode 17 data and conformance corpus,
analysis/shaping/raster dependencies, generated coverage/fallback indexes,
emoji corpus, capacities, and profile identity. Then create the host-neutral
`worth-ui-text` mechanics crate and advance the host text protocol so runtime
issues complete paragraph/style/language/direction/width/overflow constraints.
Implement original-range mapping, UAX #29 grapheme/word segmentation, UAX #14
plus qualified dictionary line opportunities, full UAX #9 bidi analysis,
script/language/style/face run segmentation, deterministic cluster fallback,
HarfBuzz-compatible complex shaping, line fitting, cluster-safe clipping and
ellipsis, logical/ink metrics, baselines, caret stops, hit testing, selection
rectangles, and the reconstructible `UiQualifiedTextLayout` artifact.
Unicode 17 RGI emoji is a first-class Phase 4 layout obligation: variation
selectors, keycaps, regional-indicator flags, tag sequences, skin-tone
modifiers, and gendered/family ZWJ sequences remain atomic through
segmentation, fallback, shaping, wrapping, ellipsis, caret movement, hit
testing, selection, and original-range mapping. Phase 5 adds their actual
color-glyph raster and native pixel proof; it does not get to repair a broken
Phase 4 cluster or fallback decision.

Headless measurement and transcripts must consume this exact artifact; the
native host may not yet render it. Close the Unicode, shaping-fixture, layout,
measurement-identity, missing-cluster, stale-affinity, capacity, unchanged-
zero, and reconstruction portions of `HP-03`. A one-run Latin implementation,
scalar fallback, separate measurement shaper, or output lacking original-range
cluster/caret geometry cannot pass this gate.

Phase 4 closes only when the append-only ledger proves
`P4-TEXT-PROFILE-01`, `P4-FONT-COLLECTION-01`, `P4-UNICODE-SEGMENTATION-01`,
`P4-EMOJI-SEQUENCE-01`, `P4-BIDI-01`, `P4-FALLBACK-01`,
`P4-SHAPING-01`, `P4-LINE-LAYOUT-01`, `P4-CAPACITY-01`,
`P4-MEASUREMENT-IDENTITY-01`, `P4-ORIGINAL-RANGE-01`,
`P4-TEXT-RECONSTRUCTION-01`, `P4-UNCHANGED-01`, and `P4-TEXT-COST-01`.
Each row owns its exact
Unicode/reference-data digest, corpus slice, production transition,
independent oracle, mutation, counters, and artifact identity; one generic
multilingual screenshot cannot satisfy them.

The Phase 4 qualification gate and production closure are separate ledger
events. `P4-TEXT-PROFILE-01` cannot become `PROVED` until the canonical v2
manifest, every referenced font/license/data/corpus artifact, generated index,
dependency feature posture, and manifest digest exist and independently
validate. The other Phase 4 rows remain `OPEN` until that row is final-source
green; a provisional manifest identity or dependency-default behavior cannot
be consumed as production configuration.

Phase 5 may trust bounded deterministic multilingual/RTL/complex-script/emoji
layout and measurement from repository or explicitly application-bundled fonts,
with no system-font or adapter-layout authority.

### Phase 5: Color glyphs, emoji, atlas, and native text presentation

Consume only Phase 4 `UiQualifiedTextLayout` artifacts. Implement typed alpha
and color glyph raster batches, grayscale outline and color outline/bitmap
source order, receipt-attributed glyph-run commands, separate bounded alpha and
RGBA atlases, live-layout pinning, deterministic candidate eviction, staged
uploads, pure-DPI raster replacement, text-scale/layout replacement, and
complete layout/raster/atlas reconstruction from mounted authority. Exercise
the full Unicode 17 RGI emoji corpus classes, including variation selectors,
flags, modifiers, tags, keycaps, and multi-person ZWJ sequences; no color layer
or cluster may be dropped or split.

Close all remaining `HP-03` atlas, native pixel, external adjudication, cost,
saturation, and hostile mutation evidence and produce the glyph rebaseline
candidate without yet changing the cumulative Pulse expectation. Measurement,
hit testing, rasterization, and rendering must report one layout identity.

Phase 5 closes only when the ledger proves `P5-GLYPH-RASTER-01`,
`P5-COLOR-EMOJI-01`, `P5-ATLAS-01`, `P5-ATLAS-PINNING-01`,
`P5-TEXT-DPI-01`, `P5-TEXT-PIXELS-01`, `P5-TEXT-RECONSTRUCTION-01`, and
`P5-TEXT-COST-01`, including an external color-emoji observation and separate
alpha/color atlas/resource census.

Phase 6 may trust deterministic framework-grade text layout and attributable
multilingual/color-emoji pixels without system fallback, environment-selected
coverage, duplicate shaping, or unbounded retained state.

### Phase 6: Native input and presentation affinity

Replace egui input translation with host-neutral native observations for every
shipped 3.14 family. Prove exact last-completed-presentation affinity, input
before first presentation, input during successor presentation, event-time
DPI/resize basis, lossless ordering, bounded observation delivery, and the
inherited gesture/IME distinctions. Close the input portion of `HP-04` and the
3.14 interaction mutation controls through `WindowsNativeBoundaryWorld` for
real OS delivery and `NativeLifecycleProtocolWorld` for unavailable or
adversarial schedules. Exercise preedit, composition commit, cancel, canonical
range conversion, and their no-recipient, stale-affinity, stale-text-profile,
over-capacity, and unprovable-conversion stops. Prove preedit never enters payload, IME commit does
not mint semantic `edit-commit`, and a native event, current coordinate, or in-
flight frame cannot retarget itself.

Phase 7 may trust the complete native input-to-presented-target path without
targeting, intent, or publication authority in the host.

### Phase 7: Presented-source capture and snapshot integration

Implement capture admission, retained-target copy, fence-bound completion,
canonical readback, bounded buffers/maps, cancellation before effects, and
indeterminate completion after effects may begin. Join presented-source pixels
to the existing snapshot identity and independently compare them with external
client-area control points. Close the capture portion of `HP-04` through both
native worlds and the test-owned control-point manifest, keeping real readback
and injected completion claims distinct. Draw lists, expected-color tables,
reconstructed images, and compositor screenshots cannot impersonate one
another.

Phase 8 may trust exact bounded capture of the native presentation source with
typed affinity, cancellation, disposal, and external pixel correlation.

### Phase 8: Failure, recovery, and hostile shutdown

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

During Phase 2, before qualified text exists, the Platform Pulse library
supplies `PlatformPulseNativeSeedApplication`: one public-composition-root
application containing exactly one attributed filled rectangle and no text.
That seed is the only honest application the first native vertical can present
atomically. It does not impersonate the complete Pulse product journey. The
full source/Query/intent/text application replaces the seed in its ordered
phases and is the application used by the cumulative `HP-01` courtroom.

Phase 9 may trust the complete native product path under ordinary, denied,
in-flight, indeterminate, recovery, and shutdown postures.

### Phase 9: Dual-host cumulative parity and native candidate freeze

Introduce the migration-only launch grant and run `HP-01` against egui and
Worth native hosts through the same Pulse binary, composition root, runner,
versioned causal-action manifest, and evidence schema in
`PulseNativeParityWorld`. Record the migration-grant identity as the sole
differing world input. Close cumulative semantic, receipt, interaction, intent,
consequence, capture, lifecycle, and control-pixel parity; adjudicate and record
exactly one glyph-region rebaseline. Freeze the proven native candidate and
cutover inventory. This phase does not delete or silently bypass the
predecessor until parity evidence is final-source green.

Phase 10 may trust a migration-complete native candidate whose only remaining
work is authority cutover and predecessor removal.

### Phase 10: Native cutover, egui deletion, and final closure

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
Within one Phase 1 closure or operational-verification run, `P1-WORLDS-01`
owns the single 2,048-row maximum-overlap courtroom. `P1-HEADLESS-COST-01`
binds that exact artifact by digest and validates its cost-specific observation
without reconstructing or replaying the world; its cost record names one
shared mounted-world reference and zero marginal main-test or presentation
executions.
Within one Phase 2 closure or operational-verification run, `P2-WORLD-01` owns
the single `WindowsNativeBoundaryWorld` process/window/GPU execution and its
immutable result artifact. The other Phase 2 rows bind that exact artifact by
digest, validate their requirement-specific observation slice, and execute
only their own cheap hostile control. Their cost records name one shared-world
reference and zero marginal process, courtroom, presentation, or main-test
executions; they may not relaunch the same native world per row.
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
| `workspaces/worth-ui/README.md` and `AI_README.md` | Developers discover the Worth native launch path, the sealed application-preparation contract, and that mounted/runtime meaning remains above native or headless mechanics. | Public launch example compiles and dependency audit stays green. |
| `workspaces/worth-ui/docs/native-host-platform.md` | Application authors and maintainers learn application preparation/resource cleanup, lifecycle, wake scheduling, exact supported platform/text posture, unsupported-Unicode predecessor preservation, transparent surface baseline, OS IME translation versus runtime draft authority, typed failures, recovery, capture, cost, and anti-patterns. | Examples compile; qualification identities and `HP-02`-`HP-04` prove described behavior. |
| `workspaces/worth-ui/docs/text-platform.md` | Application authors and text/input/component maintainers learn v2 font-pack admission, Unicode 17 support, deterministic fallback and Last Resort posture, bidi/complex-script/emoji behavior, wrapping/overflow, measurement-layout identity, caret/selection geometry, DPI versus text scaling, capacity denials, atlas lifecycle, and the boundary between text mechanics and later editing/accessibility meaning. | Public font-pack and measurement examples compile; Unicode/HarfBuzz fixtures, the complete `HP-03` corpus, and text topology enforcement remain green. |
| `workspaces/worth-ui/docs/application-lifecycle.md` | The cumulative Pulse command and visible journey use the Worth host, including native actions and exact-zero close. | `HP-01`. |
| `workspaces/worth-ui/docs/architecture.md` and `runtime-subsystems.md` | Ownership of presentation work, contract-only headless/native mechanics, public application-platform preparation, derived retained state, and 3.15 insertion is accurate. | Topology and dependency enforcement. |
| `workspaces/worth-ui/docs/visual-inspection.md` | Snapshot capture is exact presented-source-target readback with typed affinity, cancellation, budget, disposal, and a distinct compositor-visible observation posture. | Native capture and anti-substitution evidence. |
| `workspaces/worth-ui/docs/interaction-and-intents.md` | Native observations replace egui translation without changing targeting, IME/draft phase distinctions, or intent authority. | 3.14 cumulative input evidence. |
| `_docs/worth-ui/worth_ui_roadmap.md` | 3.14.1 contract closure and 3.15 dependency remain current. | Documentary consistency audit. |

Delete or rewrite every continuing example that names eframe, egui context,
adapter repaint, or the retired host. Do not create a milestone closeout guide
or migration guide with no post-cutover audience; the ledger records the
temporary coexistence and glyph rebaseline.

## Must Ship and Preserve

Ship the owner-issued presentation-work protocol, total paint order, Worth
headless mechanics, native mechanics, and application-platform crates,
event-driven lifecycle, the sealed public native-application preparation
progression, retained attributable draw list, surface-issued transparent
baseline, indexed overlap-correct damage replay, filled rectangles, the
qualified Unicode 17 text platform, deterministic font collection/fallback,
complex shaping, bidirectional and line layout, canonical measurement/cluster/
caret geometry, multilingual and color-emoji rasterization, bounded alpha/
color atlases, level-triggered readiness, native 3.14 input observations,
exact presented-source-target
capture, failure/recovery lifecycle, exact resource census, the governed
five-world proof portfolio, independent oracles, versioned causal-action and
control-point manifests, dual-host parity, one glyph rebaseline, final egui
deletion, and recurrence gates described above.

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

Planning for each open phase begins only after its qualification records are
complete and internally consistent. In particular, Phase 4 remains blocked
until `worth-ui-global-text-v2` contains the exact asset/dependency/data/corpus
record required above. The milestone closes only when `HP-01`-`HP-05` have independent,
mutation-sensitive evidence; all governed native resources reach exact zero;
the one glyph rebaseline is recorded; the Unicode 17 conformance and RGI emoji
corpus is complete; public examples compile; continuing docs agree; the
production headless adapter is contract-only and outside runtime;
no egui-family dependency declaration, resolved edge, lockfile package, or path
remains outside explicitly classified negative fixtures; and formatting,
strict lint, line-cap, test-topology, boundary, agent-context, ordinary
certification, compile-contract, native integration, and executable-world gates
are green on the exact final source.

`milestone-3.14.1-proof-ledger.csv` is the single closure ledger. Its current
thirty-row Phase 1-2 prefix is immutable predecessor evidence. Phase 3 appends
its sixteen rows before implementation evidence is admitted; Phase 4 appends
its fourteen rows before v2 qualification work begins, and later phases append
their named rows in the same fashion. Existing rows, nonces, artifacts, claims,
and selected-source digests are not rewritten to make room. Historical rows
retain their recorded revision, source metadata, claim, and artifact byte-for-
byte. The old dirty-tree source snapshots are not reconstructible from Git and
therefore are never presented as current-source authority. The newest phase
owns one operational predecessor-handoff row that reexecutes the immutable
prefix against the current tree and retains a current whole-tree source-state
digest. This keeps the historical claim immutable without allowing a new phase
to trust stale or unreconstructible production. Every row records
phase, requirement, owner,
production boundary,
world identity/version, proof kind and evidence-schema identity,
baseline digest, scenario delta, generated seed where applicable, authority
provenance, production entry, independent oracle, mutation control, fault-
injection boundary, retained failure artifact, teardown result, construction/
execution cost, exact command, exact matched-test count, command result,
retained result-artifact identity and digest, source revision, selected-source
digest, whole-tree source-state digest, unique run nonce, source identity,
font/native profile identity and canonical manifest digest,
platform/dependency versions, structural counters, result, reopen lineage, and
final-source status for `HP-01`-`HP-05`. Font/native qualification, the
migration candidate freeze, glyph rebaseline, and post-deletion native rerun
are named ledger events. Presented-source readback and compositor-visible
client observation remain distinct evidence columns. Fixture failure remains
distinct from product denial or non-success. The ledger explains and certifies;
it cannot configure the host or substitute for production profile types.

The ordinary warm gate must satisfy the numerical phase budgets above.
Ledger commands are executed by one governed phase-closure runner. For every
required row through the requested phase it lists the compiled test target,
requires the fully qualified `--exact` test name to match exactly once, runs
it, validates requirement-specific machine-readable observations, and retains
an artifact binding
package, target, test name, matched count, exit posture, source revision, and
selected-source digest. It requires exactly one executed and passed test with
zero ignored tests. It binds the whole tracked diff except the ledger itself
and all nonignored untracked source bytes before and after the run. The artifact
also binds a canonical digest of the row's immutable claim fields—world,
scenario, authority, entries, oracle, mutation, failure boundary, teardown,
costs, counters, observations, profiles, and source identity—so recording the
execution result cannot invalidate the source-state digest and no later ledger
claim edit is invisible. It emits a run nonce and retains a diagnostic artifact
whose digest is checked by the ledger. Proved rows cannot reuse a run nonce or
result-artifact identity. The closure runner, not the retained file, is the
evidence authority: a zero-match success, trivial unrelated passing test,
stale source state, missing required observation, or hand-authored artifact
cannot close a phase.
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
