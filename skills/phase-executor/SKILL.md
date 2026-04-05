---
name: phase-executor
description: Keep Codex moving inside a single implementation phase without letting it drift, false-close, or jump ahead. Use when one phase requires many back-and-forth implementation batches and OpenClaw needs a small set of high-quality interventions rather than generic project-management narration.
---

# Phase Executor

Use this skill for one phase only.

The implementation spec is binding. The governing documents are binding:
- `C:\Users\Esther\Documents\Programming\forge_workspace\forge\_docs\coding_guidelines\MENTALITY.md`
- `C:\Users\Esther\Documents\Programming\forge_workspace\forge\_docs\coding_guidelines\architectural_guidelines.md`
- `C:\Users\Esther\Documents\Programming\forge_workspace\forge\_docs\coding_guidelines\domain_standards.md`
- `C:\Users\Esther\Documents\Programming\forge_workspace\forge\_docs\coding_guidelines\performance_guidelines.md`

## Responsibility

Do not try to force one-shot completion. Your job is to keep Codex aligned, honest, and advancing inside the current phase until the phase is substantively complete and ready for hostile QA.

Most of the time, the right move is simple encouragement plus boundary reminder. Escalate intervention only when the work becomes unclear, drifts, or starts performing false closure.

## Intervention ladder

1. Encourage when the batch was good.
2. Ask for reasoning when the move is non-obvious.
3. Recenter when the work drifts from the plan or governing docs.
4. Ask Codex to identify remaining meaningful work when it sounds done but probably is not.
5. Escalate to the user only if a non-trivial architectural concern survives explanation and recentering.

## Prompts

### Encourage

```text
Strong batch. Keep going.

Stay aligned with the phase plan, preserve the governing architecture, and take the next strongest in-phase step. Do not jump ahead.
```

### Encourage with hype

```text
That was real progress. Keep going.

You are moving in the right direction. Stay disciplined, stay inside the phase boundary, and keep pushing the strongest remaining in-phase work. Maintain the standard.
```

### Recenter

```text
Stop and recenter.

Reread the implementation spec, `MENTALITY.md`, and `architectural_guidelines.md`. Then restate the phase objective, what is actually in scope right now, and the next concrete in-phase steps before continuing.
```

### Explain reasoning

```text
Explain your reasoning.

What are you doing, why is it the correct next move, what authority justifies it, and what are your next steps? Be concrete.
```

### Identify remaining work

```text
Identify the strongest meaningful work still remaining in this phase.

Do not list cosmetic polish. Focus on the real remaining gaps against the plan, architecture, domain standards, performance standards, and proof strength. Then continue with the strongest remaining in-phase batch.
```

## Completion signal

Hand off to `phase-qa` only when the current phase appears substantively complete and Codex is no longer surfacing obvious in-phase batches.
