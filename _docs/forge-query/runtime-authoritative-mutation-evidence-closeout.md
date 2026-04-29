# Runtime Authoritative Mutation Evidence Closeout

This closeout freezes what downstream runtimes may rely on from the
cross-runtime authority-evidence lane right now.

## Safe To Build Now

- workspace.insert/update/delete/batch receipts preserve declared-versus-resolved target evidence together with touched-aspect fallout
- existing-truth binding, same-batch symbolic target reference, naming mutation, and continuity mutation evidence are part of the ordinary public receipt and inspection story
- batch and import-style authority sessions preserve aggregate existing-binding, symbolic-target, naming, continuity, causality, and provenance digests
- downstream domains may rely on Query receipts and inspection instead of rebuilding target-recovery, naming, or continuity explanation glue locally

## Must Not Assume Yet

- authority-mutation evidence closes durable restart, temporal, async, or store-backed mutation semantics
- unsupported identity-binding, naming, or continuity families remain fail-closed until explicitly admitted
- downstream code may bypass Query receipts and inspect raw bridge/runtime provenance bags directly

## Migration Guidance

- move authoritative mutation onto workspace.insert/update/delete/batch and consume receipts plus inspect output as the domain explanation contract
- delete local existing-target rebinding, naming outcome reconstruction, and continuity breadcrumb glue once equivalent Query evidence is available
- treat unsupported mutation-evidence neighbors as fail-closed support gates rather than compatibility seams

## Bridge Carry-Forward Contract

- bridge writeback artifacts can carry target, causality, provenance, naming, and continuity evidence into one Query-facing contract
- batch/session authority bundles preserve aggregate existing-binding, symbolic-target, naming, continuity, causality, and provenance digests
- replay-safe request and receipt digests remain part of the carry-forward story for admitted authority sessions

## Certified Support Families

- existing-truth binding: `direct_entity_identity`
- symbolic same-batch target reference: `same_batch_declared_target`
- naming mutation families: `attach_new_target`, `attach_existing_target`, `rebind_target`, `remove`
- continuity mutation families: `rebind_existing_target`, `split_existing_target`

## Verification

- `cargo fmt -p forge-query`
- `cargo check -p forge-query --tests`
- `cargo test --manifest-path crates/forge-query/Cargo.toml --test phase_boundaries_compile_fail`
- `cargo test -p forge-query`
- `cargo fmt -p forge-runtime-bridge`
- `cargo check -p forge-runtime-bridge --tests`
- `cargo test -p forge-runtime-bridge`
- `cargo test --manifest-path crates/forge-runtime-bridge/Cargo.toml --test phase_boundaries_compile_fail`
- `git diff --check`
