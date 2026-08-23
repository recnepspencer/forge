# worth-store-recovery-physics

Owns the narrow pure-law portion of physical recovery: WAL segments, LSNs,
pageLSNs, source precedence, operation-fate reconciliation, idempotent page
redo, and bounded redo planning.

This is physical recovery machinery. It must not become an alternate semantic
truth source.

It does not own filesystem discovery, runtime progression, Store coordination,
offline observation, report protocols, or physical effects. Those belong to
the recovery runtime, Store, offline verifier, Foundational protocol
vocabulary, and C.4 backend respectively.

The C.7 boundary and C.8 handoff are documented in
[_docs/worth-store/physical-durability-and-checkpoints.md](../../../../_docs/worth-store/physical-durability-and-checkpoints.md).

For C.8, this crate remains pure meaning only. Its public laws decide
current/previous source precedence, WAL-prefix continuity, checkpoint-covered
WAL ranges, pageLSN apply/skip eligibility, operation fates, and finite plan
cost. It owns no clock, filesystem walk, Store authority, process lifecycle,
observer report, runtime effect, replay surface, or reconstruction result.

The recovery runtime consumes these laws through its concrete owner facades;
the offline verifier interprets persisted bytes independently. Ordinary
runtime and observer lanes must not import replay or reconstruction. C.8 is a
fresh physical reopen, not backup/PITR, rollback, or semantic repair. The
runtime's fixed limits and the observer's four-axis limits are admitted by
their owning entry points rather than configured by this pure crate.
