# Worth UI Workspace

This workspace is the dedicated home for the Worth UI product facade,
DSL-to-runtime application lifecycle, Query binding edge, host contract,
native and headless mechanics, and certification surfaces.

## Canonical Reading Order

1. [AI discovery](./AI_README.md)
2. [Architecture](./docs/architecture.md)
3. [Authored composition](./docs/authored-composition.md)
4. [Interaction and intents](./docs/interaction-and-intents.md)
5. [Application lifecycle](./docs/application-lifecycle.md)
6. [Native host platform](./docs/native-host-platform.md)
7. [Application inspection](./docs/inspection.md)
8. [Runtime subsystems](./docs/runtime-subsystems.md)
9. [Query-backed UI views](./docs/query-binding.md)
10. [Milestone 3.10.1 migration](./docs/migration-3.10.1.md)

The longer contributor orientation is
[docs/worth-ui-readme.md](./docs/worth-ui-readme.md). Roadmap and milestone
specifications remain under `_docs/worth-ui` at the repository root.

## Dependency Direction

- `worth-ui` is the public product facade. Query-backed UI work enters through
  `worth_ui::facade::query_binding`; application lifecycle stays under
  `worth_ui::facade::app`. Product code does not import the binding crate
  directly or acquire Query authority through a workspace extension trait.
- `worth-ui-query-binding` is the only production crate in this workspace that
  translates Worth Query authority into Worth UI artifacts.
- `worth-ui-runtime` owns UI admission, graph/allocation behavior, framework
  turns, typed intent admission and execution, and mounted runtime truth; it
  consumes binding-owned UI artifacts and does not import Query directly.
- `worth-ui-host-contract` owns the sealed revision-4 initial/delta/unchanged
  presentation protocol. `worth-ui-host-headless` and
  `worth-ui-host-native` consume only that contract; neither imports runtime.
- `worth-ui-native-platform` owns effect-free native application preparation
  and the one-shot platform binding. It is the sole native-display entrypoint.
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

For product actions, follow native input through a presentation-bound semantic
interaction, typed route and payload, UI admission, and the exact application
provider. Product or Query mutation then performs its own admission before the
declared consequence enters ordinary rebind. See
[Interaction and intents](./docs/interaction-and-intents.md); a control,
renderer, adapter, or admitted UI request owns no callback or domain authority.
