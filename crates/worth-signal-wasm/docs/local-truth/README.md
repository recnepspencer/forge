# Local Truth

Use local truth when a browser-only application needs branches, exact
aspect-level merge decisions, and manual resolution without pretending that a
Signal execution branch is the application database.

Retained history is authority-backed rather than a UI cache: `history(...)`
returns the branch's commit graph, and `historicalSnapshot(...)` reads the
sealed values at any retained ancestor without moving a branch head.

- [Branch Merge And Manual Resolution](./branch-merge.md)
- [Standalone And Platform Authority Boundaries](./authority-boundaries.md)
