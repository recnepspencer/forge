# Forge Query Aspect API Finalization Closeout

This closeout is the dependency contract for downstream runtime work that wants
to build on Forge Query's mutation surface before we rip JSON out of the lower
crates.

## Closed Scope

The finalized public mutation surface is safe to build against now:

- `workspace.insert(...)`
- `workspace.update(...)`
- `workspace.delete(...)`
- `workspace.submissions()?.submit_batch(commands)`
- `preview.insert(...)`
- `preview.update(...)`
- `preview.delete(...)`
- `preview.batch(...)`
- runtime receipts, `workspace.state(...)`, and `workspace.inspect(...)` for
  aspect-authored mutation

These are the ordinary public story. New runtime-facing code should default to
them.

## Lower-Level Scope

These surfaces still exist, but they are not co-equal with the preferred API:

- `ForgeQueryWriteCommand::InsertAspects`
- `ForgeQueryWriteCommand::UpdateAspect`
- `ForgeQueryWriteCommand::UpdateAspects`
- `ForgeQueryWriteCommand::Delete`

Direct workspace write and batch helpers are sealed. Command-shaped mutation
flows through `workspace.write_intent(...)`, `workspace.write_batch_intent(...)`,
or the explicit `workspace.submissions()` lane.

## Support-Gated Scope

These names remain part of the public vocabulary, but they are still support
gates rather than stabilized mutation families:

- `workspace.intent(...)`
- `workspace.next_effect_intent(...)`

They must keep failing typed and early until the runtime explicitly admits that
family.

## Safe To Build Now

- aspect-native authoritative CRUD through workspace.insert/update/delete plus explicit submission batches
- preview-local aspect-native CRUD through preview.insert/update/delete/batch
- runtime receipts, state snapshots, and inspection for aspect-authored mutation
- domain runtimes that keep async execution, store durability, and substrate ownership behind their own adapter boundary
- wasm-facing and deployed runtime APIs that compile against ForgeQueryWorkspace without depending on payload-shaped internals

## Must Not Assume Yet

- JSON has already been removed from forge-query, forge-relational, forge-store, or the runtime bridge internally
- lower-level write commands are the preferred ordinary public story
- intent authority, effect-intent consumption, temporal execution, async/resource execution, or mixed-cause delivery are admitted stable mutation families
- store-backed parity, durable restart/reload, or cross-process replay semantics are closed and certified
- downstream runtimes may reach into lower-crate mutation/storage internals instead of staying on the Forge Query facade

JSON may still exist as an internal lowering adapter while the substrate
rewrite is still ahead of us. That is allowed internally. It is not the public
semantic model and it is not what downstream code should learn from.

## Migration Guidance

- author new runtime code against workspace.insert/update/delete, workspace.submissions()?.submit_batch(commands), and preview.insert/update/delete/batch
- treat ForgeQueryWriteCommand::* as lower-level command artifacts owned by explicit intent or submission lanes, not the daily-driver API
- use `workspace.public_mutation_surface_report()` when a runtime or doc needs the exact preferred-versus-lower-level-versus-support-gated mutation posture
- keep direct workspace write and batch helpers sealed; publish command-shaped mutation through workspace.write_intent(...) or workspace.submissions()
- keep mutation receipts, state snapshots, and inspect output as the downstream explanation contract
- gate intent-shaped authority crossings through support admission until that family is explicitly stabilized
- move JSON removal work underneath this facade instead of teaching new code to depend on payload lowering

## Closeout Evidence

The aspect API closeout is now a first-class runtime artifact through
`workspace.public_aspect_api_finalization_closeout()`. It is derived from the
same public support matrix, mutation surface report, and naming contract
that drive the executable tests.

Its self-check answers are:

- preferred public mutation DX is aspect-native
- support-gated mutation neighbors stay fail-closed
- write-family support remains synchronized with the public matrix
- lower-level seams stay explicit rather than co-equal
- downstream runtimes may build on the facade now, while lower-crate JSON removal remains an internal rewrite

Required verification commands:

- `cargo fmt -p forge-query`
- `cargo check -p forge-query --tests`
- `cargo test --manifest-path crates/forge-query/Cargo.toml --test phase_boundaries_compile_fail`
- `cargo test -p forge-query`
- `cargo test -p forge-query runtime_public_mutation_surface_report_lists_only_live_lower_level_command_surfaces`
- `cargo test -p forge-query runtime_public_aspect_api_finalization_closeout_answers_substrate_handoff_questions`
- `git diff --check`
