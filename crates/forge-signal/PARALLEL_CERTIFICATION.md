# forge-signal Parallel Certification Contract

This document is the release contract for the platform-grade parallel execution tranche in `forge-signal`.

For the broader adversarial matrix that ties runtime promises to concrete test lanes, see [_docs/engineering/forge_signal_adversarial_testing_matrix.md](/Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/engineering/forge_signal_adversarial_testing_matrix.md).

## Deterministic Guarantees

The runtime guarantees that the following are deterministic for logically equivalent runs across serial, staged-parallel, and full-parallel execution:

- task identities and semantic segment identities
- diagnostics summaries retained by the runtime
- explanation summaries emitted from retained or deterministically reconstructed runtime artifacts
- provenance artifacts emitted as canonical vertex/edge graphs when retained or reconstructed under the active policy
- replay event ordering and payloads
- semantic execution-report fields and counters

The guarantee is defined against canonical runtime artifacts, not thread completion order.
Where runtime policies differ in richness, equivalence is defined against the overlapping guaranteed surface: replay, stable semantic IDs, report counters, and any artifact families the compared policies both retain or reconstruct.

## Canonicalization Rules

The runtime normalizes observable semantic artifacts before they become retained truth:

- semantic batches merge by canonical stage/task/segment order
- provenance edges and vertices sort by stable node/aspect/subscription keys
- changed regions and detail labels are emitted in canonical order
- replay events are ordered by reserved sequence, never by wall-clock completion
- reordered-but-equivalent dependency and region insertion orders must converge to identical semantic artifacts

## Intentional Serial Boundaries

The following boundaries remain intentionally serial in this architecture:

- plan construction
- stage ordering
- semantic batch merge/finalization order
- transaction commit/rollback boundary transitions

Parallelism is applied to precompute and graph-local apply where the runtime can preserve deterministic merge semantics.

## Semantic Equivalence

Two runs are considered semantically equivalent when the following match byte-for-byte after canonicalization:

- diagnostics summary subset used by the certification gates
- explanation artifact summary
- provenance graph artifact
- replay event stream
- semantic execution-report counters and segment metadata

Mechanical executor differences that do not alter retained semantic artifacts are not part of semantic equivalence.

## Ownership Boundary

`forge-signal` owns:

- current-run canonical replay artifacts
- retained diagnostics, explanation, provenance, and replay truth for the active runtime
- deterministic execution/report semantics

`forge-relational` or other persistence layers own:

- durable storage across runs
- relational indexing and query
- cross-run comparison workflows beyond the runtime-local artifact surface

## Storage Profile Coverage

The certification surface is not limited to the default build. `forge-signal` must compile and pass the targeted harness lane under:

- `profile-compact`
- `profile-standard`
- `profile-extended`

The core-profile gate exists to catch serialization or runtime assumptions that accidentally hard-code one storage width.

## Release Gates

Fast required gates:

- `bash scripts/ci/check_signal_failure_matrix.sh`
- `bash scripts/ci/check_signal_contract_matrix.sh`
- `bash scripts/ci/check_signal_resource_bounds.sh`
- `bash scripts/ci/check_signal_core_profiles.sh`
- `bash scripts/ci/check_signal_semantic_snapshots.sh "$DIR"`
- `cargo test -p forge-signal --lib --features parallel adversarial_parallel -- --nocapture`

Slow certification gates:

- `bash scripts/ci/check_signal_parallel_determinism_cert.sh 4 "$DIR"`
- `bash scripts/ci/run_signal_perf_lane.sh`

## Failure Acceptance Matrix

| Surface | Required guarantee | Enforced by |
| --- | --- | --- |
| Full-parallel apply failure | No partial node commit, no semantic leakage | `tests::adversarial_parallel::full_parallel_apply_failure_does_not_leak_partial_semantic_state` |
| Transaction precompute failure | Failure + rollback diagnostics retained | `tests::diagnostics::execution_failures_and_rollbacks_automatically_record_diagnostics` |
| Event begin failure | Graph rewound, rollback retained | `tests::diagnostics::event_bus_begin_failures_record_failure_and_rollback_diagnostics` |
| Event flush / commit-promotion failure | No committed semantic outcome leakage | `tests::diagnostics::commit_promotion_failures_record_failure_and_rollback_diagnostics` and `logic::transaction::tests::hostile_commit_failure_does_not_leak_committed_semantic_outcome` |
| Poisoned transaction | Commit/rollback produce poisoned outcome and rewind | `logic::transaction::tests::poisoned_transaction_returns_poisoned_outcome` and `logic::transaction::tests::poisoned_rollback_rewinds_graph` |
| Repeated rollback/commit churn | Latest retained semantic state stays current and bounded | `logic::transaction::tests::hostile_rollback_and_commit_cycles_do_not_leak_semantic_events` and `tests::diagnostics::repeated_rollbacks_keep_latest_rollback_current_and_bounded` |

## Performance Evidence

The performance lane emits JSON for these workload shapes:

- `deep-chain-512`
- `wide-stage-256`
- `partition-tolerance-96`

For each workload, the certification artifact records:

- runtime policy (`operational`, `development`, `forensic`)
- executor profile
- core storage profile identifier
- planning time
- total execution elapsed
- snapshot time
- precompute time
- apply time
- semantic finalization time
- residual time not explained by the phase counters

The runtime is considered ready to move on only when these artifacts are stable enough to support informed tuning decisions, even if strict speedup thresholds remain domain-specific.
