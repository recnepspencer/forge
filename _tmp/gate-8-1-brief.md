# Task Brief: Implement Runtime Phase 8 Gate 8.1

You are implementing one gate of a specified milestone phase in the WORTH
platform. Read this brief fully before doing anything else.

## Mandatory reading order

Read these before you write any code. They are not optional context.

1. `AGENTS.md` — the engineering constitution and hard invariants.
2. `_docs/coding_guidelines/` — read every file. `MENTALITY.md`,
   `arch_laws.md`, `composition_laws.md`, `domain_structure_laws.md`,
   `dx_laws.md`, `perf_laws.md`, `testing_laws.md`.
3. `workspaces/worth-query/crates/worth-query/docs/AI_README.md` — the runtime
   authority model.
4. `_docs/WORTH-query/milestone-9.16-runtime-phase-8.md` — your governing
   specification. Sections §4.2, §4.3, §6.1, §7, §8, and §9 Gate 8.1 are
   directly binding on this work. §5 and §12 constrain what you may not do.
5. `_docs/WORTH-query/milestone-9.16.md` — the parent milestone. Product
   Decision Lock decisions 61 and 68 govern the aftermath model.

## Mandatory skill

You must follow `skills/implementation-batch/SKILL.md`. Read it and execute its
four ordered stages in order:

1. Select the slice — already selected for you: **Gate 8.1 only**.
2. **Boundary review** — produce the boundary brief before planning.
3. **Implementation plan** — write it before editing any code.
4. **Implement and verify.**

Do not edit code before stages 2 and 3 are complete and stated in your output.

Your code must also satisfy `skills/code-quality-qa/SKILL.md` (composition and
domain topology, the 400-line file cap) and `skills/qa-tests/SKILL.md` (test
and fixture honesty) as you write. Read both. Do **not** read or use
`skills/spec-designer/SKILL.md` — the specification is already written and is
not yours to change.

## What Gate 8.1 must establish

Your governing text is §9 Gate 8.1 of the phase-8 spec. The requirements are
R8.16, R8.17, R8.18, R8.19, R8.20, R8.21, R8.57, R8.58. Additionally binding:

- **R8.0** — the reconciliation policy. An aftermath authority already exists
  at `workspaces/worth-query/crates/worth-query/src/domain_installation/operation_aftermath/`
  and a seven-variant declaration contract exists at
  `worth-query-installation/src/domain_operation/semantic_contracts.rs:258`.
  You must preserve the existing path as sole authority until the destination
  proves parity, then cut every covered consumer over atomically and retire
  exactly the predecessor in the same slice. **A second independently
  reachable aftermath classification, admission, or denial lane is not lawful
  at any point, including transiently.**
- **R8.52 / R8.53** — the two-axis model. Correction authority and correction
  mechanism are separate typed contracts; the four published posture names are
  **derived**, never declared. Populate the mechanism axis with exactly
  `RecordedInverse` and `Compensation`. Do **not** create a re-derivation
  mechanism, and do **not** create an empty placeholder for one.
- **R8.59 / R8.60** — do not widen the existing unit slot (see §12 PB1); mint
  no aftermath-local unit/measure/amount/currency vocabulary; introduce no new
  stringly semantic family; introduce no new ordinary-branch string literal;
  construct no `BranchId` in tests — test worlds receive branch identity from
  world construction.

Destination topology is §7 of the spec. Follow it exactly, including the
`correction_mechanism/` directory shape and `published_posture.rs` as the sole
site where the four law-14 names are produced.

## Hard boundaries

- **Gate 8.1 only.** Do not implement Gates 8.2–8.6. No recovery handle, no
  external-effect dispatch, no undo, no redo, no lineage.
- Do not repair the §12 platform-boundary defects PB1, PB2, PB4. They are
  routed to later phases. Recording them is not your job either.
- Do not modify any file under `_docs/`. The specification is authoritative
  and is not yours to edit. If you believe the spec is wrong or incomplete,
  **stop and report the conflict** instead of working around it or amending it.
- Do not weaken a type, widen an API, add a fallback, or add a compatibility
  shim to make something compile. If the honest path is blocked, report the
  blocker.
- Respect the 400-line file cap. Split touched oversized files.

## Verification

Run at minimum:

```
cargo run --manifest-path tools/boundary-check/Cargo.toml -- --root .
cargo run --manifest-path tools/agent-context/Cargo.toml -- check
```

Plus the focused owner tests for every package you changed, formatting, and the
line-cap guard in `dirty` scope
(`scripts/ci/check_workspace_rust_line_caps.sh`). Run compile-fail evidence
where you established a type or construction boundary — R8.21 and R8.58 are
type-level absence claims and need negative cases with positive twins.

## Reporting

End your turn with the report `implementation-batch` Stage 4 requires: the
boundary reviewed, the slice built, material files changed, competing-authority
paths removed, verification results including anything you did not run, and
remaining work. Be explicit and accurate about what you did not finish or could
not prove. An honest incomplete report is correct; a confident false one is the
worst possible outcome.

You will be reviewed after this turn by an auditor following
`skills/qa-loop/SKILL.md`, which attempts to falsify both your implementation
and your report.
