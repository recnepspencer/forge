# worth-store

This is the thin public facade for the rebuilt Worth Store workspace.

It should re-export stable public APIs and compose lower crates. It should not
become the place where authority, physical format, recovery, compatibility,
maintenance, or certification logic is implemented.

## Physical Durability And Checkpoints

`physical_runtime` exposes the ordinary Store-owned durability contract.
Callers submit through `ServingPhysicalRuntime::record_submission()`, receive a
typed preparation posture, and start or execute only `PreparedPhysicalMutation`.
The terminal fate is completed, proven no effect, or indeterminate. Only the
completed typestate can become `PhysicalMutationAcknowledgment`.

The acknowledgment is physical truth under one qualified backend profile. It
is not semantic commit, Query truth, or transaction authority. Dropping a
mutation handle abandons observation only; explicit cancellation cannot turn a
possibly effectful mutation into a safe retry. `close_plan()` drains Store-owned
mutation and checkpoint work before Signal, residency, and media shutdown.

Managed fuzzy checkpoints are submitted through
`ServingPhysicalRuntime::checkpoints()`. A completed checkpoint carries its
exact contiguous retained WAL tail, idempotency-binding compaction, and lawful
WAL reclamation observation. Checkpoint existence alone grants no deletion
authority.

See [_docs/worth-store/physical-durability-and-checkpoints.md](../../../../_docs/worth-store/physical-durability-and-checkpoints.md)
for the caller and operator contract.

## Physical Residency Failures

`PhysicalRecordResidencyFailure` is the Store-owned facade for a lower physical
residency denial. `kind()` supplies a broad policy class; `reason()` supplies
the exact causal reason and retains small actionable fields such as bounded-load
limits or candidate cardinality. Callers use the reason to distinguish a bad
declaration or payload from live identity contention, pin/dirty posture,
writeback-claim contention, receipt mismatch, lifecycle closure, or pressure.

The reason vocabulary carries no pool control, retry authority, Signal handle,
scheduler receipt, backend authority, `worth-proof` witness, Foundational
value, or semantic residency claim. Pressure follows its dedicated
`PhysicalRecordPressureEvidence` path, and a terminated frame load follows its
typed terminal projection.
