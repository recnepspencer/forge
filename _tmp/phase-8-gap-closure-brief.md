# Phase 8 — Corrective Slice: Close The Recorded Gaps

Phase 8's gates are green, but four gaps were **recorded rather than fixed**.
That was my error in briefing you: on PB1 I explicitly wrote "if you judge it
out of scope, say so," which handed you an escape hatch the specification does
not permit.

§10.5's standing check is the rule: *if the answer is "a later phase will handle
it," the spec has a gap here.* Recording a defect with an owner and a deadline
is documentation, not closure. This slice closes them.

## Mandatory reading

`AGENTS.md`, `_docs/coding_guidelines/`, and §13 of
`_docs/WORTH-query/milestone-9.16-runtime-phase-8.md` (the PB statements).
Execute `skills/implementation-batch/SKILL.md`; satisfy
`skills/code-quality-qa/SKILL.md` and `skills/qa-tests/SKILL.md`.

## 1. PB1 — the `Currency` type parameter (R8.59/R8.61)

`ApplicationFieldRef` carries an eighth type parameter named `Currency`, with
`ApplicationFieldCurrency`, `NoApplicationCurrency`, and `ApplicationCurrencyRef`
alongside it. This is a **finance concept in a platform-generic signature**, on a
platform that must serve medical, CAD, and chip simulation equally.

Rename to the platform-generic concept the slot actually is: a unit-of-measure /
dimension marker. `Unit`, `ApplicationFieldUnit`, `NoApplicationUnit`,
`ApplicationUnitRef` or equivalent — pick one vocabulary and apply it
consistently.

**Scope discipline:** rename the *platform* slot. The Bank's genuine currency
usage is a domain instance of that slot and stays named for what it is. Roughly
108 files mention "Currency"; most are legitimate Bank finance vocabulary. Do
not rename those.

This must land **now**, not in Phase 9. The deadline recorded in the ledger is
"before Phase 9's facade snapshot" — a snapshot freezes the public contract, and
a name frozen there becomes a permanent migration surface for every consumer.
Doing it after the snapshot costs a migration; doing it now costs a rename.

## 2. PB2 — `ApplicationCapabilityAmountDimension`

Same defect, smaller: `ApplicationCapabilityAmountDimension` (a type alias in
`application_capability/scope.rs:165`) and the `amount` field at line 223 name a
finance magnitude in platform-generic capability vocabulary. Six files.

Rename to the platform concept — a magnitude/quantity bound. Same discipline:
the platform slot changes, domain instances keep their honest names.

## 3. PB4 — the ordinary-branch literal

Exactly **one production site**: `primary_graph/bootstrap_publication.rs:158`
constructs `BranchId("main".to_string())` while
`primary_graph/application_branch.rs` already owns
`PRIMARY_APPLICATION_BRANCH`. Use the constant.

Four test-local counterfeits also construct `BranchId("main")` directly —
`managed_run/semantic_basis.rs:150`, `managed_run/truth_read_request.rs:78`,
`primary_graph/application_query/basis/historical_authority.rs:33`,
`primary_graph/provider/idempotency.rs:238`. Route them through the same owner
so the ordinary branch has one source of truth.

Add a mechanical residue check: no `BranchId("main")` outside
`application_branch.rs`.

## 4. Q8.3 — posture construction authority

This is the deliberate carry, and it should not be carried.

Five variants of `ExternalEffectPosture` still take only `identity` and
`predecessor`: `ProviderCommit`, `EmittedApplicationCausality`,
`DispatchAttempt`, `ExternalAcknowledgement`, `ExternalCompletion`. Only
`Compensation` and `Reconciliation` require `ExternalEffectPostureEvidence`.

R8.22 says *"No posture is derivable from possession of an earlier one."*
Today that holds because the type is not exported — visibility, not
unrepresentability. Inside the crate, code holding a predecessor's causal link
can construct any successor.

Apply the treatment you already built. `ProviderCommit` is the rootless origin
and needs no predecessor evidence. The four successors —
`EmittedApplicationCausality`, `DispatchAttempt`, `ExternalAcknowledgement`,
`ExternalCompletion` — should each require evidence that the transition actually
occurred, so possession of a predecessor is not sufficient to mint a successor.

Then R8.22 reads `PROVED` on unrepresentability rather than on export policy,
and Phase 8 closes with **zero** carries.

## 5. §11 row 11 — the multi-node leak probe

Recorded as: *"handle non-leak proved; multi-node session/queue is Bank Phase
5."*

Do not accept that at face value, and do not assert it back to me. Gate 8.2
faced the same shape — the external boundary "did not exist" — and the gate
built `bank-external-rail` rather than deferring. The question is whether row 11
is genuinely the same kind of build or genuinely larger.

So: **attempt it.** Row 11 is "a crashed user node mid-recovery — no leaked
handle, session, or queue."

- The handle half is proved. Keep it.
- For session and queue: determine concretely what exists today. If the runtime
  owns session and queue resources that can be probed by dropping or killing
  their owner within a single process, build that probe now — a crashed node's
  *effect* on runtime-owned resources is testable without a second node.
- If genuinely nothing can be proved without a multi-node substrate that does
  not exist, say exactly which resource types are unreachable and why, **with
  the code that shows it** — not a category label.

An honest, evidenced boundary is acceptable. "That belongs to a later phase" as
an assertion is not.

## Ledger

Update `_docs/WORTH-query/milestone-9.16-runtime-phase-8-closure-ledger.md`:

- PB1, PB2, PB4 move to **CLOSED** in the platform-boundary table with the
  rename evidence, and R8.59/R8.61's evidence columns updated.
- Q8.3 moves to **CLOSED** if you complete item 4; the exit condition's
  "deliberate carry" section then describes zero carries and should say so.
- Row 11 updated with whatever it truthfully is after item 5.

If any item genuinely cannot close, that is a finding with evidence — not a
deferral.

## Standard

Standing verification set, every row by name, `--lib` five runs all reported.
PB1 touches a widely-used signature, so expect broad churn and confirm the three
Query consumer targets and the Bank suite hold at their current counts:
**80 / 313 / 37 / 22 / 14**.
