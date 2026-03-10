# Forge Relational Complexity Budgets

`forge-relational` does not treat asymptotic cost as a vibe or an optimization note. Authority-path algorithms must declare their current time complexity, declare the budget they are expected to satisfy, and name executable proof tests that keep the declaration honest in CI.

## Rules

- Every authority-path or visibility-path algorithm that can dominate runtime cost must appear in the runtime complexity registry.
- Every registry entry must name at least one executable proof test.
- CI must run the proof tests directly.
- Complexity declarations can describe current debt honestly; they must not pretend a touched-state algorithm exists when the implementation still performs full-state work.
- Budget proofs must use production surfaces and runtime counters, not hand-waved comments.

## Current Hot-Path Contracts

- `runtime.current_state.clone`
  - Current complexity: `O(entity_slots + relation_slots + adjacency_edges)`
  - Status: `Debt`
  - Why debt: commit staging still clones full state once per authoritative commit

- `runtime.snapshot_pin_maintenance`
  - Current complexity: `O(snapshot_delta_records)`
  - Status: `Verified`
  - Guard: commit and release paths must not rebuild all snapshot pin counters

- `runtime.visible_entities.scan`
  - Current complexity: `O(entity_slots)`
  - Status: `Verified`
  - Guard: visibility scans must report slot-scan and materialization cost explicitly

- `runtime.visible_relations.scan`
  - Current complexity: `O(relation_slots)`
  - Status: `Verified`
  - Guard: relation visibility scans must report slot-scan and materialization cost explicitly

- `runtime.retention.pass`
  - Current complexity: `O(chunks_with_retained_records + reclaim_batch_size + changed_live_history)`
  - Status: `Verified`
  - Guard: retention scans remain chunk-filtered and live history trimming stays tied to changed records

- `runtime.relation_adjacency.lookup`
  - Current complexity: `O(out_degree)` / `O(in_degree)`
  - Status: `Verified`
  - Guard: forward and reverse relation traversal must not require full relation scans

- `runtime.invariant.materialization`
  - Current complexity: `O(entity_slots + relation_slots)` today
  - Status: `Debt`
  - Why debt: invariant execution still materializes full visible sets for some rules

## CI Enforcement

The CI entrypoint is:

- `scripts/ci/check_relational_complexity_budgets.sh`

It fails when:

- the complexity registry disappears
- the complexity contract test module disappears
- the complexity budget doc disappears
- any proof lane stops passing

## Ratchet Policy

- Verified contracts may only stay flat or get cheaper.
- Debt contracts must remain explicit until they are replaced with stronger budgets.
- When an algorithm is improved, tighten the declared complexity and its proof tests in the same change.
