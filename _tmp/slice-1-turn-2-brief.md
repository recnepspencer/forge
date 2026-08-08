# Slice 1, Turn 2 — Close The Three Remaining Doors

The keystone is correct. `resolve_operation_aftermath` in `installed.rs` derives
coverage from the operation's own `decision_reads` and the catalog from
`schema.binding_identity()`, `install_application_aftermath` is genuinely
`pub(crate)`, and the compile-fail proves a real privacy error. That part is
done and must not be disturbed.

Three findings. All three are the same defect the slice exists to remove.

## S1-F1 (High) — the coverage type was renamed, not retired

Your report says **"Retired: `AftermathDeclaredReadCoverage`."** It was not
retired. It was renamed to `OperationDeclaredReadFields`, and:

- `from_field_slots` is still `pub`
- it is exported from `worth-query-installation/src/lib.rs:113`
- it is re-exported through the **top-level facade**,
  `worth-query/src/facade/exports_domain.rs:51`
- there are **44 call sites**, most passing caller-authored literals, including
  `from_field_slots(Vec::<String>::new())`
- `bank-domain/tests/estate_aftermath_contract.rs:16-26` still authors coverage
  by hand and still falls through to an empty list

The doc comment reads *"Coverage … is taken from these slots — never from a
caller-authored coverage list at application-operation compile."* That sentence
is true of the application path and false of the type. **A comment asserting a
property the type does not enforce is how the last four defects hid.**

Fix: the type carries the derivation, not a public constructor. Make
`from_field_slots` `pub(crate)` — or better, delete it and have the type built
only by the resolution site from `decision_reads`. Remove it from both export
surfaces. Every test-side call site must obtain coverage the way production does:
from an installed operation. A fixture that hand-authors coverage is the Q8.1
defect returning.

## S1-F2 (High, R8.0) — a second public aftermath install door

`install_domain_operation_aftermath` is exported through
`worth-query/src/facade/exports_domain.rs:49` and accepts the operation slot, a
canonical-identity string, the declared contract, read field slots, and the
lowering catalog — every one caller-supplied.

You reported this honestly under "out-of-scope defects noticed," and it is not
out of scope. Your brief quoted the governing rule verbatim:

> **A second independently reachable aftermath installation lane is unlawful at
> any point, including transiently.**

That is R8.0, and it is the rule this whole correction exists to enforce. Two
doors into aftermath installation — one derived, one caller-authored — is
precisely the condition Q8.18 describes.

Fix: domain operations compile aftermath the same way application operations now
do, from their installed declaration. If the domain declaration genuinely cannot
carry an aftermath member yet, then the door closes and domain fixtures go
without aftermath until it can — they do not keep a caller-authored installer
alive for convenience. Removing capability is the correct outcome when the
honest path does not exist yet.

## S1-F3 (High) — silent declaration/installation divergence

`operation_program_installation/aftermath.rs::bind` returns the schema unchanged
for `DelegateCapability` and `RequestEmergencyAccess`:

```rust
if matches!(capability,
    EstateCapabilityOperation::DelegateCapability
    | EstateCapabilityOperation::RequestEmergencyAccess) {
    return schema;
}
```

But `declared_aftermath_for` still returns `Some(RecordedInverse { … ExactPriorTruth … })`
for both. So the domain declares a correction, installation silently drops it,
and the installed operation reports `aftermath: None` — **not correctable**, with
nothing anywhere recording that a declared correction was discarded.

This is a *new* instance of the defect class, created while fixing another. It is
worse than the state it replaced: before, the correction was declared and
non-functional; now it is declared and invisible.

R8.16 requires that missing or host-authored aftermath be **rejected at
installation**. A silent skip is not a rejection.

Fix — pick one and state which:

1. **Reject.** Installation fails when an operation's declaration carries an
   aftermath contract whose pre-image demand its declared reads cannot cover.
   The Bank then either adds the covering reads or changes the declaration. This
   is the honest option and matches R8.16.
2. **Change the declaration.** If those two genuinely cannot support
   `RecordedInverse` — because activation/request lanes are framework-owned —
   then `declared_aftermath_for` must stop claiming they can, and say what they
   are instead.

What is not acceptable is a declaration and an installation that disagree with
no one noticing.

## Required test

Beyond the fixes: **an operation whose declared aftermath is not covered by its
declared reads must fail installation, by name.** Positive twin: a covered one
installs. This is R8.2's actual guarantee and it still has no adversarial test
that performs the substitution.

## Out of scope

Unchanged. Do not touch the handle binding, recovery authority, undo/redo
progression, lineage, transport payload, or pre-image retention. Slices 2–10.

## Verification

Full standing set from correction plan §5, every target with its scope, five
`--lib` runs all reported, `cargo fmt --all --check` both workspaces, exit codes
captured.

## Reporting

State plainly, for each of the three: what you changed, and **what a caller can
still supply**. If you retire something, verify it is gone — `rg` for the symbol
and report the count, rather than reporting "retired." Your last report said a
type was retired while 44 call sites were using it under a new name.
