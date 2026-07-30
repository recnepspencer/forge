# Worth UI Workspace

This workspace is the dedicated home for the Worth UI product facade,
DSL-to-runtime application lifecycle, Query binding edge, host contract,
adapters, and certification surfaces.

## Canonical Reading Order

1. [AI discovery](./AI_README.md)
2. [Architecture](./docs/architecture.md)
3. [Authored composition](./docs/authored-composition.md)
4. [Application lifecycle](./docs/application-lifecycle.md)
5. [Application inspection](./docs/inspection.md)
6. [Runtime subsystems](./docs/runtime-subsystems.md)
7. [Query-backed UI views](./docs/query-binding.md)
8. [Milestone 3.10.1 migration](./docs/migration-3.10.1.md)

The longer contributor orientation is
[docs/worth-ui-readme.md](./docs/worth-ui-readme.md). Roadmap and milestone
specifications remain under `_docs/worth-ui` at the repository root.

## Dependency Direction

- `worth-ui-theme` owns design-token truth.
- `worth-ui-components` owns purely presentational widget surfaces.
- `worth-ui` is the public product facade. Query-backed UI work enters through
  `worth_ui::facade::query_binding`; application lifecycle stays under
  `worth_ui::facade::app`. Product code does not import the binding crate
  directly or acquire Query authority through a workspace extension trait.
- `worth-ui-query-binding` is the only production crate in this workspace that
  translates Worth Query authority into Worth UI artifacts.
- `worth-ui-runtime` owns UI admission, graph/allocation behavior, framework
  turns, and mounted runtime truth; it consumes binding-owned UI artifacts and
  does not import Query directly.
- `worth-ui-certification` proves the public lifecycle and structural
  anti-bypass boundaries through production facades.
- The root Worth workspace should not treat these crates as ordinary
  top-level members once this nested workspace owns them.

For projected product data, install Query through the public host contract,
register the returned shape-specific projection on `WorthUi::app()`, and feed
Query-issued observations into the ordinary application rebind path. See
[Query-backed UI views](./docs/query-binding.md) for the compiled registration
grammar and [Application lifecycle](./docs/application-lifecycle.md) for the
permanent Platform Pulse journey.
