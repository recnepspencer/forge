# Worth UI Workspace

This workspace is the dedicated home for Worth UI: the public UI facade,
runtime-owned UI composition, Query binding edge, host contract, adapters, and
certification surfaces.

AI/runtime orientation lives at `docs/worth-ui-readme.md`. The application-facing
Query workflow lives at `docs/query-binding.md`.

Dependency direction:

- `worth-ui-theme` owns design-token truth.
- `worth-ui-components` owns purely presentational widget surfaces.
- `worth-ui-adapters`, `worth-ui-state`, and `worth-ui-types` are retired
  migration residue and must not survive as ordinary production authority
  paths.
- `worth-ui` is the public product facade. Query-backed UI work enters through
  its `facade::query_binding` namespace.
- `worth-ui-query-binding` is the only production crate in this workspace that
  translates Worth Query authority into Worth UI artifacts.
- `worth-ui-runtime` owns UI admission, graph/allocation behavior, framework
  turns, and mounted runtime truth; it consumes binding-owned UI artifacts and
  does not import Query directly.
- The root Worth workspace should not treat these crates as ordinary
  top-level members once this nested workspace owns them.

Docs remain under `_docs/worth-ui` at the repository root so roadmap and
milestone material stay in one canonical documentation tree.
