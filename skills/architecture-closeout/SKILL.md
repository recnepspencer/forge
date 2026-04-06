---
name: architecture-closeout
description: Run final architectural QA on a completed milestone, drive final structural cleanup, and produce an honest milestone closeout. Use when implementation and test work are done and the remaining question is whether the resulting structure is clean, durable, and true to the governing architecture.
---

# Architecture Closeout

Use this skill after milestone implementation and test hardening are materially done.

The implementation spec is binding. The governing documents are binding:
- `C:\Users\Esther\Documents\Programming\forge_workspace\forge\_docs\coding_guidelines\MENTALITY.md`
- `C:\Users\Esther\Documents\Programming\forge_workspace\forge\_docs\coding_guidelines\architectural_guidelines.md`
- `C:\Users\Esther\Documents\Programming\forge_workspace\forge\_docs\coding_guidelines\domain_standards.md`
- `C:\Users\Esther\Documents\Programming\forge_workspace\forge\_docs\coding_guidelines\performance_guidelines.md`

## Boundary of responsibility

OpenClaw owns this final review pass and the decision to accept the milestone structurally.

Do not tell Codex to "run architecture-closeout." Review the resulting structure yourself. Send Codex only the concrete cleanup findings or the closeout-writing request.

These skills are for the orchestrator only. Codex must never be told they exist.

## Prompt fidelity

Both prompts in this skill are canonical.

Preserve wording exactly.
Do not paraphrase them into softer language.
Do not add orchestration commentary.
Do not mention the skill itself to Codex.

## Final architecture QA prompt

Canonical prompt. Preserve wording exactly.

```text
Perform a final architectural QA of the completed milestone.

Look specifically for:
- god files
- duplicated logic
- weak ownership boundaries
- false abstractions
- domain leakage
- interface dishonesty
- convenience structure that violates the governing architecture
- file or module decomposition that does not reflect the true domain and runtime shape

Do not ask whether it merely works. Ask whether the structure is clean, honest, and durable.
```

## Closeout prompt

Canonical prompt. Preserve wording exactly.

```text
Write the milestone closeout document.

Include:
1. milestone objective
2. phase-by-phase implementation summary
3. major design decisions
4. adversarial constraints addressed
5. tests added or strengthened
6. major QA findings and how they were resolved
7. residual risks or deferred items
8. overall assessment against the spec, architecture, domain standards, and performance standards

Write it as an honest engineering closeout, not a victory lap.
```
