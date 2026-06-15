# Milestone 4: Authoring DX Reset and Shopify Dashboard Product Hardening

## Goal

Replace the current low-level Worth UI authoring surface with a first-class
authoring model for `app -> workspace -> page -> layout -> content -> surface ->
component -> appearance`, then prove that model by building a serious
Shopify-style native admin dashboard on top of the existing Worth UI compiler,
runtime, and Forge Query integration.

This milestone is not allowed to rebuild the runtime, rebuild Query, or invent
page-local shell folklore. It must refine the author-facing layer while
preserving the stronger lower substrate that already exists.

## Why This Milestone Exists

Milestones 1 through 3 already established the hard lower architecture:

- typed capability registration and frozen snapshot lookup
- source-to-artifact compiler phases
- structural legality over mosaic topology
- Query-aware binding semantics and runtime dependency hooks
- hot replacement, durable state reconciliation, and execution-plan lowering

What is still weak is the author-facing language that feeds those systems.
Today, the crate already has serious runtime authority and replacement logic,
but authoring is still too close to low-level `component` / `surface` /
`binding` blocks with `region` / `mount` statements.

Milestone 4 exists to fix that mismatch honestly:

- preserve the existing proof chain
- replace the weak authoring surface
- prove the result against a dense real product

## Governing Summaries

`MENTALITY.md`

- The main thing this document protects is foundation-first honesty.
- The spec must solve the real structural mismatch now instead of layering
  convenience syntax over the current low-level authoring and calling that done.

`arch_laws.md`

- The main thing this document protects is proof-carrying structure.
- The spec must add authoring phases that lower into the existing typed
  artifact/runtime chain rather than bypassing it with ad hoc runtime objects.

`composition_laws.md`

- The main thing this document protects is named semantic decomposition.
- The spec must keep layout, content, runtime binding, appearance, actions, and
  evidence as separate responsibilities instead of one giant page DSL.

`domain_structure_laws.md`

- The main thing this document protects is boundary truth in the tree.
- The spec must keep authority, derivation, shell ownership, runtime truth, and
  presentation artifacts structurally separate.

`perf_laws.md`

- The main thing this document protects is bounded execution tied to semantic
  delta.
- The spec must consume the existing snapshot, dependency-hook, invalidation,
  and lane substrate rather than introducing props, broad scans, or UI-local
  dependency graphs.

`dx_laws.md`

- The main thing this document protects is honest layered ergonomics.
- The common path must read like page-authoring intent, while lower runtime and
  plan boundaries remain visible where responsibility genuinely changes.

`worth_ui_roadmap.md`

- The main thing this roadmap protects is milestone ordering around real
  platform capability.
- Milestone 4 belongs here because the next blocker is no longer lower runtime
  machinery; it is authoring and product-hardening over machinery that already
  exists.

## Adversarial Constraint

This milestone must survive the kind of native dashboard that breaks weak UI
systems:

- one persistent workspace shell with rails, top bar, page host, inspector
  dock, overlay layer, toast layer, and status surfaces
- multiple materially different pages such as overview, products, orders, and
  customers
- dynamic page instances such as product detail, order detail, filtered search
  result, and contextual inspector flows opened from runtime data
- dense live data surfaces that update through Forge Query invalidation rather
  than parent hydration or local reconciliation
- mixed `fit`, `fill`, `share`, `clamp`, `ratio`, and user-resizable regions
- shared seams and touching regions that must dedupe borders by default
- hot reload, restore, and theme or density changes without breaking runtime
  truth, identity, or layout continuity

If the dashboard only works by introducing props plumbing, page-local shell
state, broad registry scans, or style/layout/runtime blob objects, the
milestone fails even if the pixels look better.

## Product Decision Lock

The following are locked for this milestone:

- The existing public runtime-preparation chain remains authoritative:
  `WorthUiRuntimeLaunchBuilder::prepare_for` ->
  `WorthUiParsedSourceToArtifactInputLowerer` ->
  `WorthUiArtifactInputResolver` ->
  `WorthUiStructuralLegalityLowerer` ->
  `WorthUiBindingSemanticsLowerer` ->
  `WorthUiIdentitySeedLowerer` ->
  `WorthUiCanonicalArtifactAssembler`.
- `CapabilitySnapshot`, `CapabilitySnapshotIndex`, and
  `CapabilitySupportCatalog` remain the only capability/support authority.
- `WorthUiMosaicStructureFacts` remains the structural target for layout
  topology after lowering.
- `ViewBindingDescriptor`, `WorthUiBoundBindingSemantics`,
  `WorthUiRuntimeDependencyHook`, and `WorthUiQuerySupportReceipt` remain the
  Query-facing runtime substrate that Worth UI must consume rather than shadow.
- `WorthUiRuntimeHost`, durable state families, query-binding comparison, live
  rebind planning, and lane execution remain runtime-owned.
- `WorthUiVirtualizedDataFrameTarget` and the virtualized data lane remain the
  basis for repeated or windowed live data surfaces.
- Dynamic navigation must be typed page instantiation over declared page
  templates, not route strings or app-local scene switching.
- The current top-level source language is too weak, but the current artifact
  and runtime chain is not the problem. New authoring constructs must lower
  into the existing chain instead of forking it.

## Target DX

The target for this milestone must look substantially better than the current
`component ... { region ... mount ... }` authoring. The common path should read
like product structure, not parser substrate.

The end-state target for this milestone is intentionally concrete:

```rust
app ShopifyAdminApp {
    theme ShopifyAdminTheme
    workspace AdminWorkspace
}
```

```rust
workspace AdminWorkspace {
    title "Shopify Admin"

    shell {
        topbar AdminTopbar
        rail AdminPrimaryRail
        page_host AdminPageHost
        inspector AdminInspectorDock
        status AdminStatusBar
        overlays [CommandPaletteOverlay, GlobalSearchOverlay]
        toasts AdminToastCenter
    }

    pages [
        OverviewPage,
        ProductsPage,
        OrdersPage,
        CustomersPage,
    ]

    dynamic_pages [
        ProductDetailPage(product_id: ProductId),
        OrderDetailPage(order_id: OrderId),
    ]
}
```

```rust
page ProductsPage {
    title "Products"
    runtime ProductsRuntime
    layout ProductsLayout
    content ProductsContent
}
```

```rust
page ProductDetailPage(product_id: ProductId) {
    title "Product"
    runtime ProductDetailRuntime(product_id)
    layout ProductDetailLayout
    content ProductDetailContent
}
```

```rust
runtime ProductsRuntime {
    live_views {
        ProductRows: shop.products.table();
        SelectedProduct: shop.products.selected();
        ProductTimeline: shop.products.activity(SelectedProduct);
    }

    computed {
        InventorySummary: shop.products.inventory_summary(ProductRows);
        PublishTarget: shop.products.publish_target(SelectedProduct);
    }

    posture {
        ProductsPagePosture: shop.products.page_posture(ProductRows);
        PublishPosture: shop.products.publish_posture(PublishTarget);
    }

    effects {
        PublishSelectedProduct: shop.products.publish(PublishTarget);
        ArchiveSelectedProduct: shop.products.archive(PublishTarget);
    }
}
```

```rust
layout ProductsLayout {
    column {
        row height fit {
            slot toolbar
        }

        row height fill {
            column width clamp(min: rail.md, preferred: share(2), max: rail.xl)
            scroll_owner {
                slot filters
            }

            column width fill {
                row height clamp(min: panel.lg, preferred: share(5), max: ratio(3, 5))
                scroll_owner {
                    slot collection
                }

                row height share(2) scroll_owner {
                    slot activity
                }
            }

            column width clamp(
                min: inspector.md,
                preferred: share(2),
                max: inspector.xl,
            ) {
                slot inspector
            }
        }

        row height fit {
            slot status
        }
    }
}
```

```rust
content ProductsContent {
    toolbar -> ProductsToolbarSurface
    filters -> ProductFiltersSurface
    collection -> ProductsGridSurface
    activity -> ProductActivitySurface
    inspector -> ProductInspectorSurface
    status -> ProductStatusSurface
}
```

```rust
surface ProductsGridSurface {
    iterate ProductRows as ProductRow
    present ProductCard in card_grid(columns: clamp(min: 2, preferred: 4, max: 6))
}
```

```rust
surface ProductsBulkActionsMenu {
    iterate BulkActionItems as BulkActionItem
    present BulkActionMenuItem in menu
}
```

```rust
surface ProductsGroupedActionsMenu {
    iterate_groups BulkActionGroups as BulkActionGroup {
        section BulkActionGroup.label {
            iterate BulkActionGroup.items as BulkActionItem
            present BulkActionMenuItem in menu
        }
    }
}
```

```rust
component ProductCard {
    uses row ProductRow
    uses posture PublishPosture

    appearance ProductCardAppearance

    column gap space.300 {
        header {
            title row.title
            badge row.status tone status
        }

        media {
            image row.primary_image
        }

        stack {
            price row.price
            inventory row.inventory_count
            sku row.sku
        }

        actions {
            primary "Make live" -> PublishSelectedProduct
            danger "Archive" -> ArchiveSelectedProduct
        }
    }
}
```

```rust
component BulkActionMenuItem {
    uses item BulkActionItem

    menu_item {
        label item.label
        icon item.icon
        action item.action
        posture item.posture
    }
}
```

```rust
appearance ProductCardAppearance {
    chrome card
    background surface.raised
    padding space.400
    gap space.300
    radius radius.300
    seams isolated
    shadow elevation.200

    action_tones {
        primary button.primary
        danger button.danger
    }
}
```

What this target is protecting:

- pages are selected by typed reference, not stringly page IDs
- layout is structural and content is occupancy
- runtime truth is declared by family and consumed directly
- repeated data, repeated UI projections, and grouped repeated projections are
  explicit through `iterate` and `iterate_groups`, not hidden in naming tricks
- components are local anatomy, not parent data pipes
- appearance is separate from layout and runtime
- seams are automatic by default, with posture-aware override only when needed

The milestone should also lock these ergonomics rules:

- identity comes from typed symbols and file/module ownership, not manual string
  page IDs
- pages are declared once and referenced by typed symbol from the workspace
- repeated namespace prefixes like `validation.surface.surface_atlas` are not
  acceptable end-state DX
- single-use page-local structure should stay inline by default
- extraction into named `Runtime`, `Layout`, `Content`, or `Appearance`
  declarations should happen only when reuse, independent inspection, or
  conceptual weight justifies it
- dynamic pages are declared as typed templates and opened as typed instances,
  not synthesized from strings or opaque route payloads

That means the most common path should be even lighter than the extracted
end-state examples above. The default page authoring should be allowed to read
like this:

```rust
page ProductsPage {
    title "Products"

    runtime {
        live_views {
            ProductRows: shop.products.table();
            SelectedProduct: shop.products.selected();
        }

        posture {
            PublishPosture: shop.products.publish_posture(SelectedProduct);
        }

        effects {
            PublishSelectedProduct: shop.products.publish(SelectedProduct);
        }
    }

    layout {
        column {
            row height fit { slot toolbar }
            row height fill { slot collection }
            row height fit { slot status }
        }
    }

    content {
        toolbar -> ProductsToolbarSurface
        collection -> ProductsGridSurface
        status -> ProductStatusSurface
    }
}
```

The extracted form is for weight and reuse. The inline form is the ordinary
path.

## Seam And Style Model

The milestone must not treat seams and style as late decorative paint. They are
part of the product grammar.

The default posture should feel like a real admin app:

- structural shell siblings are flush by default
- touching shell regions merge seams by default
- page content regions own inset padding and internal gap rhythm
- cards, popovers, overlays, and detached inspectors own independent perimeters
- scroll owners paint and size the full region they own rather than shrinking to
  inner content width
- content groups should look intentional before any app-local overrides exist

The target visual grammar is:

```rust
appearance AdminWorkspaceShellAppearance {
    chrome shell
    background surface.canvas
    padding none
    seams merged
}
```

```rust
appearance AdminPageSectionAppearance {
    chrome section
    background surface.subtle
    padding space.400
    gap space.400
    seams merged
}
```

```rust
appearance ProductCardAppearance {
    chrome card
    background surface.raised
    padding space.400
    gap space.300
    radius radius.300
    seams isolated
    shadow elevation.200

    action_tones {
        primary button.primary
        danger button.danger
    }
}
```

The intent behind those defaults:

- `shell` means flush platform structure with shared boundaries and no accidental
  cardification
- `section` means content grouping inside a page with its own inset rhythm but
  still structurally part of the page plane
- `card` means detached visual ownership with radius, independent perimeter,
  and room around it

Seam override vocabulary should stay tiny and obvious:

```rust
seams merged
seams isolated
seams hidden
seams emphasized
```

That is enough to express the important cases:

- merged shell borders
- detached cards and overlays
- intentionally invisible joins
- emphasized split boundaries such as inspector and resize rails

The milestone should also protect the web-layout lessons that are still worth
keeping:

- scroll containers fill their owned region
- resize handles keep the region they divide visually honest
- center content gets breathing room from rails and inspectors
- stacked sections have explicit gap or seam posture instead of accidental
  border collisions
- background color grows with the owned region, not only with intrinsic child
  size

## Phase Plan

### Phase 1: Authoring Hierarchy Entry Layer

This phase adds the first-class authoring hierarchy above the current
`component` / `surface` / `binding` / `token` block model while preserving the
existing proof-bearing lower pipeline.

**Relevant subsystems**
- source parsing and parsed declaration topology
- source-to-artifact-input lowering
- canonical runtime launch preparation

**Relevant APIs**
- `WorthUiSourceParser`
- `WorthUiParsedSourceToArtifactInputLowerer`
- `WorthUiArtifactInput`
- `WorthUiResolvedArtifactInput`
- `WorthUiRuntimeLaunchBuilder::prepare_for`

**DX target**

By the end of this phase, the common entry path should read like this:

```rust
app ShopifyAdminApp {
    theme ShopifyAdminTheme
    workspace AdminWorkspace
}

workspace AdminWorkspace {
    pages [OverviewPage, ProductsPage, OrdersPage, CustomersPage]
}

page ProductsPage {
    runtime ProductsRuntime
    layout ProductsLayout
    content ProductsContent
}
```

And the lighter ordinary path should also be valid when those sections are only
used once:

```rust
page ProductsPage {
    runtime { ... }
    layout { ... }
    content { ... }
}
```

Not like this:

```rust
component validation.surface.surface_atlas {
    region validation.region.main {
        mount validation.surface.inspector
            placement validation.placement.sidebar;
    }
}
```

**Build shape**
- Add new top-level authoring constructs for `app`, `workspace`, `page`,
  `layout`, `content`, `appearance`, and typed runtime declarations.
- Keep these constructs as author-facing declarations only; they must lower into
  the current artifact-input model instead of introducing a second runtime
  artifact family.
- Introduce typed references so pages, layouts, content blocks, and appearances
  are referenced symbolically after declaration rather than by ad hoc strings in
  later phases.
- Allow single-use runtime/layout/content sections to remain inline so the
  common path does not force extraction ceremony.
- Preserve the current lower proof chain unchanged after authoring has been
  translated into artifact input.

**Warnings**
- Do not bypass the current `artifact_input -> resolved -> structured -> bound
  -> identity_seeded -> artifact` chain.
- Do not collapse the hierarchy into one generic `screen` or `view` object.
- Do not let `workspace` or `page` become new authority objects that compete
  with the canonical artifact.

**Test requirements**
- Equivalent old-form and new-form authoring that describe the same structure
  must lower to equivalent artifact input and equivalent canonical artifact.
- Mis-nested authoring such as `page` outside `workspace` must be rejected at
  the new authoring boundary rather than leaking downstream as generic parse
  failure.

**Engineering decisions**
- New authoring constructs are syntactic and semantic sugar over the existing
  proof chain, not a replacement for it.
- File-derived identity is preferred where it stays stable and removes ceremony.
- Extraction is by semantic weight or reuse, not by default.

**Open questions**
- Whether `content` should remain a distinct declaration or become a typed
  section inside `page`, provided the layout/content boundary stays explicit.

### Phase 2: Workspace-Owned Shell and Typed Page Navigation

This phase builds the persistent workspace shell as a real platform concept so
pages vary inside a durable host rather than each page re-implementing shell
chrome.

**Relevant subsystems**
- canonical artifact authoring and identity
- durable state reconciliation
- plan swap and restore continuity

**Relevant APIs**
- `WorthUiRuntimeHost`
- durable state families including panel, tab, and splitter state
- shell-facing surfaces already proven in Milestone 3 and the validation app

**DX target**

By the end of this phase, shell ownership should read like this:

```rust
workspace AdminWorkspace {
    shell {
        topbar AdminTopbar
        rail AdminPrimaryRail
        page_host AdminPageHost
        inspector AdminInspectorDock
        status AdminStatusBar
        overlays [CommandPaletteOverlay]
        toasts AdminToastCenter
    }

    pages [OverviewPage, ProductsPage, OrdersPage, CustomersPage]

    dynamic_pages [
        ProductDetailPage(product_id: ProductId),
        OrderDetailPage(order_id: OrderId),
    ]
}
```

The author should not need to restate shell chrome inside each page.

Workspace composition should also be light enough that adding a new page feels
like extending a product, not wiring a runtime:

```rust
workspace AdminWorkspace {
    pages [OverviewPage, ProductsPage, OrdersPage, CustomersPage]
}
```

The workspace chooses pages by typed symbol, not by page IDs or route strings.
Dynamic work should feel equally native:

```rust
component ProductCard {
    actions {
        primary "Open product" -> navigate ProductDetailPage(row.id)
    }
}
```

And multiple typed page instances should be allowed to coexist inside the page
host without inventing a second navigation model.

**Build shape**
- Add workspace-level shell authoring for top bar, rail, page host, inspector
  dock, status region, toast layer, and overlay layer.
- Make page navigation choose typed page references within workspace scope.
- Add typed dynamic page templates and typed page-instance navigation so
  runtime-driven product detail and contextual pages can open inside the same
  workspace shell.
- Ensure page switches preserve the active workspace shell and only replace the
  page-hosted content subtree.
- Ensure multiple dynamic page instances can coexist when their typed page
  instance identity differs.
- Ensure restore and reload treat shell continuity as workspace-owned platform
  state rather than page-local widget memory.

**Warnings**
- Do not let pages smuggle permanent shell regions into page-local layout.
- Do not make toasts or overlays page-owned when their scope is workspace-wide.
- Do not fall back to route strings, raw path fragments, or app-local scene
  registries for dynamic page opening.

**Test requirements**
- Switching between pages with different page layouts must preserve workspace
  shell state and stable shell identity.
- Restart and restore with a non-default last-active page must converge to the
  same shell geometry, page selection, and eligible durable state.
- Opening multiple dynamic page instances such as different product details
  must preserve typed instance identity, tab/page-host continuity, and restore
  determinism.
- Invalid dynamic page instantiation must fail at the typed navigation boundary
  rather than becoming a dead route or blank shell.

**Engineering decisions**
- Reuse the existing reload, restore, and durable-state substrate rather than
  inventing a second workspace continuity model.
- Keep page-host selection distinct from page-content resolution.
- Static pages and dynamic page instances share one navigation model; dynamic
  pages are page templates plus typed context, not a separate router.

**Open questions**
- Whether the first version should expose dynamic page instances only through
  explicit `navigate PageTemplate(args...)` actions or also through runtime-
  populated navigation lists in workspace chrome.

### Phase 3: Mosaic DX That Lowers to Existing Structural Facts

This phase makes mosaics read like the Worth-native replacement for flex/grid
while still lowering into the existing structural legality and mosaic-facts
pipeline.

**Relevant subsystems**
- structural body lowering
- mosaic structure facts
- sizing, placement, and state-slot legality

**Relevant APIs**
- `WorthUiMosaicStructureFacts`
- `WorthUiStructuralLegalityLowerer`
- `MosaicSizingKind`
- `MosaicSizingContractDescriptor`
- `MosaicScrollOwnership`
- `MosaicResizePermission`
- `MosaicSizingPersistence`

**DX target**

By the end of this phase, dashboard layout should read like a Worth-native
structural language:

```rust
layout ProductsLayout {
    column {
        row height fit {
            slot toolbar
        }

        row height fill {
            column width clamp(min: rail.md, preferred: share(2), max: rail.xl)
            scroll_owner {
                slot filters
            }

            column width fill {
                row height ratio(3, 5) scroll_owner {
                    slot collection
                }

                row height share(2) scroll_owner {
                    slot activity
                }
            }

            column width clamp(
                min: inspector.md,
                preferred: share(2),
                max: inspector.xl,
            ) {
                slot inspector
            }
        }
    }
}
```

The author should not have to think in flexbox percentages or overflow hacks.

**Build shape**
- Add author-facing sizing vocabulary for `fit`, `fill`, `share`, `clamp`,
  `fixed`, `ratio`, and typed min/max bounds.
- Add explicit region-level scroll ownership and resizable-region declaration.
- Lower the friendly layout vocabulary into region kinds, sizing contracts,
  placement policies, and state slots that the current structural legality layer
  already understands.
- Extend the substrate only where an honest mapping does not already exist.

**Warnings**
- Do not expose web-era `flex` or `grid` metaphors as the main model.
- Do not hide expensive measurement or broad reflow behind cheap-looking
  vocabulary.

**Test requirements**
- Nested combinations of `fit`, `fill`, `share`, `clamp`, and `ratio` must
  lower deterministically and preserve legality under module reorder.
- User-resizable regions must preserve eligible persisted size state across
  reload and restart without snap-back or hidden geometry fallback.

**Engineering decisions**
- Friendly sizing syntax should compile into current sizing contracts whenever
  that mapping is semantically honest.
- If `share` cannot be expressed honestly with current substrate, extend the
  substrate instead of faking it with hidden derived weights.

**Open questions**
- None.

### Phase 4: Separate Layout Topology from Content Slotting

This phase adds an explicit content-slot layer so page structure and what gets
mounted into that structure are different authoring responsibilities.

**Relevant subsystems**
- page/layout/content authoring
- structural lowering to regions and mounts
- canonical artifact assembly

**Relevant APIs**
- `WorthUiParsedSourceToArtifactInputLowerer`
- `WorthUiMosaicRegionFacts`
- `WorthUiMosaicMountFacts`
- `WorthUiCanonicalArtifactAssembler`

**DX target**

By the end of this phase, structure and occupancy should be distinct:

```rust
layout ProductsLayout {
    column {
        row height fit { slot toolbar }
        row height fill { slot collection }
        row height fit { slot status }
    }
}

content ProductsContent {
    toolbar -> ProductsToolbarSurface
    collection -> ProductsGridSurface
    status -> ProductStatusSurface
}
```

The author should be able to read layout without knowing what is mounted, and
read content without re-learning geometry.

When a page is simple enough, the same distinction should still be expressible
inline without forcing extraction:

```rust
page OverviewPage {
    layout {
        column {
            row height fit { slot hero }
            row height fill { slot overview }
        }
    }

    content {
        hero -> OverviewHeroSurface
        overview -> OverviewMetricsSurface
    }
}
```

**Build shape**
- Add typed region-slot declarations in layouts.
- Add typed content declarations that fill slots with surfaces or surface groups.
- Lower layout and content together into the same region/mount facts currently
  consumed by structural legality.
- Preserve deterministic mount ordering and stable identity even when layout and
  content are authored in separate declarations.

**Warnings**
- Do not merge slot filling back into raw region statements once this phase
  lands.
- Do not let content declarations own sizing, scroll rules, or shell topology.

**Test requirements**
- Reordering content-slot declarations without changing meaning must preserve
  equivalent canonical structure and identity.
- Assigning a surface to an unknown slot or an illegal slot class must fail at
  this phase with localized diagnostics.

**Engineering decisions**
- Layout owns structure; content owns occupancy; later appearance owns chrome.
- Slotting should reduce ceremony without weakening deterministic structure.

**Open questions**
- None.

### Phase 5: Typed Runtime Declaration Families Over Forge Query

This phase resets runtime authoring so pages and components declare retained
runtime surfaces by family instead of hiding everything in one flat binding
bucket.

**Relevant subsystems**
- view binding descriptors
- bound binding semantics
- runtime dependency hook derivation
- query support admission

**Relevant APIs**
- `ViewBindingDescriptor`
- `WorthUiBoundBindingSemantics`
- `WorthUiRuntimeDependencyHook`
- `WorthUiQuerySupportReceipt`
- `WorthUiArtifactInputResolver`

**DX target**

By the end of this phase, runtime declaration should read like this:

```rust
runtime ProductsRuntime {
    live_views {
        ProductRows: shop.products.table();
        SelectedProduct: shop.products.selected();
    }

    computed {
        PublishTarget: shop.products.publish_target(SelectedProduct);
    }

    posture {
        PublishPosture: shop.products.publish_posture(PublishTarget);
    }

    effects {
        PublishSelectedProduct: shop.products.publish(PublishTarget);
    }
}
```

Not like one undifferentiated bucket where live views, posture, and effects all
look the same.

For the common path, local runtime declaration should also be allowed inline:

```rust
page ProductsPage {
    runtime {
        live_views {
            ProductRows: shop.products.table();
        }

        effects {
            PublishSelectedProduct: shop.products.publish(selected_product());
        }
    }
}
```

**Build shape**
- Add typed runtime declaration families such as `live_views`, `computed`,
  `effects`, `posture`, and other proved families as needed.
- Add explicit visibility/import rules so pages, surfaces, and components can
  only consume runtime artifacts that are intentionally exposed to them.
- Lower these families into the current view-binding and runtime-hook substrate
  instead of inventing page-local hydration or local result-state models.
- Preserve Query-owned support, admission, live-compatibility, async/result,
  denial, inspection, and explanation posture where the current substrate
  already carries it.

**Warnings**
- Do not create a UI-local dependency graph where Forge Query already owns one.
- Do not collapse live views, computed surfaces, and action posture into one
  untyped runtime bucket.

**Test requirements**
- Illegal visibility, illegal family mixing, and unresolved runtime references
  must fail at lowering time with typed diagnostics.
- Live runtime invalidation must update consumers through retained runtime
  surfaces without parent re-hydration or props threading.

**Engineering decisions**
- Existing runtime dependency hook derivation remains the correct lower seam.
- Query support receipts remain authoritative evidence rather than mirrored
  local enums.

**Open questions**
- Whether workspace-scoped runtime artifacts may be re-exported under page-local
  aliases or must always be referenced directly.

### Phase 6: Iteration Bindings and Repeated Runtime Projections

This phase makes repeated runtime-backed UI explicit so card grids, tables,
timelines, menus, palettes, toolbars, and other repeated projections bind
honestly to runtime truth and virtualization where needed.

**Relevant subsystems**
- repeated runtime bindings
- virtualized data lane
- stable identity and query invalidation
- repeated action and projection surfaces

**Relevant APIs**
- `WorthUiVirtualizedDataFrameTarget`
- query binding comparison and rebind planning
- view-binding handles and visible-range execution

**DX target**

By the end of this phase, repeated rendering should read like this:

```rust
surface ProductsGridSurface {
    iterate ProductRows as ProductRow
    present ProductCard in card_grid(columns: share(4))
}
```

And repeated runtime-driven projections should read like this:

```rust
surface ProductsBulkActionsMenu {
    iterate BulkActionItems as BulkActionItem
    present BulkActionMenuItem in menu
}
```

Grouped repeated projections should also be first-class:

```rust
surface ProductsGroupedActionsMenu {
    iterate_groups BulkActionGroups as BulkActionGroup {
        section BulkActionGroup.label {
            iterate BulkActionGroup.items as BulkActionItem
            present BulkActionMenuItem in menu
        }
    }
}
```

And component-local repeated access should read like this:

```rust
component ProductCard {
    uses row ProductRow
}
```

```rust
component BulkActionMenuItem {
    uses item BulkActionItem
}
```

Not like `.row`, hidden suffix conventions, or parent-plumbed row props.

The author should be able to understand the repeated data or repeated
projection contract by reading `iterate ... as ...` once, not by
reverse-engineering naming convention.

**Build shape**
- Add explicit iteration artifacts for repeated runtime projections, including
  content iteration and projection iteration.
- Content iteration covers repeated cards, rows, grids, lists, timelines, and
  similar view content.
- Projection iteration covers repeated menu items, palette entries, toolbar
  items, inspector sections, and other runtime-populated command or projection
  surfaces.
- Add a first-class grouped iteration artifact for grouped repeated
  projections such as menu sections, grouped action clusters, inspector
  sections, and similar two-level repeated structures.
- Let surfaces and components consume row-scoped or item-scoped runtime
  bindings directly without parent props or naming conventions.
- Tie repeated live surfaces to existing virtualized data lane mechanics where
  collection size or visibility windows require it.
- Preserve stable item identity across filtering, sorting, selection,
  invalidation, regrouping, and window movement.
- Preserve stable group identity, canonical group ordering, and canonical item
  ordering within each group.
- Carry collection-level posture so empty, loading, deferred, denied, and
  partial states can be expressed at the iteration boundary rather than faked by
  local wrapper logic.
- Carry group-level posture and optional section chrome metadata so grouped
  iteration does not devolve into local wrapper folklore.
- Preserve item-scoped action access so repeated menu items and repeated cards
  can project direct runtime-backed actions without closure glue.

**Warnings**
- Do not hide repeated rendering behind string conventions like `.row`.
- Do not force off-screen rows or items to materialize just to preserve a
  friendly API.
- Do not treat repeated menus or command groups as a separate ad hoc runtime
  path from repeated content.
- Do not jump from grouped iteration to arbitrary recursive tree iteration in
  this milestone.

**Test requirements**
- Row identity must remain stable across reorder, filter, and partial
  invalidation where the underlying runtime identity is still the same.
- Virtualized views must prove off-screen rows do not trigger full collection
  materialization or broad scan frame targets.
- Runtime-populated menu or action-item iteration must preserve stable item
  identity and item-scoped action meaning across invalidation and regrouping.
- Collection-level empty, denied, deferred, and partial posture must remain
  explicit and typed instead of collapsing into local placeholder branches.
- Grouped iteration must preserve stable group identity, stable item identity
  within group, and deterministic group/item ordering across invalidation and
  regrouping.
- Group-level empty, denied, deferred, and partial posture must remain typed
  and must not require app-local wrapper surfaces.

**Engineering decisions**
- Repeated data should ride on existing lane specialization rather than invent a
  second collection runtime.
- Iteration artifacts must carry enough proof that row/item lookup remains
  direct and that repeated projection surfaces do not become hidden broad scans.
- Grouped iteration is first-class in this milestone, but it remains narrow:
  two-level grouped projection/content support is in scope; arbitrary recursive
  tree iteration is not.

**Open questions**
- Whether grouped iteration should allow optional section-level appearance
  recipes in the first version or defer that to a follow-on styling refinement.

### Phase 7: Components as Local Visual Anatomy, Not Data Plumbing

This phase defines components as local visual structure and action affordance
boundaries while keeping runtime truth in Forge Query and binding semantics in
Worth UI.

**Relevant subsystems**
- component authoring
- appearance attachment
- action and posture consumption

**Relevant APIs**
- `ComponentDescriptor`
- `ComponentPropSchema`
- command projection surfaces
- bound binding semantics produced by earlier phases

**DX target**

By the end of this phase, a component should read like local anatomy:

```rust
component ProductCard {
    uses row ProductRow
    uses posture PublishPosture

    column gap space.300 {
        header {
            title row.title
            badge row.status tone status
        }

        media {
            image row.primary_image
        }

        stack {
            price row.price
            inventory row.inventory_count
        }

        actions {
            primary "Make live" -> PublishSelectedProduct
            danger "Archive" -> ArchiveSelectedProduct
        }
    }
}
```

The component should not need parent props just to reach its runtime truth.

**Build shape**
- Add readable component authoring for local anatomy such as headers, media,
  summaries, pricing rows, pills, action groups, and inline editor regions.
- Require components to declare the runtime, row, posture, and action artifacts
  they consume explicitly.
- Keep components local to visual anatomy; they must not become page or shell
  layout owners.
- Keep data access direct through retained runtime artifacts rather than
  parent-to-child props threading.

**Warnings**
- Do not recreate React-style props as the primary composition model.
- Do not allow hidden runtime reads that are not declared at the component
  boundary.

**Test requirements**
- Components that reference undeclared runtime or row bindings must fail at the
  authoring/lowering boundary rather than at render time.
- Equivalent components with different cosmetic recipes must preserve the same
  runtime dependencies, binding meaning, and action identity.

**Engineering decisions**
- Existing prop-schema metadata is not a license to build a props-driven runtime
  model on top of the crate.
- Components optimize for local readability and explicit resource use.

**Open questions**
- Whether tiny purely decorative subcomponents should remain inline-only or earn
  separate declaration forms later.

### Phase 8: Appearance, Theme, and Seam Arbitration

This phase separates structure from chrome and solves shared boundaries at the
platform level instead of making every touching region paint its own borders.

**Relevant subsystems**
- theme token surfaces
- component and surface appearance
- mosaic adjacency and region boundaries

**Relevant APIs**
- `ThemeTokenDescriptor`
- surface and component descriptors
- mosaic region topology derived by structural lowering

**DX target**

By the end of this phase, appearance should read like this:

```rust
appearance AdminWorkspaceShellAppearance {
    chrome shell
    background surface.canvas
    padding none
    seams merged
}

appearance AdminPageSectionAppearance {
    chrome section
    background surface.subtle
    padding space.400
    gap space.400
    seams merged
}

appearance ProductCardAppearance {
    chrome card
    background surface.raised
    padding space.400
    gap space.300
    radius radius.300
    seams isolated
    shadow elevation.200

    action_tones {
        primary button.primary
        danger button.danger
    }
}
```

And shell/page composition should be able to read like this:

```rust
surface ProductsGridSurface {
    appearance AdminPageSectionAppearance
    iterate ProductRows as ProductRow
    present ProductCard in card_grid(
        columns: clamp(min: 2, preferred: 4, max: 6),
        gap: space.400,
    )
}
```

Region adjacency should not require manually authoring both touching borders
just to avoid double seams, and cards should not visually slop together just
because they live inside one scrolling region.

**Build shape**
- Add semantic appearance recipes for surfaces and components that own radius,
  padding, elevation, seam posture, tones, and typography usage without owning
  structure.
- Add default seam arbitration derived from mosaic adjacency and boundary
  posture so touching regions dedupe seams automatically.
- Keep detached overlays, cards, popovers, and elevated elements capable of
  retaining independent perimeters.
- Keep theme tokens authoritative for color, seam weight, spacing, radius,
  elevation, and density inputs.
- Add explicit default visual postures for shell, section, and card-like chrome
  so authors do not have to manually rediscover basic admin-app composition on
  every page.
- Ensure scroll-owner backgrounds and resize-adjacent surfaces paint the full
  owned region instead of shrinking to intrinsic child size.

**Warnings**
- Do not solve seam merging with post hoc pixel overlap hacks.
- Do not let appearance blocks smuggle layout or runtime posture decisions.
- Do not make authors hand-author margins between every grid card, section, and
  rail just to get reasonable breathing room.

**Test requirements**
- Touching horizontal, vertical, nested, and T-junction regions must dedupe or
  preserve seams according to explicit boundary posture rules.
- Theme or density swaps must preserve structural topology, slot assignment, and
  runtime truth while changing appearance output deterministically.
- Scroll-owner backgrounds must expand with owned width/height under resize
  instead of only painting behind intrinsic child content.
- Adjacent section regions, detached cards, and emphasized split boundaries
  must each render the correct seam ownership without duplicated borders.

**Engineering decisions**
- Seam topology derives from layout adjacency; theme only influences seam
  styling.
- Appearance lowers into inspectable semantic recipes rather than opaque paint
  callbacks.
- Default shell, section, and card posture is part of the platform contract,
  not app-local styling folklore.

**Open questions**
- Whether splitters should own distinct seam posture or inherit the seam of the
  regions they separate.

### Phase 9: Action Projection, Toasts, Overlays, and Inspector Flows

This phase unifies product actions so buttons, menus, palette entries,
inspectors, overlays, and toasts all project one retained action story.

**Relevant subsystems**
- command projection
- action posture and denial presentation
- workspace overlays and toast center

**Relevant APIs**
- command projection descriptors and registries
- bound query and posture semantics
- runtime diagnostics and outcome surfaces already exposed through the facade

**DX target**

By the end of this phase, action usage should read like one shared action story:

```rust
component ProductCard {
    actions {
        primary "Make live" -> PublishSelectedProduct
        danger "Archive" -> ArchiveSelectedProduct
    }
}

surface ProductsBulkActionsMenu {
    iterate BulkActionItems as BulkActionItem
    present BulkActionMenuItem in menu
}

overlay CommandPaletteOverlay {
    project [PublishSelectedProduct, ArchiveSelectedProduct]
}

toast AdminToastCenter {
    present action_outcomes
}
```

The same action meaning should be reusable across button, inspector, palette,
overlay, and toast surfaces without redefining the action per surface.
Repeated action projections should also be able to come from runtime data rather
than only from static lists.

**Build shape**
- Add authoring for product actions that resolve to typed retained runtime
  targets.
- Allow the same action meaning to project into command palette, toolbar,
  card-button, context-menu, and inspector surfaces.
- Allow runtime-populated action collections to project through explicit
  iteration artifacts into menus, palettes, and grouped action surfaces.
- Add workspace-owned overlay and toast recipes that bind to typed action
  outcomes and posture.
- Preserve structured denial, advisory, async, recovery, and inspection posture
  rather than flattening actions into enabled/disabled booleans.

**Warnings**
- Do not create a parallel UI-local command runtime.
- Do not reduce action posture to anonymous booleans where existing runtime
  meaning is richer.

**Test requirements**
- One action identity must remain stable across multiple projections such as
  toolbar, palette, and inspector invocation paths.
- Failed, deferred, and denied actions must preserve typed posture and recovery
  semantics through overlays and toasts.
- Runtime-populated action menus must preserve item identity and action meaning
  under invalidation, regrouping, and posture change.

**Engineering decisions**
- Command identity remains platform-owned and must not be duplicated per surface.
- Feedback rides on retained runtime lanes wherever that substrate already
  exists.

**Open questions**
- Whether pages may define local toast recipes that still emit through the
  workspace toast center.

### Phase 10: Shopify Dashboard Proof Workspace

This phase builds the real product proof: a native Shopify-style admin
workspace that uses the new authoring model under varied page shapes.

**Relevant subsystems**
- workspace, page, layout, content, runtime, component, and appearance surfaces
- shell continuity and navigation
- repeated live data and action projection

**Relevant APIs**
- all authoring and lowering surfaces introduced by Phases 1 through 9
- existing validation app runtime launch and evidence host surfaces

**Build shape**
- Build one workspace with at least overview, products, orders, and customers
  pages.
- Include at least one dynamic detail-page family such as `ProductDetailPage`
  or `OrderDetailPage` that opens typed page instances from runtime data.
- Ensure those pages are materially different in geometry and interaction:
  summary-card overview, dense products collection with inspector editing,
  status-heavy orders flow, and an alternate customers balance.
- Ensure the dynamic detail flow can open multiple instances, restore them, and
  keep them distinct inside the shared page host.
- Use the same workspace shell, runtime substrate, action projection, seam
  rules, and appearance system across the page set.
- Keep the app native and Worth-owned; no browser-shaped fallback artifacts are
  allowed.

**Warnings**
- Do not fake page diversity by swapping labels on one generic template.
- Do not patch page-specific layout pain with app-local geometry state.

**Test requirements**
- Switching among materially different pages must preserve workspace continuity,
  shell state, and typed navigation identity.
- Each proof page must exercise a different combination of layout, runtime,
  iteration, action, or inspector behavior so that page diversity actually hardens
  the platform.
- Dynamic detail-page instances opened from repeated runtime data must preserve
  typed page-instance identity across open, close, reopen, reload, and restore.

**Engineering decisions**
- One serious product family is more valuable than a wide set of toy pages.
- The proof app should stay honest about gaps instead of compensating with local
  infrastructure.

**Open questions**
- None.

### Phase 11: Reload, Restore, and Evidence Hardening

This phase closes the milestone by proving the new authoring model remains hot
reload-safe, restore-safe, and performance-honest under real dashboard use.

**Relevant subsystems**
- runtime host replacement and admission
- durable state reconciliation
- query-binding comparison and live rebind planning
- diagnostics and counter surfaces

**Relevant APIs**
- `WorthUiRuntimeHost`
- query-binding comparison and posture drift surfaces
- durable state families
- reload, plan, and diagnostics evidence already exposed by the validation app

**Build shape**
- Add evidence views that surface active artifact identity, plan identity,
  replace/no-op/deny outcomes, durable-state carry-forward, and relevant query
  support posture.
- Certify valid reload, invalid reload, equivalent reload, and restore behavior
  against the real dashboard workspace.
- Prove theme/density swaps do not alter runtime truth, binding posture, or
  seam topology.
- Add counter-backed proof that the dashboard is still consuming direct lookup,
  retained runtime, and bounded invalidation rather than hidden broad scans.

**Warnings**
- Visual survival alone is not enough; receipts and counters must prove the
  architectural claim.
- Evidence surfaces must not become a second privileged control path.

**Test requirements**
- Invalid reloads must preserve the previously active artifact and expose typed
  diagnostics without blanking the shell.
- Dashboard interactions and reloads must prove no broad registry lookup,
  broad artifact scan, or pseudo-hydration path has entered the hot path.

**Engineering decisions**
- Reuse the existing runtime evidence and validation-app surfaces where they
  remain honest.
- Preserve current runtime authority boundaries; Milestone 4 only adds better
  authoring and better product proof over them.

**Open questions**
- Whether screenshot-golden capture belongs in this milestone or the next one.

## Must Ship

- first-class authoring for `app`, `workspace`, `page`, `layout`, `content`,
  typed runtime families, `component`, and `appearance`
- workspace-owned shell and typed page navigation
- mosaic-native sizing and scroll semantics that lower into existing structural
  facts
- typed runtime declaration families that consume existing Forge Query-facing
  binding surfaces
- explicit iteration bindings for repeated live collections
- appearance and seam arbitration that separate chrome from structure
- one real Shopify-style native admin workspace with materially different pages
- reload, restore, diagnostics, and counter evidence proving the dashboard uses
  the platform honestly

## Must Preserve

- the existing proof chain from source through canonical artifact
- frozen snapshot and support-catalog authority boundaries
- runtime ownership of active artifact, active plan, durable state, and Query
  posture drift
- virtualized-data lane ownership of repeated live collection execution
- no browser, DOM, CSS, React, or web-view implementation escapes
- no UI-local dependency graph, props runtime, hydration graph, or shell shadow
  runtime

## Acceptance Evidence

- a reader can locate the new authoring hierarchy in code and see where it
  lowers into existing artifact and runtime phases
- equivalent old-form and new-form authoring prove convergence on the same
  canonical artifact meaning
- the Shopify workspace runs with overview, products, orders, and customers
  pages inside one persistent shell
- repeated live data proves stable iteration identity and bounded visibility
  execution
- seam arbitration dedupes touching boundaries by default and diverges only when
  posture says it should
- reload, restore, and theme/density changes preserve runtime truth and shell
  continuity
- receipts, counters, and diagnostics prove the product surface is consuming the
  existing substrate rather than bypassing it

## Sequencing Notes

- Phase 1 must land before any further product polish. Otherwise the milestone
  will fossilize the current weak authoring model behind nicer screenshots.
- Reuse existing lower substrate aggressively when it already expresses the
  needed proof. Add substrate only where the current lower form cannot honestly
  represent the desired authoring or runtime meaning.
- Query-facing runtime behavior must keep following `forge-query` strength
  rather than reinventing local support, result-state, recovery, or explanation
  models.
- The milestone earns its place only if it improves authoring while preserving
  the existing runtime/compiler integrity already built in Milestones 1 through
  3.
