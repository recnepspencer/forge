# Forge Harness Architecture

## Purpose

`forge-harness` is a first-class workspace crate for generic execution harness infrastructure.

It exists to provide a stable substrate for:

- parity tools
- replay tools
- diagnostics inspectors
- lineage/provenance inspectors
- bridge-flow drivers
- performance rigs
- regression benches

It is not a `forge-signal`-specific test helper layer, and it is not the validator itself.

## Principles

- Stable core, extensible edge
- Open extension, closed truth
- Capability honesty over convenient overclaiming
- Owned, versioned records instead of borrowed internals
- Laravel-inspired DX without hidden execution magic

## Dependency Rule

- `forge-harness` must remain domain-neutral
- runtime crates may depend on it to implement adapters
- `forge-harness` must not depend on runtime crates
- kernel/domain harness code becomes a client of `forge-harness`, not the source of truth for its design

## Core Model

The harness core defines:

- `HarnessAdapter`
- `HarnessCapabilities`
- scenario plans and fixtures
- stable record identity
- mutation batches
- execution requests and profiles
- time markers and clock domains
- feed batches and execution phases
- workload budgets
- attachment records
- compatibility checks
- export formats
- record archives
- versioned records for runs, snapshots, events, diagnostics, explanations, provenance, and comparisons
- hook interfaces at architectural seams
- event subscriptions and projections
- run matrices and parity suites

The harness core does not define runtime-specific semantics such as planner stages, node entries, truth diffs, or geometry oracles.

## Capability Truthfulness

Capabilities are machine-readable and fail closed.

If a tool requests a mode, diagnostics level, capture depth, or comparison mode that the adapter does not support, the harness must reject the request explicitly.

This is the primary guardrail against claiming a runtime feature is complete when it is only partial or absent.

## Records

All generic records are:

- owned
- serializable
- versioned
- identity-carrying
- suitable for JSON interchange and regression storage

Runtime-specific depth belongs in extension payloads, not in the generic base envelope.

The base model must still support common cross-domain concerns directly:

- logical time
- replay time
- feed or tick batches
- phase-aware execution
- workload budgets
- budget usage reporting
- attached artifacts

## Laravel Inspiration

Laravel is an explicit DX inspiration for the harness:

- readable builders
- testbench-style environment setup
- named profiles
- fakes/doubles
- batteries-included infrastructure
- fluent benches for common run paths

The harness does not adopt Laravel-style ambient magic, container opacity, or hidden execution side effects.

The intended DX shape is:

- simple scenario and mutation builders
- named profiles for common modes
- a testbench-style wrapper for common `fixture -> mutate -> request -> profile -> run` flows
- run-matrix and parity helpers for profile sweeps
- event subscriptions and projection helpers for timeline tooling
- easy replay and event-stream entrypoints without forcing users through low-level runner plumbing

## Cross-Domain Foundation

The harness must be credible for more than signal execution.

That means the core model must leave room for:

- relational replay and history runs
- bridge patch routing and causality transfer
- kernel tolerance oracles and artifact attachments
- fintech feed sequencing and audit runs
- game-engine tick phases and frame budgets

Adapters may expose these differently, but the harness core must provide the nouns.

## Contract Hardening

The harness now treats these as first-class contract features rather than follow-up cleanup:

- stable scenario, run, snapshot, diagnostics, explanation, provenance, event-stream, and replay IDs
- explicit schema compatibility checks
- replay compatibility checks
- export helpers for durable JSON artifacts
- record archives for grouped exports
- replay request and replay record types
- event stream records in addition to point events
- budget-aware run outcomes and budget usage capture

This is the baseline for making replay, regression storage, and cross-runtime tooling durable instead of ad hoc.

## First Adapter

`forge-signal` is the first adapter.

Its adapter must expose truthful execution semantics only:

- `Serial`
- `StagedParallel`, meaning staged parallel precompute only

It must not leak internal storage or planner structures through harness contracts.
