---
name: milestone-executor
description: Orchestrate a single milestone through phased implementation, Codex delegation, hostile QA, escalation, and closeout. Use when OpenClaw should drive a multi-phase implementation against an existing spec while enforcing Forge-specific standards rather than generic MVP execution habits.
---

# Milestone Executor

Use this skill to run one milestone end to end.

The implementation spec is binding. The governing documents are binding:
- `C:\Users\Esther\Documents\Programming\forge_workspace\forge\_docs\coding_guidelines\MENTALITY.md`
- `C:\Users\Esther\Documents\Programming\forge_workspace\forge\_docs\coding_guidelines\architectural_guidelines.md`
- `C:\Users\Esther\Documents\Programming\forge_workspace\forge\_docs\coding_guidelines\domain_standards.md`
- `C:\Users\Esther\Documents\Programming\forge_workspace\forge\_docs\coding_guidelines\performance_guidelines.md`

Work to an aerospace-grade, chip-simulator-grade standard. Do not optimize for MVP closure, cosmetic completeness, or local success that leaves structural weakness behind.

## Boundary of responsibility

This skill is for OpenClaw, not for Codex.

OpenClaw owns:
- phase sequencing
- deciding when to invoke other skills
- deciding when QA happens
- deciding when QA ends
- deciding when a phase starts
- deciding when a phase ends
- deciding when milestone-level work advances
- deciding when to report to the user
- deciding when escalation is required
- deciding whether Codex should keep going, explain itself, recenter, or address findings
- synthesizing all user-facing progress reports
- maintaining the milestone's single ongoing Codex execution session

Codex owns:
- implementation
- answering targeted questions about its reasoning
- addressing concrete QA findings
- writing the requested closeout document

Codex must never be told that these skills exist. These skills are management instructions for the orchestrator only.

Do not tell Codex to "use" workspace skills. Do not mention skill names in Codex-facing prompts. Do not explain the skill system, the orchestration layer, or OpenClaw's internal workflow to Codex.

Codex should only be told what it needs for the current engineering task.

## Boundary invariants

- OpenClaw decides all phase transitions.
- OpenClaw decides when QA begins and when QA is satisfied.
- OpenClaw owns all user-facing progress reports and milestone summaries.
- OpenClaw owns all escalation to the user.
- Codex is never asked to manage the orchestration layer.
- Codex is never asked to choose whether to move to the next phase.
- Codex is never told about the existence of these skills.
- OpenClaw must maintain one continuous Codex session for the entire milestone.
- OpenClaw must keep Codex attached to the same persistent terminal window/session across phase work, QA fix loops, and milestone completion.
- Do not create a fresh Codex process, terminal, or session per phase.
- Only reset or replace the Codex session if there is a real failure such as crash, terminal corruption, unrecoverable stuck state, or explicit user instruction.

## Codex session policy

Treat the Codex terminal as a persistent working environment, not a disposable request runner.

The milestone should normally use:
- one Codex session
- one persistent terminal window/session
- many messages over that same running context

Do not churn terminals between phases. Do not spawn a new Codex process just because the workflow advanced to a new stage. Preserve continuity unless there is a concrete technical reason not to.

## Codex tooling rule

For orchestrated milestone work, do not use `codex exec`.

`codex exec` is a one-shot surface and is not acceptable for this workflow. It encourages fresh-process churn, broken continuity, and passive "fire once and wait" behavior.

Use only regular Codex in a persistent PTY-backed terminal session for milestone work. The orchestrator must continue talking to that same live Codex session throughout the milestone.

## Loop-driving rule

Codex will not complete the orchestration loop on its own from one launch prompt. OpenClaw must actively drive the loop.

OpenClaw must:
- poll the live Codex session
- read the completed batch carefully
- immediately decide the next step
- immediately send the next prompt into the same live session when more work is needed
- repeat until the phase is actually closed

Do not confuse "Codex is running" with "the loop is being driven correctly."

## Progress reporting rule

OpenClaw must report progress proactively without waiting to be asked.

Send the user an update immediately when:
- a Codex loop finishes
- a new Codex loop starts
- a blocker appears
- a phase closes

Each automatic update should include:
- what changed
- whether the phase is actually clean yet
- what next loop was started, if any

## Prompt fidelity

This skill contains canonical prompts and adaptable prompts.

For canonical prompts:
- preserve the wording exactly except for slot substitution such as phase id, spec path, or concrete findings
- do not summarize
- do not embellish
- do not add orchestration commentary
- do not add motivational framing unless the canonical prompt already contains it

For adaptable prompts:
- preserve the intent and constraints
- minor wording changes are acceptable
- keep them brief

## Operating stance

- Treat Codex as capable. Give it standards, constraints, and direction, not babying.
- Existing code is evidence, not authority. The spec and governing docs are authority.
- Ask Codex for explanation before escalating to the user.
- Escalate to the user only for non-trivial architectural concerns, conflicting governing authority, or meaningful scope-shaping decisions.
- After each phase, send the user a concise report, but do not stop unless escalation is required.

## Sequence

1. Read the implementation spec and the four governing documents.
2. Enter Phase 1 and hand execution to `phase-executor`.
3. When implementation for the phase appears substantively complete, invoke `phase-qa`.
4. If phase QA reports findings, send them back to Codex and loop phase execution plus QA until there are no findings.
5. Report phase completion to the user and move to the next phase.
6. After all phases are complete, run a whole-milestone hostile QA.
7. Invoke `test-auditor` only after the entire milestone is implemented.
8. If test audit reports findings, send them back to Codex and loop until there are no meaningful gaps.
9. Invoke `architecture-closeout`.
10. If final architecture QA reports findings, send them back to Codex and loop until the structure is clean.
11. Ask Codex to write the milestone closeout document and review it for honesty and completeness.

## Forbidden patterns

Never send Codex prompts like:
- "Use the milestone-executor skill"
- "Run phase-qa when you are done"
- "Notify the orchestrator"
- "Call `openclaw system event`"
- "Decide whether this phase is done and move to the next one"
- "Start a new terminal/process/session for this phase"
- "I am opening a fresh Codex session for the next phase"
- "Use `codex exec` for this milestone pass"
- explanations about OpenClaw internals, orchestration, or skill routing

OpenClaw should internalize those responsibilities and send Codex only the substantive engineering directive.

## Phase completion rule

A phase is complete only when:
- phase-spec requirements are implemented
- architecture still reflects the governing shape
- relevant domain invariants are enforced
- relevant performance constraints are respected or explicitly named as remaining debt
- phase QA reports no findings
- no obvious in-scope cleanup remains

Do not run a personal OpenClaw test audit at phase boundaries. Test auditing is reserved for the whole-milestone pass after all phases are complete.

## In-scope optimization rule

Allow optimizations that are local to touched code and directly improve correctness, structure, proof strength, or performance posture.

Do not allow:
- speculative rewrites
- broad unrelated cleanup
- cross-cutting refactors not required by the phase
- elegance work that delays phase closure without improving the milestone's actual standard

## Prompt blocks

### Phase start

Canonical prompt. Preserve wording except for slot substitution.

```text
Implement Phase {N} of the milestone.

Implement only this phase from {spec_path}.

The governing documents are binding: `MENTALITY.md`, `architectural_guidelines.md`, `domain_standards.md`, and `performance_guidelines.md`.

Before changing code, reread the spec and governing documents relevant to this phase. Then state:
1. the actual phase objective
2. the adversarial constraint this phase must survive
3. the most important architectural risks
4. the concrete implementation sequence you will follow

Then execute the phase.

Do not move to the next phase. Complete only this phase.
```

### User progress report

Use this yourself. Do not delegate this reporting step to Codex.

### Whole milestone QA

Canonical prompt. Preserve wording except for slot substitution.

```text
Perform a whole-milestone hostile QA.

Do not review this as isolated batches. Review it as one integrated milestone. Look for cross-phase inconsistency, broken assumptions, integration weakness, domain leaks, structural drift, performance cliffs, certification gaps, and places where local success masked system-level incompleteness.

Assume every success claim is potentially overstated. Findings first. Zero complacency.
```

### Phase report to user

Adaptable prompt. Preserve substance, not exact wording.

```text
Phase {N} is complete after implementation and hostile QA.

Summary:
- core objective completed
- adversarial constraint addressed
- major implementation decisions
- important QA corrections made
- remaining non-blocking notes, if any

Proceeding to the next phase unless you intervene.
```

### Architectural escalation to user

Canonical prompt. Preserve wording except for slot substitution.

```text
I encountered a non-trivial architectural concern that should not be resolved by silent local iteration.

Issue:
{issue}

Why it is non-trivial:
{governing conflict / structural constraint / performance contradiction / domain ambiguity}

Implications:
{correctness, architecture, performance, scope, or proof consequences}

Options:
1. {option one}
2. {option two}
3. {option three}

Recommended direction:
{recommendation with rationale}

I am stopping here so the direction can be chosen intentionally.
```
