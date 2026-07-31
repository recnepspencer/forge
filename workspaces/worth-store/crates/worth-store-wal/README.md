# worth-store-wal

Owns WAL meaning: frame grammar, LSN topology, stable WAL identities,
path-free append declarations, bounded inspection, crash taxonomy, and
recovery-source precedence.

Ordinary callers use `WalAppendFrontier` and `plan_wal_frame_append` to produce
immutable bytes and an exact next frontier. This crate does not open an
ordinary writer, schedule I/O, perform a filesystem effect, settle Store
mutation truth, or construct final acknowledgment. Store owns that
orchestration through Signal, its scheduler and executor, and the C.4 media
owner.

Path-bound append planning exists only under `certification-authority` for
lower-mechanism certification and reconstructive inspection. It is not an
ordinary Store write path.
