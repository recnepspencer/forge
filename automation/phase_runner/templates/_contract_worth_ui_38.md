---

# Worth UI Milestone 3.8 operating contract

There is no human in this loop. Approval policy is `never` and the sandbox is
full-access.

The durable runner contract is:

- static config is read-only
- the event log is authoritative
- the projection file is derived status for reading, not editing
- chat is the artifact of record for plans, findings, explanations, and command
  summaries

Never edit the config file, event log, or projection file directly from a turn.
The only state transition you may make is the final `RUNNER_EVENT:` marker that
the orchestrator will validate and append.

## Load before acting

Reason from real sources, never from a phase title alone. Read the spec, this
phase's scoped paths, relevant public APIs, and the project context:

{project.context_files}

Read `MENTALITY.md` and `arch_laws.md` with special attention every turn. Read
`dx_laws.md` whenever planning or changing a public caller experience.

## Milestone 3.8 authority and churn rails

Preserve this authority chain:

`admitted measurement basis + admitted allocation neighborhood -> allocation
plan -> committed allocation receipt/report and replan transaction -> host
consumption and inspection`

- Host mechanics, preview state, renderer scans, helper caches, and
  certification fixtures may not mint allocation truth.
- Keep candidate and preview allocation distinct from committed receipt truth.
  Freshness, lag, and denial live in report lineage, not receipt mutation.
- Use typed invalidation and neighborhood-local recompute. Do not substitute
  root/page-wide fallback when locality cannot be proved.
- Extend admitted graph, Query-consumption, evidence, and inspection lanes;
  do not recreate their truth locally.
- Preserve deterministic replay for admitted inputs and make typed denial the
  ordinary outcome for incompatible generation, policy, posture, or locality.

## Bias toward action and honest cutovers

This runner exists to finish the milestone. Once the real seam is clear, move
to code. Replace a dishonest ordinary path with the owning production seam;
do not add a helper, shim, compatibility bridge, or certification-only detour.
For parallel-lane migration, finish the mechanical cutover before broad proof.
Compiler errors, imports, and type-boundary failures are the guide until the
new lane owns ordinary behavior.

## Proof and payload discipline

The runner sends exactly the turn named by `current`: {turns}.

During iteration, use the narrowest relevant proof: `cargo check`,
`cargo test --no-run`, touched module/integration tests, or named compile-fail
fences. Run broad suites only when phase acceptance explicitly requires them.

Keep logs, long plans, findings, and command output in chat. Runner payload
notes are compact pointers only.

## Event discipline

End with exactly one compact JSON marker. Its event type must match the turn:

- `plan` -> `plan_posted`
- `implement` -> `implementation_completed`
- `review` -> `review_failed` or `review_passed`
- `repair` -> `repair_completed`
- `test_review` -> `test_review_failed` or `test_review_passed`
- `test_repair_plan` -> `test_repair_plan_posted`
- `test_repair_implement` -> `test_repair_completed`
- `code_quality_review` -> `code_quality_review_failed` or
  `code_quality_review_passed`
- `code_quality_repair` -> `code_quality_repair_completed`

If a turn includes a runner turn-instance id requirement, echo it exactly. If a
recovery turn says prior work completed, reconstruct the honest outcome rather
than repeating the work.
