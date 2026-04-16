# Forge Store Milestone Unlock Map

## Purpose

This is the simple execution reference for
[forge_store_roadmap.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/forge_store_roadmap.md).

Use it like this:

- "We finished Milestone X."
- "What can we now fully move on to?"

This document answers that directly.

## Rule

`persist canonical authority once, parallelize derived storage work around it`

If a milestone unlocks other milestones, that means those later milestones now
have enough foundation to proceed honestly.

## Unlock Map

### After Milestone 1: Canonical Commit Persistence And Artifact Authority

You can fully move on to:

- `Milestone 2` Operating Modes And Lifecycle Contracts
- early `Milestone 7` Durable Schema/Lineage/Cursor/Checkpoint artifacts

Why:

- canonical durable artifact meaning is now defined

### After Milestone 2: Operating Modes And Lifecycle Contracts

You can fully move on to:

- `Milestone 3` WAL-Coordinated Durable Mode And Crash Recovery
- continue `Milestone 7` honestly

Why:

- durable vs embedded vs absent mode boundaries are now explicit

### After Milestone 3: WAL-Coordinated Durable Mode And Crash Recovery

You can fully move on to:

- `Milestone 4` Snapshot Persistence And Point-In-Time Restore
- `Milestone 5` Structural Delta Storage And Branch Delta Layering
- durable integration work inside `Milestone 7`

Why:

- the durable authority path is now real

### After Milestone 4: Snapshot Persistence And Point-In-Time Restore

You are partway clear for:

- `Milestone 10` Retention/Compaction/Reclamation
- later `Milestone 12` Replication/Capsules/Integrity

But you should not fully move on yet without `Milestone 5`.

Why:

- snapshot basis is real, but the physical branch/delta model is not fully
  closed yet

### After Milestone 5: Structural Delta Storage And Branch Delta Layering

You can fully move on to:

- `Milestone 6` Aspect-Aware Physical Layout And Structural Blocks
- `Milestone 9` Bulk Ingest And Bulk Transform Paths

And, together with `Milestone 4`, you can fully move on to:

- `Milestone 10` Retention/Compaction/Reclamation

Why:

- shared-base creation, branch-delta reads, rewrite, rebuild, and control-lane
  parity are now closed and machine-certified

Why:

- branch/delta storage is now honest enough for physical-layout and chunked
  work
- Milestone 7 may continue in parallel because its schema/lineage/cursor and
  checkpoint artifacts should bind to branch/frontier authority, not to
  backend-local delta stack shape

### After Milestone 6: Aspect-Aware Physical Layout And Structural Blocks

You can fully move on to:

- `Milestone 8` Live-Query Substrate And Durable Sync Basis

And, together with `Milestone 4` and `Milestone 5`, you can fully move on to:

- `Milestone 10` Retention/Compaction/Reclamation

Why:

- physical narrowing and structural reuse are now honest enough for live-query
  and retention work

### After Milestone 7: Durable Schema/Lineage/Cursor/Checkpoint Artifacts

You can fully move on to:

- `Milestone 8` Live-Query Substrate And Durable Sync Basis

Why:

- live-query now has real durable basis, cursor, and schema/lineage support

### After Milestone 8: Live-Query Substrate And Durable Sync Basis

You can fully move on to:

- `Milestone 12` Replication, Capsules, And Integrity Verification

Why:

- export/replication can now reason about durable read/sync basis honestly

### After Milestone 9: Bulk Ingest And Bulk Transform Paths

You do not unlock a new major foundation milestone by itself.

What it means:

- bulk operational capability is now ready
- later certification can include bulk workloads honestly

### After Milestone 10: Retention, Compaction, And Reclamation

You can fully move on to:

- `Milestone 11` Tiering And Durable Working-Set Intelligence
- `Milestone 12` Replication, Capsules, And Integrity Verification
- `Milestone 17` Native Blob And Object Storage

Why:

- rebuild and retention rules are now stable enough for these later systems

### After Milestone 11: Tiering And Durable Working-Set Intelligence

You can fully move on to:

- `Milestone 18` Admission Control And Budget Contracts

Why:

- budget controls now have a real placement/tiering model to govern

### After Milestone 12: Replication, Capsules, And Integrity Verification

You can fully move on to:

- `Milestone 13` Time-Travel Diff Acceleration And Merge-Assistance Artifacts
- `Milestone 14` Derived Durable Artifact Families And Accuracy Taxonomy
- `Milestone 17` Native Blob And Object Storage

Why:

- export, rebuild, and integrity contracts are now stable enough for advanced
  derived and blob programs

### After Milestone 13: Time-Travel Diff Acceleration And Merge-Assistance Artifacts

You are clear to keep going on advanced derived-storage programs, but it does
not unlock a new major foundation milestone by itself.

### After Milestone 14: Derived Durable Artifact Families And Accuracy Taxonomy

You can fully move on to:

- `Milestone 15` Analysis Lanes
- `Milestone 16` Correspondence Indexes, Structural Fingerprints, And Locality Clustering

Why:

- these programs now inherit an honest accuracy/trust model

### After Milestone 15: Analysis Lanes

You do not unlock a new major foundation milestone by itself.

What it means:

- analysis durability is now real
- domain certification can include basis-pinned analysis honestly

### After Milestone 16: Correspondence Indexes, Structural Fingerprints, And Locality Clustering

You do not unlock a new major foundation milestone by itself.

What it means:

- advanced derived lookup and locality programs are now real
- domain certification can include them honestly

### After Milestone 17: Native Blob And Object Storage

You can fully move on to:

- `Milestone 18` Admission Control And Budget Contracts
- certification with blob-bearing workloads

Why:

- budget controls and certification now have the real blob model available

### After Milestone 18: Admission Control And Budget Contracts

You can fully move on to:

- `Milestone 19` Generic Store Certification Program

Why:

- the store now has explicit operational limits and failure policy

### After Milestone 19: Generic Store Certification Program

You can fully move on to:

- `Milestone 20` Domain Store Certification Program

Why:

- the generic store is now proven strongly enough to certify domain fit

## Fast View

If you want the shortest possible version:

- `M1` unlocks `M2` and early `M7`
- `M2` unlocks `M3`
- `M3` unlocks `M3.5`
- `M3.5` unlocks `M3.6`
- `M3.6` unlocks `M4`, `M5`, and durable `M7`
- `M5` unlocks `M6` and `M9`
- `M6 + M7` unlock `M8`
- `M4 + M5 + M6` unlock `M10`
- `M10` unlocks `M11`, `M12`, and `M17`
- `M12` unlocks `M13`, `M14`, and `M17`
- `M14` unlocks `M15` and `M16`
- `M11 + M17` unlock `M18`
- `M18` unlocks `M19`
- `M19` unlocks `M20`

## Recommended "What Next?" Answers

If you just closed:

- `M3`: do `M4`, `M5`, and durable `M7`
- `M5`: do `M6` and `M9`
- `M6`: finish toward `M8` and `M10`
- `M10`: do `M11`, `M12`, and `M17`
- `M12`: do `M13`, `M14`, and `M17`
- `M14`: do `M15` and `M16`
- `M18`: do `M19`

## Companion Documents

- [forge_store_roadmap.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/forge_store_roadmap.md)
- [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/test-requirements.md)
