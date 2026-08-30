# WORTH Store offline integrity observer

This crate is the process-independent C.9 observation owner. It will walk a
closed or isolated Store artifact tree under explicit entry, byte, open-file,
depth, symlink, elapsed-time, and report-size limits and emit descriptive
integrity observations. It must not import the live Store, recovery,
buffer-pool, or runtime-integrity crates, and its findings grant no decoder,
recovery, quarantine, or repair authority.

Phase 2 installs only the independent dependency, bounded request, report
destination, version-1 protocol identity, and declaration-only root-protocol
boundary. A file destination is rejected when it is lexically equal to or
beneath the declared Store root. The future report-emission boundary must also
canonicalize both paths and repeat that exclusion; this descriptive request
does not claim filesystem identity or perform I/O.

The artifact walk, separately implemented family readers and checksum
calculation, report wire format, comparison, and executable arrive with their
gated family phases. No parser, checksum, traversal, classifier, command, or
process orchestration exists in this Phase 2 crate.

The only normal dependencies are `worth-foundational` for cross-runtime
protocol descriptions and `worth-store-physical-format` for
`integrity_declarations`. Imports from runtime codecs, checksum execution,
Store, recovery, runtime integrity, maintenance, repair, operations, or the
legacy offline verifier are forbidden.
