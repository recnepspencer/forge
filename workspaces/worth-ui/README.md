# Worth UI Workspace

This workspace is the dedicated home for Worth UI and the surviving
presentational Worth UI crates while the desktop platform is rebuilt on
clearer architectural boundaries.

AI/runtime orientation lives at `docs/worth-ui-readme.md`. Read it before planning
runtime architecture, hot-reload boundaries, graph work, or host adapter work.

Dependency direction:

- `worth-ui-theme` owns design-token truth.
- `worth-ui-components` owns purely presentational widget surfaces.
- `worth-ui-adapters`, `worth-ui-state`, and `worth-ui-types` are retired
  migration residue and must not survive as ordinary production authority
  paths.
- `worth-ui` is the platform crate and may depend on lower Worth runtime crates
  such as Query, Foundational, Proof, and Signal through explicit workspace
  dependencies.
- The root Worth workspace should not treat these crates as ordinary
  top-level members once this nested workspace owns them.

Docs remain under `_docs/worth-ui` at the repository root so roadmap and
milestone material stay in one canonical documentation tree.
