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
