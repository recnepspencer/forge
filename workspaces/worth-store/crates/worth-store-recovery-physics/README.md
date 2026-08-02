# worth-store-recovery-physics

Owns Roadmap 2 S.4: WAL segments, LSNs, pageLSNs, checkpoint manifests,
redo/replay rules, source precedence, idempotent replay, and bounded recovery.

This is physical recovery machinery. It must not become an alternate semantic
truth source.

`wal_recovery_basis` admits reopened WAL append receipts, crash posture, and
durability observations as persisted recovery facts. It does not execute the
ordinary WAL path or construct Store acknowledgment. `checkpoint_cutover`,
`source_precedence`, `redo_replay`, and `page_redo` consume bounded artifacts in
fresh-process recovery; ordinary Store publication does not import replay
authority.

The C.7 boundary and C.8 handoff are documented in
[_docs/worth-store/physical-durability-and-checkpoints.md](../../../../_docs/worth-store/physical-durability-and-checkpoints.md).
