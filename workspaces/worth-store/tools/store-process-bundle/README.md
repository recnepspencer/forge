# Store process bundle

`worth-store-process-bundle` is the tooling-owned authority for fresh-process
Store evidence builds. It exposes sealed production and bounded-residency
recipes, builds writer, observer, and recovery roles in separate locked Cargo
invocations, binds the exact compiler-selected executables, and carries the
metadata-derived local source closure and feature graph alongside them.

Each campaign receives a `FreshProcessCargoTarget` allocated as an exclusive
child of the configured Cargo target cache. `CARGO_TARGET_DIR` selects only
that parent cache; it never selects the final campaign directory. The typed
target must remain alive while its bound executables are used and must be
explicitly closed after the campaign releases them.

The crate intentionally depends only on mechanical Cargo metadata, JSON, and
digest facilities. It does not depend on Store runtime crates, recovery
physics, or the courtroom runner, so neither process authority nor courtroom
policy can redefine the build boundary.
