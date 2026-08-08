# Task Brief: Implement Runtime Phase 8 Gate 8.2

Gate 8.1 is closed and audited: 28 ledger rows proved, 372 consumer tests
passing, predecessors retired. You are continuing into Gate 8.2.

## Mandatory reading

Same constitution as before — re-read if your context has rolled:
`AGENTS.md`, all of `_docs/coding_guidelines/`, and
`workspaces/worth-query/crates/worth-query/docs/AI_README.md`.

Your governing specification is
`_docs/WORTH-query/milestone-9.16-runtime-phase-8.md`. For this gate, §9 Gate
8.2 is binding, and these sections constrain it directly:

- **§5 G3** — there is no external-effect boundary anywhere in this repository.
  You are building the first one. Read this section before designing anything.
- **§5 G4** — no typed provider correlation identity exists; provider session
  identities are `String` and are diagnostic-grade only.
- **§5 G6** — trusted time is authorization-scoped. This gate owns fixing it
  (it is also §12 PB3, the one platform-boundary row Phase 8 owns).
- **§5 G7** — Relational CDC is a *candidate* dispatch substrate. Evaluate it;
  do not assume it either way.
- **§6.4 / §6.5 D1** — the transactional outbox, and why it is structural
  rather than a cost optimization.
- **§8** — the counter contract. This gate adds the `external_dispatch` phase
  slot (R8.13); adding it must break every construction site until supplied.

Milestone decisions 67 and 68 in `_docs/WORTH-query/milestone-9.16.md` govern
the posture distinctions and the anchoring law.

## Mandatory skill

Execute `skills/implementation-batch/SKILL.md` — four ordered stages, boundary
review and plan before any code. Your code must satisfy
`skills/code-quality-qa/SKILL.md` and `skills/qa-tests/SKILL.md`. Do not read
or use `skills/spec-designer/SKILL.md`.

## The entry condition is currently unmet — read this carefully

Gate 8.2's entry requires **a real controllable external boundary in the Bank
world, in its own process**. It does not exist: `worth-query-bank-world/crates/`
contains only courtroom, domain, estate-certification, http-adapter, and server.

Building it is therefore inside this gate's causal closure. Ownership split,
from §5 G3 — do not blur it:

- **Query owns** the installed external-effect contract, the typed posture
  ladder, correlation identity, and the rule that no posture is derivable from
  possession of an earlier one.
- **The Bank world owns** the real external service, its faults, and its
  transport. It is a separate process reachable over a real network boundary —
  the same standard Bank Phase 5 applies to user nodes.
- **No lower runtime owns** effect dispatch. Relational commits truth; it does
  not call the world.

### The failure mode I will be auditing for above all others

The exit proof states plainly: *"An in-process fake sharing the runtime's truth
source does not close this gate."*

Under pressure to finish, the tempting move is a fault-injecting test double in
the same process that reports timeouts and duplicate acknowledgements without a
real boundary. **That does not close Gate 8.2 and I will find it.** The whole
point of this gate is that the authoritative answer lives somewhere the runtime
cannot interrogate synchronously; a same-process fake shares the runtime's truth
source and therefore proves nothing about indeterminacy.

**If you cannot finish both halves in this turn, stop after the Query-side
authority and say so.** An honest report of "R8.22-R8.27 built, external
boundary process not yet built, gate not closeable" is a correct outcome. A
gate reported closed on an in-process double is the worst outcome available to
you and will be rejected.

## What Gate 8.2 must establish

R8.22 through R8.27, plus the gap resolutions this gate owns:

- **R8.4 / R8.25 / R8.55** — dispatch intent co-committed with the mutation.
  Precedent: `provider/idempotency.rs` already writes a Query-owned entity into
  the operation's own `MutationIntent`. Operations declaring no external effect
  must pay exactly zero — prove it.
- **R8.5** — correlation binds to typed Query identity, never to a
  provider-supplied `String`. Provider strings may appear in diagnostics and
  may not appear in any equality that decides whether a transition is admitted.
- **R8.7** — one time source. Consume the existing host-published source; if
  that requires generalizing the authorization-scoped owner's name and
  visibility, that rename is part of this gate. Do not fork it. Callers and
  transport adapters still cannot supply a sample or choose the evaluation
  moment. Every expiry decision records its exact sample in the decision facts.
- **R8.8** — if you use Relational CDC as the delivery mechanism, say so
  explicitly and prove no CDC subscriber makes an application authority or
  disclosure decision, and that a CDC checkpoint cannot be readmitted as a
  Query dispatch posture. If you do not use it, do not build a second change
  stream over Relational either.
- **R8.26** — `Indeterminate` and `PartialEffect` currently discard the
  correlation evidence the layer beneath them already produced. The provider
  layer has `WorthQueryProviderCompareAndCommitOutcome::Indeterminate(failure)`
  and the session protocol distinguishes `CommitRecoveryRequired` from
  `AbortRecoveryRequired`. Carry that up; do not re-derive it.

## Hard boundaries

- Gate 8.2 only. No recovery handle (8.3), no undo (8.4), no redo (8.5).
- No `_docs/` edits. If the spec is wrong, stop and report the conflict.
- Do not repair PB1, PB2, or PB4. PB3 (the time source) is this gate's.
- No re-derivation mechanism and no placeholder for one.
- 400-line cap; split touched oversized files.

## Verification

Real output required, not descriptions:

- focused owner tests for every package changed
- `scripts/ci/check_workspace_rust_line_caps.sh dirty` — the real script; bash
  is available
- `cargo run --manifest-path tools/boundary-check/Cargo.toml -- --root .`
- `cargo run --manifest-path tools/agent-context/Cargo.toml -- check`
- the three `worth-query` consumer targets must stay green:
  `installed_operating_world` (313), `public_declarative_journeys` (37),
  `runtime_public_journeys` (22)

Your turn-2 report on Gate 8.1 was accurate throughout. Hold that standard.
