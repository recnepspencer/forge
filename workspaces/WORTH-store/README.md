# WORTH Store Rebuild Workspace

This workspace is the clean rebuild of WORTH Store. It separates the semantic
durability roadmap from the physical database foundation so the new store does
not inherit the heap-shaped implementation boundary from the legacy crate.

Dependency direction:

- `worth-store-contracts` is the bottom shared vocabulary.
- Roadmap 2 physical foundation crates may depend on contracts and lower
  physical crates.
- Roadmap 1 semantic durable-program crates may consume physical contracts and
  facades, but physical foundation crates may not depend on semantic programs.
- `worth-store` is a thin public composition crate.
- certification crates sit at the top and consume evidence from lower crates.

The existing root `crates/worth-store` remains legacy semantic evidence and
compatibility until an explicit migration plan retires or wraps it.
