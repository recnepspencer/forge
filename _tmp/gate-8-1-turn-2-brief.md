# Gate 8.1 — Turn 2 Corrections

Your turn 1 was audited under `skills/qa-loop/SKILL.md` against a closure ledger
written before your code was inspected. Real work landed: `published_posture.rs`
is a correct sole derivation site with both contradiction cases denied, the
monolith `operation_aftermath/` is genuinely gone, and the bank's
`EstateAftermath`/`NoMutation` vocabulary is retired to residue-check strings
only. Those rows pass.

Three defects and two process failures block closure. Fix root causes. Do not
paper over any of these with an extra assertion.

Re-read `skills/qa-tests/SKILL.md` before you start — all three defects are
test-honesty failures, and D3 is the one that matters most.

## D3 (highest severity) — a fixture constructor is public production API

`worth-query-installation/src/application_aftermath/fixture_irreversible.rs`
defines `install_irreversible_aftermath`, which is exported from that crate's
`lib.rs` facade and re-exported again through
`worth-query/src/facade/exports_domain.rs`. Any consumer can call it. It mints
installed aftermath contracts from hardcoded
`CanonicalDigestId::new([0x11; 32])` and `[0x22; 32]`.

Two separate violations:

1. **Testing law 25** forbids test-only production paths, hidden constructors,
   and privileged fixture authority. Testability must come from honest
   boundaries. A fixture on the public domain facade is the exact prohibited
   shape.
2. **It hollows out your own identity evidence.** R8.19 and R8.20 claim
   classification binds to operation, schema, package, and compatibility
   generation. Any test whose world was built through this fixture carries
   *fabricated* schema and package identity, so a drift attack against that
   world proves nothing about real binding.

Required correction: remove it from production source and from both facades.
Test worlds must obtain aftermath contracts through the same
`install_application_aftermath` path production uses, with identities issued by
causal world construction rather than by constant byte arrays. If that makes
fixtures verbose, the honest fix is a test-side world builder in the test
crate — not a shortcut exported from production.

Then re-examine every test that used it: their identity-binding claims were
resting on counterfeit worlds and must be re-proved.

## D1 — R8.58 is half-proven

R8.58 requires `ProvisionalDiscard` **and `NoMutation`** unreachable from every
aftermath type. A grep of the whole test tree for `NoMutation` returns nothing.
Your compile-fail case covers only `ProvisionalDiscard`. Add the `NoMutation`
case, and cover the same surfaces you covered for `ProvisionalDiscard`.

## D2 — no positive twins

`tests/ui/application_aftermath/` contains two `.rs` files, both negative.
Testing law 20 requires each negative case to have a corresponding valid case,
and the turn-1 brief said so explicitly. Without a twin, a negative case proves
only that *something* fails to compile — not that the boundary sits where you
claim. Add a positive twin per negative case: a reversible contract that *does*
expose its undo action, and a posture value that *does* exist.

## P1 — the line-cap check was not actually run

You reported "Dirty line-cap (PowerShell equivalent; bash unavailable) | pass".
A self-written substitute for a named CI guard is not that guard. Bash is
available in this environment. Run the real one:

```
scripts/ci/check_workspace_rust_line_caps.sh dirty
```

Report its actual output. If it fails, split the files.

## P2 — trybuild artifacts

`worth-query-certification/wip/` contains stale `.stderr` copies and should not
be committed — remove it and confirm it is ignored. The committed `.stderr`
files are CRLF while trybuild emits LF, which is what produced those copies;
make the committed files match what the suite actually produces so the pass is
real rather than incidental.

I inspected both `.stderr` bodies and they do fail for the intended reason
(correct `E0599`, correct symbols), so the oracle itself is sound. Keep it that
way: never accept generated stderr without reading it.

## Verification required this turn

The disk was full during turn 1, so **none of your turn-1 verification claims
could be independently confirmed**. Build caches have now been cleared and a
full rebuild is required. Re-run and report actual output for:

- the `application_aftermath` tests in `worth-query-installation`
- the `bank-domain` estate aftermath test
- the trybuild compile-fail suite, including the new twins and `NoMutation`
- `scripts/ci/check_workspace_rust_line_caps.sh dirty`
- `cargo run --manifest-path tools/boundary-check/Cargo.toml -- --root .`
- `cargo run --manifest-path tools/agent-context/Cargo.toml -- check`

Report real command output. If something fails or you cannot run it, say so
plainly — an accurate incomplete report is correct, and a confident false one
is the worst outcome. Your turn-1 report was accurate apart from the line-cap
claim; keep that standard and fix that one.

## Unchanged boundaries

Gate 8.1 only. No `_docs/` edits. No PB1/PB2/PB4 repairs. No re-derivation
mechanism and no placeholder for one. No second aftermath lane at any point.
