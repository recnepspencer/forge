# Forge — Agent Directives

You are working on **Forge**, a from-scratch B-Rep geometry kernel written in Rust.

## Primary Mission

Build a production-quality CAD kernel that is **correct by construction**. Topology decisions are driven by certified predicates, never raw floats. Every operation is deterministic and replayable.

## Before You Code

1. Read the relevant agent rules in `.agent/rules/`. All rules are **always-on**.
2. Consult `llms.txt` at the project root to locate documentation.
3. Check `DEVELOPMENT_BLUEPRINT.MD` for the current milestone and what's already implemented.
4. Follow the `/add-feature` or `/new-module` workflow when creating new code.

## Non-Negotiable Invariants

- **D3 Firewall**: All topology mutations require `CertifiedTriSign`. Raw `f64` comparisons are compile errors.
- **D6 Transactions**: All topology changes go through `MutableDraft`. Commit on success, drop to rollback.
- **D1 Determinism**: Same inputs → same outputs. Always. No exceptions.
- **Doc comments only**: Use `///` and `//!`. Inline `//` comments are almost always wrong.
- **Linear tests**: No loops or conditionals in test functions.

## Current State

The project is in **Phase 0** (Foundation Layer). Milestones 0.1, 0.2, and 0.2.1 are complete. The `forge-math` crate has the filtered arithmetic pipeline and `CertifiedTriSign`.
