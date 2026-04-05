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
7. Invoke `test-auditor`.
8. If test audit reports findings, send them back to Codex and loop until there are no meaningful gaps.
9. Invoke `architecture-closeout`.
10. If final architecture QA reports findings, send them back to Codex and loop until the structure is clean.
11. Ask Codex to write the milestone closeout document and review it for honesty and completeness.

## Phase completion rule

A phase is complete only when:
- phase-spec requirements are implemented
- architecture still reflects the governing shape
- relevant domain invariants are enforced
- relevant performance constraints are respected or explicitly named as remaining debt
- tests for the phase are materially meaningful
- phase QA reports no findings
- no obvious in-scope cleanup remains

## In-scope optimization rule

Allow optimizations that are local to touched code and directly improve correctness, structure, proof strength, or performance posture.

Do not allow:
- speculative rewrites
- broad unrelated cleanup
- cross-cutting refactors not required by the phase
- elegance work that delays phase closure without improving the milestone's actual standard

## Prompt blocks

### Phase start

```text
Implement Phase {N} of the milestone.

The implementation spec is binding. The governing documents are binding: `MENTALITY.md`, `architectural_guidelines.md`, `domain_standards.md`, and `performance_guidelines.md`.

Before changing code, reread the spec and governing documents relevant to this phase. Then state:
1. the actual phase objective
2. the adversarial constraint this phase must survive
3. the most important architectural risks
4. the concrete implementation sequence you will follow

Then execute the phase.

Do not move to the next phase. Complete only this phase.
```

### Whole milestone QA

```text
Perform a whole-milestone hostile QA.

Do not review this as isolated batches. Review it as one integrated milestone. Look for cross-phase inconsistency, broken assumptions, integration weakness, domain leaks, structural drift, performance cliffs, certification gaps, and places where local success masked system-level incompleteness.

Assume every success claim is potentially overstated. Findings first. Zero complacency.
```

### Phase report to user

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
