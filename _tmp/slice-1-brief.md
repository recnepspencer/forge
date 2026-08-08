# Slice 1: Aftermath Becomes A Compiled Operation Contract Field

**This is a narrow slice. Do exactly this and stop.** Target diff: 400–600 lines.
If your plan exceeds that, say so and propose a split before writing code.

## Reading

1. `AGENTS.md`; `_docs/coding_guidelines/` (all files).
2. `_docs/WORTH-query/milestone-9.16-runtime-phase-8-correction-plan.md` — §1 is
   the principle this slice applies. Read it first; the rest of the plan is not
   yours this turn.
3. Closure ledger § Reopening → **Q8.18**. That finding is your whole scope.
4. `skills/implementation-batch/SKILL.md`, `skills/code-quality-qa/SKILL.md`,
   `skills/qa-tests/SKILL.md`. Not `spec-designer`.

## The finding

`WorthQueryCompiledApplicationOperationContracts`
(`worth-query-installation/src/application_operation/contracts.rs:43`) compiles
authorization, ability requirements, graph reads, touches, effects, invariants,
decision facts, resources, program, budgets, mutation preconditions, execution
posture, and external effect — **and no aftermath.**

Aftermath is installed through a separate free function
(`application_aftermath/install.rs:117`):

```rust
pub fn install_application_aftermath(
    package_identity: &CanonicalDigestId,
    schema_or_domain_identity: &CanonicalDigestId,
    operation_slot: &str,
    compatibility_generation: u64,
    declared: &DeclaredApplicationAftermathContract,
    declared_reads: &AftermathDeclaredReadCoverage,
    lowering_catalog: &AftermathLoweringCorrespondenceCatalog,
) -> Result<WorthQueryInstalledAftermathContract, _>
```

Every identity is caller-supplied. Worse, `validate_preimage_coverage(declared,
declared_reads)` validates the recorded-inverse pre-image demand against
**`declared_reads`, which the caller wrote** — not against the operation's
installed `graph_reads`. R8.2's guarantee ("installation rejects a demand not
covered by declared reads") is therefore vacuous: a caller passing an empty
coverage list passes validation. Bank tests do exactly that:

```rust
install_application_aftermath(
    &CanonicalDigestId::new([0x71; 32]),
    &CanonicalDigestId::new([0x72; 32]),
    "DisburseEstate",
    generation,
    &declared,
    &AftermathDeclaredReadCoverage::new(Vec::<String>::new()),
    &AftermathLoweringCorrespondenceCatalog::empty(),
)
```

## Build this

**1. Aftermath compiles with the operation.**

Add an `aftermath` field to `WorthQueryCompiledApplicationOperationContracts`
and to `WorthQueryApplicationOperationContractSources`, resolved the way its
siblings are — via an `operation_aftermath(schema.installed_declaration()
.members(), operation)` lookup beside the existing `operation_external_effect`
call in `installed.rs`. The declaration already carries the contract
(`bank_domain::estate::aftermath::declared_aftermath_for` is the Bank side);
installation must read it from the installed declaration, not receive it.

**2. Identities are derived, never passed.**

Package identity, schema identity, and operation slot are all already in scope
at the compile site (`schema.binding_identity()`, the `operation` value). The
installed aftermath contract's canonical basis must be prepared from those, so
no caller can name a different operation than the one being installed.

**3. Coverage comes from the operation's own reads.**

`validate_preimage_coverage` takes the operation's compiled `graph_reads`
instead of a caller-supplied list. A `RecordedInverse` pre-image demand naming a
field the operation does not read must fail installation. This is R8.2's actual
requirement and it currently has no enforcement.

Delete `AftermathDeclaredReadCoverage` if nothing else needs it. Do not keep it
as an optional override.

**4. Retire the free function's public surface.**

`install_application_aftermath` should stop being the public installation door.
If an internal form is still needed for compilation, make it `pub(crate)` and
drive it from the operation compile path. Follow **R8.0**: preserve the existing
path until the destination proves parity, then cut every consumer over
atomically and retire the predecessor in the same slice. **A second
independently reachable aftermath installation lane is unlawful at any point,
including transiently.**

**5. Consumers.**

Bank and test worlds obtain the installed aftermath from the installed
operation, not by calling an installer with literals. Expect to touch the
`install_notify_death_aftermath` / `install_disburse_aftermath` helpers in
`bank-server/tests/.../phase8_cross_gate.rs` and `phase8_redo_support.rs`. Test
worlds construct through production installation (**R8.65**) — a fixture that
fabricates a digest is the Q8.1 defect returning.

## Out of scope this turn

Do not touch: the handle binding, recovery authority, undo/redo progression,
lineage, the transport payload, or pre-image *retention*. Those are slices 2–10
and each has its own turn. You may **not** fix a defect you notice in them; name
it in your report instead.

## Adversarial tests — required, not optional

The uniform failure across all ten findings is that no test attacked the
substitution. This slice is not done without tests that attempt it:

1. **A pre-image demand naming a field the operation does not read is rejected
   at installation.** Positive twin: a demand covered by the operation's reads
   installs.
2. **The installed aftermath identity is derived from the operation being
   installed.** Two operations with identical declared aftermath but different
   slots produce different installed identities.
3. **Compile-fail:** no public path constructs an installed aftermath contract
   from caller-supplied identities. Expect a privacy error (`E0603`/`E0624`), not
   an arity error — an arity error proves only that a parameter moved.

Do not write a test whose assertion the compiler already guarantees.

## Verification

The full standing set from correction plan §5, **every target by name, with its
scope**, five `--lib` runs all reported, and `cargo fmt --all --check` in both
workspaces. Capture exit codes; do not infer success from empty output.

No `#[allow(...)]` in new or touched code.

## Reporting

Report what you built, what you retired, what you could not prove, and — the row
that matters most for this plan — **what a caller can still supply to this path
after your change.** If the answer is anything beyond the authority object and
the target identity, name it and justify it.

Your entire diff will be read line by line. An honest incomplete report is
correct; a confident false one is the worst possible outcome.
