# Slice 2: Adopt `worth-proof` For Recovery Authority

**Narrow slice. Target diff ~250 lines.** If your plan exceeds that, say so and
propose a split before writing code.

## Read this first, before anything else

**`crates/worth-proof/README.md`.** Read it in full. Then read the working
example in `workspaces/worth-query/crates/worth-query/src/application/declaration_progression/`
— particularly `recipe.rs`, `admitted.rs`, and `rebind.rs`.

This is not background. It is the substance of the slice.

Then: `AGENTS.md`; `_docs/coding_guidelines/` (all files);
`_docs/WORTH-query/milestone-9.16-runtime-phase-8-correction-plan.md` **§1b and
§1a**; the closure ledger's **Q8.20**. Skills:
`skills/implementation-batch/SKILL.md`, `skills/code-quality-qa/SKILL.md`,
`skills/qa-tests/SKILL.md`. Not `spec-designer`.

## Why this slice exists

`worth-query-execution` **does not depend on `worth-proof`.** Zero uses in the
aftermath surface. Its sibling crates — `worth-query-installation`,
`worth-query`, `worth-query-publication`, `worth-query-certification` — all do,
and the declaration lane next door is built on it: `Recipe`, `Unresolved`,
`RecipeStageKind`, `AssumptionBasis`, `AuthorityWitness::from_authority_marker`,
and 109 uses of `TransitionOutcome`.

Phase 8 hand-rolled a weaker parallel substrate. Fifteen types in
`application_aftermath/` use private-fields-plus-`pub(crate) mint` to
approximate what `Proof<P, A>` and `AuthorityWitness<A>` already provide,
sealed.

**Fifteen hand-rolled sealed types is the signal that a substrate exists and is
not being used.**

## The defect (Q8.20)

```rust
pub struct WorthQueryRecoveryEffectAuthority {
    handle_slot: WorthQueryRecoveryRegistrySlot,
    _private: (),
}
// ensure_for: if handle.registry_slot() != self.handle_slot { deny }
```

`RecoveryRegistryState::new` starts `next_slot: 1` in **every** runtime.
Authority minted against runtime A's slot 1 satisfies runtime B's slot-1
handle. The check is a `u64` comparison against a value that is not unique.

No adversarial test attacks this.

## Build this

**1. Add `worth-proof` to `worth-query-execution`'s `Cargo.toml`.**

Check `tools/boundary-check` first — the crate DAG is snapshotted, and adding a
dependency may need the snapshot updated. If `boundary-check` denies this edge,
**stop and report it** rather than working around it. That tool is the authority
on topology.

**2. Read this before choosing a surface — it is the trap in this slice.**

`AuthorityWitness` and `Proof` are **zero-sized**:

```rust
pub struct AuthorityWitness<A>(PhantomData<A>) where A: AuthorityMarker;
pub struct Proof<P, A>(PhantomData<(P, A)>);
pub fn from_authority_marker(_marker: A) -> Self { Self(PhantomData) }  // pub!
```

Two consequences, both fatal to the naive fix:

- **A ZST cannot distinguish runtime A from runtime B.** Replacing
  `handle_slot: u64` with `AuthorityWitness<RecoveryMarker>` would *delete the
  only check that exists* and make Q8.20 worse while looking like adoption.
- **`from_authority_marker` is `pub` and discards its argument.** Any caller who
  can name the marker type mints the witness. That is the caller-supplied
  evidence defect one layer up. The README says so directly: *"a caller-selected
  `AuthorityMarker` cannot open a governed runtime door."*

A witness proves **a lane exists**. It is not identity. Do not use it as identity.

**3. Carry runtime identity as a value, in `AssumptionBasis`.**

`AssumptionBasis<B> { value: B }` (`crates/worth-proof/src/assumption/basis.rs`)
is the value-carrying surface, and it is what the declaration lane uses.

Files you will be editing — no guessing:

| File | Change |
|---|---|
| `worth-query-execution/src/domain_computation/application_aftermath/recovery_progression/fresh_authority.rs` | `WorthQueryRecoveryEffectAuthority`, `WorthQueryRecoveryInspectAuthority`, `ensure_for`, `admit_recovery_effect_authority`, `admit_recovery_inspect_authority` |
| `worth-query-execution/src/domain_computation/managed_run/recovery_registry.rs` | slot allocation (`next_slot: 1` per instance — the defect's root) |
| `worth-query-execution/src/domain_computation/application_aftermath/recovery_handle/handle.rs` | handle-side identity, if the binding needs to carry it |
| `worth-query-execution/Cargo.toml` | the new dependency |

The authority must carry a basis that is unique **per runtime instance**, not
per registry slot. The runtime already has identity available —
`self.runtime.authority_identity()` and
`receipt.provider_runtime_instance_id()` are both in scope at the mint site in
`fresh_authority.rs`. Use them; do not invent a new identity scheme.

**Minting stays owner-private.** `AssumptionBasis::new` is `pub`, so the
owner-specific type must wrap it and keep its own constructor `pub(crate)` —
exactly the README's **Owner-Specific Runtime Types** pattern: Proof law
underneath, owner type on top, Query facade outward. If a consumer can build the
basis, you have moved the defect rather than fixed it.

**4. Evaluate the stronger option and report on it, even if you do not take it.**

A value comparison is rung 3 on the plan's §1a ladder. Rung 1 would be a
*brand*: making the runtime instance a type parameter so cross-runtime use fails
to compile rather than being denied at runtime. Consider it, say whether it is
feasible here, and if you reject it say why. A rung-3 fix is acceptable for this
slice **if** you state that rung 1 was considered and why it was not taken.

**5. Retire `ensure_for`'s bare slot comparison** once the basis subsumes it. Do
not keep both — a check that can no longer fail reads as protection while
providing none. Follow **R8.0**: no second independently reachable authority
lane at any point, including transiently.

**4. Drop `_private: ()`** from these types. The fields are already private;
struct-literal construction from outside the module is already impossible. It is
ceremony, and it was removed from `dispose.rs` in an earlier pass for the same
reason.

## Required adversarial test

The one that does not exist and is the whole point:

> **Two runtimes, each with a live recovery handle at registry slot 1.**
> Authority minted from runtime A must not admit any transition on runtime B's
> handle. Positive twin: authority from A admits A's handle.

If your change makes that test *impossible to write* because the code no longer
compiles, that is the best outcome — say so, and leave a compile-fail case in
`worth-query-certification` proving it, expecting a **type error**, not an arity
error.

**Second required test, from the trap above:** a consumer outside
`worth-query-execution` must not be able to construct the authority. Compile-fail
expecting a **privacy error**. If your design lets a consumer reach
`AuthorityWitness::from_authority_marker` or `AssumptionBasis::new` for these
types, that test will fail and the slice is not done.

Do not write a test whose assertion the compiler already guarantees.

## Out of scope

Do not touch: the `recovery_target` / `aftermath` parameters (slice 3), handle
typestate and freshness (slice 4), linearity (slice 5), undo/redo progression,
lineage, transport payload, retention. Name anything you notice; fix nothing.

## Verification

Standing set from correction plan §5, every target **with its scope**, five
`--lib` runs all reported, `cargo fmt --all --check` in both workspaces, exit
codes captured — do not infer success from empty output.

Note the composition guards now run at edit time and in `cargo test`. Advisories
are not blockers, but a new 5+ parameter function on a privileged path is
exactly the smell this correction exists to remove.

No `#[allow(...)]`.

## Reporting

Say plainly:

1. What you changed.
2. **Which `worth-proof` surfaces you used, and which you deliberately did not.**
3. **What a caller can still supply** to these paths.
4. Which §1a rung each guarantee lands on.

If you hand-rolled anything `worth-proof` already provides, justify it in
writing against the README's **Owner-Specific Runtime Types** section. "It was
simpler" is not a justification.

Your entire diff will be read line by line. Your last report was mangled by a
connection retry and said nothing about the work — the code was fine, but a
report that does not address the brief is worth nothing. If the connection
drops, re-state the report.
