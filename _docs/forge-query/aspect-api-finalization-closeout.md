# Forge Query Aspect API Finalization Closeout

This closeout is the dependency contract for downstream runtime work that wants
to build on Forge Query's mutation surface before we rip JSON out of the lower
crates.

## Closed Scope

The finalized public mutation surface is safe to build against now:

- `workspace.insert(...)`
- `workspace.update(...)`
- `workspace.delete(...)`
- `workspace.batch(...)`
- `preview.insert(...)`
- `preview.update(...)`
- `preview.delete(...)`
- `preview.batch(...)`
- runtime receipts, `workspace.state(...)`, and `workspace.inspect(...)` for
  aspect-authored mutation

These are the ordinary public story. New runtime-facing code should default to
them.

## Compatibility Scope

These surfaces still exist, but they are not co-equal with the preferred API:

- `workspace.write(...)`
- `ForgeQueryWriteCommand::InsertAspects`
- `ForgeQueryWriteCommand::UpdateAspect`
- `ForgeQueryWriteCommand::UpdateAspects`
- `ForgeQueryWriteCommand::Delete`

`ForgeQueryWriteCommand::Insert` is deprecated compatibility only. It exists so
older or lower-level code can keep compiling while the public surface stays
stable, but it is not the daily-driver mutation API.

`workspace.write(...)` is intentionally kept as a stable expert compatibility
seam through the substrate rewrite. That is a maintenance boundary, not a
signal that ordinary downstream runtime code should keep building on it.

## Support-Gated Scope

These names remain part of the public vocabulary, but they are still support
gates rather than stabilized mutation families:

- `workspace.intent(...)`
- `workspace.next_effect_intent(...)`

They must keep failing typed and early until the runtime explicitly admits that
family.

## Safe To Build Now

- aspect-native authoritative CRUD through workspace.insert/update/delete/batch
- preview-local aspect-native CRUD through preview.insert/update/delete/batch
- runtime receipts, state snapshots, and inspection for aspect-authored mutation
- domain runtimes that keep async execution, store durability, and substrate ownership behind their own adapter boundary
- wasm-facing and deployed runtime APIs that compile against ForgeQueryWorkspace without depending on payload-shaped internals

## Must Not Assume Yet

- JSON has already been removed from forge-query, forge-relational, forge-store, or the runtime bridge internally
- payload-first compatibility commands are the preferred ordinary public story
- intent authority, effect-intent consumption, temporal execution, async/resource execution, or mixed-cause delivery are admitted stable mutation families
- store-backed parity, durable restart/reload, or cross-process replay semantics are closed and certified
- downstream runtimes may reach into lower-crate mutation/storage internals instead of staying on the Forge Query facade

JSON may still exist as an internal lowering adapter while the substrate
rewrite is still ahead of us. That is allowed internally. It is not the public
semantic model and it is not what downstream code should learn from.

## Migration Guidance

- author new runtime code against workspace.insert/update/delete/batch and preview.insert/update/delete/batch
- treat workspace.write(...) and ForgeQueryWriteCommand::* as compatibility or lower-level seams, not the daily-driver API
- keep workspace.write(...) available as an expert compatibility seam during the substrate rewrite, but do not require it in ordinary downstream runtime APIs
- keep mutation receipts, state snapshots, and inspect output as the downstream explanation contract
- gate intent-shaped authority crossings through support admission until that family is explicitly stabilized
- move JSON removal work underneath this facade instead of teaching new code to depend on payload lowering

## Closeout Evidence

The aspect API closeout is now a first-class runtime artifact through
`workspace.public_aspect_api_finalization_closeout()`. It is derived from the
same public support matrix, mutation compatibility report, and naming contract
that drive the executable tests.

Its self-check answers are:

- preferred public mutation DX is aspect-native
- payload-first ordinary authoring is closed off
- support-gated mutation neighbors stay fail-closed
- write-family support remains synchronized with the public matrix
- compatibility seams stay explicit rather than co-equal
- downstream runtimes may build on the facade now, while lower-crate JSON removal remains an internal rewrite

Required verification commands:

- `cargo fmt -p forge-query`
- `cargo check -p forge-query --tests`
- `cargo test --manifest-path crates/forge-query/Cargo.toml --test phase_boundaries_compile_fail`
- `cargo test -p forge-query`
- `cargo test -p forge-query runtime_public_mutation_compatibility_report_marks_payload_insert_deprecated`
- `cargo test -p forge-query runtime_public_aspect_api_finalization_closeout_answers_substrate_handoff_questions`
- `git diff --check`
