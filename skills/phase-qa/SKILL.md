---
name: phase-qa
description: Run hostile engineering QA on a completed phase and force correction loops until there are no meaningful findings. Use when a phase appears complete and needs rigorous review against the spec, architecture, domain standards, performance standards, and deeper system intent.
---

# Phase QA

Use this skill after implementation of a phase appears complete.

The implementation spec is binding. The governing documents are binding:
- `C:\Users\Esther\Documents\Programming\forge_workspace\forge\_docs\coding_guidelines\MENTALITY.md`
- `C:\Users\Esther\Documents\Programming\forge_workspace\forge\_docs\coding_guidelines\architectural_guidelines.md`
- `C:\Users\Esther\Documents\Programming\forge_workspace\forge\_docs\coding_guidelines\domain_standards.md`
- `C:\Users\Esther\Documents\Programming\forge_workspace\forge\_docs\coding_guidelines\performance_guidelines.md`

## Boundary of responsibility

OpenClaw performs this review.

Do not tell Codex to "run phase-qa" or to invoke this skill. Review the code and tests yourself. Then send Codex only the resulting findings and required corrections.

These skills are for the orchestrator only. Codex must never be told they exist.

Do not perform the dedicated adversarial test audit here. That belongs only to the final milestone-level `test-auditor` pass.

## Prompt fidelity

Both prompts in this skill are canonical.

Preserve their wording exactly except for inserting the actual findings when using the correction loop.
Do not soften them.
Do not add workflow commentary.
Do not mention the skill itself to Codex.

## Standard

Review as a hostile engineer, not as a celebratory assistant. The bar is production grade. Look for things that technically pass while violating the deeper intent of the milestone or the spirit of the system.

At phase level, focus on implementation integrity, architectural integrity, domain correctness, performance posture, and obvious testing omissions that block the phase. Do not do the full personal adversarial test review here.

## QA prompt

Canonical prompt. Preserve wording exactly.

```text
Perform a brutal QA of this phase.

Evaluate the phase against:
- the implementation spec
- `architectural_guidelines.md`
- `domain_standards.md`
- `performance_guidelines.md`
- the spirit of the system vision, not merely literal checkbox compliance

Assume the bar is production-grade. Look for implementation gaps, architectural dishonesty, hidden complexity, semantic weakness, proof weakness, performance blindness, shallow tests, and anything that technically passes while violating the deeper intent.

Report findings first.

For each finding, state:
1. what is wrong
2. why it matters
3. what authority it violates
4. what correction is required

If there are no findings, say so explicitly only after genuinely hostile review.
```

## Correction-loop prompt

Canonical prompt. Preserve wording exactly except for inserting findings.

```text
Address these QA findings completely.

Do not negotiate with them, work around them, or minimize them. Correct them in a way that preserves the governing architecture and improves the phase in substance, not appearance.

After corrections, reassess whether any related weakness remains, including small but real ones. Then continue until the phase is ready for another hostile QA pass.
```

## Output rule

Your review output should be findings-first and concrete. Do not pad the review with celebration, generic praise, or meta commentary about the workflow.
