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

## Standard

Review as a hostile engineer, not as a celebratory assistant. The bar is production grade. Look for things that technically pass while violating the deeper intent of the milestone or the spirit of the system.

## QA prompt

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

```text
Address these QA findings completely.

Do not negotiate with them, work around them, or minimize them. Correct them in a way that preserves the governing architecture and improves the phase in substance, not appearance.

After corrections, reassess whether any related weakness remains, including small but real ones. Then continue until the phase is ready for another hostile QA pass.
```
