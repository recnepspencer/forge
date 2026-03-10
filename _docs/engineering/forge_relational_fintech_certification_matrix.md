# Forge Relational Fintech Certification Matrix

## Purpose

This document defines the certification matrix for the `forge-relational` fintech test domain.

The goal is not to accumulate more tests. The goal is to ensure that the fintech domain closes specific coverage gaps in the real `forge-relational` API surface before we move on to another domain.

Each workflow we add should close an explicit uncovered cell in this matrix:

- API family
- bug class
- workflow expression

A workflow that cannot name those three things is probably filler.

---

## Scope

This matrix is specifically about what the fintech test domain should certify for `forge-relational`.

It is not the whole crate certification story.

Other existing crate-level suites still matter for:

- isolated runtime contracts
- durability contract details
- replay contract details
- history and lineage queries
- profile/backend matrix coverage
- index and storage metadata specifics

The fintech domain exists to prove that those capabilities survive in realistic, hostile, world-shaped workflows with branch, replay, recovery, audit, and persistence pressure.

---

## Relational Surface Reference

The fintech matrix should be aligned to the real `forge-relational` public surface and architectural direction, not just the currently convenient workflow tests.

Relevant sources:

- [`crates/forge-relational/src/lib.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/lib.rs)
- [`crates/forge-relational/src/facade.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/facade.rs)
- [`crates/forge-relational/src/presentation/api.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/presentation/api.rs)
- [`crates/forge-relational/src/logic/runtime/mod.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/logic/runtime/mod.rs)
- [`crates/forge-relational/src/transactions/logic/mod.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/transactions/logic/mod.rs)
- [`crates/forge-relational/src/history/logic/mod.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/history/logic/mod.rs)
- [`crates/forge-relational/src/replay/logic/mod.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/replay/logic/mod.rs)
- [`crates/forge-relational/src/durability/logic/mod.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/durability/logic/mod.rs)
- [`_docs/engineering/forge_harness_workflow_certification_design.md`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/engineering/forge_harness_workflow_certification_design.md)

The long-term target domains include topology-style and other authority-heavy runtimes. That means the fintech domain should explicitly exercise:

- partition-aware entity and relation truth
- serialized authority under branch churn
- savepoint and rollback semantics
- replay and canonical envelope integrity
- checkpoint, tail recovery, and compaction behavior
- publication, patch, diagnostics, and artifact consistency
- branch-local audit and correction truth
- honest workflow certification overlap
- complexity and budget hooks

---

## API Families

These are the `forge-relational` API families that fintech can and should help certify.

### 1. World Assembly and Runtime Construction

Primary surfaces:

- `RelationalRuntimeApi`
- `RelationalRuntimeBuilder`
- runtime profile and durability configuration
- crate-local fintech world assembly

Why it matters:

- bad world setup can hide runtime bugs
- configuration drift can silently change guarantees

### 2. Authoritative Transactions, Savepoints, and Rollback

Primary surfaces:

- `begin_transaction`
- `TransactionOptions`
- `WorkerIntentBatch`
- `create_savepoint`
- `rollback_to_savepoint`
- commit authority behavior

Why it matters:

- hostile workflows should never leak partial truth
- savepoint bugs will be subtle and expensive

### 3. Snapshot and MVCC Read Semantics

Primary surfaces:

- `snapshot`
- `read_snapshot`
- snapshot-visible truth
- branch-local visibility

Why it matters:

- stale or leaked visibility invalidates the whole authority model

### 4. Query Packet and Read-Surface Execution

Primary surfaces:

- `QueryWorkPacket`
- `ReadTarget`
- packet execution over entities and relations
- case-specific read probes

Why it matters:

- workflows only certify what they can actually observe

### 5. Branch, History, and Merge Semantics

Primary surfaces:

- `create_branch`
- branch heads
- ancestor chain
- merge planning and merge parent ordering

Why it matters:

- branch locality is a core product contract

### 6. Replay and Canonical Reconstruction

Primary surfaces:

- `replay_commit`
- canonical commit envelopes
- replay mismatch surfaces
- replay failure classes

Why it matters:

- replay drift is already a known regression surface

### 7. Checkpoint, Recovery, and Compaction

Primary surfaces:

- `checkpoint`
- `recovery_plan`
- `recover`
- `compact_store`

Why it matters:

- durability truth must survive real workflow churn, not just contract demos

### 8. Publication, Patch, and Diagnostics Observability

Primary surfaces:

- `latest_patch`
- `latest_replay`
- `latest_publication_bundle`
- `diagnostics`

Why it matters:

- observability disagreement is still a correctness bug

### 9. Index, Lineage, and Invariant Metadata

Primary surfaces:

- lineage graph and correspondence metadata
- index generations
- runtime invariants and storage metadata

Why it matters:

- authoritative truth includes metadata and forensic structure, not only payload rows

### 10. Harness and Certification Honesty

Primary surfaces:

- workflow certification adapter
- artifact capture
- invariant scheduling
- overlap-aware guarantees
- regression target modeling

Why it matters:

- a dishonest certification layer produces false confidence

### 11. Complexity and Budget Enforcement

Primary surfaces:

- complexity contracts
- complexity counters
- budget outcome hooks

Why it matters:

- the runtime must remain hostile-workflow-safe under scale and adversarial shaping

---

## Bug Classes

These are the relational bug classes the fintech domain should target.

### B1. Wrong Truth

Entity or relation state is semantically wrong after a workflow.

### B2. Branch Leakage

Branch-local mutation leaks into the wrong branch, or reads observe the wrong branch head.

### B3. Snapshot Visibility Drift

Snapshot-visible truth changes when it should not, or fails to expose the correct historical state.

### B4. Query Surface Blindness

Packets, probes, or read summaries fail to expose the workflow truth they are supposed to certify.

### B5. Savepoint / Rollback Failure

Partial changes survive a rollback or rollback diagnostics lie.

### B6. Merge / History Inconsistency

Branch ancestry, merge parents, or merge locality becomes semantically inconsistent.

### B7. Replay Drift

Canonical replay diverges, targets the wrong branch, or loses parent-chain integrity.

### B8. Recovery Drift

Recovered truth, branch heads, or checkpoint-tail replay diverges from authoritative state.

### B9. Durability / Compaction Corruption

Checkpoint selection, corruption fallback, or segment compaction breaks recoverable truth.

### B10. Observability Disagreement

Patch, replay, publication, diagnostics, and truth surfaces disagree about what happened.

### B11. Metadata Drift

Lineage, correspondence, index, or invariant metadata diverges from the truth workflow it describes.

### B12. Harness Overclaim / False Comparison

The certification layer claims overlap, artifacts, or invariants the runtime does not actually guarantee.

### B13. Complexity / Budget Regression

Hostile workflows silently exceed intended complexity envelopes.

---

## Workflow Families

These are the workflow families the fintech domain should eventually provide.

Each family exists to close specific matrix cells.

### W1. Seeded World Assembly

Purpose:

- prove the default world comes up alive, seeded, branchable, partitioned, and structurally correct

Primary API families:

- 1
- 4

Primary bug classes:

- B1
- B4

Current status:

- Covered

Current files:

- [`crates/forge-relational/src/tests/domains/fintech/workflows.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/tests/domains/fintech/workflows.rs)
- [`crates/forge-relational/src/tests/domains/fintech/fixture/mod.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/tests/domains/fintech/fixture/mod.rs)

### W2. Late Trade Correction on Analysis Branch

Purpose:

- prove branch-local correction truth, audit truth, and baseline snapshot preservation

Primary API families:

- 2
- 3
- 5
- 8

Primary bug classes:

- B1
- B2
- B3
- B10

Current status:

- Covered

Current files:

- [`crates/forge-relational/src/tests/domains/fintech/workflows.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/tests/domains/fintech/workflows.rs)
- [`crates/forge-relational/src/tests/domains/fintech/actions/corrections.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/tests/domains/fintech/actions/corrections.rs)
- [`crates/forge-relational/src/tests/domains/fintech/actions/audits.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/tests/domains/fintech/actions/audits.rs)

### W3. Intraday Risk Breach

Purpose:

- prove market/risk mutation, breach visibility, and replay-locality under analysis-branch stress

Primary API families:

- 2
- 4
- 5
- 6

Primary bug classes:

- B1
- B2
- B4
- B7

Current status:

- Covered

Current files:

- [`crates/forge-relational/src/tests/domains/fintech/workflows.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/tests/domains/fintech/workflows.rs)
- [`crates/forge-relational/src/tests/domains/fintech/actions/risk.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/tests/domains/fintech/actions/risk.rs)

### W4. Failed Settlement Repair

Purpose:

- prove settlement repair truth, cash-event repair, and audit visibility on a branch-local repair lane

Primary API families:

- 2
- 4
- 5
- 8

Primary bug classes:

- B1
- B2
- B4
- B10

Current status:

- Covered

Current files:

- [`crates/forge-relational/src/tests/domains/fintech/workflows.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/tests/domains/fintech/workflows.rs)
- [`crates/forge-relational/src/tests/domains/fintech/actions/settlements.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/tests/domains/fintech/actions/settlements.rs)

### W5. Savepoint Rollback and Saved Commit

Purpose:

- prove savepoint rollback removes transient trade mutation and committed post-savepoint truth remains branch-local

Primary API families:

- 2
- 5

Primary bug classes:

- B2
- B5

Current status:

- Covered

Current files:

- [`crates/forge-relational/src/tests/domains/fintech/workflows.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/tests/domains/fintech/workflows.rs)
- [`crates/forge-relational/src/tests/domains/fintech/actions/savepoints.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/tests/domains/fintech/actions/savepoints.rs)

### W6. Branch Divergence and Merge

Purpose:

- prove divergent branch mutation and merge execution remain history-coherent

Primary API families:

- 2
- 5

Primary bug classes:

- B2
- B6

Current status:

- Covered

Current files:

- [`crates/forge-relational/src/tests/domains/fintech/workflows.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/tests/domains/fintech/workflows.rs)
- [`crates/forge-relational/src/tests/domains/fintech/actions/merges.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/tests/domains/fintech/actions/merges.rs)

### W7. Replay Regression and Replay Certification

Purpose:

- prove replay stays branch-local, preserves parent chains, and keeps the known replay drift visible as an explicit regression target

Primary API families:

- 5
- 6
- 10

Primary bug classes:

- B7
- B12

Current status:

- Covered

Current files:

- [`crates/forge-relational/src/tests/domains/fintech/certification/plans/regressions.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/tests/domains/fintech/certification/plans/regressions.rs)
- [`crates/forge-relational/src/tests/domains/fintech/certification/tests.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/tests/domains/fintech/certification/tests.rs)

### W8. Checkpoint Tail Recovery

Purpose:

- prove post-checkpoint hostile workflow state remains recoverable and queryable

Primary API families:

- 7
- 8

Primary bug classes:

- B8
- B10

Current status:

- Covered

Current files:

- [`crates/forge-relational/src/tests/domains/fintech/workflows.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/tests/domains/fintech/workflows.rs)
- [`crates/forge-relational/src/tests/domains/fintech/actions/recovery.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/tests/domains/fintech/actions/recovery.rs)

### W9. Corrupt Checkpoint Fallback

Purpose:

- prove the fintech workflow world can exercise corrupt-checkpoint recovery planning honestly

Primary API families:

- 7

Primary bug classes:

- B8
- B9

Current status:

- Covered

Current files:

- [`crates/forge-relational/src/tests/domains/fintech/workflows.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/tests/domains/fintech/workflows.rs)
- [`crates/forge-relational/src/tests/domains/fintech/failure_injection/durability.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/tests/domains/fintech/failure_injection/durability.rs)

### W10. Durable Compaction

Purpose:

- prove compaction remains legal after fintech workflow checkpoints and does not collapse the store into unrecoverable state

Primary API families:

- 7

Primary bug classes:

- B9

Current status:

- Covered

Current files:

- [`crates/forge-relational/src/tests/domains/fintech/workflows.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/tests/domains/fintech/workflows.rs)

### W11. Observability Surface Agreement

Purpose:

- prove patch, replay, publication, and diagnostics surfaces stay honest relative to workflow truth

Primary API families:

- 8
- 10

Primary bug classes:

- B10
- B12

Current status:

- Covered

Current files:

- [`crates/forge-relational/src/tests/domains/fintech/probes/observability.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/tests/domains/fintech/probes/observability.rs)
- [`crates/forge-relational/src/tests/domains/fintech/certification/artifacts.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/tests/domains/fintech/certification/artifacts.rs)
- [`crates/forge-relational/src/tests/domains/fintech/workflows.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/tests/domains/fintech/workflows.rs)

### W12. Query Probe Honesty

Purpose:

- prove case-level probes actually expose the domain surfaces claimed by the workflows

Primary API families:

- 4
- 10

Primary bug classes:

- B4
- B12

Current status:

- Covered

Current files:

- [`crates/forge-relational/src/tests/domains/fintech/probes/case_truth.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/tests/domains/fintech/probes/case_truth.rs)
- [`crates/forge-relational/src/tests/domains/fintech/workflows.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/tests/domains/fintech/workflows.rs)

### W13. Harness Workflow Certification Baselines

Purpose:

- prove the new workflow runner can honestly certify at least one serious baseline and the main hostile seeded cases

Primary API families:

- 10

Primary bug classes:

- B12

Current status:

- Covered

Current files:

- [`crates/forge-relational/src/tests/domains/fintech/certification/tests.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/tests/domains/fintech/certification/tests.rs)
- [`crates/forge-relational/src/tests/domains/fintech/certification/adapter.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/tests/domains/fintech/certification/adapter.rs)

### W14. Metadata Preservation Through Hostile Workflows

Purpose:

- prove lineage, correspondence, index, and invariant metadata survive hostile fintech workflows, replay, and recovery

Primary API families:

- 7
- 9

Primary bug classes:

- B8
- B11

Current status:

- Covered

Current files:

- [`crates/forge-relational/src/tests/domains/fintech/workflows.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/tests/domains/fintech/workflows.rs)
- [`crates/forge-relational/src/tests/domains/fintech/actions/metadata.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/tests/domains/fintech/actions/metadata.rs)
- [`crates/forge-relational/src/tests/domains/fintech/invariants/case_workflows.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/tests/domains/fintech/invariants/case_workflows.rs)

### W15. Snapshot Release and Historical Visibility

Purpose:

- prove historical snapshots stay stable under hostile branch mutation and released snapshots obey the intended contract

Primary API families:

- 3

Primary bug classes:

- B3

Current status:

- Covered

Current files:

- [`crates/forge-relational/src/tests/domains/fintech/workflows.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/tests/domains/fintech/workflows.rs)
- [`crates/forge-relational/src/tests/domains/fintech/actions/snapshots.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/tests/domains/fintech/actions/snapshots.rs)
- [`crates/forge-relational/src/tests/domains/fintech/probes/case_truth.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/tests/domains/fintech/probes/case_truth.rs)

### W16. Complexity and Budget Certification

Purpose:

- prove hostile workflows remain within declared complexity envelopes and eventually produce harness budget artifacts

Primary API families:

- 11
- 10

Primary bug classes:

- B13
- B12

Current status:

- Covered

Current files:

- [`crates/forge-relational/src/tests/domains/fintech/complexity/mod.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/tests/domains/fintech/complexity/mod.rs)
- [`crates/forge-relational/src/tests/domains/fintech/workflows.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/tests/domains/fintech/workflows.rs)
- [`crates/forge-relational/src/tests/domains/fintech/certification/artifacts.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/tests/domains/fintech/certification/artifacts.rs)
- [`crates/forge-relational/src/tests/domains/fintech/certification/adapter.rs`](/Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-relational/src/tests/domains/fintech/certification/adapter.rs)

---

## Current Coverage Summary

### Covered

- W1 Seeded World Assembly
- W2 Late Trade Correction on Analysis Branch
- W3 Intraday Risk Breach
- W4 Failed Settlement Repair
- W5 Savepoint Rollback and Saved Commit
- W6 Branch Divergence and Merge
- W7 Replay Regression and Replay Certification
- W8 Checkpoint Tail Recovery
- W9 Corrupt Checkpoint Fallback
- W10 Durable Compaction
- W12 Query Probe Honesty
- W13 Harness Workflow Certification Baselines
- W14 Metadata Preservation Through Hostile Workflows
- W15 Snapshot Release and Historical Visibility
- W11 Observability Surface Agreement
- W16 Complexity and Budget Certification

### Missing

---

## Rules for New Workflow Additions

Every new fintech workflow should name:

- one primary workflow family from this matrix
- one primary API family
- one primary bug class
- one invariant that should fail for the intended regression

Every new certification plan should also name:

- the case role it targets
- the artifact surfaces it expects
- the overlap guarantees it relies on
- whether it is baseline, adversarial, recovery, regression, or complexity-oriented

If a proposed test cannot explain those fields, it probably does not belong in the certification suite yet.
