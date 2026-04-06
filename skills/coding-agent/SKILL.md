---
name: coding-agent
description: Delegate coding work to Codex through one persistent PTY-backed terminal session. Use when OpenClaw should orchestrate milestone or loop-driven coding work in Codex while preserving strict session continuity, active loop ownership, proactive reporting, and no `codex exec`.
---

# Coding Agent

This workspace override governs how OpenClaw uses Codex for Forge engineering work.

## Primary rule

For milestone or loop-driven coding work, use only regular `codex` in one persistent PTY-backed terminal session.

Do not use `codex exec`.

Do not use one-shot Codex runs for work that requires:
- multiple implementation batches
- QA/correction loops
- continued follow-up prompts
- milestone continuity

## Session rule

Maintain one continuous Codex terminal session for the entire milestone.

OpenClaw must:
- start Codex once in a persistent PTY session
- keep using that same live session across phase work, QA correction loops, and milestone completion
- send follow-up prompts into that same session
- avoid spawning replacement sessions unless a real failure forces a reset

Only replace the Codex session if there is:
- a crash
- terminal corruption
- unrecoverable stuck state
- explicit user instruction

Phase boundaries are management boundaries, not session boundaries.

## Tooling rule

Use:
- PTY-backed terminal execution to launch regular `codex`
- process control actions to drive the same live session over time

Do not use:
- `codex exec`
- fresh Codex launches per phase
- casual respawns after mistakes
- parallel Codex sessions for one milestone

## Launch pattern

Start Codex once:

```bash
bash pty:true workdir:~/project background:true command:"codex --full-auto"
```

Then keep driving that same session:

```bash
process action:submit sessionId:XXX data:"<next prompt>"
process action:poll sessionId:XXX
process action:log sessionId:XXX
```

## Orchestrator rule

OpenClaw owns the loop. Codex does not.

OpenClaw must:
- poll the live Codex session
- read the completed batch carefully
- decide the next step immediately
- send the next prompt into the same live session immediately when more work is needed
- repeat until the current phase is actually closed

Do not mistake “Codex is running” for “the loop is under control.”

## Reporting rule

OpenClaw must report progress proactively without waiting to be asked.

Send the user an update immediately when:
- a Codex loop finishes
- a new Codex loop starts
- a blocker appears
- a phase closes

Each automatic update should include:
- what changed
- exact current test status if known
- whether the phase is actually clean yet
- what next loop was started, if any

Do not wait for the user to ask what happened.

## Recovery rule

If Codex finishes with:
- analysis instead of implementation
- incomplete fixes
- unresolved findings
- environmental blockers that still allow progress

then OpenClaw must immediately send the next corrective prompt into the same live session.

Do not drift into passive status reporting.
Do not stop because the previous batch was informative.
Do not ask the user what to do next unless there is a real blocker or architectural escalation.

## Boundaries

OpenClaw is the manager.
Codex is the coding harness.

Do not tell Codex:
- that skills exist
- that it is inside an OpenClaw workflow
- to notify the orchestrator
- to call `openclaw system event`
- to decide whether to move to the next phase
- to manage the orchestration layer

Codex should only receive the engineering directive for the current step.

## Forge-specific rule

These milestone workflows are governed by:
- `C:\Users\Esther\Documents\Programming\forge_workspace\forge\_docs\coding_guidelines\MENTALITY.md`
- `C:\Users\Esther\Documents\Programming\forge_workspace\forge\_docs\coding_guidelines\architectural_guidelines.md`
- `C:\Users\Esther\Documents\Programming\forge_workspace\forge\_docs\coding_guidelines\domain_standards.md`
- `C:\Users\Esther\Documents\Programming\forge_workspace\forge\_docs\coding_guidelines\performance_guidelines.md`

When coding work is milestone-driven, also follow the workspace manager skills:
- `milestone-executor`
- `phase-executor`
- `phase-qa`
- `test-auditor`
- `architecture-closeout`

These are orchestrator instructions, not Codex task context.

## Forbidden patterns

Never do these for milestone work:
- start with `codex exec`
- move to a fresh Codex session for the next phase
- tell Codex to run QA itself because a manager skill exists
- wait for the user to ask for progress before reporting
- treat retries as cheap and keep spawning replacements

## Canonical operating summary

For milestone work:
1. start one persistent PTY Codex session
2. send the current engineering directive
3. poll and read the result
4. report the result to the user immediately
5. if more work remains, send the next prompt immediately into the same session
6. repeat until the milestone is actually complete
