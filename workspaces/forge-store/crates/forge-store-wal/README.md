# forge-store-wal

Owns Roadmap 1 Milestones 3, 3.5, and 3.6 as semantic write-path contracts:
durable-mode commit acknowledgment, log-before-acknowledge, write-path
certification, crash taxonomy, and recovery source precedence.

Physical WAL mechanics belong in `forge-store-recovery-physics`. This crate
binds them to semantic durable-mode expectations.
