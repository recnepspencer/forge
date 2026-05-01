# Runtime Authoritative Mutation Evidence Closeout

This closeout freezes what downstream runtimes may rely on from the
cross-runtime authority-evidence lane right now.

## Safe To Build Now

- workspace.insert/update/delete/batch receipts preserve declared-versus-resolved target evidence together with touched-aspect fallout
- existing-truth binding, same-batch symbolic target reference, same-batch symbolic aspect reference, naming mutation, and continuity mutation evidence are part of the ordinary public receipt and inspection story
- existing-truth assertions now distinguish retained authoritative assertions from backend-verified assertions on the public receipt and inspection surface
- mixed existing-truth authority sessions now preserve aggregate mode evidence that distinguishes retained assertions, backend-verified assertions, verified updates, and verified deletes without reconstructing that story from component receipts
- existing-truth probes now expose a typed backend-verified probe lane for current authoritative values without smuggling that truth through mutation receipts
- existing-truth verified updates now expose a typed backend-verified update lane that proves current authoritative values before applying update-family mutation receipts
- existing-truth verified deletes now expose a typed backend-verified delete lane that proves current authoritative values before applying delete-family mutation receipts
- existing-truth batch receipts, scalar inspection, and probe surfaces keep retained assertions, backend-verified assertions, backend-verified probes, verified updates, and verified deletes semantically distinct under mixed authority sessions
- batch and import-style authority sessions preserve aggregate existing-binding, symbolic-target, symbolic-aspect, naming, continuity, causality, and provenance digests
- downstream domains may rely on Query receipts and inspection instead of rebuilding target-recovery, naming, or continuity explanation glue locally
- downstream domains may rely on `verify_existing(...)` only when the active backend actually supports backend verification; unsupported backends remain typed and fail-closed
- downstream domains may rely on `update_existing_verified(...)` only when the active backend actually supports backend verification; unsupported backends remain typed and fail-closed
- downstream domains may rely on `delete_existing_verified(...)` only when the active backend actually supports backend verification; unsupported backends remain typed and fail-closed

## Must Not Assume Yet

- authority-mutation evidence closes durable restart, temporal, async, or store-backed mutation semantics
- unsupported identity-binding, naming, or continuity families remain fail-closed until explicitly admitted
- unsupported existing-truth binding, assertion, verified-mutation, and probe neighbors remain typed and fail-closed rather than degrading into best-effort compatibility
- downstream code may bypass Query receipts and inspect raw bridge/runtime provenance bags directly

## Migration Guidance

- move authoritative mutation onto workspace.insert/update/delete/batch and consume receipts plus inspect output as the domain explanation contract
- use `workspace.assert_existing(...)` for retained assertion receipts and `workspace.verify_existing(...)` when the backend must prove current stored truth before returning an assertion receipt
- use `workspace.probe_existing(...)` when the domain needs current authoritative aspect values as input rather than a retained assertion receipt
- use `workspace.update_existing_verified(...)` when the backend must prove current stored truth immediately before an existing-target update-family mutation
- use `workspace.delete_existing_verified(...)` when the backend must prove current stored truth immediately before an existing-target delete-family mutation
- delete local existing-target rebinding, naming outcome reconstruction, and continuity breadcrumb glue once equivalent Query evidence is available
- treat unsupported mutation-evidence neighbors as fail-closed support gates rather than compatibility seams

## Bridge Carry-Forward Contract

- bridge writeback artifacts can carry target, causality, provenance, naming, and continuity evidence into one Query-facing contract
- batch/session authority bundles preserve aggregate existing-binding, symbolic-target, naming, continuity, causality, and provenance digests
- replay-safe request and receipt digests remain part of the carry-forward story for admitted authority sessions

## Certified Support Families

- existing-truth binding: `direct_entity_identity`, `direct_relation_identity`
- existing-truth assertion modes: `retained_authoritative_assertion`, `backend_verified_assertion`
- existing-truth probe modes: `backend_verified_probe`
- existing-truth verified mutation modes: `backend_verified_update`, `backend_verified_delete`
- symbolic same-batch target reference: `same_batch_declared_target`
- symbolic same-batch aspect reference: `same_batch_declared_entity_identity`
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
