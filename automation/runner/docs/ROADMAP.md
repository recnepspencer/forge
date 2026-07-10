# Runner Roadmap

## Purpose

This roadmap governs the automation runner as its own subsystem.

The runner is not milestone-local glue. It is a reusable orchestration product
that must survive long-running implementation programs, repeated QA loops,
handoffs between runs, and future expansion into multi-agent and multi-run
coordination.

## Why This Road Exists

The current durable runner already proved that append-only authority, derived
projection, and resumable execution are the right direction. It also revealed
the next hard problem: orchestration, prompts, role policy, recovery,
notifications are still too entangled to scale cleanly.

This road exists to make the runner:

- structurally decomposed instead of script-accumulated
- prompt-authoritative instead of template-scattered
- role-first instead of session-default driven
- graph-orchestrated instead of branch-heavy imperative control flow
- operator-legible under crash, stall, blocker, and review-loop pressure
- importable into future projects rather than trapped as one workspace script

## Milestone Sequence

### Milestone 1: Graph-Orchestrated Runner Foundation

Freeze the new runner constitution:

- canonical authority surfaces for run config, event ledger, derived projection,
  and LangGraph checkpoint state
- prompt assets separated from prompt assemblies, bindings, and runtime
  instantiations
- role registry and model/session policy separated from prompt content
- graph-owned execution programs for standard loops, custom prompt phases,
  recovery, and completion handoff
- operator signal policy for blocker, crash, no-edit stall, timeout, and loop
  escalation

This milestone must be strong enough that later growth lands in named
subsystems instead of back into a single mosh pit.

### Milestone 2: Coordinated Parallelism And Multi-Agent Handoffs

Add first-class support for:

- parallel specialist branches with declared join points
- judge and arbiter roles
- dependent runs and cross-run wait conditions
- explicit artifact handoff contracts between roles and runs

### Milestone 3: Importable Runner Product Boundary

Turn the runner into a repo-grade product boundary with:

- clean package exports
- project-local adapters and notifier hooks
- stable scaffold generation
- reusable project import posture

## Current Priority

Milestone 1 is next because every desired improvement the runner now needs
depends on its constitutional split being made real first.
