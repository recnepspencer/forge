# Milestone 3.14.1 Phase 5 Implementation Plan

## Objective

Freeze destination authority for color glyphs, atlases, and native text
presentation before any raster or atlas production begins. Phase 4 remains
the closed layout and measurement owner. Phase 5 may consume
`UiQualifiedTextLayout` only.

## Destination Topology

| Responsibility | Owner | Destination types | Forbidden consumers |
|---|---|---|---|
| Typed alpha and color raster batches | `worth-ui-text` | `UiAlphaRasterBatch`, `UiColorRasterBatch`, `UiGlyphRasterCost` | hosts, runtime, Query |
| Mounted layout authority | `worth-ui-runtime` | `Arc<UiQualifiedTextLayout>` | hosts may borrow views only |
| Borrowed layout views | `worth-ui-host-contract` | `UiQualifiedTextLayoutView` | no owned layout artifact |
| Alpha and RGBA atlas lifecycle | `worth-ui-host-native` | `UiAlphaAtlasLifecycle`, `UiRgbaAtlasLifecycle`, `UiAtlasPin` | text crate, headless, egui |
| Paint-span identity | `worth-ui-runtime` mounting | mounted paint-span plus logical straight RGBA | no visual-order color assignment |

No Phase 5 consumer may reshape, refallback, rebreak, or consult system fonts.

## Ledger Inventory And Future Proof Boundary

| Requirement | Owner | Production boundary | Oracle | Mutation |
|---|---|---|---|---|
| `P5-PREDECESSOR-01` | worth-ui-certification | current Phase 1-4 handoff | predecessor artifact | stale-phase-four-source |
| `P5-GLYPH-RASTER-01` | worth-ui-text | real typed raster production | unimplemented causal oracle | consumer-reshape-or-system-font |
| `P5-COLOR-EMOJI-01` | worth-ui-text | exhaustive intrinsic-color clusters | unimplemented causal oracle | emoji-tint-or-split |
| `P5-ATLAS-01` | worth-ui-host-native | real separate atlas owners | unimplemented causal oracle | host-atlas-escape |
| `P5-ATLAS-PINNING-01` | worth-ui-host-native | live-layout pin lifecycle | unimplemented causal oracle | live-layout-unpin |
| `P5-TEXT-DPI-01` | worth-ui-text | pure DPI raster replacement | unimplemented causal oracle | stale-dpi-raster |
| `P5-TEXT-SPAN-PAINT-01` | worth-ui-runtime | paint-span identity | unimplemented causal oracle | single-color-or-visual-order-or-layout-regen |
| `P5-TEXT-PIXELS-01` | worth-ui-host-native | headless/native paint-span pixels | unimplemented external oracle | transcript-pixel-mismatch |
| `P5-TEXT-RECONSTRUCTION-01` | worth-ui-runtime | mounted-authority reconstruction | unimplemented causal oracle | stale-raster-reuse |
| `P5-TEXT-COST-01` | worth-ui-text | ordinary vs reconstructive cost | unimplemented slope oracle | complete-document-rescan |
| `P5-CLOSE-01` | worth-ui-certification | Phase 5 closure | unmapped until every feature proof exists | open-requirement |

These rows stay `OPEN`. Only `P5-PREDECESSOR-01` has a governed smoke mapping.
The destination topology tests are not ledger proofs and emit no row counters or
mutation receipts. Full Phase 5 closure fails its completeness check until the
nine feature mappings and the close mapping are backed by real production.

## Cost And Resource Vocabulary

- Ordinary lane: glyph rasters for newly damaged paint spans at the current
  DPI and text scale. Cost is the `ordinary` lane in `UiGlyphRasterCost`.
- Reconstructive lane: full layout, raster, and atlas rebuild from mounted
  authority after loss. Cost is the `reconstructive` lane in
  `UiGlyphRasterCost`.
- Atlas resources: one bounded alpha atlas and one bounded RGBA atlas, each
  with live-layout pins and deterministic candidate eviction.
- Color-only successor: reuse the exact `UiQualifiedTextLayout`, shaped
  glyphs, geometry, and atlas entries; update only affected glyph-run
  foreground payloads.

## Implementation Batches After This Freeze

1. Produce alpha and color raster batches from mounted layouts inside
   `worth-ui-text` only.
2. Exercise the full Unicode 17 RGI emoji corpus without cluster or layer
   loss.
3. Admit separate native atlas lifecycles with pinning and staged upload.
4. Bind paint-span identity through headless transcripts and native pixels.
5. Close remaining `HP-03` atlas, pixel, cost, and reconstruction rows.

## Query Authority Cutover Status

The bounded vertical slice is in place; the full host cutover is not.
Phase 5 readiness therefore stays OPEN.

Completed in this slice:

- One schema-derived application schema, `WorthUiApplicationSchema`.
- Typed production constructors on `UiProjectionFieldRequirement`.
- Compile-fail twins for cross-schema, cross-aspect, and wrong-value-type.
- Declaration macros from `worth-query-decl`.
- Measurement and size are distinct typed aspects and fields even though the
  legacy native adapter spells both keys `value`.
- Typed field authority is carried to the low native request adapter; native
  key text is no longer the equality or duplicate-selection identity.
- The raw-workspace lookup helper is crate-private. `WorthUiQueryWorkspaceExt`
  remains certification and test only, and neither runtime nor product facades
  export a wrapper that implies an audience-facade cutover.
- The product `worth-ui` facade does not export the workspace extension.

Blocking Query debt that keeps readiness OPEN:

- The low Query adapter still translates typed WORTH UI fields into legacy
  native display-key strings because Query's installed request API requires it.
- Product view declaration still authors aspect and field names as strings.
- Intent authority still declares `input_string_field("source_revision")`
  and `input_string_field("status")`.
- `UiProjectionFieldRequirement::declared` remains for invalid-input and
  fixture claims only.
- Production still depends on `worth-query` runtime types, including
  `WorthQueryWorkspace`. Query's current host-audience facade exposes the new
  decomposed runtime rather than this legacy workspace owner; a future cutover
  must migrate the owner itself and may not add another compatibility wrapper.
