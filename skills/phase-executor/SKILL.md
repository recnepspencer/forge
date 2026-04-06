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

## Boundary of responsibility

This skill governs how OpenClaw steers Codex. It is not a prompt that Codex should be told to invoke.

Do not tell Codex:
- to use `phase-executor`
- that it is inside an OpenClaw workflow
- to emit orchestration events
- to decide when milestone-level orchestration happens
- to decide when a phase ends
- to decide whether the milestone should advance
- that the skill system exists at all

## Boundary invariants

- These skills are for the orchestrator to be a good manager. They are not part of Codex's task context.
- Codex must never be told about the existence of these skills.
- OpenClaw owns phase boundaries and milestone boundaries.
- OpenClaw owns the decision to continue, recenter, ask for reasoning, or request remaining work.
- Maintain the same Codex terminal/process/session for the milestone.
- Do not create a new Codex process or terminal for each phase.
- Keep sending follow-up messages into the same live Codex session unless a real failure forces a reset.

## Persistent session rule

This skill assumes Codex is already running in a persistent terminal session for the milestone.

Your job is to steer that ongoing session:
- encourage it
- recenter it
- question it
- direct its next batch

Do not replace the session just because a phase boundary was crossed. Phase boundaries are management boundaries, not terminal boundaries.

Do not use `codex exec` for this workflow. Use only a persistent PTY-backed regular Codex session.

## Active ownership rule

Do not lapse into passive status reporting.

Your job is not:
- to wait for the user to ask what happened
- to repeat the last finished batch without taking the next step
- to assume Codex will self-drive the next loop

Your job is:
- to read the completed batch
- to choose the next intervention immediately
- to send that intervention into the same live Codex session
- to report progress automatically

## Automatic reporting rule

When a loop finishes:
1. send the user a concise progress report immediately
2. if more work remains, start the next loop immediately in the same Codex session

Do not wait for the user to ask for the update before reporting it.
Do not wait for the user to ask whether the next loop has started.

## Responsibility

Do not try to force one-shot completion. Your job is to keep Codex aligned, honest, and advancing inside the current phase until the phase is substantively complete and ready for hostile QA.

Most of the time, the right move is simple encouragement plus boundary reminder. Escalate intervention only when the work becomes unclear, drifts, or starts performing false closure.

## Intervention ladder

1. Encourage when the batch was good.
2. Ask for reasoning when the move is non-obvious.
3. Recenter when the work drifts from the plan or governing docs.
4. Ask Codex to identify remaining meaningful work when it sounds done but probably is not.
5. Escalate to the user only if a non-trivial architectural concern survives explanation and recentering.

## Prompting rule

The default intervention is brief encouragement with a boundary reminder.

Do not over-narrate. Do not restate the whole phase plan every turn. Do not pretend Codex can be commanded into one-shot completion. Use the smallest intervention that preserves alignment and momentum.

## Prompt fidelity

This skill contains canonical prompts and adaptable prompts.

Canonical prompts:
- `Recenter`
- `Explain reasoning`
- `Identify remaining work`

Adaptable prompts:
- `Encourage`
- `Encourage with hype`

For canonical prompts:
- preserve wording exactly
- do not soften them
- do not add workflow commentary
- do not wrap them in explanations about OpenClaw

For adaptable prompts:
- preserve intent
- keep them short
- do not turn them into speeches

## Prompts

### Encourage

Adaptable prompt. Preserve substance, not exact wording.

```text
Strong batch. Keep going.

Stay aligned with the phase plan, preserve the governing architecture, and take the next strongest in-phase step. Do not jump ahead.
```

### Encourage with hype

Adaptable prompt. Preserve substance, not exact wording.

```text
That was real progress. Keep going.

You are moving in the right direction. Stay disciplined, stay inside the phase boundary, and keep pushing the strongest remaining in-phase work. Maintain the standard.
```

### Recenter

Canonical prompt. Preserve wording exactly.

```text
Stop and recenter.

Reread the implementation spec, `MENTALITY.md`, and `architectural_guidelines.md`. Then restate the phase objective, what is actually in scope right now, and the next concrete in-phase steps before continuing.
```

### Explain reasoning

Canonical prompt. Preserve wording exactly.

```text
Explain your reasoning.

What are you doing, why is it the correct next move, what authority justifies it, and what are your next steps? Be concrete.
```

### Identify remaining work

Canonical prompt. Preserve wording exactly.

```text
Identify the strongest meaningful work still remaining in this phase.

Do not list cosmetic polish. Focus on the real remaining gaps against the plan, architecture, domain standards, performance standards, and proof strength. Then continue with the strongest remaining in-phase batch.
```

## When to use which prompt

- Use `Encourage` or `Encourage with hype` after a real batch that is still aligned.
- Use `Explain reasoning` when the latest move is ambiguous or structurally suspicious.
- Use `Recenter` when Codex is drifting from the phase boundary, the spec, or the governing docs.
- Use `Identify remaining work` when Codex sounds complete but the phase likely still has real substance left.

## Completion signal

Hand off to `phase-qa` only when the current phase appears substantively complete and Codex is no longer surfacing obvious in-phase batches.
