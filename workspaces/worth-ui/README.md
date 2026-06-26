# Worth UI Workspace

This workspace is the dedicated home for Worth UI and the legacy Forge UI
stack while the desktop platform is rebuilt on clearer architectural
boundaries.

AI/runtime orientation lives at `docs/WORTH_UI_README.md`. Read it before planning
runtime architecture, hot-reload boundaries, graph work, or host adapter work.

Dependency direction:

- `forge-ui-types` is the bottom UI vocabulary crate.
- `forge-ui-theme`, `forge-ui-components`, `forge-ui-adapters`, and
  `forge-ui-state` remain local compatibility/support crates until Worth UI
  absorbs or replaces them explicitly.
- `worth-ui` is the platform crate and may depend on lower Forge runtime crates
  such as Query, Foundational, Proof, and Signal through explicit workspace
  dependencies.
- The root `forge` workspace should not treat these crates as ordinary
  top-level members once this nested workspace owns them.

Docs remain under `_docs/worth-ui` at the repository root so roadmap and
milestone material stay in one canonical documentation tree.
